use ast::{helpers::collect_import_from_member, ModModule, Pattern};
use python_ast::{self as ast, visitor, visitor::Visitor, Expr, Stmt};

use super::{
    branches::BranchKind, builtins::BUILTINS, FromImport, Import, ScopeKind, SemanticModel,
    SymbolFlags, SymbolId, SymbolKind,
};

pub struct Indexer<'a> {
    pub semantic: SemanticModel<'a>,
}

impl<'a> Indexer<'a> {
    pub fn new(semantic: SemanticModel<'a>) -> Self {
        Self { semantic }
    }

    pub fn semantic(&self) -> &SemanticModel<'a> {
        &self.semantic
    }

    fn add_builtin_symbols(&mut self) {
        for builtin in BUILTINS {
            let symbol_id = self.semantic.push_builtin(builtin);
            let scope = self.semantic.global_scope_mut();
            scope.add_symbol(builtin, symbol_id);
        }
    }

    fn add_symbol(&mut self, name: &'a str, kind: SymbolKind<'a>, flags: SymbolFlags) -> SymbolId {
        let symbol_id = self.semantic.push_symbol(name, kind, flags);

        if name.starts_with('_') {
            self.semantic.symbols[symbol_id].flags |= SymbolFlags::PRIVATE;
        } else if name.chars().all(|char| char.is_uppercase()) {
            self.semantic.symbols[symbol_id].flags |= SymbolFlags::CONSTANT;
        }

        let scope_id = self.semantic.scope_id;
        let scope = &mut self.semantic.scopes[scope_id];

        scope.add_symbol(name, symbol_id);
        symbol_id
    }

    fn handle_node_store(&mut self, id: &'a str) {
        self.add_symbol(id, SymbolKind::Variable, SymbolFlags::empty());
    }

    fn handle_node_load(&mut self, expr: &Expr) {
        let Expr::Name(ident) = expr else { return };
        self.semantic.resolve_load(ident);
    }
}

