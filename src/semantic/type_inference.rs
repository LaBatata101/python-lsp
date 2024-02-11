use ast::{
    visitor::{self, Visitor},
    AnyNodeRef, BoolOp,
};
use itertools::Itertools;
use python_ast::{self as ast, Expr, Stmt};
use ruff_text_size::Ranged;

use crate::semantic::SymbolKind;

use super::{branches::BranchKind, ScopeId, SemanticModel, Symbol, SymbolId};

// TODO: infer types from `typing` module

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum PythonType {
    String,
    Bytes,
    None,
    Ellipsis,
    Dict {
        key: Box<ResolvedType>,
        value: Box<ResolvedType>,
    },
    List(Box<ResolvedType>),
    Set(Box<ResolvedType>),
    Tuple(Box<ResolvedType>),
    Bool,
    Int,
    Float,
    Complex,
    Class(SymbolId),
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum ResolvedType {
    KnownType(PythonType),
    Union(Vec<ResolvedType>),
    AnyType,
    Unknown,
}

fn resolve_symbol_type_in_scope(
    name: &str,
    scope_id: ScopeId,
    semantic: &SemanticModel,
) -> Option<ResolvedType> {
    semantic
        .lookup_symbol_in_scope(name, scope_id)
        .map(|symbol_id| {
            let symbol = semantic.symbol(symbol_id);

            if symbol.is_builtin() {
                return resolve_builtin_type(name);
            }
            let Some(declaration) = symbol.declaration(semantic) else {
                // Symbol hasn't been declared
                return ResolvedType::Unknown;
            };

            symbol
                .branch(semantic)
                .and_then(|branch| get_branch_constrain(branch.kind))
                .and_then(|constrain| constrain.resolve_type(symbol, semantic))
                .unwrap_or_else(|| resolve_declaration_type(&declaration, semantic, scope_id))
        })
}

enum Constrain<'a> {
    IsInstance(&'a Expr),
}

impl<'a> Constrain<'a> {
    fn resolve_type(&self, symbol: &Symbol, semantic: &SemanticModel) -> Option<ResolvedType> {
        match self {
            Constrain::IsInstance(constrain) => {
                let args = constrain.as_call_expr().unwrap();
                let object = &args.arguments.args[0];
                let type_ = &args.arguments.args[1];

                if matches!(object, Expr::Name(ast::NameExpr {id, ..}) if id != symbol.name) {
                    return None;
                }

                let ty = if let Expr::Tuple(ast::TupleExpr { elts, .. }) = type_ {
                    ResolvedType::Union(
                        elts.iter()
                            .map(|expr| resolve_expr_type(expr, semantic))
                            .collect_vec(),
                    )
                } else {
                    resolve_expr_type(type_, semantic)
                };

                Some(ty)
            }
        }
    }
}

fn get_branch_constrain(branch_kind: BranchKind) -> Option<Constrain> {
    let constrain_expr = match branch_kind {
        BranchKind::If(ast::IfStmt { test, .. }) => Some(test.as_ref()),
        BranchKind::ElifElse(ast::ElifElseClause {
            test: Some(expr), ..
        }) => Some(expr),
        _ => return None,
    };

    constrain_expr
        .and_then(|expr| is_valid_isinstance_call(expr).then(|| Constrain::IsInstance(expr)))
}

fn is_valid_isinstance_call(expr: &Expr) -> bool {
    if let Expr::Call(ast::CallExpr {
        func, arguments, ..
    }) = expr
    {
        if arguments.len() < 2 {
            return false;
        }
        matches!(func.as_ref(), Expr::Name(ast::NameExpr {id, ..}) if id == "isinstance")
    } else {
        false
    }
}

fn is_expr_builtin_type(expr: &Expr) -> bool {
    match expr {
        Expr::Name(ast::NameExpr { id, .. }) => {
            matches!(
                id.as_str(),
                "set" | "list" | "dict" | "tuple" | "bool" | "str" | "int" | "float" | "complex"
            )
        }
        Expr::Tuple(ast::TupleExpr { elts, .. }) => {
            elts.iter().take(2).any(|expr| is_expr_builtin_type(expr))
        }
        Expr::Subscript(ast::SubscriptExpr { value, .. }) => is_expr_builtin_type(value),
        _ => false,
    }
}

fn resolve_builtin_type(name: &str) -> ResolvedType {
    match name {
        "set" => ResolvedType::KnownType(PythonType::Set(Box::new(ResolvedType::Unknown))),
        "list" => ResolvedType::KnownType(PythonType::List(Box::new(ResolvedType::Unknown))),
        "dict" => ResolvedType::KnownType(PythonType::Dict {
            key: Box::new(ResolvedType::Unknown),
            value: Box::new(ResolvedType::Unknown),
        }),
        "tuple" => ResolvedType::KnownType(PythonType::Tuple(Box::new(ResolvedType::Unknown))),

        "bool" => ResolvedType::KnownType(PythonType::Bool),
        "str" => ResolvedType::KnownType(PythonType::String),
        "int" => ResolvedType::KnownType(PythonType::Int),
        "float" => ResolvedType::KnownType(PythonType::Float),
        "complex" => ResolvedType::KnownType(PythonType::Complex),
        // class if class.chars().next().is_some_and(|char| char.is_uppercase()) => {
        //     ResolvedPythonType::KnownType(PythonType::Class(symbol_id))
        // }
        _ => ResolvedType::Unknown,
    }
}

fn resolve_expr_type_in_scope(
    expr: &Expr,
    semantic: &SemanticModel,
    scope_id: ScopeId,
) -> ResolvedType {
    match expr {
        Expr::NoneLiteral(_)
        | Expr::BooleanLiteral(_)
        | Expr::StringLiteral(_)
        | Expr::FString(_)
        | Expr::BytesLiteral(_)
        | Expr::NumberLiteral(_) => infer_literal_type(expr),
        // TODO: should we check if there's any `Unknown` type in `Compare` and return `Unknown`?
        Expr::Compare(_) => ResolvedType::KnownType(PythonType::Bool),
        Expr::Name(ast::NameExpr { id, range, .. }) => {
            resolve_symbol_type_in_scope(id, scope_id, semantic)
                .or_else(|| {
                    semantic
                        .enclosing_scope(&id, range.start())
                        .map(|scope_id| resolve_symbol_type_in_scope(&id, scope_id, semantic))?
                })
                .unwrap_or(ResolvedType::Unknown)
        }
        Expr::Call(ast::CallExpr { func, .. }) => {
            // TODO: Use the arguments passed to the function call to help infer the type
            // of the function more precisely
            resolve_expr_type_in_scope(func, semantic, scope_id)
        }
        Expr::Attribute(ast::AttributeExpr { value, attr, .. }) if attr.is_valid() => {
            if let ResolvedType::KnownType(resolved_type) =
                resolve_expr_type_in_scope(value, semantic, scope_id)
            {
                return match resolved_type {
                    PythonType::Class(symbol_id) => {
                        let symbol = semantic.symbol(symbol_id);
                        let SymbolKind::Class(scope_id) = symbol.kind else {
                            unreachable!()
                        };

                        resolve_symbol_type_in_scope(&attr.id, scope_id, semantic)
                            .unwrap_or(ResolvedType::Unknown)
                    }
                    _ => ResolvedType::Unknown,
                };
            }
            ResolvedType::AnyType
        }
        Expr::BinOp(ast::BinOpExpr {
            op: ast::Operator::BitOr,
            ..
        }) => {
            let mut exprs = Vec::new();
            traverse_union(
                &mut |expr, _| {
                    exprs.push(expr);
                },
                semantic,
                expr,
            );

            resolve_maybe_union_type(exprs.iter().map(|&expr| expr), semantic, scope_id)
        }
        Expr::BinOp(ast::BinOpExpr { left, right, .. }) => {
            resolve_bin_op_type(left, right, semantic, scope_id)
        }
        Expr::BoolOp(ast::BoolOpExpr {
            op: BoolOp::Or,
            values,
            ..
        }) => resolve_maybe_union_type(values.iter(), semantic, scope_id),
        Expr::BoolOp(_) => ResolvedType::KnownType(PythonType::Bool),
        Expr::List(ast::ListExpr { elts, .. }) => {
            let elements_type = resolve_maybe_union_type(elts.iter(), semantic, scope_id);
            ResolvedType::KnownType(PythonType::List(Box::new(elements_type)))
        }
        Expr::Dict(ast::DictExpr { keys, values, .. }) => {
            let key = resolve_maybe_union_type(
                keys.iter().filter_map(|expr| expr.as_ref()),
                semantic,
                scope_id,
            );
            let value = resolve_maybe_union_type(values.iter(), semantic, scope_id);
            ResolvedType::KnownType(PythonType::Dict {
                key: Box::new(key),
                value: Box::new(value),
            })
        }
        Expr::Set(ast::SetExpr { elts, .. }) => {
            let elements_type = resolve_maybe_union_type(elts.iter(), semantic, scope_id);
            ResolvedType::KnownType(PythonType::Set(Box::new(elements_type)))
        }
        Expr::Tuple(ast::TupleExpr { elts, .. }) => {
            let elements_type = resolve_maybe_union_type(elts.iter(), semantic, scope_id);
            ResolvedType::KnownType(PythonType::Tuple(Box::new(elements_type)))
        }
        Expr::Subscript(ast::SubscriptExpr { value, slice, .. }) => {
            resolve_subscript_type(value, slice, semantic, scope_id)
        }
        _ => ResolvedType::Unknown,
    }
}

fn is_typing(expr: &Expr, semantic: &SemanticModel) -> bool {
    match expr {
        Expr::Name(ast::NameExpr { id, .. }) => semantic.match_typing_expr(expr, &id),
        Expr::Attribute(ast::AttributeExpr { attr, .. }) => semantic.match_typing_expr(expr, &attr),
        _ => false,
    }
}

fn map_typing_to_resolved_type(expr: &Expr) -> ResolvedType {
    let name = match expr {
        Expr::Name(ast::NameExpr { id, .. }) => id.as_str(),
        Expr::Attribute(ast::AttributeExpr { attr, .. }) => attr.as_str(),
        _ => unreachable!(),
    };

    match name {
        "List" => ResolvedType::KnownType(PythonType::List(Box::new(ResolvedType::Unknown))),
        "Union" => ResolvedType::Union(vec![]),
        _ => ResolvedType::Unknown,
    }
}

fn resolve_subscript_type(
    value: &Expr,
    slice: &Expr,
    semantic: &SemanticModel,
    scope_id: ScopeId,
) -> ResolvedType {
    let mut value_type = if is_expr_builtin_type(value) {
        resolve_expr_type_in_scope(value, semantic, scope_id)
    } else if is_typing(value, semantic) {
        map_typing_to_resolved_type(value)
    } else {
        ResolvedType::Unknown
    };

    if matches!(
        value_type,
        ResolvedType::KnownType(
            PythonType::Set(_)
                | PythonType::List(_)
                | PythonType::Dict { .. }
                | PythonType::Tuple(_)
        )
    ) {
        let mut slice_type = if is_expr_builtin_type(slice) {
            if let Expr::Tuple(ast::TupleExpr { elts, .. }) = slice {
                if elts.len() == 1 {
                    resolve_expr_type_in_scope(&elts[0], semantic, scope_id)
                } else {
                    ResolvedType::Union(
                        elts.iter()
                            .map(|expr| resolve_expr_type_in_scope(expr, semantic, scope_id))
                            .collect(),
                    )
                }
            } else {
                resolve_expr_type_in_scope(slice, semantic, scope_id)
            }
        } else {
            return value_type;
        };

        match &mut value_type {
            ResolvedType::KnownType(
                PythonType::Set(inner) | PythonType::List(inner) | PythonType::Tuple(inner),
            ) => {
                std::mem::swap(inner.as_mut(), &mut slice_type);
            }
            ResolvedType::KnownType(PythonType::Dict {
                key: key_type,
                value: value_type,
            }) => match &mut slice_type {
                ty @ ResolvedType::KnownType(_) => {
                    std::mem::swap(ty, key_type);
                }
                ResolvedType::Union(types) => {
                    let mut types = types.iter_mut();
                    if let Some(ty) = types.next() {
                        std::mem::swap(ty, key_type);
                    }
                    if let Some(ty) = types.next() {
                        std::mem::swap(ty, value_type);
                    }
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    value_type
}

pub fn resolve_expr_type(expr: &Expr, semantic: &SemanticModel) -> ResolvedType {
    resolve_expr_type_in_scope(expr, semantic, ScopeId::global())
}

fn resolve_bin_op_type(
    lhs: &Expr,
    rhs: &Expr,
    semantic: &SemanticModel,
    scope_id: ScopeId,
) -> ResolvedType {
    let type1 = resolve_expr_type_in_scope(lhs, semantic, scope_id);
    let type2 = resolve_expr_type_in_scope(rhs, semantic, scope_id);

    match (type1, type2) {
        (ResolvedType::KnownType(PythonType::Int), ResolvedType::KnownType(PythonType::Int)) => {
            ResolvedType::KnownType(PythonType::Int)
        }

        (
            ResolvedType::KnownType(PythonType::Int | PythonType::Float),
            ResolvedType::KnownType(PythonType::Float),
        ) => ResolvedType::KnownType(PythonType::Float),

        (ResolvedType::KnownType(PythonType::Float), ResolvedType::KnownType(PythonType::Int)) => {
            ResolvedType::KnownType(PythonType::Float)
        }

        (
            ResolvedType::KnownType(PythonType::Int | PythonType::Float | PythonType::Complex),
            ResolvedType::KnownType(PythonType::Complex),
        ) => ResolvedType::KnownType(PythonType::Complex),

        (
            ResolvedType::KnownType(PythonType::Complex),
            ResolvedType::KnownType(PythonType::Int | PythonType::Float),
        ) => ResolvedType::KnownType(PythonType::Complex),

        (_, ResolvedType::AnyType) => ResolvedType::AnyType,
        (ResolvedType::AnyType, _) => ResolvedType::AnyType,

        (_, ResolvedType::Unknown) => ResolvedType::Unknown,
        (ResolvedType::Unknown, _) => ResolvedType::Unknown,
        _ => ResolvedType::Unknown,
    }
}

// TODO: remove Declaration type
fn resolve_declaration_type(
    declaration: &AnyNodeRef,
    semantic: &SemanticModel,
    scope_id: ScopeId,
) -> ResolvedType {
    match declaration {
        AnyNodeRef::StmtImport(_) => {
            // TODO: need multi-file analysis for this
            ResolvedType::Unknown
        }
        AnyNodeRef::StmtAssign(ast::AssignStmt { value, .. }) => {
            resolve_expr_type_in_scope(value, semantic, scope_id)
        }
        AnyNodeRef::StmtAnnAssign(ast::AnnAssignStmt { annotation, .. }) => {
            resolve_expr_type_in_scope(annotation, semantic, scope_id)
        }
        AnyNodeRef::StmtClassDef(ast::ClassDefStmt { name, .. }) => semantic
            .lookup_symbol_in_scope(&name.id, scope_id)
            .map(|symbol_id| ResolvedType::KnownType(PythonType::Class(symbol_id)))
            .unwrap_or(ResolvedType::Unknown),
        AnyNodeRef::ParameterWithDefault(ast::ParameterWithDefault {
            default, parameter, ..
        }) => {
            if let Some(ref expr) = parameter.annotation {
                resolve_expr_type_in_scope(expr, semantic, scope_id)
            } else if let Some(ref expr) = default {
                resolve_expr_type_in_scope(expr, semantic, scope_id)
            } else {
                ResolvedType::AnyType
            }
        }
        AnyNodeRef::Parameter(ast::Parameter { annotation, .. }) => {
            let Some(ref expr) = annotation else {
                return ResolvedType::AnyType;
            };

            resolve_expr_type_in_scope(expr, semantic, scope_id)
        }
        AnyNodeRef::StmtFunctionDef(ast::FunctionDefStmt {
            name,
            returns,
            body,
            ..
        }) => {
            let symbol_kind =
                semantic.symbol(semantic.lookup_symbol_in_scope(&name.id, scope_id).unwrap());

            let scope_id = match symbol_kind.kind {
                SymbolKind::Function(scope_id) | SymbolKind::Method(scope_id) => scope_id,
                _ => unreachable!(),
            };

            if let Some(returns) = returns {
                return resolve_expr_type(returns, semantic);
            }

            let mut return_collector = ReturnVisitor::default();
            return_collector.visit_body(body);

            if !return_collector.returns.is_empty() {
                return resolve_maybe_union_type(
                    return_collector.returns.iter().map(|&expr| expr),
                    semantic,
                    scope_id,
                );
            }

            // Functions in Python implicitly return `None` when there's no `return`
            // statement.
            ResolvedType::KnownType(PythonType::None)
        }
        _ => todo!("{:?}", declaration),
    }
}

fn resolve_maybe_union_type<'a>(
    exprs: impl Iterator<Item = &'a Expr>,
    semantic: &SemanticModel,
    scope_id: ScopeId,
) -> ResolvedType {
    let mut has_different_types = false;
    let mut types = Vec::new();

    for type_ in exprs.map(|expr| resolve_expr_type_in_scope(&expr, semantic, scope_id)) {
        if !types.contains(&type_) {
            if types.last().is_some_and(|last_type| type_ != *last_type) {
                has_different_types = true;
            }
            types.push(type_);
        }
    }

    if has_different_types {
        ResolvedType::Union(types)
    } else {
        types.pop().unwrap_or(ResolvedType::Unknown)
    }
}

fn infer_literal_type(literal: &Expr) -> ResolvedType {
    let python_type = match literal {
        Expr::NoneLiteral(_) => PythonType::None,
        Expr::BooleanLiteral(_) => PythonType::Bool,
        Expr::StringLiteral(_) | Expr::FString(_) => PythonType::String,
        Expr::BytesLiteral(_) => PythonType::Bytes,
        Expr::EllipsisLiteral(_) => PythonType::Ellipsis,
        Expr::NumberLiteral(ast::NumberLiteralExpr { value, .. }) => match value {
            ast::Number::Int(_) => PythonType::Int,
            ast::Number::Float(_) => PythonType::Float,
            ast::Number::Complex { .. } => PythonType::Complex,
        },
        _ => unreachable!("expression is not a literal"),
    };

    ResolvedType::KnownType(python_type)
}

#[derive(Default)]
struct ReturnVisitor<'a> {
    returns: Vec<&'a Expr>,
}

impl<'a, 'b> Visitor<'b> for ReturnVisitor<'a>
where
    'b: 'a,
{
    fn visit_stmt(&mut self, stmt: &'b Stmt) {
        match stmt {
            Stmt::Return(ast::ReturnStmt {
                value: Some(expr), ..
            }) => self.returns.push(expr),
            _ => visitor::walk_stmt(self, stmt),
        }
    }

    fn visit_expr(&mut self, expr: &'b Expr) {
        match expr {
            Expr::Yield(ast::YieldExpr {
                value: Some(expr), ..
            }) => self.returns.push(expr),
            _ => visitor::walk_expr(self, expr),
        }
    }
}

pub fn traverse_union<'a, F>(func: &mut F, semantic: &SemanticModel, expr: &'a Expr)
where
    F: FnMut(&'a Expr, &'a Expr),
{
    fn inner<'a, F>(
        func: &mut F,
        semantic: &SemanticModel,
        expr: &'a Expr,
        parent: Option<&'a Expr>,
    ) where
        F: FnMut(&'a Expr, &'a Expr),
    {
        // Ex) x | y
        if let Expr::BinOp(ast::BinOpExpr {
            op: ast::Operator::BitOr,
            left,
            right,
            range: _,
        }) = expr
        {
            // The union data structure usually looks like this:
            //  a | b | c -> (a | b) | c
            //
            // However, parenthesized expressions can coerce it into any structure:
            //  a | (b | c)
            //
            // So we have to traverse both branches in order (left, then right), to report members
            // in the order they appear in the source code.

            // Traverse the left then right arms
            inner(func, semantic, left, Some(expr));
            inner(func, semantic, right, Some(expr));
            return;
        }

        // Ex) Union[x, y]
        // if let Expr::Subscript(ast::ExprSubscript { value, slice, .. }) = expr {
        //     if semantic.match_typing_expr(value, "Union") {
        //         if let Expr::Tuple(ast::ExprTuple { elts, .. }) = slice.as_ref() {
        //             // Traverse each element of the tuple within the union recursively to handle cases
        //             // such as `Union[..., Union[...]]
        //             elts.iter()
        //                 .for_each(|elt| inner(func, semantic, elt, Some(expr)));
        //             return;
        //         }
        //     }
        // }

        // Otherwise, call the function on expression, if it's not the top-level expression.
        if let Some(parent) = parent {
            func(expr, parent);
        }
    }

    inner(func, semantic, expr, None);
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use python_ast::{
        visitor::Visitor, ConversionFlag, Expr, FStringElement, FStringPart, ModModule, Number,
    };
    use python_parser::parse_program;

    use super::resolve_expr_type;
    use crate::semantic::{
        index_ast,
        type_inference::{PythonType, ResolvedType},
    };

    fn expr_to_str(expr: &Expr) -> String {
        let mut out = String::new();
        match expr {
            Expr::Call(call) => {
                out.push_str(&expr_to_str(&call.func));
                out.push('(');
                for arg in &call.arguments.args {
                    out.push_str(&expr_to_str(&arg));
                }
                out.push(')');
            }
            Expr::Name(name) => {
                out.push_str(&name.id);
            }
            Expr::Attribute(attr) => {
                out.push_str(&expr_to_str(&attr.value));
                out.push('.');
                out.push_str(&attr.attr);
            }
            Expr::NoneLiteral(_) => out.push_str("None"),
            Expr::EllipsisLiteral(_) => out.push_str("..."),
            Expr::BooleanLiteral(boolean) => {
                if boolean.value {
                    out.push_str("True");
                } else {
                    out.push_str("False");
                }
            }
            Expr::FString(fstring) => {
                out.push_str("f\"");
                for part in fstring.value.iter() {
                    match part {
                        FStringPart::Literal(str) => out.push_str(&str.value),
                        FStringPart::FString(fstr) => {
                            for element in fstr.elements.iter() {
                                match element {
                                    FStringElement::Literal(literal) => {
                                        out.push_str(&literal.value)
                                    }
                                    FStringElement::Expression(expr) => {
                                        // TODO: handle  format spec
                                        out.push('{');
                                        if let Some(debug) = &expr.debug_text {
                                            out.push_str(&debug.leading);
                                            out.push_str(&expr_to_str(&expr.expression));
                                            out.push_str(&debug.trailing);
                                        } else {
                                            out.push_str(&expr_to_str(&expr.expression));
                                        }

                                        match expr.conversion {
                                            ConversionFlag::None => {}
                                            ConversionFlag::Str => out.push_str("!s"),
                                            ConversionFlag::Ascii => out.push_str("!a"),
                                            ConversionFlag::Repr => out.push_str("!r"),
                                        }
                                        out.push('}');
                                    }
                                    FStringElement::Invalid(invalid) => {
                                        out.push_str(&invalid.value)
                                    }
                                }
                            }
                        }
                    }
                }
                out.push('"');
            }
            Expr::StringLiteral(str) => {
                out.push('"');
                out.push_str(str.value.to_str());
                out.push('"');
            }
            Expr::BytesLiteral(bytes) => {
                out.push_str("b\"");
                let bytes_str: String = bytes
                    .value
                    .iter()
                    .flat_map(|byte| &byte.value)
                    .map(|byte| *byte as char)
                    .collect();
                out.push_str(&bytes_str);
                out.push('"');
            }
            Expr::NumberLiteral(nbr) => match &nbr.value {
                Number::Int(int) => out.push_str(&format!("{int}")),
                Number::Float(float) => out.push_str(&float.to_string()),
                Number::Complex { .. } => todo!(),
            },
            Expr::BinOp(bin_op) => {
                let op_str = match bin_op.op {
                    python_ast::Operator::Add => "+",
                    python_ast::Operator::Sub => "-",
                    python_ast::Operator::Mult => "*",
                    python_ast::Operator::MatMult => "@",
                    python_ast::Operator::Div => "/",
                    python_ast::Operator::Mod => "%",
                    python_ast::Operator::Pow => "**",
                    python_ast::Operator::LShift => "<<",
                    python_ast::Operator::RShift => ">>",
                    python_ast::Operator::BitOr => "|",
                    python_ast::Operator::BitXor => "^",
                    python_ast::Operator::BitAnd => "&",
                    python_ast::Operator::FloorDiv => "//",
                };
                out.push_str(&expr_to_str(&bin_op.left));
                out.push(' ');
                out.push_str(op_str);
                out.push(' ');
                out.push_str(&expr_to_str(&bin_op.right));
            }
            Expr::BoolOp(bool_op) => {
                let _op_str = match bool_op.op {
                    python_ast::BoolOp::And => "and",
                    python_ast::BoolOp::Or => "or",
                };
                todo!()
            }
            Expr::List(list) => {
                out.push('[');
                for elt in &list.elts {
                    out.push_str(&expr_to_str(elt));
                }
                out.push(']');
            }
            _ => panic!("Not supported! {expr:?}"),
        }
        out
    }

    #[derive(Default)]
    struct InferTypeVisitor<'a> {
        infer_type_calls: Vec<&'a python_ast::CallExpr>,
    }

    impl<'a> Visitor<'a> for InferTypeVisitor<'a> {
        fn visit_expr(&mut self, expr: &'a Expr) {
            match expr {
                Expr::Call(call) if matches!(call.func.as_ref(), Expr::Name(python_ast::NameExpr {id, ..}) if id == "infer_type") =>
                {
                    self.infer_type_calls.push(call);
                }
                _ => (),
            }
        }
    }

    fn collect_types_to_infer(ast: &ModModule) -> Vec<&Expr> {
        let mut infer_type_visitor = InferTypeVisitor::default();
        infer_type_visitor.visit_body(&ast.body);

        infer_type_visitor
            .infer_type_calls
            .iter()
            .flat_map(|call| &call.arguments.args)
            .collect()
    }

    fn test_types(source: &str, expected_types: Vec<ResolvedType>) {
        let parsed_file = parse_program(source);
        let ast = parsed_file.ast.as_module().unwrap();
        let types_to_infer = collect_types_to_infer(ast);
        let indexer = index_ast(ast);

        assert!(!types_to_infer.is_empty(), "No `infer_type` calls found!");

        let types = types_to_infer
            .iter()
            .map(|type_to_infer| resolve_expr_type(type_to_infer, indexer.semantic()))
            .collect_vec();
        assert!(
            types.len() == expected_types.len(),
            "Diff in expected types:\n\tInferred types: {:?}\n\tExpected types: {:?}",
            types,
            expected_types
        );

        for ((ty, expected_type), expression) in types
            .into_iter()
            .zip(expected_types.into_iter())
            .zip(types_to_infer.iter())
        {
            assert_eq!(
                ty,
                expected_type,
                "for expression `{}`",
                expr_to_str(expression)
            );
        }
    }

    #[test]
    fn test_type_infer1() {
        let source = r#"
class Clazz:
 def foo(): 
    if x:
        return 1
    elif y:
        return 2
    else:
        return "string"

c = Clazz()

def bar() -> Clazz: ...
def foobar(a): return a

infer_type(c.foo())
infer_type(bar())
infer_type(foobar(42))
"#;
        test_types(
            source,
            vec![
                ResolvedType::Union(vec![
                    ResolvedType::KnownType(PythonType::Int),
                    ResolvedType::KnownType(PythonType::String),
                ]),
                ResolvedType::KnownType(PythonType::Class(157u32.into())),
                ResolvedType::AnyType,
            ],
        );
    }

    #[test]
    fn test_type_infer2() {
        let source = r#"
def foo(a: int, y: bool): 
    infer_type(a)
"#;
        test_types(source, vec![ResolvedType::KnownType(PythonType::Int)]);
    }

    #[test]
    fn test_type_infer3() {
        let source = r#"
class Test:
    ...

a = True
y = 0

if isinstance(a, int):
    infer_type(a)
elif isinstance(y, Test):
    infer_type(y)
"#;
        // TODO: semantic model doesnt support the same symbol in different branches
        test_types(
            source,
            vec![
                ResolvedType::KnownType(PythonType::Int),
                ResolvedType::KnownType(PythonType::Class(157u32.into())),
            ],
        );
    }

    #[test]
    fn test_type_infer4() {
        // TODO: add type for Ellipsis
        let source = r#"
infer_type(1, 1.0, 1e5j)
infer_type("hello world!", b"hello bytes!")
infer_type(True)
infer_type(None)
infer_type(...)
"#;
        test_types(
            source,
            vec![
                ResolvedType::KnownType(PythonType::Int),
                ResolvedType::KnownType(PythonType::Float),
                ResolvedType::KnownType(PythonType::Complex),
                ResolvedType::KnownType(PythonType::String),
                ResolvedType::KnownType(PythonType::Bytes),
                ResolvedType::KnownType(PythonType::Bool),
                ResolvedType::KnownType(PythonType::None),
                ResolvedType::Unknown,
            ],
        );
    }

    #[test]
    fn test_type_infer5() {
        let source = r#"
infer_type(1 < 2)
infer_type(a and b)
infer_type(a < b and c >= f)
infer_type(a < b or c >= f)
infer_type(1 or None)
infer_type(1 or 2.5 or None)
"#;
        test_types(
            source,
            vec![
                ResolvedType::KnownType(PythonType::Bool),
                ResolvedType::KnownType(PythonType::Bool),
                ResolvedType::KnownType(PythonType::Bool),
                ResolvedType::KnownType(PythonType::Bool),
                ResolvedType::Union(vec![
                    ResolvedType::KnownType(PythonType::Int),
                    ResolvedType::KnownType(PythonType::None),
                ]),
                ResolvedType::Union(vec![
                    ResolvedType::KnownType(PythonType::Int),
                    ResolvedType::KnownType(PythonType::Float),
                    ResolvedType::KnownType(PythonType::None),
                ]),
            ],
        );
    }

    #[test]
    fn test_type_infer6() {
        let source = r#"
infer_type(1 + 2, 1. + 2.4, 1e5J // 1j)
infer_type(1.0 ** 42, 1J / 3.14, 1j + 6)
infer_type(None + 0)

a = 1.0
infer_type(5 * a)

def f(a): return a
infer_type(f(77) - 5)
"#;
        test_types(
            source,
            vec![
                ResolvedType::KnownType(PythonType::Int),
                ResolvedType::KnownType(PythonType::Float),
                ResolvedType::KnownType(PythonType::Complex),
                ResolvedType::KnownType(PythonType::Float),
                ResolvedType::KnownType(PythonType::Complex),
                ResolvedType::KnownType(PythonType::Complex),
                ResolvedType::Unknown,
                ResolvedType::KnownType(PythonType::Float),
                ResolvedType::AnyType,
            ],
        );
    }

    #[test]
    fn type_infer_list() {
        let source = r#"
infer_type([])
infer_type([1, 2, 3])
infer_type([1, "string", 3])
"#;
        test_types(
            source,
            vec![
                ResolvedType::KnownType(PythonType::List(Box::new(ResolvedType::Unknown))),
                ResolvedType::KnownType(PythonType::List(Box::new(ResolvedType::KnownType(
                    PythonType::Int,
                )))),
                ResolvedType::KnownType(PythonType::List(Box::new(ResolvedType::Union(vec![
                    ResolvedType::KnownType(PythonType::Int),
                    ResolvedType::KnownType(PythonType::String),
                ])))),
            ],
        );
    }

    #[test]
    fn type_infer_dict() {
        let source = r#"
infer_type({})
infer_type({1: "hello", 2: "world"})
infer_type({"list": [1, 2, 3], "bool": True})
infer_type({"list": None, 1: 3.14})
"#;
        test_types(
            source,
            vec![
                ResolvedType::KnownType(PythonType::Dict {
                    key: Box::new(ResolvedType::Unknown),
                    value: Box::new(ResolvedType::Unknown),
                }),
                ResolvedType::KnownType(PythonType::Dict {
                    key: Box::new(ResolvedType::KnownType(PythonType::Int)),
                    value: Box::new(ResolvedType::KnownType(PythonType::String)),
                }),
                ResolvedType::KnownType(PythonType::Dict {
                    key: Box::new(ResolvedType::KnownType(PythonType::String)),
                    value: Box::new(ResolvedType::Union(vec![
                        ResolvedType::KnownType(PythonType::List(Box::new(
                            ResolvedType::KnownType(PythonType::Int),
                        ))),
                        ResolvedType::KnownType(PythonType::Bool),
                    ])),
                }),
                ResolvedType::KnownType(PythonType::Dict {
                    key: Box::new(ResolvedType::Union(vec![
                        ResolvedType::KnownType(PythonType::String),
                        ResolvedType::KnownType(PythonType::Int),
                    ])),
                    value: Box::new(ResolvedType::Union(vec![
                        ResolvedType::KnownType(PythonType::None),
                        ResolvedType::KnownType(PythonType::Float),
                    ])),
                }),
            ],
        );
    }

    #[test]
    fn type_infer_set() {
        let source = r#"
infer_type(set())
infer_type({1, 2, 3, 4})
infer_type({"str1", "str2"})
infer_type({"str1", None, 42})
"#;
        test_types(
            source,
            vec![
                ResolvedType::KnownType(PythonType::Set(Box::new(ResolvedType::Unknown))),
                ResolvedType::KnownType(PythonType::Set(Box::new(ResolvedType::KnownType(
                    PythonType::Int,
                )))),
                ResolvedType::KnownType(PythonType::Set(Box::new(ResolvedType::KnownType(
                    PythonType::String,
                )))),
                ResolvedType::KnownType(PythonType::Set(Box::new(ResolvedType::Union(vec![
                    ResolvedType::KnownType(PythonType::String),
                    ResolvedType::KnownType(PythonType::None),
                    ResolvedType::KnownType(PythonType::Int),
                ])))),
            ],
        );
    }

    #[test]
    fn type_infer_tuple() {
        let source = r#"
a = 1

infer_type(())
infer_type((1, 2, 3))
infer_type(("str1", a, 42))
"#;
        test_types(
            source,
            vec![
                ResolvedType::KnownType(PythonType::Tuple(Box::new(ResolvedType::Unknown))),
                ResolvedType::KnownType(PythonType::Tuple(Box::new(ResolvedType::KnownType(
                    PythonType::Int,
                )))),
                ResolvedType::KnownType(PythonType::Tuple(Box::new(ResolvedType::Union(vec![
                    ResolvedType::KnownType(PythonType::String),
                    ResolvedType::KnownType(PythonType::Int),
                ])))),
            ],
        );
    }

    #[test]
    fn type_infer_ann_assign() {
        let source = r#"
s: set[int] = set()
d: dict[int, bool] = {}
l: list[str] = []
t: tuple[bool] = ()

infer_type(s)
infer_type(d)
infer_type(l)
infer_type(t)

d1: dict[int, ] = {}
d2: dict[int] = {}
# third type is ignored
d3: dict[str, int | float, bool] = {}
infer_type(d1)
infer_type(d3)
infer_type(d2)

nested: list[list[bool]] = []
infer_type(nested)
"#;
        test_types(
            source,
            vec![
                ResolvedType::KnownType(PythonType::Set(Box::new(ResolvedType::KnownType(
                    PythonType::Int,
                )))),
                ResolvedType::KnownType(PythonType::Dict {
                    key: Box::new(ResolvedType::KnownType(PythonType::Int)),
                    value: Box::new(ResolvedType::KnownType(PythonType::Bool)),
                }),
                ResolvedType::KnownType(PythonType::List(Box::new(ResolvedType::KnownType(
                    PythonType::String,
                )))),
                ResolvedType::KnownType(PythonType::Tuple(Box::new(ResolvedType::KnownType(
                    PythonType::Bool,
                )))),
                ResolvedType::KnownType(PythonType::Dict {
                    key: Box::new(ResolvedType::KnownType(PythonType::Int)),
                    value: Box::new(ResolvedType::Unknown),
                }),
                ResolvedType::KnownType(PythonType::Dict {
                    key: Box::new(ResolvedType::KnownType(PythonType::String)),
                    value: Box::new(ResolvedType::Union(vec![
                        ResolvedType::KnownType(PythonType::Int),
                        ResolvedType::KnownType(PythonType::Float),
                    ])),
                }),
                ResolvedType::KnownType(PythonType::Dict {
                    key: Box::new(ResolvedType::KnownType(PythonType::Int)),
                    value: Box::new(ResolvedType::Unknown),
                }),
                ResolvedType::KnownType(PythonType::List(Box::new(ResolvedType::KnownType(
                    PythonType::List(Box::new(ResolvedType::KnownType(PythonType::Bool))),
                )))),
            ],
        );
    }

    #[test]
    fn type_infer_typing_module() {
        let source = r#"
import typing
#from typing import List

l: typing.List[int] = []
#l: List[int] = []
infer_type(l)
"#;
        test_types(source, vec![]);
    }
}
