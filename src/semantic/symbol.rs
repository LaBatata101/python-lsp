use std::ops::{Deref, DerefMut};

use bitflags::bitflags;
use python_ast::AnyNodeRef;
use ruff_index::{newtype_index, IndexSlice, IndexVec};

use super::{
    branches::{Branch, BranchId},
    nodes::NodeId,
    ResolvedReferenceId, ScopeId, SemanticModel,
};

#[newtype_index]
pub struct SymbolId;

#[derive(Debug, Default)]
pub struct Symbols<'a>(IndexVec<SymbolId, Symbol<'a>>);

impl<'a> Symbols<'a> {
    pub fn push(&mut self, symbol: Symbol<'a>) -> SymbolId {
        self.0.push(symbol)
    }
}

impl<'a> Deref for Symbols<'a> {
    type Target = IndexSlice<SymbolId, Symbol<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Symbols<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub enum SymbolKind<'a> {
    Enum,
    Import(Import<'a>),
    Builtin,
    Variable,
    Parameter,
    FromImport(FromImport<'a>),
    Function(ScopeId),
    Class(ScopeId),
    Method(ScopeId),
}

/// A binding for an `import`, keyed on the name to which the import is bound.
/// Ex) `import foo` would be keyed on "foo".
/// Ex) `import foo as bar` would be keyed on "bar".
#[derive(Debug, Clone)]
pub struct Import<'a> {
    /// The full name of the module being imported.
    /// Ex) Given `import foo`, `qualified_name` would be "foo".
    /// Ex) Given `import foo as bar`, `qualified_name` would be "foo".
    pub call_path: Box<[&'a str]>,
}

/// A binding for a member imported from a module, keyed on the name to which the member is bound.
/// Ex) `from foo import bar` would be keyed on "bar".
/// Ex) `from foo import bar as baz` would be keyed on "baz".
#[derive(Debug, Clone)]
pub struct FromImport<'a> {
    /// The full name of the member being imported.
    /// Ex) Given `from foo import bar`, `qualified_name` would be "foo.bar".
    /// Ex) Given `from foo import bar as baz`, `qualified_name` would be "foo.bar".
    pub call_path: Box<[&'a str]>,
}

bitflags! {
    pub struct SymbolFlags: u8 {
        const CONSTANT = 1 << 0;
        const PRIVATE = 1 << 1;
    }
}

#[derive(Debug, Clone)]
pub struct Symbol<'a> {
    pub scope: ScopeId,
    pub name: &'a str,
    pub kind: SymbolKind<'a>,
    pub references: Vec<ResolvedReferenceId>,
    pub flags: SymbolFlags,
    pub node_id: Option<NodeId>,
    pub branch_id: Option<BranchId>,
}

impl<'a> Symbol<'a> {
    pub fn is_constant(&self) -> bool {
        self.flags.intersects(SymbolFlags::CONSTANT)
    }

    pub fn is_builtin(&self) -> bool {
        matches!(self.kind, SymbolKind::Builtin)
    }

    pub fn declaration<'b>(&'b self, semantic: &'b SemanticModel<'b>) -> Option<AnyNodeRef<'b>> {
        self.node_id
            .map(|node_id| {
                if semantic.nodes[node_id].is_expression() {
                    semantic
                        .nodes
                        .parend_id(node_id)
                        .map(|parent_id| semantic.nodes[parent_id])
                } else {
                    Some(semantic.nodes[node_id])
                }
            })
            .flatten()
    }

    pub fn stmt_node<'b>(&'b self, semantic: &'b SemanticModel<'b>) -> Option<AnyNodeRef<'b>> {
        self.node_id.map(|stmt_id| semantic.stmt_node(stmt_id))
    }

    pub fn branch<'b>(&self, semantic: &'b SemanticModel<'b>) -> Option<&'b Branch<'b>> {
        self.branch_id.map(|id| &semantic.branches[id])
    }
}