impl<'a, 'b> Visitor<'b> for Indexer<'a>
where
    'b: 'a,
{
    fn visit_stmt(&mut self, stmt: &'b Stmt) {
        self.semantic.push_node(stmt);

        match stmt {
            Stmt::FunctionDef(ast::FunctionDefStmt {
                name,
                body,
                parameters,
                ..
            }) if name.is_valid() => {
                let scope_id = self.semantic.push_scope(ScopeKind::Function);
                let symbol_kind = if self.semantic.current_scope().kind.is_class() {
                    SymbolKind::Method(scope_id)
                } else {
                    SymbolKind::Function(scope_id)
                };
                self.add_symbol(&name.id, symbol_kind, SymbolFlags::empty());
                self.semantic.scope_id = scope_id;

                self.visit_parameters(parameters);
                self.visit_body(body);

                self.semantic.pop_scope();
            }
            Stmt::ClassDef(ast::ClassDefStmt { name, body, .. }) if name.is_valid() => {
                let scope_id = self.semantic.push_scope(ScopeKind::Class);
                self.add_symbol(&name.id, SymbolKind::Class(scope_id), SymbolFlags::empty());
                self.semantic.scope_id = scope_id;
                self.visit_body(body);
                self.semantic.pop_scope();
            }
            Stmt::Import(ast::ImportStmt { names, .. }) => names
                .iter()
                .filter(|alias| alias.name.is_valid())
                .for_each(|alias| {
                    let name = alias.asname.as_ref().unwrap_or(&alias.name);
                    let call_path: Box<[&str]> = alias.name.split('.').collect();
                    self.add_symbol(
                        &name,
                        SymbolKind::Import(Import { call_path }),
                        SymbolFlags::empty(),
                    );
                }),
            Stmt::ImportFrom(ast::ImportFromStmt {
                names,
                level,
                module,
                ..
            }) => names
                .iter()
                .filter(|alias| alias.name.is_valid())
                .for_each(|alias| {
                    let name = alias.asname.as_ref().unwrap_or(&alias.name);
                    let call_path =
                        collect_import_from_member(*level, module.as_deref(), &alias.name)
                            .into_boxed_slice();
                    self.add_symbol(
                        &name,
                        SymbolKind::FromImport(FromImport { call_path }),
                        SymbolFlags::empty(),
                    );
                }),
            Stmt::If(
                if_stmt @ ast::IfStmt {
                    test,
                    body,
                    elif_else_clauses,
                    ..
                },
            ) => {
                self.visit_expr(test);
                self.semantic.push_branch(BranchKind::If(if_stmt));
                self.visit_body(body);
                self.semantic.pop_branch();

                for clause in elif_else_clauses {
                    self.visit_elif_else_clause(clause);
                    self.semantic.pop_branch();
                }
            }
            _ => visitor::walk_stmt(self, stmt),
        }

        self.semantic.pop_node();
    }

    fn visit_elif_else_clause(&mut self, elif_else_clause: &'b ast::ElifElseClause) {
        if let Some(test) = &elif_else_clause.test {
            self.visit_expr(test);
        }
        self.semantic
            .push_branch(BranchKind::ElifElse(elif_else_clause));
        self.visit_body(&elif_else_clause.body);
    }

    fn visit_expr(&mut self, expr: &'b Expr) {
        self.semantic.push_node(expr);

        match expr {
            Expr::Name(ast::NameExpr { id, ctx, .. }) => match ctx {
                ast::ContextExpr::Load => self.handle_node_load(expr),
                ast::ContextExpr::Store => self.handle_node_store(id),
                _ => {}
            },
            Expr::Lambda(ast::LambdaExpr {
                body, parameters, ..
            }) => {
                let scope_id = self.semantic.push_scope(ScopeKind::Lambda);
                self.semantic.scope_id = scope_id;
                if let Some(parameters) = parameters {
                    self.visit_parameters(parameters);
                }
                self.visit_expr(body);
                self.semantic.pop_scope();
            }
            _ => visitor::walk_expr(self, expr),
        }

        self.semantic.pop_node();
    }

    fn visit_parameters(&mut self, parameters: &'b ast::Parameters) {
        self.semantic.push_node(parameters);
        for parameter_with_default in &parameters.posonlyargs {
            self.semantic.push_node(parameter_with_default);
            self.visit_parameter(&parameter_with_default.parameter);
        }
        for parameter_with_default in &parameters.args {
            self.semantic.push_node(parameter_with_default);
            self.visit_parameter(&parameter_with_default.parameter);
        }
        if let Some(arg) = &parameters.vararg {
            self.visit_parameter(arg);
        }
        for parameter_with_default in &parameters.kwonlyargs {
            self.semantic.push_node(parameter_with_default);
            self.visit_parameter(&parameter_with_default.parameter);
        }
        if let Some(arg) = &parameters.kwarg {
            self.visit_parameter(arg);
        }
    }

    fn visit_parameter(&mut self, parameter: &'b ast::Parameter) {
        self.semantic.push_node(parameter);
        if parameter.name.is_valid() {
            self.add_symbol(&parameter.name, SymbolKind::Parameter, SymbolFlags::empty());
        }
    }

    fn visit_with_item(&mut self, with_item: &'b ast::WithItem) {
        if let Some(target) = with_item.optional_vars.as_ref() {
            self.visit_expr(target);
        }
    }

    fn visit_pattern(&mut self, pattern: &'b ast::Pattern) {
        match pattern {
            Pattern::MatchAs(match_as) => {
                if let Some(name) = match_as.name.as_ref() {
                    if name.is_valid() {
                        self.add_symbol(&name.id, SymbolKind::Variable, SymbolFlags::empty());
                    }
                }

                if let Some(pattern) = match_as.pattern.as_ref() {
                    self.visit_pattern(pattern);
                }
            }
            Pattern::MatchSequence(ast::PatternMatchSequence { patterns, .. })
            | Pattern::MatchOr(ast::PatternMatchOr { patterns, .. }) => {
                for pattern in patterns {
                    self.visit_pattern(pattern);
                }
            }
            Pattern::MatchStar(match_star) => {
                if let Some(name) = match_star.name.as_ref() {
                    if name.is_valid() {
                        self.add_symbol(&name.id, SymbolKind::Variable, SymbolFlags::empty());
                    }
                }
            }
            Pattern::MatchMapping(match_mapping) => {
                for pattern in &match_mapping.patterns {
                    self.visit_pattern(pattern);
                }
                if let Some(name) = match_mapping.rest.as_ref() {
                    if name.is_valid() {
                        self.add_symbol(&name.id, SymbolKind::Variable, SymbolFlags::empty());
                    }
                }
            }
            _ => visitor::walk_pattern(self, pattern),
        }
    }
}

pub fn index_ast<'a>(ast: &'a ModModule) -> Indexer<'a> {
    let mut indexer = Indexer::new(SemanticModel::new());
    indexer.add_builtin_symbols();
    indexer.visit_body(&ast.body);

    indexer
}

#[cfg(test)]
mod tests {
    use python_ast::visitor::Visitor;
    use python_parser::parse_program;

    use crate::semantic::{Indexer, SemanticModel};
    #[test]
    fn test_indexer() {
        let source = "
a = 0
def t(): a
def x(): a
";
        let parsed_file = parse_program(source);
        let mut indexer = Indexer::new(SemanticModel::new());
        indexer.visit_body(&parsed_file.ast.as_module().unwrap().body);
        dbg!(indexer.semantic());
    }
}
