use std::collections::HashMap;

use ast::{
    call_path::{collect_call_path, CallPath},
    AnyNodeRef, Expr,
};
use python_ast::{self as ast};
use ruff_text_size::{Ranged, TextSize};
use smallvec::smallvec;

use crate::semantic::{FromImport, Import};

use super::{
    branches::{Branch, BranchId, BranchKind, Branches},
    nodes::{NodeId, Nodes},
    ResolvedReference, ResolvedReferenceId, ResolvedReferences, Scope, ScopeId, ScopeKind, Scopes,
    Symbol, SymbolFlags, SymbolId, SymbolKind, Symbols,
};

#[derive(Debug)]
pub struct SemanticModel<'a> {
    pub nodes: Nodes<'a>,
    node_id: Option<NodeId>,
    branch_id: Option<BranchId>,
    pub scopes: Scopes<'a>,
    pub scope_id: ScopeId,
    pub symbols: Symbols<'a>,
    resolved_references: ResolvedReferences,
    pub branches: Branches<'a>,
    /// Map from [`ast::NameExpr`] node (represented as a [`NameId`]) to the [`Symbol`] to which
    /// it resolved (represented as a [`SymbolId`]).
    resolved_names: HashMap<NameId, SymbolId>,
}

/// A unique identifier for an [`ast::ExprName`]. No two names can even appear at the same location
/// in the source code, so the starting offset is a cheap and sufficient unique identifier.
#[derive(Debug, Hash, PartialEq, Eq)]
struct NameId(TextSize);

impl From<&ast::NameExpr> for NameId {
    fn from(name: &ast::NameExpr) -> Self {
        Self(name.start())
    }
}

impl<'a> SemanticModel<'a> {
    pub fn new() -> Self {
        SemanticModel {
            node_id: None,
            branch_id: None,
            scope_id: ScopeId::global(),
            symbols: Symbols::default(),
            nodes: Nodes::default(),
            scopes: Scopes::default(),
            resolved_references: ResolvedReferences::default(),
            branches: Branches::default(),
            resolved_names: HashMap::new(),
        }
    }

    pub fn push_node<T: Into<AnyNodeRef<'a>>>(&mut self, node: T) {
        self.node_id = Some(self.nodes.insert(node.into(), self.node_id));
    }

    pub fn pop_node(&mut self) {
        let node_id = self.node_id.expect("Attempt to pop without node");
        self.node_id = self.nodes.parend_id(node_id);
    }

    pub fn push_symbol(
        &mut self,
        name: &'a str,
        kind: SymbolKind<'a>,
        flags: SymbolFlags,
    ) -> SymbolId {
        self.symbols.push(Symbol {
            name,
            kind,
            flags,
            node_id: self.node_id,
            scope: self.scope_id,
            branch_id: self.branch_id,
            references: Vec::new(),
        })
    }

    pub fn push_builtin(&mut self, name: &'a str) -> SymbolId {
        self.symbols.push(Symbol {
            scope: ScopeId::global(),
            name,
            kind: SymbolKind::Builtin,
            references: Vec::new(),
            flags: SymbolFlags::empty(),
            node_id: None,
            branch_id: None,
        })
    }

    pub fn push_scope(&mut self, kind: ScopeKind) -> ScopeId {
        let id = self.scopes.push_scope(kind, self.scope_id);
        // self.scope_id = id;
        id
    }

    pub fn pop_scope(&mut self) {
        self.scope_id = self.scopes[self.scope_id]
            .parent
            .expect("attempted to pop without scope");
    }

    pub fn push_branch(&mut self, kind: BranchKind<'a>) -> Option<BranchId> {
        self.branch_id = Some(self.branches.insert(Branch {
            parent: self.branch_id,
            kind,
        }));
        self.branch_id
    }

    pub fn pop_branch(&mut self) {
        let node_id = self.branch_id.expect("Attempt to pop without branch");
        self.branch_id = self.branches.parent_id(node_id);
    }

    pub fn current_scope(&self) -> &Scope {
        &self.scopes[self.scope_id]
    }

    pub fn resolve_load(&mut self, name: &ast::NameExpr) {
        for scope_id in self.scopes.ancestor_ids(self.scope_id) {
            let scope = &self.scopes[scope_id];
            if let Some(symbol_id) = scope.get_symbol(&name.id) {
                let reference_id = self.resolved_references.push(self.scope_id, name.range);
                let symbol = &mut self.symbols[symbol_id];
                symbol.references.push(reference_id);

                self.resolved_names.insert(name.into(), symbol_id);

                if symbol.branch_id.is_none() && self.branch_id.is_some() {
                    symbol.branch_id = self.branch_id;
                }
            }
        }
    }

    pub fn lookup_symbol(&self, name: &str) -> Option<SymbolId> {
        self.lookup_symbol_in_scope(name, ScopeId::global())
    }

    pub fn lookup_symbol_in_scope(&self, name: &str, scope_id: ScopeId) -> Option<SymbolId> {
        for scope_id in self.scopes.ancestor_ids(scope_id) {
            let scope = &self.scopes[scope_id];
            if let Some(symbol_id) = scope.get_symbol(name) {
                return Some(symbol_id);
            }
        }

        None
    }

    pub fn symbol(&self, symbol_id: SymbolId) -> &Symbol {
        &self.symbols[symbol_id]
    }

    pub fn global_scope(&self) -> &Scope {
        self.scopes.global()
    }

    pub fn global_scope_mut(&mut self) -> &mut Scope<'a> {
        self.scopes.global_mut()
    }

    /// For a given `node_id`, returns an [`AnyNodeRef`] that represents a statement.
    ///
    /// # Panics
    /// If the node referred by `node_id` isn't a statement.
    pub fn stmt_node(&self, node_id: NodeId) -> AnyNodeRef {
        self.nodes
            .ancestor_ids(node_id)
            .find_map(|id| self.nodes[id].is_statement().then_some(self.nodes[id]))
            .expect("No statement found")
    }

    pub fn parent_node(&self) -> Option<&AnyNodeRef<'a>> {
        self.node_id.map(|node_id| {
            self.nodes
                .parend_id(node_id)
                .map(|parent_id| &self.nodes[parent_id])
        })?
    }

    pub fn enclosing_scope(&self, name: &str, offset: TextSize) -> Option<ScopeId> {
        for (id, scope) in self.scopes.iter().enumerate() {
            if let Some(symbol_id) = scope.get_symbol(name) {
                if let Some(stmt) = self.symbol(symbol_id).stmt_node(self) {
                    if stmt.range().contains(offset) {
                        return Some(id.into());
                    }
                }

                // return dbg!(&self.symbol(symbol_id).references)
                //     .iter()
                //     .find_map(|&reference_id| {
                //         dbg!(self.resolved_references[reference_id].range)
                //             .contains(dbg!(offset))
                //             .then(|| self.resolved_references[reference_id].scope_id)
                //     });
            }
        }
        None
    }

    /// Return `true` if the `Expr` is a reference to `typing.${target}`.
    pub fn match_typing_expr(&self, expr: &Expr, target: &str) -> bool {
        self.resolve_call_path(expr)
            .is_some_and(|call_path| self.match_typing_call_path(&call_path, target))
    }

    /// Return `true` if the call path is a reference to `typing.${target}`.
    pub fn match_typing_call_path(&self, call_path: &CallPath, target: &str) -> bool {
        if matches!(
            call_path.as_slice(),
            ["typing" | "_typeshed" | "typing_extensions", member] if *member == target
        ) {
            return true;
        }

        // if self.typing_modules.iter().any(|module| {
        //     let mut module: CallPath = from_unqualified_name(module);
        //     module.push(target);
        //     *call_path == module
        // }) {
        //     return true;
        // }

        false
    }

    /// Resolves the [`ast::NameExpr`] to the [`SymbolId`] of the symbol it refers to, if any.
    pub fn resolve_name(&self, name: &ast::NameExpr) -> Option<SymbolId> {
        self.resolved_names.get(&name.into()).copied()
    }

    /// Resolve the [`ResolvedReference`] for the given [`ResolvedReferenceId`].
    #[inline]
    pub fn reference(&self, id: ResolvedReferenceId) -> &ResolvedReference {
        &self.resolved_references[id]
    }

    /// Resolves the [`Expr`] to a fully-qualified symbol-name, if `value` resolves to an imported
    /// or builtin symbol.
    ///
    /// E.g., given:
    ///
    ///
    /// ```python
    /// from sys import version_info as python_version
    /// print(python_version)
    /// ```
    ///
    /// ...then `resolve_call_path(${python_version})` will resolve to `sys.version_info`.
    pub fn resolve_call_path(&'a self, value: &'a Expr) -> Option<CallPath<'a>> {
        /// Return the [`ast::ExprName`] at the head of the expression, if any.
        const fn match_head(value: &Expr) -> Option<&ast::NameExpr> {
            match value {
                Expr::Attribute(ast::AttributeExpr { value, .. }) => match_head(value),
                Expr::Name(name) => Some(name),
                _ => None,
            }
        }

        // If the name was already resolved, look it up; otherwise, search for the symbol.
        let head = match_head(value)?;
        let binding = self
            .resolve_name(head)
            .or_else(|| self.lookup_symbol(&head.id))
            .map(|id| self.symbol(id))?;

        match &binding.kind {
            SymbolKind::Builtin => {
                if value.is_name_expr() {
                    // Ex) `dict`
                    Some(smallvec!["", head.id.as_str()])
                } else {
                    // Ex) `dict.__dict__`
                    let value_path = collect_call_path(value)?;
                    Some(
                        std::iter::once("")
                            .chain(value_path.iter().copied())
                            .collect(),
                    )
                }
            }
            SymbolKind::FromImport(FromImport { call_path }) => {
                let value_path = collect_call_path(value)?;
                let (_, tail) = value_path.split_first()?;

                let resolved: CallPath =
                    if call_path.first().map_or(false, |segment| *segment == ".") {
                        todo!("add module path to semantic model");
                        // from_relative_import(self.module_path?, call_path, tail)?
                    } else {
                        call_path.iter().chain(tail.iter()).copied().collect()
                    };
                Some(resolved)
            }
            SymbolKind::Import(Import { call_path }) => {
                let value_path = collect_call_path(value)?;
                let (_, tail) = value_path.split_first()?;
                let resolved: CallPath = call_path.iter().chain(tail.iter()).copied().collect();
                Some(resolved)
            }
            SymbolKind::Function(_) | SymbolKind::Class(_) | SymbolKind::Method(_) => {
                todo!()
                // let value_path = collect_call_path(value)?;
                // let resolved: CallPath = self
                //     .module_path?
                //     .iter()
                //     .map(String::as_str)
                //     .chain(value_path)
                //     .collect();
                // Some(resolved)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use python_ast::visitor::Visitor;
    use python_parser::parse_program;

    use crate::semantic::{Indexer, SemanticModel};

    #[test]
    fn test_semantic_model() {
        let source = r#"
class Test:
    def foo(self):
        ...

def bar(): ...
a = bar()
a
b = Test()
"#;
        let parsed_file = parse_program(source);
        let mut indexer = Indexer::new(SemanticModel::new());
        indexer.visit_body(&parsed_file.ast.as_module().unwrap().body);
        assert_debug_snapshot!(indexer.semantic());
    }
}
