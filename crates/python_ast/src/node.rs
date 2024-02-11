use crate::visitor::preorder::PreorderVisitor;
use crate::{
    self as ast, Alias, ArgOrKeyword, Arguments, Comprehension, Decorator, ExceptHandler, Expr,
    FStringElement, Keyword, MatchCase, Mod, Parameter, ParameterWithDefault, Parameters, Pattern,
    PatternArguments, PatternKeyword, Stmt, TypeParam, TypeParamParamSpec, TypeParamTypeVar,
    TypeParamTypeVarTuple, TypeParams, WithItem,
};
use ruff_text_size::{Ranged, TextRange};
use std::ptr::NonNull;

pub trait AstNode: Ranged {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized;
    fn cast_ref(kind: AnyNodeRef) -> Option<&Self>;

    /// Returns the [`AnyNodeRef`] referencing this node.
    fn as_any_node_ref(&self) -> AnyNodeRef;

    /// Consumes `self` and returns its [`AnyNode`] representation.
    fn into_any_node(self) -> AnyNode;

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized;
}

#[derive(Clone, Debug, is_macro::Is, PartialEq)]
pub enum AnyNode {
    ModModule(ast::ModModule),
    ModessionExpr(ast::ModExpression),
    StmtFunctionDef(ast::FunctionDefStmt),
    StmtClassDef(ast::ClassDefStmt),
    StmtReturn(ast::ReturnStmt),
    StmtDelete(ast::DeleteStmt),
    StmtTypeAlias(ast::TypeAliasStmt),
    StmtAssign(ast::AssignStmt),
    StmtAugAssign(ast::AugAssignStmt),
    StmtAnnAssign(ast::AnnAssignStmt),
    StmtFor(ast::ForStmt),
    StmtWhile(ast::WhileStmt),
    StmtIf(ast::IfStmt),
    StmtWith(ast::WithStmt),
    StmtMatch(ast::MatchStmt),
    StmtRaise(ast::RaiseStmt),
    StmtTry(ast::TryStmt),
    StmtAssert(ast::AssertStmt),
    StmtImport(ast::ImportStmt),
    StmtImportFrom(ast::ImportFromStmt),
    StmtGlobal(ast::GlobalStmt),
    StmtNonlocal(ast::NonlocalStmt),
    StmtExpr(ast::ExprStmt),
    StmtPass(ast::PassStmt),
    StmtBreak(ast::BreakStmt),
    StmtContinue(ast::ContinueStmt),
    StmtIpyEscapeCommand(ast::IpyEscapeCommandStmt),
    BoolOpExpr(ast::BoolOpExpr),
    NamedExprExpr(ast::NamedExpr),
    BinOpExpr(ast::BinOpExpr),
    UnaryOpExpr(ast::UnaryOpExpr),
    LambdaExpr(ast::LambdaExpr),
    IfExpr(ast::IfExpr),
    DictExpr(ast::DictExpr),
    SetExpr(ast::SetExpr),
    ListCompExpr(ast::ListCompExpr),
    SetCompExpr(ast::SetCompExpr),
    DictCompExpr(ast::DictCompExpr),
    GeneratorExpExpr(ast::GeneratorExpExpr),
    AwaitExpr(ast::AwaitExpr),
    YieldExpr(ast::YieldExpr),
    YieldFromExpr(ast::YieldFromExpr),
    CompareExpr(ast::CompareExpr),
    CallExpr(ast::CallExpr),
    FStringExpr(ast::FStringExpr),
    StringLiteralExpr(ast::StringLiteralExpr),
    BytesLiteralExpr(ast::BytesLiteralExpr),
    NumberLiteralExpr(ast::NumberLiteralExpr),
    BooleanLiteralExpr(ast::BooleanLiteralExpr),
    NoneLiteralExpr(ast::NoneLiteralExpr),
    EllipsisLiteralExpr(ast::EllipsisLiteralExpr),
    AttributeExpr(ast::AttributeExpr),
    SubscriptExpr(ast::SubscriptExpr),
    StarredExpr(ast::StarredExpr),
    NameExpr(ast::NameExpr),
    ListExpr(ast::ListExpr),
    TupleExpr(ast::TupleExpr),
    SliceExpr(ast::SliceExpr),
    IpyEscapeCommandExpr(ast::IpyEscapeCommandExpr),
    InvalidExpr(ast::InvalidExpr),
    ExceptHandlerExceptHandler(ast::ExceptHandlerExceptHandler),
    FStringessionElementExpr(ast::FStringExpressionElement),
    FStringLiteralElement(ast::FStringLiteralElement),
    FStringInvalidElement(ast::FStringInvalidElement),
    PatternMatchValue(ast::PatternMatchValue),
    PatternMatchSingleton(ast::PatternMatchSingleton),
    PatternMatchSequence(ast::PatternMatchSequence),
    PatternMatchMapping(ast::PatternMatchMapping),
    PatternMatchClass(ast::PatternMatchClass),
    PatternMatchStar(ast::PatternMatchStar),
    PatternMatchAs(ast::PatternMatchAs),
    PatternMatchOr(ast::PatternMatchOr),
    PatternArguments(PatternArguments),
    PatternKeyword(PatternKeyword),
    PatternMatchInvalid(ast::PatternMatchInvalid),
    Comprehension(Comprehension),
    Arguments(Arguments),
    Parameters(Parameters),
    Parameter(Parameter),
    ParameterWithDefault(ParameterWithDefault),
    Keyword(Keyword),
    Alias(Alias),
    WithItem(WithItem),
    MatchCase(MatchCase),
    Decorator(Decorator),
    ElifElseClause(ast::ElifElseClause),
    TypeParams(TypeParams),
    TypeParamTypeVar(TypeParamTypeVar),
    TypeParamTypeVarTuple(TypeParamTypeVarTuple),
    TypeParamParamSpec(TypeParamParamSpec),
    FString(ast::FString),
    StringLiteral(ast::StringLiteral),
    BytesLiteral(ast::BytesLiteral),
}

impl AnyNode {
    pub fn statement(self) -> Option<Stmt> {
        match self {
            AnyNode::StmtFunctionDef(node) => Some(Stmt::FunctionDef(node)),
            AnyNode::StmtClassDef(node) => Some(Stmt::ClassDef(node)),
            AnyNode::StmtReturn(node) => Some(Stmt::Return(node)),
            AnyNode::StmtDelete(node) => Some(Stmt::Delete(node)),
            AnyNode::StmtTypeAlias(node) => Some(Stmt::TypeAlias(node)),
            AnyNode::StmtAssign(node) => Some(Stmt::Assign(node)),
            AnyNode::StmtAugAssign(node) => Some(Stmt::AugAssign(node)),
            AnyNode::StmtAnnAssign(node) => Some(Stmt::AnnAssign(node)),
            AnyNode::StmtFor(node) => Some(Stmt::For(node)),
            AnyNode::StmtWhile(node) => Some(Stmt::While(node)),
            AnyNode::StmtIf(node) => Some(Stmt::If(node)),
            AnyNode::StmtWith(node) => Some(Stmt::With(node)),
            AnyNode::StmtMatch(node) => Some(Stmt::Match(node)),
            AnyNode::StmtRaise(node) => Some(Stmt::Raise(node)),
            AnyNode::StmtTry(node) => Some(Stmt::Try(node)),
            AnyNode::StmtAssert(node) => Some(Stmt::Assert(node)),
            AnyNode::StmtImport(node) => Some(Stmt::Import(node)),
            AnyNode::StmtImportFrom(node) => Some(Stmt::ImportFrom(node)),
            AnyNode::StmtGlobal(node) => Some(Stmt::Global(node)),
            AnyNode::StmtNonlocal(node) => Some(Stmt::Nonlocal(node)),
            AnyNode::StmtExpr(node) => Some(Stmt::Expr(node)),
            AnyNode::StmtPass(node) => Some(Stmt::Pass(node)),
            AnyNode::StmtBreak(node) => Some(Stmt::Break(node)),
            AnyNode::StmtContinue(node) => Some(Stmt::Continue(node)),
            AnyNode::StmtIpyEscapeCommand(node) => Some(Stmt::IpyEscapeCommand(node)),

            AnyNode::ModModule(_)
            | AnyNode::ModessionExpr(_)
            | AnyNode::BoolOpExpr(_)
            | AnyNode::NamedExprExpr(_)
            | AnyNode::BinOpExpr(_)
            | AnyNode::UnaryOpExpr(_)
            | AnyNode::LambdaExpr(_)
            | AnyNode::IfExpr(_)
            | AnyNode::DictExpr(_)
            | AnyNode::SetExpr(_)
            | AnyNode::ListCompExpr(_)
            | AnyNode::SetCompExpr(_)
            | AnyNode::DictCompExpr(_)
            | AnyNode::GeneratorExpExpr(_)
            | AnyNode::AwaitExpr(_)
            | AnyNode::YieldExpr(_)
            | AnyNode::YieldFromExpr(_)
            | AnyNode::CompareExpr(_)
            | AnyNode::CallExpr(_)
            | AnyNode::FStringessionElementExpr(_)
            | AnyNode::FStringLiteralElement(_)
            | AnyNode::FStringInvalidElement(_)
            | AnyNode::FStringExpr(_)
            | AnyNode::StringLiteralExpr(_)
            | AnyNode::BytesLiteralExpr(_)
            | AnyNode::NumberLiteralExpr(_)
            | AnyNode::BooleanLiteralExpr(_)
            | AnyNode::NoneLiteralExpr(_)
            | AnyNode::EllipsisLiteralExpr(_)
            | AnyNode::AttributeExpr(_)
            | AnyNode::SubscriptExpr(_)
            | AnyNode::StarredExpr(_)
            | AnyNode::NameExpr(_)
            | AnyNode::ListExpr(_)
            | AnyNode::TupleExpr(_)
            | AnyNode::SliceExpr(_)
            | AnyNode::IpyEscapeCommandExpr(_)
            | AnyNode::InvalidExpr(_)
            | AnyNode::ExceptHandlerExceptHandler(_)
            | AnyNode::PatternMatchValue(_)
            | AnyNode::PatternMatchSingleton(_)
            | AnyNode::PatternMatchSequence(_)
            | AnyNode::PatternMatchMapping(_)
            | AnyNode::PatternMatchClass(_)
            | AnyNode::PatternMatchStar(_)
            | AnyNode::PatternMatchAs(_)
            | AnyNode::PatternMatchOr(_)
            | AnyNode::PatternArguments(_)
            | AnyNode::PatternKeyword(_)
            | AnyNode::PatternMatchInvalid(_)
            | AnyNode::Comprehension(_)
            | AnyNode::Arguments(_)
            | AnyNode::Parameters(_)
            | AnyNode::Parameter(_)
            | AnyNode::ParameterWithDefault(_)
            | AnyNode::Keyword(_)
            | AnyNode::Alias(_)
            | AnyNode::WithItem(_)
            | AnyNode::MatchCase(_)
            | AnyNode::Decorator(_)
            | AnyNode::TypeParams(_)
            | AnyNode::TypeParamTypeVar(_)
            | AnyNode::TypeParamTypeVarTuple(_)
            | AnyNode::TypeParamParamSpec(_)
            | AnyNode::FString(_)
            | AnyNode::StringLiteral(_)
            | AnyNode::BytesLiteral(_)
            | AnyNode::ElifElseClause(_) => None,
        }
    }

    pub fn expression(self) -> Option<Expr> {
        match self {
            AnyNode::BoolOpExpr(node) => Some(Expr::BoolOp(node)),
            AnyNode::NamedExprExpr(node) => Some(Expr::NamedExpr(node)),
            AnyNode::BinOpExpr(node) => Some(Expr::BinOp(node)),
            AnyNode::UnaryOpExpr(node) => Some(Expr::UnaryOp(node)),
            AnyNode::LambdaExpr(node) => Some(Expr::Lambda(node)),
            AnyNode::IfExpr(node) => Some(Expr::IfExp(node)),
            AnyNode::DictExpr(node) => Some(Expr::Dict(node)),
            AnyNode::SetExpr(node) => Some(Expr::Set(node)),
            AnyNode::ListCompExpr(node) => Some(Expr::ListComp(node)),
            AnyNode::SetCompExpr(node) => Some(Expr::SetComp(node)),
            AnyNode::DictCompExpr(node) => Some(Expr::DictComp(node)),
            AnyNode::GeneratorExpExpr(node) => Some(Expr::GeneratorExp(node)),
            AnyNode::AwaitExpr(node) => Some(Expr::Await(node)),
            AnyNode::YieldExpr(node) => Some(Expr::Yield(node)),
            AnyNode::YieldFromExpr(node) => Some(Expr::YieldFrom(node)),
            AnyNode::CompareExpr(node) => Some(Expr::Compare(node)),
            AnyNode::CallExpr(node) => Some(Expr::Call(node)),
            AnyNode::FStringExpr(node) => Some(Expr::FString(node)),
            AnyNode::StringLiteralExpr(node) => Some(Expr::StringLiteral(node)),
            AnyNode::BytesLiteralExpr(node) => Some(Expr::BytesLiteral(node)),
            AnyNode::NumberLiteralExpr(node) => Some(Expr::NumberLiteral(node)),
            AnyNode::BooleanLiteralExpr(node) => Some(Expr::BooleanLiteral(node)),
            AnyNode::NoneLiteralExpr(node) => Some(Expr::NoneLiteral(node)),
            AnyNode::EllipsisLiteralExpr(node) => Some(Expr::EllipsisLiteral(node)),
            AnyNode::AttributeExpr(node) => Some(Expr::Attribute(node)),
            AnyNode::SubscriptExpr(node) => Some(Expr::Subscript(node)),
            AnyNode::StarredExpr(node) => Some(Expr::Starred(node)),
            AnyNode::NameExpr(node) => Some(Expr::Name(node)),
            AnyNode::ListExpr(node) => Some(Expr::List(node)),
            AnyNode::TupleExpr(node) => Some(Expr::Tuple(node)),
            AnyNode::SliceExpr(node) => Some(Expr::Slice(node)),
            AnyNode::IpyEscapeCommandExpr(node) => Some(Expr::IpyEscapeCommand(node)),
            AnyNode::InvalidExpr(range) => Some(Expr::Invalid(range)),

            AnyNode::ModModule(_)
            | AnyNode::ModessionExpr(_)
            | AnyNode::StmtFunctionDef(_)
            | AnyNode::StmtClassDef(_)
            | AnyNode::StmtReturn(_)
            | AnyNode::StmtDelete(_)
            | AnyNode::StmtTypeAlias(_)
            | AnyNode::StmtAssign(_)
            | AnyNode::StmtAugAssign(_)
            | AnyNode::StmtAnnAssign(_)
            | AnyNode::StmtFor(_)
            | AnyNode::StmtWhile(_)
            | AnyNode::StmtIf(_)
            | AnyNode::StmtWith(_)
            | AnyNode::StmtMatch(_)
            | AnyNode::StmtRaise(_)
            | AnyNode::StmtTry(_)
            | AnyNode::StmtAssert(_)
            | AnyNode::StmtImport(_)
            | AnyNode::StmtImportFrom(_)
            | AnyNode::StmtGlobal(_)
            | AnyNode::StmtNonlocal(_)
            | AnyNode::StmtExpr(_)
            | AnyNode::StmtPass(_)
            | AnyNode::StmtBreak(_)
            | AnyNode::StmtContinue(_)
            | AnyNode::StmtIpyEscapeCommand(_)
            | AnyNode::ExceptHandlerExceptHandler(_)
            | AnyNode::FStringessionElementExpr(_)
            | AnyNode::FStringLiteralElement(_)
            | AnyNode::FStringInvalidElement(_)
            | AnyNode::PatternMatchValue(_)
            | AnyNode::PatternMatchSingleton(_)
            | AnyNode::PatternMatchSequence(_)
            | AnyNode::PatternMatchMapping(_)
            | AnyNode::PatternMatchClass(_)
            | AnyNode::PatternMatchStar(_)
            | AnyNode::PatternMatchAs(_)
            | AnyNode::PatternMatchOr(_)
            | AnyNode::PatternArguments(_)
            | AnyNode::PatternKeyword(_)
            | AnyNode::PatternMatchInvalid(_)
            | AnyNode::Comprehension(_)
            | AnyNode::Arguments(_)
            | AnyNode::Parameters(_)
            | AnyNode::Parameter(_)
            | AnyNode::ParameterWithDefault(_)
            | AnyNode::Keyword(_)
            | AnyNode::Alias(_)
            | AnyNode::WithItem(_)
            | AnyNode::MatchCase(_)
            | AnyNode::Decorator(_)
            | AnyNode::TypeParams(_)
            | AnyNode::TypeParamTypeVar(_)
            | AnyNode::TypeParamTypeVarTuple(_)
            | AnyNode::TypeParamParamSpec(_)
            | AnyNode::FString(_)
            | AnyNode::StringLiteral(_)
            | AnyNode::BytesLiteral(_)
            | AnyNode::ElifElseClause(_) => None,
        }
    }

    pub fn module(self) -> Option<Mod> {
        match self {
            AnyNode::ModModule(node) => Some(Mod::Module(node)),
            AnyNode::ModessionExpr(node) => Some(Mod::Expression(node)),

            AnyNode::StmtFunctionDef(_)
            | AnyNode::StmtClassDef(_)
            | AnyNode::StmtReturn(_)
            | AnyNode::StmtDelete(_)
            | AnyNode::StmtTypeAlias(_)
            | AnyNode::StmtAssign(_)
            | AnyNode::StmtAugAssign(_)
            | AnyNode::StmtAnnAssign(_)
            | AnyNode::StmtFor(_)
            | AnyNode::StmtWhile(_)
            | AnyNode::StmtIf(_)
            | AnyNode::StmtWith(_)
            | AnyNode::StmtMatch(_)
            | AnyNode::StmtRaise(_)
            | AnyNode::StmtTry(_)
            | AnyNode::StmtAssert(_)
            | AnyNode::StmtImport(_)
            | AnyNode::StmtImportFrom(_)
            | AnyNode::StmtGlobal(_)
            | AnyNode::StmtNonlocal(_)
            | AnyNode::StmtExpr(_)
            | AnyNode::StmtPass(_)
            | AnyNode::StmtBreak(_)
            | AnyNode::StmtContinue(_)
            | AnyNode::StmtIpyEscapeCommand(_)
            | AnyNode::BoolOpExpr(_)
            | AnyNode::NamedExprExpr(_)
            | AnyNode::BinOpExpr(_)
            | AnyNode::UnaryOpExpr(_)
            | AnyNode::LambdaExpr(_)
            | AnyNode::IfExpr(_)
            | AnyNode::DictExpr(_)
            | AnyNode::SetExpr(_)
            | AnyNode::ListCompExpr(_)
            | AnyNode::SetCompExpr(_)
            | AnyNode::DictCompExpr(_)
            | AnyNode::GeneratorExpExpr(_)
            | AnyNode::AwaitExpr(_)
            | AnyNode::YieldExpr(_)
            | AnyNode::YieldFromExpr(_)
            | AnyNode::CompareExpr(_)
            | AnyNode::CallExpr(_)
            | AnyNode::FStringessionElementExpr(_)
            | AnyNode::FStringLiteralElement(_)
            | AnyNode::FStringInvalidElement(_)
            | AnyNode::FStringExpr(_)
            | AnyNode::StringLiteralExpr(_)
            | AnyNode::BytesLiteralExpr(_)
            | AnyNode::NumberLiteralExpr(_)
            | AnyNode::BooleanLiteralExpr(_)
            | AnyNode::NoneLiteralExpr(_)
            | AnyNode::EllipsisLiteralExpr(_)
            | AnyNode::AttributeExpr(_)
            | AnyNode::SubscriptExpr(_)
            | AnyNode::StarredExpr(_)
            | AnyNode::NameExpr(_)
            | AnyNode::ListExpr(_)
            | AnyNode::TupleExpr(_)
            | AnyNode::SliceExpr(_)
            | AnyNode::IpyEscapeCommandExpr(_)
            | AnyNode::InvalidExpr(_)
            | AnyNode::ExceptHandlerExceptHandler(_)
            | AnyNode::PatternMatchValue(_)
            | AnyNode::PatternMatchSingleton(_)
            | AnyNode::PatternMatchSequence(_)
            | AnyNode::PatternMatchMapping(_)
            | AnyNode::PatternMatchClass(_)
            | AnyNode::PatternMatchStar(_)
            | AnyNode::PatternMatchAs(_)
            | AnyNode::PatternMatchOr(_)
            | AnyNode::PatternArguments(_)
            | AnyNode::PatternKeyword(_)
            | AnyNode::PatternMatchInvalid(_)
            | AnyNode::Comprehension(_)
            | AnyNode::Arguments(_)
            | AnyNode::Parameters(_)
            | AnyNode::Parameter(_)
            | AnyNode::ParameterWithDefault(_)
            | AnyNode::Keyword(_)
            | AnyNode::Alias(_)
            | AnyNode::WithItem(_)
            | AnyNode::MatchCase(_)
            | AnyNode::Decorator(_)
            | AnyNode::TypeParams(_)
            | AnyNode::TypeParamTypeVar(_)
            | AnyNode::TypeParamTypeVarTuple(_)
            | AnyNode::TypeParamParamSpec(_)
            | AnyNode::FString(_)
            | AnyNode::StringLiteral(_)
            | AnyNode::BytesLiteral(_)
            | AnyNode::ElifElseClause(_) => None,
        }
    }

    pub fn pattern(self) -> Option<Pattern> {
        match self {
            AnyNode::PatternMatchValue(node) => Some(Pattern::MatchValue(node)),
            AnyNode::PatternMatchSingleton(node) => Some(Pattern::MatchSingleton(node)),
            AnyNode::PatternMatchSequence(node) => Some(Pattern::MatchSequence(node)),
            AnyNode::PatternMatchMapping(node) => Some(Pattern::MatchMapping(node)),
            AnyNode::PatternMatchClass(node) => Some(Pattern::MatchClass(node)),
            AnyNode::PatternMatchStar(node) => Some(Pattern::MatchStar(node)),
            AnyNode::PatternMatchAs(node) => Some(Pattern::MatchAs(node)),
            AnyNode::PatternMatchOr(node) => Some(Pattern::MatchOr(node)),
            AnyNode::PatternMatchInvalid(node) => Some(Pattern::Invalid(node)),

            AnyNode::ModModule(_)
            | AnyNode::ModessionExpr(_)
            | AnyNode::StmtFunctionDef(_)
            | AnyNode::StmtClassDef(_)
            | AnyNode::StmtReturn(_)
            | AnyNode::StmtDelete(_)
            | AnyNode::StmtTypeAlias(_)
            | AnyNode::StmtAssign(_)
            | AnyNode::StmtAugAssign(_)
            | AnyNode::StmtAnnAssign(_)
            | AnyNode::StmtFor(_)
            | AnyNode::StmtWhile(_)
            | AnyNode::StmtIf(_)
            | AnyNode::StmtWith(_)
            | AnyNode::StmtMatch(_)
            | AnyNode::StmtRaise(_)
            | AnyNode::StmtTry(_)
            | AnyNode::StmtAssert(_)
            | AnyNode::StmtImport(_)
            | AnyNode::StmtImportFrom(_)
            | AnyNode::StmtGlobal(_)
            | AnyNode::StmtNonlocal(_)
            | AnyNode::StmtExpr(_)
            | AnyNode::StmtPass(_)
            | AnyNode::StmtBreak(_)
            | AnyNode::StmtContinue(_)
            | AnyNode::StmtIpyEscapeCommand(_)
            | AnyNode::BoolOpExpr(_)
            | AnyNode::NamedExprExpr(_)
            | AnyNode::BinOpExpr(_)
            | AnyNode::UnaryOpExpr(_)
            | AnyNode::LambdaExpr(_)
            | AnyNode::IfExpr(_)
            | AnyNode::DictExpr(_)
            | AnyNode::SetExpr(_)
            | AnyNode::ListCompExpr(_)
            | AnyNode::SetCompExpr(_)
            | AnyNode::DictCompExpr(_)
            | AnyNode::GeneratorExpExpr(_)
            | AnyNode::AwaitExpr(_)
            | AnyNode::YieldExpr(_)
            | AnyNode::YieldFromExpr(_)
            | AnyNode::CompareExpr(_)
            | AnyNode::CallExpr(_)
            | AnyNode::FStringessionElementExpr(_)
            | AnyNode::FStringLiteralElement(_)
            | AnyNode::FStringInvalidElement(_)
            | AnyNode::FStringExpr(_)
            | AnyNode::StringLiteralExpr(_)
            | AnyNode::BytesLiteralExpr(_)
            | AnyNode::NumberLiteralExpr(_)
            | AnyNode::BooleanLiteralExpr(_)
            | AnyNode::NoneLiteralExpr(_)
            | AnyNode::EllipsisLiteralExpr(_)
            | AnyNode::AttributeExpr(_)
            | AnyNode::SubscriptExpr(_)
            | AnyNode::StarredExpr(_)
            | AnyNode::NameExpr(_)
            | AnyNode::ListExpr(_)
            | AnyNode::TupleExpr(_)
            | AnyNode::SliceExpr(_)
            | AnyNode::IpyEscapeCommandExpr(_)
            | AnyNode::InvalidExpr(_)
            | AnyNode::ExceptHandlerExceptHandler(_)
            | AnyNode::PatternArguments(_)
            | AnyNode::PatternKeyword(_)
            | AnyNode::Comprehension(_)
            | AnyNode::Arguments(_)
            | AnyNode::Parameters(_)
            | AnyNode::Parameter(_)
            | AnyNode::ParameterWithDefault(_)
            | AnyNode::Keyword(_)
            | AnyNode::Alias(_)
            | AnyNode::WithItem(_)
            | AnyNode::MatchCase(_)
            | AnyNode::Decorator(_)
            | AnyNode::TypeParams(_)
            | AnyNode::TypeParamTypeVar(_)
            | AnyNode::TypeParamTypeVarTuple(_)
            | AnyNode::TypeParamParamSpec(_)
            | AnyNode::FString(_)
            | AnyNode::StringLiteral(_)
            | AnyNode::BytesLiteral(_)
            | AnyNode::ElifElseClause(_) => None,
        }
    }

    pub fn except_handler(self) -> Option<ExceptHandler> {
        match self {
            AnyNode::ExceptHandlerExceptHandler(node) => Some(ExceptHandler::ExceptHandler(node)),

            AnyNode::ModModule(_)
            | AnyNode::ModessionExpr(_)
            | AnyNode::StmtFunctionDef(_)
            | AnyNode::StmtClassDef(_)
            | AnyNode::StmtReturn(_)
            | AnyNode::StmtDelete(_)
            | AnyNode::StmtTypeAlias(_)
            | AnyNode::StmtAssign(_)
            | AnyNode::StmtAugAssign(_)
            | AnyNode::StmtAnnAssign(_)
            | AnyNode::StmtFor(_)
            | AnyNode::StmtWhile(_)
            | AnyNode::StmtIf(_)
            | AnyNode::StmtWith(_)
            | AnyNode::StmtMatch(_)
            | AnyNode::StmtRaise(_)
            | AnyNode::StmtTry(_)
            | AnyNode::StmtAssert(_)
            | AnyNode::StmtImport(_)
            | AnyNode::StmtImportFrom(_)
            | AnyNode::StmtGlobal(_)
            | AnyNode::StmtNonlocal(_)
            | AnyNode::StmtExpr(_)
            | AnyNode::StmtPass(_)
            | AnyNode::StmtBreak(_)
            | AnyNode::StmtContinue(_)
            | AnyNode::StmtIpyEscapeCommand(_)
            | AnyNode::BoolOpExpr(_)
            | AnyNode::NamedExprExpr(_)
            | AnyNode::BinOpExpr(_)
            | AnyNode::UnaryOpExpr(_)
            | AnyNode::LambdaExpr(_)
            | AnyNode::IfExpr(_)
            | AnyNode::DictExpr(_)
            | AnyNode::SetExpr(_)
            | AnyNode::ListCompExpr(_)
            | AnyNode::SetCompExpr(_)
            | AnyNode::DictCompExpr(_)
            | AnyNode::GeneratorExpExpr(_)
            | AnyNode::AwaitExpr(_)
            | AnyNode::YieldExpr(_)
            | AnyNode::YieldFromExpr(_)
            | AnyNode::CompareExpr(_)
            | AnyNode::CallExpr(_)
            | AnyNode::FStringessionElementExpr(_)
            | AnyNode::FStringLiteralElement(_)
            | AnyNode::FStringInvalidElement(_)
            | AnyNode::FStringExpr(_)
            | AnyNode::StringLiteralExpr(_)
            | AnyNode::BytesLiteralExpr(_)
            | AnyNode::NumberLiteralExpr(_)
            | AnyNode::BooleanLiteralExpr(_)
            | AnyNode::NoneLiteralExpr(_)
            | AnyNode::EllipsisLiteralExpr(_)
            | AnyNode::AttributeExpr(_)
            | AnyNode::SubscriptExpr(_)
            | AnyNode::StarredExpr(_)
            | AnyNode::NameExpr(_)
            | AnyNode::ListExpr(_)
            | AnyNode::TupleExpr(_)
            | AnyNode::SliceExpr(_)
            | AnyNode::IpyEscapeCommandExpr(_)
            | AnyNode::InvalidExpr(_)
            | AnyNode::PatternMatchValue(_)
            | AnyNode::PatternMatchSingleton(_)
            | AnyNode::PatternMatchSequence(_)
            | AnyNode::PatternMatchMapping(_)
            | AnyNode::PatternMatchClass(_)
            | AnyNode::PatternMatchStar(_)
            | AnyNode::PatternMatchAs(_)
            | AnyNode::PatternMatchOr(_)
            | AnyNode::PatternArguments(_)
            | AnyNode::PatternKeyword(_)
            | AnyNode::PatternMatchInvalid(_)
            | AnyNode::Comprehension(_)
            | AnyNode::Arguments(_)
            | AnyNode::Parameters(_)
            | AnyNode::Parameter(_)
            | AnyNode::ParameterWithDefault(_)
            | AnyNode::Keyword(_)
            | AnyNode::Alias(_)
            | AnyNode::WithItem(_)
            | AnyNode::MatchCase(_)
            | AnyNode::Decorator(_)
            | AnyNode::TypeParams(_)
            | AnyNode::TypeParamTypeVar(_)
            | AnyNode::TypeParamTypeVarTuple(_)
            | AnyNode::TypeParamParamSpec(_)
            | AnyNode::FString(_)
            | AnyNode::StringLiteral(_)
            | AnyNode::BytesLiteral(_)
            | AnyNode::ElifElseClause(_) => None,
        }
    }

    pub const fn is_statement(&self) -> bool {
        self.as_ref().is_statement()
    }

    pub const fn is_expression(&self) -> bool {
        self.as_ref().is_expression()
    }

    pub const fn is_module(&self) -> bool {
        self.as_ref().is_module()
    }

    pub const fn is_pattern(&self) -> bool {
        self.as_ref().is_pattern()
    }

    pub const fn is_except_handler(&self) -> bool {
        self.as_ref().is_except_handler()
    }

    pub const fn as_ref(&self) -> AnyNodeRef {
        match self {
            Self::ModModule(node) => AnyNodeRef::ModModule(node),
            Self::ModessionExpr(node) => AnyNodeRef::ModessionExpr(node),
            Self::StmtFunctionDef(node) => AnyNodeRef::StmtFunctionDef(node),
            Self::StmtClassDef(node) => AnyNodeRef::StmtClassDef(node),
            Self::StmtReturn(node) => AnyNodeRef::StmtReturn(node),
            Self::StmtDelete(node) => AnyNodeRef::StmtDelete(node),
            Self::StmtTypeAlias(node) => AnyNodeRef::StmtTypeAlias(node),
            Self::StmtAssign(node) => AnyNodeRef::StmtAssign(node),
            Self::StmtAugAssign(node) => AnyNodeRef::StmtAugAssign(node),
            Self::StmtAnnAssign(node) => AnyNodeRef::StmtAnnAssign(node),
            Self::StmtFor(node) => AnyNodeRef::StmtFor(node),
            Self::StmtWhile(node) => AnyNodeRef::StmtWhile(node),
            Self::StmtIf(node) => AnyNodeRef::StmtIf(node),
            Self::StmtWith(node) => AnyNodeRef::StmtWith(node),
            Self::StmtMatch(node) => AnyNodeRef::StmtMatch(node),
            Self::StmtRaise(node) => AnyNodeRef::StmtRaise(node),
            Self::StmtTry(node) => AnyNodeRef::StmtTry(node),
            Self::StmtAssert(node) => AnyNodeRef::StmtAssert(node),
            Self::StmtImport(node) => AnyNodeRef::StmtImport(node),
            Self::StmtImportFrom(node) => AnyNodeRef::StmtImportFrom(node),
            Self::StmtGlobal(node) => AnyNodeRef::StmtGlobal(node),
            Self::StmtNonlocal(node) => AnyNodeRef::StmtNonlocal(node),
            Self::StmtExpr(node) => AnyNodeRef::StmtExpr(node),
            Self::StmtPass(node) => AnyNodeRef::StmtPass(node),
            Self::StmtBreak(node) => AnyNodeRef::StmtBreak(node),
            Self::StmtContinue(node) => AnyNodeRef::StmtContinue(node),
            Self::StmtIpyEscapeCommand(node) => AnyNodeRef::StmtIpyEscapeCommand(node),
            Self::BoolOpExpr(node) => AnyNodeRef::BoolOpExpr(node),
            Self::NamedExprExpr(node) => AnyNodeRef::NamedExprExpr(node),
            Self::BinOpExpr(node) => AnyNodeRef::BinOpExpr(node),
            Self::UnaryOpExpr(node) => AnyNodeRef::UnaryOpExpr(node),
            Self::LambdaExpr(node) => AnyNodeRef::LambdaExpr(node),
            Self::IfExpr(node) => AnyNodeRef::IfExpr(node),
            Self::DictExpr(node) => AnyNodeRef::DictExpr(node),
            Self::SetExpr(node) => AnyNodeRef::SetExpr(node),
            Self::ListCompExpr(node) => AnyNodeRef::ListCompExpr(node),
            Self::SetCompExpr(node) => AnyNodeRef::SetCompExpr(node),
            Self::DictCompExpr(node) => AnyNodeRef::DictCompExpr(node),
            Self::GeneratorExpExpr(node) => AnyNodeRef::GeneratorExpExpr(node),
            Self::AwaitExpr(node) => AnyNodeRef::AwaitExpr(node),
            Self::YieldExpr(node) => AnyNodeRef::YieldExpr(node),
            Self::YieldFromExpr(node) => AnyNodeRef::YieldFromExpr(node),
            Self::CompareExpr(node) => AnyNodeRef::CompareExpr(node),
            Self::CallExpr(node) => AnyNodeRef::CallExpr(node),
            Self::FStringessionElementExpr(node) => AnyNodeRef::FStringessionElementExpr(node),
            Self::FStringLiteralElement(node) => AnyNodeRef::FStringLiteralElement(node),
            Self::FStringInvalidElement(node) => AnyNodeRef::FStringInvalidElement(node),
            Self::FStringExpr(node) => AnyNodeRef::FStringExpr(node),
            Self::StringLiteralExpr(node) => AnyNodeRef::StringLiteralExpr(node),
            Self::BytesLiteralExpr(node) => AnyNodeRef::BytesLiteralExpr(node),
            Self::NumberLiteralExpr(node) => AnyNodeRef::NumberLiteralExpr(node),
            Self::BooleanLiteralExpr(node) => AnyNodeRef::BooleanLiteralExpr(node),
            Self::NoneLiteralExpr(node) => AnyNodeRef::NoneLiteralExpr(node),
            Self::EllipsisLiteralExpr(node) => AnyNodeRef::EllipsisLiteralExpr(node),
            Self::AttributeExpr(node) => AnyNodeRef::AttributeExpr(node),
            Self::SubscriptExpr(node) => AnyNodeRef::SubscriptExpr(node),
            Self::StarredExpr(node) => AnyNodeRef::StarredExpr(node),
            Self::NameExpr(node) => AnyNodeRef::NameExpr(node),
            Self::ListExpr(node) => AnyNodeRef::ListExpr(node),
            Self::TupleExpr(node) => AnyNodeRef::TupleExpr(node),
            Self::SliceExpr(node) => AnyNodeRef::SliceExpr(node),
            Self::IpyEscapeCommandExpr(node) => AnyNodeRef::IpyEscapeCommandExpr(node),
            Self::InvalidExpr(node) => AnyNodeRef::InvalidExpr(node),
            Self::ExceptHandlerExceptHandler(node) => AnyNodeRef::ExceptHandlerExceptHandler(node),
            Self::PatternMatchValue(node) => AnyNodeRef::PatternMatchValue(node),
            Self::PatternMatchSingleton(node) => AnyNodeRef::PatternMatchSingleton(node),
            Self::PatternMatchSequence(node) => AnyNodeRef::PatternMatchSequence(node),
            Self::PatternMatchMapping(node) => AnyNodeRef::PatternMatchMapping(node),
            Self::PatternMatchClass(node) => AnyNodeRef::PatternMatchClass(node),
            Self::PatternMatchStar(node) => AnyNodeRef::PatternMatchStar(node),
            Self::PatternMatchAs(node) => AnyNodeRef::PatternMatchAs(node),
            Self::PatternMatchOr(node) => AnyNodeRef::PatternMatchOr(node),
            Self::PatternArguments(node) => AnyNodeRef::PatternArguments(node),
            Self::PatternKeyword(node) => AnyNodeRef::PatternKeyword(node),
            Self::PatternMatchInvalid(node) => AnyNodeRef::PatternMatchInvalid(node),
            Self::Comprehension(node) => AnyNodeRef::Comprehension(node),
            Self::Arguments(node) => AnyNodeRef::Arguments(node),
            Self::Parameters(node) => AnyNodeRef::Parameters(node),
            Self::Parameter(node) => AnyNodeRef::Parameter(node),
            Self::ParameterWithDefault(node) => AnyNodeRef::ParameterWithDefault(node),
            Self::Keyword(node) => AnyNodeRef::Keyword(node),
            Self::Alias(node) => AnyNodeRef::Alias(node),
            Self::WithItem(node) => AnyNodeRef::WithItem(node),
            Self::MatchCase(node) => AnyNodeRef::MatchCase(node),
            Self::Decorator(node) => AnyNodeRef::Decorator(node),
            Self::TypeParams(node) => AnyNodeRef::TypeParams(node),
            Self::TypeParamTypeVar(node) => AnyNodeRef::TypeParamTypeVar(node),
            Self::TypeParamTypeVarTuple(node) => AnyNodeRef::TypeParamTypeVarTuple(node),
            Self::TypeParamParamSpec(node) => AnyNodeRef::TypeParamParamSpec(node),
            Self::FString(node) => AnyNodeRef::FString(node),
            Self::StringLiteral(node) => AnyNodeRef::StringLiteral(node),
            Self::BytesLiteral(node) => AnyNodeRef::BytesLiteral(node),
            Self::ElifElseClause(node) => AnyNodeRef::ElifElseClause(node),
        }
    }

    /// Returns the node's [`kind`](NodeKind) that has no data associated and is [`Copy`].
    pub const fn kind(&self) -> NodeKind {
        self.as_ref().kind()
    }
}

impl AstNode for ast::ModModule {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::ModModule(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::ModModule(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::ModModule { body, range: _ } = self;
        visitor.visit_body(body);
    }
}

impl AstNode for ast::ModExpression {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::ModessionExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::ModessionExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::ModExpression { body, range: _ } = self;
        visitor.visit_expr(body);
    }
}
impl AstNode for ast::FunctionDefStmt {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StmtFunctionDef(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StmtFunctionDef(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::FunctionDefStmt {
            parameters,
            body,
            decorator_list,
            returns,
            type_params,
            ..
        } = self;

        for decorator in decorator_list {
            visitor.visit_decorator(decorator);
        }

        if let Some(type_params) = type_params {
            visitor.visit_type_params(type_params);
        }

        visitor.visit_parameters(parameters);

        for expr in returns {
            visitor.visit_annotation(expr);
        }

        visitor.visit_body(body);
    }
}
impl AstNode for ast::ClassDefStmt {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StmtClassDef(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StmtClassDef(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::ClassDefStmt {
            arguments,
            body,
            decorator_list,
            type_params,
            ..
        } = self;

        for decorator in decorator_list {
            visitor.visit_decorator(decorator);
        }

        if let Some(type_params) = type_params {
            visitor.visit_type_params(type_params);
        }

        if let Some(arguments) = arguments {
            visitor.visit_arguments(arguments);
        }

        visitor.visit_body(body);
    }
}
impl AstNode for ast::ReturnStmt {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StmtReturn(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StmtReturn(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::ReturnStmt { value, range: _ } = self;
        if let Some(expr) = value {
            visitor.visit_expr(expr);
        }
    }
}
impl AstNode for ast::DeleteStmt {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StmtDelete(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StmtDelete(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::DeleteStmt { targets, range: _ } = self;
        for expr in targets {
            visitor.visit_expr(expr);
        }
    }
}
impl AstNode for ast::TypeAliasStmt {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StmtTypeAlias(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StmtTypeAlias(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::TypeAliasStmt {
            range: _,
            name,
            type_params,
            value,
        } = self;

        visitor.visit_expr(name);
        if let Some(type_params) = type_params {
            visitor.visit_type_params(type_params);
        }
        visitor.visit_expr(value);
    }
}
impl AstNode for ast::AssignStmt {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StmtAssign(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StmtAssign(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::AssignStmt {
            targets,
            value,
            range: _,
        } = self;

        for expr in targets {
            visitor.visit_expr(expr);
        }

        visitor.visit_expr(value);
    }
}
impl AstNode for ast::AugAssignStmt {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StmtAugAssign(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StmtAugAssign(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::AugAssignStmt {
            target,
            op,
            value,
            range: _,
        } = self;

        visitor.visit_expr(target);
        visitor.visit_operator(op);
        visitor.visit_expr(value);
    }
}
impl AstNode for ast::AnnAssignStmt {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StmtAnnAssign(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StmtAnnAssign(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::AnnAssignStmt {
            target,
            annotation,
            value,
            range: _,
            simple: _,
        } = self;

        visitor.visit_expr(target);
        visitor.visit_annotation(annotation);
        if let Some(expr) = value {
            visitor.visit_expr(expr);
        }
    }
}
impl AstNode for ast::ForStmt {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StmtFor(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StmtFor(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::ForStmt {
            target,
            iter,
            body,
            orelse,
            ..
        } = self;

        visitor.visit_expr(target);
        visitor.visit_expr(iter);
        visitor.visit_body(body);
        visitor.visit_body(orelse);
    }
}
impl AstNode for ast::WhileStmt {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StmtWhile(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StmtWhile(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::WhileStmt {
            test,
            body,
            orelse,
            range: _,
        } = self;

        visitor.visit_expr(test);
        visitor.visit_body(body);
        visitor.visit_body(orelse);
    }
}
impl AstNode for ast::IfStmt {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StmtIf(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StmtIf(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::IfStmt {
            test,
            body,
            elif_else_clauses,
            range: _,
        } = self;

        visitor.visit_expr(test);
        visitor.visit_body(body);
        for clause in elif_else_clauses {
            visitor.visit_elif_else_clause(clause);
        }
    }
}
impl AstNode for ast::ElifElseClause {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::ElifElseClause(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::ElifElseClause(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::ElifElseClause {
            range: _,
            test,
            body,
        } = self;
        if let Some(test) = test {
            visitor.visit_expr(test);
        }
        visitor.visit_body(body);
    }
}
impl AstNode for ast::WithStmt {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StmtWith(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StmtWith(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::WithStmt {
            items,
            body,
            is_async: _,
            range: _,
        } = self;

        for with_item in items {
            visitor.visit_with_item(with_item);
        }
        visitor.visit_body(body);
    }
}
impl AstNode for ast::MatchStmt {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StmtMatch(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StmtMatch(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::MatchStmt {
            subject,
            cases,
            range: _,
        } = self;

        visitor.visit_expr(subject);
        for match_case in cases {
            visitor.visit_match_case(match_case);
        }
    }
}
impl AstNode for ast::RaiseStmt {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StmtRaise(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StmtRaise(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::RaiseStmt {
            exc,
            cause,
            range: _,
        } = self;

        if let Some(expr) = exc {
            visitor.visit_expr(expr);
        };
        if let Some(expr) = cause {
            visitor.visit_expr(expr);
        };
    }
}
impl AstNode for ast::TryStmt {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StmtTry(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StmtTry(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::TryStmt {
            body,
            handlers,
            orelse,
            finalbody,
            is_star: _,
            range: _,
        } = self;

        visitor.visit_body(body);
        for except_handler in handlers {
            visitor.visit_except_handler(except_handler);
        }
        visitor.visit_body(orelse);
        visitor.visit_body(finalbody);
    }
}
impl AstNode for ast::AssertStmt {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StmtAssert(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StmtAssert(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::AssertStmt {
            test,
            msg,
            range: _,
        } = self;
        visitor.visit_expr(test);
        if let Some(expr) = msg {
            visitor.visit_expr(expr);
        }
    }
}
impl AstNode for ast::ImportStmt {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StmtImport(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StmtImport(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::ImportStmt { names, range: _ } = self;

        for alias in names {
            visitor.visit_alias(alias);
        }
    }
}
impl AstNode for ast::ImportFromStmt {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StmtImportFrom(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StmtImportFrom(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::ImportFromStmt {
            range: _,
            module: _,
            names,
            level: _,
        } = self;

        for alias in names {
            visitor.visit_alias(alias);
        }
    }
}
impl AstNode for ast::GlobalStmt {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StmtGlobal(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StmtGlobal(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    #[inline]
    fn visit_preorder<'a, V>(&'a self, _visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
    }
}
impl AstNode for ast::NonlocalStmt {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StmtNonlocal(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StmtNonlocal(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    #[inline]
    fn visit_preorder<'a, V>(&'a self, _visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
    }
}
impl AstNode for ast::ExprStmt {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StmtExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StmtExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::ExprStmt { value, range: _ } = self;

        visitor.visit_expr(value);
    }
}
impl AstNode for ast::PassStmt {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StmtPass(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StmtPass(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    #[inline]
    fn visit_preorder<'a, V>(&'a self, _visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
    }
}
impl AstNode for ast::BreakStmt {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StmtBreak(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StmtBreak(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    #[inline]
    fn visit_preorder<'a, V>(&'a self, _visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
    }
}
impl AstNode for ast::ContinueStmt {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StmtContinue(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StmtContinue(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    #[inline]
    fn visit_preorder<'a, V>(&'a self, _visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
    }
}
impl AstNode for ast::IpyEscapeCommandStmt {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StmtIpyEscapeCommand(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StmtIpyEscapeCommand(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    #[inline]
    fn visit_preorder<'a, V>(&'a self, _visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
    }
}
impl AstNode for ast::BoolOpExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::BoolOpExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::BoolOpExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::BoolOpExpr {
            op,
            values,
            range: _,
        } = self;
        match values.as_slice() {
            [left, rest @ ..] => {
                visitor.visit_expr(left);
                visitor.visit_bool_op(op);
                for expr in rest {
                    visitor.visit_expr(expr);
                }
            }
            [] => {
                visitor.visit_bool_op(op);
            }
        }
    }
}
impl AstNode for ast::NamedExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::NamedExprExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::NamedExprExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::NamedExpr {
            target,
            value,
            range: _,
        } = self;
        visitor.visit_expr(target);
        visitor.visit_expr(value);
    }
}
impl AstNode for ast::BinOpExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::BinOpExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::BinOpExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::BinOpExpr {
            left,
            op,
            right,
            range: _,
        } = self;
        visitor.visit_expr(left);
        visitor.visit_operator(op);
        visitor.visit_expr(right);
    }
}
impl AstNode for ast::UnaryOpExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::UnaryOpExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::UnaryOpExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::UnaryOpExpr {
            op,
            operand,
            range: _,
        } = self;

        visitor.visit_unary_op(op);
        visitor.visit_expr(operand);
    }
}
impl AstNode for ast::LambdaExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::LambdaExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::LambdaExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::LambdaExpr {
            parameters,
            body,
            range: _,
        } = self;

        if let Some(parameters) = parameters {
            visitor.visit_parameters(parameters);
        }
        visitor.visit_expr(body);
    }
}
impl AstNode for ast::IfExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::IfExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::IfExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::IfExpr {
            test,
            body,
            orelse,
            range: _,
        } = self;

        // `body if test else orelse`
        visitor.visit_expr(body);
        visitor.visit_expr(test);
        visitor.visit_expr(orelse);
    }
}
impl AstNode for ast::DictExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::DictExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::DictExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::DictExpr {
            keys,
            values,
            range: _,
        } = self;

        for (key, value) in keys.iter().zip(values) {
            if let Some(key) = key {
                visitor.visit_expr(key);
            }
            visitor.visit_expr(value);
        }
    }
}
impl AstNode for ast::SetExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::SetExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::SetExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::SetExpr { elts, range: _ } = self;

        for expr in elts {
            visitor.visit_expr(expr);
        }
    }
}
impl AstNode for ast::ListCompExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::ListCompExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::ListCompExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::ListCompExpr {
            elt,
            generators,
            range: _,
        } = self;

        visitor.visit_expr(elt);
        for comprehension in generators {
            visitor.visit_comprehension(comprehension);
        }
    }
}
impl AstNode for ast::SetCompExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::SetCompExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::SetCompExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::SetCompExpr {
            elt,
            generators,
            range: _,
        } = self;

        visitor.visit_expr(elt);
        for comprehension in generators {
            visitor.visit_comprehension(comprehension);
        }
    }
}
impl AstNode for ast::DictCompExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::DictCompExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::DictCompExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::DictCompExpr {
            key,
            value,
            generators,
            range: _,
        } = self;

        visitor.visit_expr(key);
        visitor.visit_expr(value);

        for comprehension in generators {
            visitor.visit_comprehension(comprehension);
        }
    }
}
impl AstNode for ast::GeneratorExpExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::GeneratorExpExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::GeneratorExpExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::GeneratorExpExpr {
            elt,
            generators,
            range: _,
        } = self;
        visitor.visit_expr(elt);
        for comprehension in generators {
            visitor.visit_comprehension(comprehension);
        }
    }
}
impl AstNode for ast::AwaitExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::AwaitExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::AwaitExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::AwaitExpr { value, range: _ } = self;
        visitor.visit_expr(value);
    }
}
impl AstNode for ast::YieldExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::YieldExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::YieldExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::YieldExpr { value, range: _ } = self;
        if let Some(expr) = value {
            visitor.visit_expr(expr);
        }
    }
}
impl AstNode for ast::YieldFromExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::YieldFromExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::YieldFromExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::YieldFromExpr { value, range: _ } = self;
        visitor.visit_expr(value);
    }
}
impl AstNode for ast::CompareExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::CompareExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::CompareExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::CompareExpr {
            left,
            ops,
            comparators,
            range: _,
        } = self;

        visitor.visit_expr(left);

        for (op, comparator) in ops.iter().zip(comparators) {
            visitor.visit_cmp_op(op);
            visitor.visit_expr(comparator);
        }
    }
}
impl AstNode for ast::CallExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::CallExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::CallExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::CallExpr {
            func,
            arguments,
            range: _,
        } = self;
        visitor.visit_expr(func);
        visitor.visit_arguments(arguments);
    }
}
impl AstNode for ast::FStringExpressionElement {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::FStringessionElementExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::FStringessionElementExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::FStringExpressionElement {
            expression,
            format_spec,
            ..
        } = self;
        visitor.visit_expr(expression);

        if let Some(format_spec) = format_spec {
            for spec_part in &format_spec.elements {
                visitor.visit_f_string_element(spec_part);
            }
        }
    }
}
impl AstNode for ast::FStringLiteralElement {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::FStringLiteralElement(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::FStringLiteralElement(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, _visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
    }
}
impl AstNode for ast::FStringExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::FStringExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::FStringExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::FStringExpr { value, range: _ } = self;

        for f_string_part in value {
            match f_string_part {
                ast::FStringPart::Literal(string_literal) => {
                    visitor.visit_string_literal(string_literal);
                }
                ast::FStringPart::FString(f_string) => {
                    visitor.visit_f_string(f_string);
                }
            }
        }
    }
}
impl AstNode for ast::StringLiteralExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StringLiteralExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StringLiteralExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::StringLiteralExpr { value, range: _ } = self;

        for string_literal in value {
            visitor.visit_string_literal(string_literal);
        }
    }
}
impl AstNode for ast::BytesLiteralExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::BytesLiteralExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::BytesLiteralExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::BytesLiteralExpr { value, range: _ } = self;

        for bytes_literal in value {
            visitor.visit_bytes_literal(bytes_literal);
        }
    }
}
impl AstNode for ast::NumberLiteralExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::NumberLiteralExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::NumberLiteralExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, _visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
    }
}
impl AstNode for ast::BooleanLiteralExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::BooleanLiteralExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::BooleanLiteralExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, _visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
    }
}
impl AstNode for ast::NoneLiteralExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::NoneLiteralExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::NoneLiteralExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, _visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
    }
}
impl AstNode for ast::EllipsisLiteralExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::EllipsisLiteralExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::EllipsisLiteralExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, _visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
    }
}
impl AstNode for ast::AttributeExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::AttributeExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::AttributeExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::AttributeExpr {
            value,
            attr: _,
            ctx: _,
            range: _,
        } = self;

        visitor.visit_expr(value);
    }
}
impl AstNode for ast::SubscriptExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::SubscriptExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::SubscriptExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::SubscriptExpr {
            value,
            slice,
            ctx: _,
            range: _,
        } = self;
        visitor.visit_expr(value);
        visitor.visit_expr(slice);
    }
}
impl AstNode for ast::StarredExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StarredExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StarredExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::StarredExpr {
            value,
            ctx: _,
            range: _,
        } = self;

        visitor.visit_expr(value);
    }
}
impl AstNode for ast::NameExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::NameExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::NameExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    #[inline]
    fn visit_preorder<'a, V>(&'a self, _visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::NameExpr {
            id: _,
            ctx: _,
            range: _,
        } = self;
    }
}
impl AstNode for ast::ListExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::ListExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::ListExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::ListExpr {
            elts,
            ctx: _,
            range: _,
        } = self;

        for expr in elts {
            visitor.visit_expr(expr);
        }
    }
}
impl AstNode for ast::TupleExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::TupleExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::TupleExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::TupleExpr {
            elts,
            ctx: _,
            range: _,
        } = self;

        for expr in elts {
            visitor.visit_expr(expr);
        }
    }
}
impl AstNode for ast::SliceExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::SliceExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::SliceExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }
    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::SliceExpr {
            lower,
            upper,
            step,
            range: _,
        } = self;

        if let Some(expr) = lower {
            visitor.visit_expr(expr);
        }
        if let Some(expr) = upper {
            visitor.visit_expr(expr);
        }
        if let Some(expr) = step {
            visitor.visit_expr(expr);
        }
    }
}
impl AstNode for ast::IpyEscapeCommandExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::IpyEscapeCommandExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::IpyEscapeCommandExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    #[inline]
    fn visit_preorder<'a, V>(&'a self, _visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::IpyEscapeCommandExpr {
            range: _,
            kind: _,
            value: _,
        } = self;
    }
}
impl AstNode for ast::InvalidExpr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::InvalidExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::InvalidExpr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, _visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
    }
}
impl AstNode for ast::ExceptHandlerExceptHandler {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::ExceptHandlerExceptHandler(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::ExceptHandlerExceptHandler(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::ExceptHandlerExceptHandler {
            range: _,
            type_,
            name: _,
            body,
        } = self;
        if let Some(expr) = type_ {
            visitor.visit_expr(expr);
        }
        visitor.visit_body(body);
    }
}
impl AstNode for ast::PatternMatchValue {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::PatternMatchValue(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::PatternMatchValue(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::PatternMatchValue { value, range: _ } = self;
        visitor.visit_expr(value);
    }
}
impl AstNode for ast::PatternMatchSingleton {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::PatternMatchSingleton(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::PatternMatchSingleton(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::PatternMatchSingleton { value, range: _ } = self;
        visitor.visit_singleton(value);
    }
}
impl AstNode for ast::PatternMatchSequence {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::PatternMatchSequence(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::PatternMatchSequence(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::PatternMatchSequence { patterns, range: _ } = self;
        for pattern in patterns {
            visitor.visit_pattern(pattern);
        }
    }
}
impl AstNode for ast::PatternMatchMapping {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::PatternMatchMapping(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::PatternMatchMapping(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::PatternMatchMapping {
            keys,
            patterns,
            range: _,
            rest: _,
        } = self;
        for (key, pattern) in keys.iter().zip(patterns) {
            visitor.visit_expr(key);
            visitor.visit_pattern(pattern);
        }
    }
}
impl AstNode for ast::PatternMatchClass {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::PatternMatchClass(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::PatternMatchClass(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::PatternMatchClass {
            cls,
            arguments: parameters,
            range: _,
        } = self;
        visitor.visit_expr(cls);
        visitor.visit_pattern_arguments(parameters);
    }
}
impl AstNode for ast::PatternMatchStar {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::PatternMatchStar(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::PatternMatchStar(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    #[inline]
    fn visit_preorder<'a, V>(&'a self, _visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::PatternMatchStar { range: _, name: _ } = self;
    }
}
impl AstNode for ast::PatternMatchAs {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::PatternMatchAs(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::PatternMatchAs(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::PatternMatchAs {
            pattern,
            range: _,
            name: _,
        } = self;
        if let Some(pattern) = pattern {
            visitor.visit_pattern(pattern);
        }
    }
}
impl AstNode for ast::PatternMatchOr {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::PatternMatchOr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::PatternMatchOr(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::PatternMatchOr { patterns, range: _ } = self;
        for pattern in patterns {
            visitor.visit_pattern(pattern);
        }
    }
}
impl AstNode for PatternArguments {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::PatternArguments(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::PatternArguments(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let PatternArguments {
            range: _,
            patterns,
            keywords,
        } = self;

        for pattern in patterns {
            visitor.visit_pattern(pattern);
        }

        for keyword in keywords {
            visitor.visit_pattern_keyword(keyword);
        }
    }
}
impl AstNode for PatternKeyword {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::PatternKeyword(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::PatternKeyword(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let PatternKeyword {
            range: _,
            attr: _,
            pattern,
        } = self;

        visitor.visit_pattern(pattern);
    }
}
impl AstNode for ast::PatternMatchInvalid {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::PatternMatchInvalid(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::PatternMatchInvalid(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, _visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
    }
}

impl AstNode for Comprehension {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::Comprehension(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::Comprehension(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::Comprehension {
            range: _,
            target,
            iter,
            ifs,
            is_async: _,
        } = self;
        visitor.visit_expr(target);
        visitor.visit_expr(iter);

        for expr in ifs {
            visitor.visit_expr(expr);
        }
    }
}
impl AstNode for Arguments {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::Arguments(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::Arguments(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        for arg_or_keyword in self.arguments_source_order() {
            match arg_or_keyword {
                ArgOrKeyword::Arg(arg) => visitor.visit_expr(arg),
                ArgOrKeyword::Keyword(keyword) => visitor.visit_keyword(keyword),
            }
        }
    }
}
impl AstNode for Parameters {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::Parameters(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::Parameters(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::Parameters {
            range: _,
            posonlyargs,
            args,
            vararg,
            kwonlyargs,
            kwarg,
        } = self;
        for arg in posonlyargs.iter().chain(args) {
            visitor.visit_parameter_with_default(arg);
        }

        if let Some(arg) = vararg {
            visitor.visit_parameter(arg);
        }

        for arg in kwonlyargs {
            visitor.visit_parameter_with_default(arg);
        }

        if let Some(arg) = kwarg {
            visitor.visit_parameter(arg);
        }
    }
}
impl AstNode for Parameter {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::Parameter(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::Parameter(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::Parameter {
            range: _,
            name: _,
            annotation,
        } = self;

        if let Some(expr) = annotation {
            visitor.visit_annotation(expr);
        }
    }
}
impl AstNode for ParameterWithDefault {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::ParameterWithDefault(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::ParameterWithDefault(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::ParameterWithDefault {
            range: _,
            parameter,
            default,
        } = self;
        visitor.visit_parameter(parameter);
        if let Some(expr) = default {
            visitor.visit_expr(expr);
        }
    }
}
impl AstNode for Keyword {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::Keyword(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::Keyword(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::Keyword {
            range: _,
            arg: _,
            value,
        } = self;

        visitor.visit_expr(value);
    }
}
impl AstNode for Alias {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::Alias(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::Alias(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    #[inline]
    fn visit_preorder<'a, V>(&'a self, _visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::Alias {
            range: _,
            name: _,
            asname: _,
        } = self;
    }
}
impl AstNode for WithItem {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::WithItem(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::WithItem(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::WithItem {
            range: _,
            context_expr,
            optional_vars,
        } = self;

        visitor.visit_expr(context_expr);

        if let Some(expr) = optional_vars {
            visitor.visit_expr(expr);
        }
    }
}
impl AstNode for MatchCase {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::MatchCase(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::MatchCase(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::MatchCase {
            range: _,
            pattern,
            guard,
            body,
        } = self;

        visitor.visit_pattern(pattern);
        if let Some(expr) = guard {
            visitor.visit_expr(expr);
        }
        visitor.visit_body(body);
    }
}

impl AstNode for Decorator {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::Decorator(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::Decorator(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::Decorator {
            range: _,
            expression,
        } = self;

        visitor.visit_expr(expression);
    }
}
impl AstNode for ast::TypeParams {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::TypeParams(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::TypeParams(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::TypeParams {
            range: _,
            type_params,
        } = self;

        for type_param in type_params {
            visitor.visit_type_param(type_param);
        }
    }
}
impl AstNode for ast::TypeParamTypeVar {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::TypeParamTypeVar(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::TypeParamTypeVar(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::TypeParamTypeVar {
            bound,
            name: _,
            range: _,
        } = self;

        if let Some(expr) = bound {
            visitor.visit_expr(expr);
        }
    }
}
impl AstNode for ast::TypeParamTypeVarTuple {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::TypeParamTypeVarTuple(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::TypeParamTypeVarTuple(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    #[inline]
    fn visit_preorder<'a, V>(&'a self, _visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::TypeParamTypeVarTuple { range: _, name: _ } = self;
    }
}
impl AstNode for ast::TypeParamParamSpec {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::TypeParamParamSpec(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::TypeParamParamSpec(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    #[inline]
    fn visit_preorder<'a, V>(&'a self, _visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::TypeParamParamSpec { range: _, name: _ } = self;
    }
}
impl AstNode for ast::FString {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::FString(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::FString(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
        let ast::FString { elements, range: _ } = self;

        for fstring_element in elements {
            visitor.visit_f_string_element(fstring_element);
        }
    }
}
impl AstNode for ast::StringLiteral {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::StringLiteral(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::StringLiteral(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, _visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
    }
}
impl AstNode for ast::BytesLiteral {
    fn cast(kind: AnyNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let AnyNode::BytesLiteral(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn cast_ref(kind: AnyNodeRef) -> Option<&Self> {
        if let AnyNodeRef::BytesLiteral(node) = kind {
            Some(node)
        } else {
            None
        }
    }

    fn as_any_node_ref(&self) -> AnyNodeRef {
        AnyNodeRef::from(self)
    }

    fn into_any_node(self) -> AnyNode {
        AnyNode::from(self)
    }

    fn visit_preorder<'a, V>(&'a self, _visitor: &mut V)
    where
        V: PreorderVisitor<'a> + ?Sized,
    {
    }
}

impl From<Stmt> for AnyNode {
    fn from(stmt: Stmt) -> Self {
        match stmt {
            Stmt::FunctionDef(node) => AnyNode::StmtFunctionDef(node),
            Stmt::ClassDef(node) => AnyNode::StmtClassDef(node),
            Stmt::Return(node) => AnyNode::StmtReturn(node),
            Stmt::Delete(node) => AnyNode::StmtDelete(node),
            Stmt::TypeAlias(node) => AnyNode::StmtTypeAlias(node),
            Stmt::Assign(node) => AnyNode::StmtAssign(node),
            Stmt::AugAssign(node) => AnyNode::StmtAugAssign(node),
            Stmt::AnnAssign(node) => AnyNode::StmtAnnAssign(node),
            Stmt::For(node) => AnyNode::StmtFor(node),
            Stmt::While(node) => AnyNode::StmtWhile(node),
            Stmt::If(node) => AnyNode::StmtIf(node),
            Stmt::With(node) => AnyNode::StmtWith(node),
            Stmt::Match(node) => AnyNode::StmtMatch(node),
            Stmt::Raise(node) => AnyNode::StmtRaise(node),
            Stmt::Try(node) => AnyNode::StmtTry(node),
            Stmt::Assert(node) => AnyNode::StmtAssert(node),
            Stmt::Import(node) => AnyNode::StmtImport(node),
            Stmt::ImportFrom(node) => AnyNode::StmtImportFrom(node),
            Stmt::Global(node) => AnyNode::StmtGlobal(node),
            Stmt::Nonlocal(node) => AnyNode::StmtNonlocal(node),
            Stmt::Expr(node) => AnyNode::StmtExpr(node),
            Stmt::Pass(node) => AnyNode::StmtPass(node),
            Stmt::Break(node) => AnyNode::StmtBreak(node),
            Stmt::Continue(node) => AnyNode::StmtContinue(node),
            Stmt::IpyEscapeCommand(node) => AnyNode::StmtIpyEscapeCommand(node),
        }
    }
}

impl From<Expr> for AnyNode {
    fn from(expr: Expr) -> Self {
        match expr {
            Expr::BoolOp(node) => AnyNode::BoolOpExpr(node),
            Expr::NamedExpr(node) => AnyNode::NamedExprExpr(node),
            Expr::BinOp(node) => AnyNode::BinOpExpr(node),
            Expr::UnaryOp(node) => AnyNode::UnaryOpExpr(node),
            Expr::Lambda(node) => AnyNode::LambdaExpr(node),
            Expr::IfExp(node) => AnyNode::IfExpr(node),
            Expr::Dict(node) => AnyNode::DictExpr(node),
            Expr::Set(node) => AnyNode::SetExpr(node),
            Expr::ListComp(node) => AnyNode::ListCompExpr(node),
            Expr::SetComp(node) => AnyNode::SetCompExpr(node),
            Expr::DictComp(node) => AnyNode::DictCompExpr(node),
            Expr::GeneratorExp(node) => AnyNode::GeneratorExpExpr(node),
            Expr::Await(node) => AnyNode::AwaitExpr(node),
            Expr::Yield(node) => AnyNode::YieldExpr(node),
            Expr::YieldFrom(node) => AnyNode::YieldFromExpr(node),
            Expr::Compare(node) => AnyNode::CompareExpr(node),
            Expr::Call(node) => AnyNode::CallExpr(node),
            Expr::FString(node) => AnyNode::FStringExpr(node),
            Expr::StringLiteral(node) => AnyNode::StringLiteralExpr(node),
            Expr::BytesLiteral(node) => AnyNode::BytesLiteralExpr(node),
            Expr::NumberLiteral(node) => AnyNode::NumberLiteralExpr(node),
            Expr::BooleanLiteral(node) => AnyNode::BooleanLiteralExpr(node),
            Expr::NoneLiteral(node) => AnyNode::NoneLiteralExpr(node),
            Expr::EllipsisLiteral(node) => AnyNode::EllipsisLiteralExpr(node),
            Expr::Attribute(node) => AnyNode::AttributeExpr(node),
            Expr::Subscript(node) => AnyNode::SubscriptExpr(node),
            Expr::Starred(node) => AnyNode::StarredExpr(node),
            Expr::Name(node) => AnyNode::NameExpr(node),
            Expr::List(node) => AnyNode::ListExpr(node),
            Expr::Tuple(node) => AnyNode::TupleExpr(node),
            Expr::Slice(node) => AnyNode::SliceExpr(node),
            Expr::IpyEscapeCommand(node) => AnyNode::IpyEscapeCommandExpr(node),
            Expr::Invalid(range) => AnyNode::InvalidExpr(range),
        }
    }
}

impl From<Mod> for AnyNode {
    fn from(module: Mod) -> Self {
        match module {
            Mod::Module(node) => AnyNode::ModModule(node),
            Mod::Expression(node) => AnyNode::ModessionExpr(node),
        }
    }
}

impl From<FStringElement> for AnyNode {
    fn from(element: FStringElement) -> Self {
        match element {
            FStringElement::Literal(node) => AnyNode::FStringLiteralElement(node),
            FStringElement::Expression(node) => AnyNode::FStringessionElementExpr(node),
            FStringElement::Invalid(node) => AnyNode::FStringInvalidElement(node),
        }
    }
}

impl From<Pattern> for AnyNode {
    fn from(pattern: Pattern) -> Self {
        match pattern {
            Pattern::MatchValue(node) => AnyNode::PatternMatchValue(node),
            Pattern::MatchSingleton(node) => AnyNode::PatternMatchSingleton(node),
            Pattern::MatchSequence(node) => AnyNode::PatternMatchSequence(node),
            Pattern::MatchMapping(node) => AnyNode::PatternMatchMapping(node),
            Pattern::MatchClass(node) => AnyNode::PatternMatchClass(node),
            Pattern::MatchStar(node) => AnyNode::PatternMatchStar(node),
            Pattern::MatchAs(node) => AnyNode::PatternMatchAs(node),
            Pattern::MatchOr(node) => AnyNode::PatternMatchOr(node),
            Pattern::Invalid(node) => AnyNode::PatternMatchInvalid(node),
        }
    }
}

impl From<ExceptHandler> for AnyNode {
    fn from(handler: ExceptHandler) -> Self {
        match handler {
            ExceptHandler::ExceptHandler(handler) => AnyNode::ExceptHandlerExceptHandler(handler),
        }
    }
}

impl From<ast::ModModule> for AnyNode {
    fn from(node: ast::ModModule) -> Self {
        AnyNode::ModModule(node)
    }
}

impl From<ast::ModExpression> for AnyNode {
    fn from(node: ast::ModExpression) -> Self {
        AnyNode::ModessionExpr(node)
    }
}

impl From<ast::FunctionDefStmt> for AnyNode {
    fn from(node: ast::FunctionDefStmt) -> Self {
        AnyNode::StmtFunctionDef(node)
    }
}

impl From<ast::ClassDefStmt> for AnyNode {
    fn from(node: ast::ClassDefStmt) -> Self {
        AnyNode::StmtClassDef(node)
    }
}

impl From<ast::ReturnStmt> for AnyNode {
    fn from(node: ast::ReturnStmt) -> Self {
        AnyNode::StmtReturn(node)
    }
}

impl From<ast::DeleteStmt> for AnyNode {
    fn from(node: ast::DeleteStmt) -> Self {
        AnyNode::StmtDelete(node)
    }
}

impl From<ast::TypeAliasStmt> for AnyNode {
    fn from(node: ast::TypeAliasStmt) -> Self {
        AnyNode::StmtTypeAlias(node)
    }
}

impl From<ast::AssignStmt> for AnyNode {
    fn from(node: ast::AssignStmt) -> Self {
        AnyNode::StmtAssign(node)
    }
}

impl From<ast::AugAssignStmt> for AnyNode {
    fn from(node: ast::AugAssignStmt) -> Self {
        AnyNode::StmtAugAssign(node)
    }
}

impl From<ast::AnnAssignStmt> for AnyNode {
    fn from(node: ast::AnnAssignStmt) -> Self {
        AnyNode::StmtAnnAssign(node)
    }
}

impl From<ast::ForStmt> for AnyNode {
    fn from(node: ast::ForStmt) -> Self {
        AnyNode::StmtFor(node)
    }
}

impl From<ast::WhileStmt> for AnyNode {
    fn from(node: ast::WhileStmt) -> Self {
        AnyNode::StmtWhile(node)
    }
}

impl From<ast::IfStmt> for AnyNode {
    fn from(node: ast::IfStmt) -> Self {
        AnyNode::StmtIf(node)
    }
}

impl From<ast::ElifElseClause> for AnyNode {
    fn from(node: ast::ElifElseClause) -> Self {
        AnyNode::ElifElseClause(node)
    }
}

impl From<ast::WithStmt> for AnyNode {
    fn from(node: ast::WithStmt) -> Self {
        AnyNode::StmtWith(node)
    }
}

impl From<ast::MatchStmt> for AnyNode {
    fn from(node: ast::MatchStmt) -> Self {
        AnyNode::StmtMatch(node)
    }
}

impl From<ast::RaiseStmt> for AnyNode {
    fn from(node: ast::RaiseStmt) -> Self {
        AnyNode::StmtRaise(node)
    }
}

impl From<ast::TryStmt> for AnyNode {
    fn from(node: ast::TryStmt) -> Self {
        AnyNode::StmtTry(node)
    }
}

impl From<ast::AssertStmt> for AnyNode {
    fn from(node: ast::AssertStmt) -> Self {
        AnyNode::StmtAssert(node)
    }
}

impl From<ast::ImportStmt> for AnyNode {
    fn from(node: ast::ImportStmt) -> Self {
        AnyNode::StmtImport(node)
    }
}

impl From<ast::ImportFromStmt> for AnyNode {
    fn from(node: ast::ImportFromStmt) -> Self {
        AnyNode::StmtImportFrom(node)
    }
}

impl From<ast::GlobalStmt> for AnyNode {
    fn from(node: ast::GlobalStmt) -> Self {
        AnyNode::StmtGlobal(node)
    }
}

impl From<ast::NonlocalStmt> for AnyNode {
    fn from(node: ast::NonlocalStmt) -> Self {
        AnyNode::StmtNonlocal(node)
    }
}

impl From<ast::ExprStmt> for AnyNode {
    fn from(node: ast::ExprStmt) -> Self {
        AnyNode::StmtExpr(node)
    }
}

impl From<ast::PassStmt> for AnyNode {
    fn from(node: ast::PassStmt) -> Self {
        AnyNode::StmtPass(node)
    }
}

impl From<ast::BreakStmt> for AnyNode {
    fn from(node: ast::BreakStmt) -> Self {
        AnyNode::StmtBreak(node)
    }
}

impl From<ast::ContinueStmt> for AnyNode {
    fn from(node: ast::ContinueStmt) -> Self {
        AnyNode::StmtContinue(node)
    }
}

impl From<ast::IpyEscapeCommandStmt> for AnyNode {
    fn from(node: ast::IpyEscapeCommandStmt) -> Self {
        AnyNode::StmtIpyEscapeCommand(node)
    }
}

impl From<ast::BoolOpExpr> for AnyNode {
    fn from(node: ast::BoolOpExpr) -> Self {
        AnyNode::BoolOpExpr(node)
    }
}

impl From<ast::NamedExpr> for AnyNode {
    fn from(node: ast::NamedExpr) -> Self {
        AnyNode::NamedExprExpr(node)
    }
}

impl From<ast::BinOpExpr> for AnyNode {
    fn from(node: ast::BinOpExpr) -> Self {
        AnyNode::BinOpExpr(node)
    }
}

impl From<ast::UnaryOpExpr> for AnyNode {
    fn from(node: ast::UnaryOpExpr) -> Self {
        AnyNode::UnaryOpExpr(node)
    }
}

impl From<ast::LambdaExpr> for AnyNode {
    fn from(node: ast::LambdaExpr) -> Self {
        AnyNode::LambdaExpr(node)
    }
}

impl From<ast::IfExpr> for AnyNode {
    fn from(node: ast::IfExpr) -> Self {
        AnyNode::IfExpr(node)
    }
}

impl From<ast::DictExpr> for AnyNode {
    fn from(node: ast::DictExpr) -> Self {
        AnyNode::DictExpr(node)
    }
}

impl From<ast::SetExpr> for AnyNode {
    fn from(node: ast::SetExpr) -> Self {
        AnyNode::SetExpr(node)
    }
}

impl From<ast::ListCompExpr> for AnyNode {
    fn from(node: ast::ListCompExpr) -> Self {
        AnyNode::ListCompExpr(node)
    }
}

impl From<ast::SetCompExpr> for AnyNode {
    fn from(node: ast::SetCompExpr) -> Self {
        AnyNode::SetCompExpr(node)
    }
}

impl From<ast::DictCompExpr> for AnyNode {
    fn from(node: ast::DictCompExpr) -> Self {
        AnyNode::DictCompExpr(node)
    }
}

impl From<ast::GeneratorExpExpr> for AnyNode {
    fn from(node: ast::GeneratorExpExpr) -> Self {
        AnyNode::GeneratorExpExpr(node)
    }
}

impl From<ast::AwaitExpr> for AnyNode {
    fn from(node: ast::AwaitExpr) -> Self {
        AnyNode::AwaitExpr(node)
    }
}

impl From<ast::YieldExpr> for AnyNode {
    fn from(node: ast::YieldExpr) -> Self {
        AnyNode::YieldExpr(node)
    }
}

impl From<ast::YieldFromExpr> for AnyNode {
    fn from(node: ast::YieldFromExpr) -> Self {
        AnyNode::YieldFromExpr(node)
    }
}

impl From<ast::CompareExpr> for AnyNode {
    fn from(node: ast::CompareExpr) -> Self {
        AnyNode::CompareExpr(node)
    }
}

impl From<ast::CallExpr> for AnyNode {
    fn from(node: ast::CallExpr) -> Self {
        AnyNode::CallExpr(node)
    }
}

impl From<ast::FStringExpressionElement> for AnyNode {
    fn from(node: ast::FStringExpressionElement) -> Self {
        AnyNode::FStringessionElementExpr(node)
    }
}

impl From<ast::FStringLiteralElement> for AnyNode {
    fn from(node: ast::FStringLiteralElement) -> Self {
        AnyNode::FStringLiteralElement(node)
    }
}

impl From<ast::FStringExpr> for AnyNode {
    fn from(node: ast::FStringExpr) -> Self {
        AnyNode::FStringExpr(node)
    }
}

impl From<ast::StringLiteralExpr> for AnyNode {
    fn from(node: ast::StringLiteralExpr) -> Self {
        AnyNode::StringLiteralExpr(node)
    }
}

impl From<ast::BytesLiteralExpr> for AnyNode {
    fn from(node: ast::BytesLiteralExpr) -> Self {
        AnyNode::BytesLiteralExpr(node)
    }
}

impl From<ast::NumberLiteralExpr> for AnyNode {
    fn from(node: ast::NumberLiteralExpr) -> Self {
        AnyNode::NumberLiteralExpr(node)
    }
}

impl From<ast::BooleanLiteralExpr> for AnyNode {
    fn from(node: ast::BooleanLiteralExpr) -> Self {
        AnyNode::BooleanLiteralExpr(node)
    }
}

impl From<ast::NoneLiteralExpr> for AnyNode {
    fn from(node: ast::NoneLiteralExpr) -> Self {
        AnyNode::NoneLiteralExpr(node)
    }
}

impl From<ast::EllipsisLiteralExpr> for AnyNode {
    fn from(node: ast::EllipsisLiteralExpr) -> Self {
        AnyNode::EllipsisLiteralExpr(node)
    }
}

impl From<ast::AttributeExpr> for AnyNode {
    fn from(node: ast::AttributeExpr) -> Self {
        AnyNode::AttributeExpr(node)
    }
}

impl From<ast::SubscriptExpr> for AnyNode {
    fn from(node: ast::SubscriptExpr) -> Self {
        AnyNode::SubscriptExpr(node)
    }
}

impl From<ast::StarredExpr> for AnyNode {
    fn from(node: ast::StarredExpr) -> Self {
        AnyNode::StarredExpr(node)
    }
}

impl From<ast::NameExpr> for AnyNode {
    fn from(node: ast::NameExpr) -> Self {
        AnyNode::NameExpr(node)
    }
}

impl From<ast::ListExpr> for AnyNode {
    fn from(node: ast::ListExpr) -> Self {
        AnyNode::ListExpr(node)
    }
}

impl From<ast::TupleExpr> for AnyNode {
    fn from(node: ast::TupleExpr) -> Self {
        AnyNode::TupleExpr(node)
    }
}

impl From<ast::SliceExpr> for AnyNode {
    fn from(node: ast::SliceExpr) -> Self {
        AnyNode::SliceExpr(node)
    }
}

impl From<ast::IpyEscapeCommandExpr> for AnyNode {
    fn from(node: ast::IpyEscapeCommandExpr) -> Self {
        AnyNode::IpyEscapeCommandExpr(node)
    }
}

impl From<ast::InvalidExpr> for AnyNode {
    fn from(node: ast::InvalidExpr) -> Self {
        AnyNode::InvalidExpr(node)
    }
}

impl From<ast::ExceptHandlerExceptHandler> for AnyNode {
    fn from(node: ast::ExceptHandlerExceptHandler) -> Self {
        AnyNode::ExceptHandlerExceptHandler(node)
    }
}

impl From<ast::PatternMatchValue> for AnyNode {
    fn from(node: ast::PatternMatchValue) -> Self {
        AnyNode::PatternMatchValue(node)
    }
}

impl From<ast::PatternMatchSingleton> for AnyNode {
    fn from(node: ast::PatternMatchSingleton) -> Self {
        AnyNode::PatternMatchSingleton(node)
    }
}

impl From<ast::PatternMatchSequence> for AnyNode {
    fn from(node: ast::PatternMatchSequence) -> Self {
        AnyNode::PatternMatchSequence(node)
    }
}

impl From<ast::PatternMatchMapping> for AnyNode {
    fn from(node: ast::PatternMatchMapping) -> Self {
        AnyNode::PatternMatchMapping(node)
    }
}

impl From<ast::PatternMatchClass> for AnyNode {
    fn from(node: ast::PatternMatchClass) -> Self {
        AnyNode::PatternMatchClass(node)
    }
}

impl From<ast::PatternMatchStar> for AnyNode {
    fn from(node: ast::PatternMatchStar) -> Self {
        AnyNode::PatternMatchStar(node)
    }
}

impl From<ast::PatternMatchAs> for AnyNode {
    fn from(node: ast::PatternMatchAs) -> Self {
        AnyNode::PatternMatchAs(node)
    }
}

impl From<ast::PatternMatchOr> for AnyNode {
    fn from(node: ast::PatternMatchOr) -> Self {
        AnyNode::PatternMatchOr(node)
    }
}

impl From<ast::PatternMatchInvalid> for AnyNode {
    fn from(node: ast::PatternMatchInvalid) -> Self {
        AnyNode::PatternMatchInvalid(node)
    }
}

impl From<PatternArguments> for AnyNode {
    fn from(node: PatternArguments) -> Self {
        AnyNode::PatternArguments(node)
    }
}

impl From<PatternKeyword> for AnyNode {
    fn from(node: PatternKeyword) -> Self {
        AnyNode::PatternKeyword(node)
    }
}

impl From<Comprehension> for AnyNode {
    fn from(node: Comprehension) -> Self {
        AnyNode::Comprehension(node)
    }
}
impl From<Arguments> for AnyNode {
    fn from(node: Arguments) -> Self {
        AnyNode::Arguments(node)
    }
}
impl From<Parameters> for AnyNode {
    fn from(node: Parameters) -> Self {
        AnyNode::Parameters(node)
    }
}
impl From<Parameter> for AnyNode {
    fn from(node: Parameter) -> Self {
        AnyNode::Parameter(node)
    }
}
impl From<ParameterWithDefault> for AnyNode {
    fn from(node: ParameterWithDefault) -> Self {
        AnyNode::ParameterWithDefault(node)
    }
}
impl From<Keyword> for AnyNode {
    fn from(node: Keyword) -> Self {
        AnyNode::Keyword(node)
    }
}
impl From<Alias> for AnyNode {
    fn from(node: Alias) -> Self {
        AnyNode::Alias(node)
    }
}
impl From<WithItem> for AnyNode {
    fn from(node: WithItem) -> Self {
        AnyNode::WithItem(node)
    }
}
impl From<MatchCase> for AnyNode {
    fn from(node: MatchCase) -> Self {
        AnyNode::MatchCase(node)
    }
}
impl From<Decorator> for AnyNode {
    fn from(node: Decorator) -> Self {
        AnyNode::Decorator(node)
    }
}
impl From<TypeParams> for AnyNode {
    fn from(node: TypeParams) -> Self {
        AnyNode::TypeParams(node)
    }
}
impl From<TypeParamTypeVar> for AnyNode {
    fn from(node: TypeParamTypeVar) -> Self {
        AnyNode::TypeParamTypeVar(node)
    }
}

impl From<TypeParamTypeVarTuple> for AnyNode {
    fn from(node: TypeParamTypeVarTuple) -> Self {
        AnyNode::TypeParamTypeVarTuple(node)
    }
}

impl From<TypeParamParamSpec> for AnyNode {
    fn from(node: TypeParamParamSpec) -> Self {
        AnyNode::TypeParamParamSpec(node)
    }
}

impl From<ast::FString> for AnyNode {
    fn from(node: ast::FString) -> Self {
        AnyNode::FString(node)
    }
}

impl From<ast::StringLiteral> for AnyNode {
    fn from(node: ast::StringLiteral) -> Self {
        AnyNode::StringLiteral(node)
    }
}

impl From<ast::BytesLiteral> for AnyNode {
    fn from(node: ast::BytesLiteral) -> Self {
        AnyNode::BytesLiteral(node)
    }
}

impl Ranged for AnyNode {
    fn range(&self) -> TextRange {
        match self {
            AnyNode::ModModule(node) => node.range(),
            AnyNode::ModessionExpr(node) => node.range(),
            AnyNode::StmtFunctionDef(node) => node.range(),
            AnyNode::StmtClassDef(node) => node.range(),
            AnyNode::StmtReturn(node) => node.range(),
            AnyNode::StmtDelete(node) => node.range(),
            AnyNode::StmtTypeAlias(node) => node.range(),
            AnyNode::StmtAssign(node) => node.range(),
            AnyNode::StmtAugAssign(node) => node.range(),
            AnyNode::StmtAnnAssign(node) => node.range(),
            AnyNode::StmtFor(node) => node.range(),
            AnyNode::StmtWhile(node) => node.range(),
            AnyNode::StmtIf(node) => node.range(),
            AnyNode::StmtWith(node) => node.range(),
            AnyNode::StmtMatch(node) => node.range(),
            AnyNode::StmtRaise(node) => node.range(),
            AnyNode::StmtTry(node) => node.range(),
            AnyNode::StmtAssert(node) => node.range(),
            AnyNode::StmtImport(node) => node.range(),
            AnyNode::StmtImportFrom(node) => node.range(),
            AnyNode::StmtGlobal(node) => node.range(),
            AnyNode::StmtNonlocal(node) => node.range(),
            AnyNode::StmtExpr(node) => node.range(),
            AnyNode::StmtPass(node) => node.range(),
            AnyNode::StmtBreak(node) => node.range(),
            AnyNode::StmtContinue(node) => node.range(),
            AnyNode::StmtIpyEscapeCommand(node) => node.range(),
            AnyNode::BoolOpExpr(node) => node.range(),
            AnyNode::NamedExprExpr(node) => node.range(),
            AnyNode::BinOpExpr(node) => node.range(),
            AnyNode::UnaryOpExpr(node) => node.range(),
            AnyNode::LambdaExpr(node) => node.range(),
            AnyNode::IfExpr(node) => node.range(),
            AnyNode::DictExpr(node) => node.range(),
            AnyNode::SetExpr(node) => node.range(),
            AnyNode::ListCompExpr(node) => node.range(),
            AnyNode::SetCompExpr(node) => node.range(),
            AnyNode::DictCompExpr(node) => node.range(),
            AnyNode::GeneratorExpExpr(node) => node.range(),
            AnyNode::AwaitExpr(node) => node.range(),
            AnyNode::YieldExpr(node) => node.range(),
            AnyNode::YieldFromExpr(node) => node.range(),
            AnyNode::CompareExpr(node) => node.range(),
            AnyNode::CallExpr(node) => node.range(),
            AnyNode::FStringessionElementExpr(node) => node.range(),
            AnyNode::FStringLiteralElement(node) => node.range(),
            AnyNode::FStringInvalidElement(node) => node.range(),
            AnyNode::FStringExpr(node) => node.range(),
            AnyNode::StringLiteralExpr(node) => node.range(),
            AnyNode::BytesLiteralExpr(node) => node.range(),
            AnyNode::NumberLiteralExpr(node) => node.range(),
            AnyNode::BooleanLiteralExpr(node) => node.range(),
            AnyNode::NoneLiteralExpr(node) => node.range(),
            AnyNode::EllipsisLiteralExpr(node) => node.range(),
            AnyNode::AttributeExpr(node) => node.range(),
            AnyNode::SubscriptExpr(node) => node.range(),
            AnyNode::StarredExpr(node) => node.range(),
            AnyNode::NameExpr(node) => node.range(),
            AnyNode::ListExpr(node) => node.range(),
            AnyNode::TupleExpr(node) => node.range(),
            AnyNode::SliceExpr(node) => node.range(),
            AnyNode::IpyEscapeCommandExpr(node) => node.range(),
            AnyNode::InvalidExpr(node) => node.range(),
            AnyNode::ExceptHandlerExceptHandler(node) => node.range(),
            AnyNode::PatternMatchValue(node) => node.range(),
            AnyNode::PatternMatchSingleton(node) => node.range(),
            AnyNode::PatternMatchSequence(node) => node.range(),
            AnyNode::PatternMatchMapping(node) => node.range(),
            AnyNode::PatternMatchClass(node) => node.range(),
            AnyNode::PatternMatchStar(node) => node.range(),
            AnyNode::PatternMatchAs(node) => node.range(),
            AnyNode::PatternMatchOr(node) => node.range(),
            AnyNode::PatternArguments(node) => node.range(),
            AnyNode::PatternKeyword(node) => node.range(),
            AnyNode::PatternMatchInvalid(node) => node.range(),
            AnyNode::Comprehension(node) => node.range(),
            AnyNode::Arguments(node) => node.range(),
            AnyNode::Parameters(node) => node.range(),
            AnyNode::Parameter(node) => node.range(),
            AnyNode::ParameterWithDefault(node) => node.range(),
            AnyNode::Keyword(node) => node.range(),
            AnyNode::Alias(node) => node.range(),
            AnyNode::WithItem(node) => node.range(),
            AnyNode::MatchCase(node) => node.range(),
            AnyNode::Decorator(node) => node.range(),
            AnyNode::TypeParams(node) => node.range(),
            AnyNode::TypeParamTypeVar(node) => node.range(),
            AnyNode::TypeParamTypeVarTuple(node) => node.range(),
            AnyNode::TypeParamParamSpec(node) => node.range(),
            AnyNode::FString(node) => node.range(),
            AnyNode::StringLiteral(node) => node.range(),
            AnyNode::BytesLiteral(node) => node.range(),
            AnyNode::ElifElseClause(node) => node.range(),
        }
    }
}

#[derive(Copy, Clone, Debug, is_macro::Is, PartialEq)]
pub enum AnyNodeRef<'a> {
    ModModule(&'a ast::ModModule),
    ModessionExpr(&'a ast::ModExpression),
    StmtFunctionDef(&'a ast::FunctionDefStmt),
    StmtClassDef(&'a ast::ClassDefStmt),
    StmtReturn(&'a ast::ReturnStmt),
    StmtDelete(&'a ast::DeleteStmt),
    StmtTypeAlias(&'a ast::TypeAliasStmt),
    StmtAssign(&'a ast::AssignStmt),
    StmtAugAssign(&'a ast::AugAssignStmt),
    StmtAnnAssign(&'a ast::AnnAssignStmt),
    StmtFor(&'a ast::ForStmt),
    StmtWhile(&'a ast::WhileStmt),
    StmtIf(&'a ast::IfStmt),
    StmtWith(&'a ast::WithStmt),
    StmtMatch(&'a ast::MatchStmt),
    StmtRaise(&'a ast::RaiseStmt),
    StmtTry(&'a ast::TryStmt),
    StmtAssert(&'a ast::AssertStmt),
    StmtImport(&'a ast::ImportStmt),
    StmtImportFrom(&'a ast::ImportFromStmt),
    StmtGlobal(&'a ast::GlobalStmt),
    StmtNonlocal(&'a ast::NonlocalStmt),
    StmtExpr(&'a ast::ExprStmt),
    StmtPass(&'a ast::PassStmt),
    StmtBreak(&'a ast::BreakStmt),
    StmtContinue(&'a ast::ContinueStmt),
    StmtIpyEscapeCommand(&'a ast::IpyEscapeCommandStmt),
    BoolOpExpr(&'a ast::BoolOpExpr),
    NamedExprExpr(&'a ast::NamedExpr),
    BinOpExpr(&'a ast::BinOpExpr),
    UnaryOpExpr(&'a ast::UnaryOpExpr),
    LambdaExpr(&'a ast::LambdaExpr),
    IfExpr(&'a ast::IfExpr),
    DictExpr(&'a ast::DictExpr),
    SetExpr(&'a ast::SetExpr),
    ListCompExpr(&'a ast::ListCompExpr),
    SetCompExpr(&'a ast::SetCompExpr),
    DictCompExpr(&'a ast::DictCompExpr),
    GeneratorExpExpr(&'a ast::GeneratorExpExpr),
    AwaitExpr(&'a ast::AwaitExpr),
    YieldExpr(&'a ast::YieldExpr),
    YieldFromExpr(&'a ast::YieldFromExpr),
    CompareExpr(&'a ast::CompareExpr),
    CallExpr(&'a ast::CallExpr),
    FStringessionElementExpr(&'a ast::FStringExpressionElement),
    FStringLiteralElement(&'a ast::FStringLiteralElement),
    FStringInvalidElement(&'a ast::FStringInvalidElement),
    FStringExpr(&'a ast::FStringExpr),
    StringLiteralExpr(&'a ast::StringLiteralExpr),
    BytesLiteralExpr(&'a ast::BytesLiteralExpr),
    NumberLiteralExpr(&'a ast::NumberLiteralExpr),
    BooleanLiteralExpr(&'a ast::BooleanLiteralExpr),
    NoneLiteralExpr(&'a ast::NoneLiteralExpr),
    EllipsisLiteralExpr(&'a ast::EllipsisLiteralExpr),
    AttributeExpr(&'a ast::AttributeExpr),
    SubscriptExpr(&'a ast::SubscriptExpr),
    StarredExpr(&'a ast::StarredExpr),
    NameExpr(&'a ast::NameExpr),
    ListExpr(&'a ast::ListExpr),
    TupleExpr(&'a ast::TupleExpr),
    SliceExpr(&'a ast::SliceExpr),
    IpyEscapeCommandExpr(&'a ast::IpyEscapeCommandExpr),
    InvalidExpr(&'a ast::InvalidExpr),
    ExceptHandlerExceptHandler(&'a ast::ExceptHandlerExceptHandler),
    PatternMatchValue(&'a ast::PatternMatchValue),
    PatternMatchSingleton(&'a ast::PatternMatchSingleton),
    PatternMatchSequence(&'a ast::PatternMatchSequence),
    PatternMatchMapping(&'a ast::PatternMatchMapping),
    PatternMatchClass(&'a ast::PatternMatchClass),
    PatternMatchStar(&'a ast::PatternMatchStar),
    PatternMatchAs(&'a ast::PatternMatchAs),
    PatternMatchOr(&'a ast::PatternMatchOr),
    PatternArguments(&'a ast::PatternArguments),
    PatternKeyword(&'a ast::PatternKeyword),
    PatternMatchInvalid(&'a ast::PatternMatchInvalid),
    Comprehension(&'a Comprehension),
    Arguments(&'a Arguments),
    Parameters(&'a Parameters),
    Parameter(&'a Parameter),
    ParameterWithDefault(&'a ParameterWithDefault),
    Keyword(&'a Keyword),
    Alias(&'a Alias),
    WithItem(&'a WithItem),
    MatchCase(&'a MatchCase),
    Decorator(&'a Decorator),
    TypeParams(&'a TypeParams),
    TypeParamTypeVar(&'a TypeParamTypeVar),
    TypeParamTypeVarTuple(&'a TypeParamTypeVarTuple),
    TypeParamParamSpec(&'a TypeParamParamSpec),
    FString(&'a ast::FString),
    StringLiteral(&'a ast::StringLiteral),
    BytesLiteral(&'a ast::BytesLiteral),
    ElifElseClause(&'a ast::ElifElseClause),
}

impl<'a> AnyNodeRef<'a> {
    pub fn as_ptr(&self) -> NonNull<()> {
        match self {
            AnyNodeRef::ModModule(node) => NonNull::from(*node).cast(),
            AnyNodeRef::ModessionExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StmtFunctionDef(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StmtClassDef(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StmtReturn(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StmtDelete(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StmtTypeAlias(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StmtAssign(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StmtAugAssign(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StmtAnnAssign(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StmtFor(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StmtWhile(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StmtIf(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StmtWith(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StmtMatch(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StmtRaise(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StmtTry(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StmtAssert(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StmtImport(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StmtImportFrom(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StmtGlobal(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StmtNonlocal(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StmtExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StmtPass(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StmtBreak(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StmtContinue(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StmtIpyEscapeCommand(node) => NonNull::from(*node).cast(),
            AnyNodeRef::BoolOpExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::NamedExprExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::BinOpExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::UnaryOpExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::LambdaExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::IfExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::DictExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::SetExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::ListCompExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::SetCompExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::DictCompExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::GeneratorExpExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::AwaitExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::YieldExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::YieldFromExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::CompareExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::CallExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::FStringessionElementExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::FStringLiteralElement(node) => NonNull::from(*node).cast(),
            AnyNodeRef::FStringInvalidElement(node) => NonNull::from(*node).cast(),
            AnyNodeRef::FStringExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StringLiteralExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::BytesLiteralExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::NumberLiteralExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::BooleanLiteralExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::NoneLiteralExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::EllipsisLiteralExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::AttributeExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::SubscriptExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StarredExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::NameExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::ListExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::TupleExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::SliceExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::IpyEscapeCommandExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::InvalidExpr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::ExceptHandlerExceptHandler(node) => NonNull::from(*node).cast(),
            AnyNodeRef::PatternMatchValue(node) => NonNull::from(*node).cast(),
            AnyNodeRef::PatternMatchSingleton(node) => NonNull::from(*node).cast(),
            AnyNodeRef::PatternMatchSequence(node) => NonNull::from(*node).cast(),
            AnyNodeRef::PatternMatchMapping(node) => NonNull::from(*node).cast(),
            AnyNodeRef::PatternMatchClass(node) => NonNull::from(*node).cast(),
            AnyNodeRef::PatternMatchStar(node) => NonNull::from(*node).cast(),
            AnyNodeRef::PatternMatchAs(node) => NonNull::from(*node).cast(),
            AnyNodeRef::PatternMatchOr(node) => NonNull::from(*node).cast(),
            AnyNodeRef::PatternArguments(node) => NonNull::from(*node).cast(),
            AnyNodeRef::PatternKeyword(node) => NonNull::from(*node).cast(),
            AnyNodeRef::PatternMatchInvalid(node) => NonNull::from(*node).cast(),
            AnyNodeRef::Comprehension(node) => NonNull::from(*node).cast(),
            AnyNodeRef::Arguments(node) => NonNull::from(*node).cast(),
            AnyNodeRef::Parameters(node) => NonNull::from(*node).cast(),
            AnyNodeRef::Parameter(node) => NonNull::from(*node).cast(),
            AnyNodeRef::ParameterWithDefault(node) => NonNull::from(*node).cast(),
            AnyNodeRef::Keyword(node) => NonNull::from(*node).cast(),
            AnyNodeRef::Alias(node) => NonNull::from(*node).cast(),
            AnyNodeRef::WithItem(node) => NonNull::from(*node).cast(),
            AnyNodeRef::MatchCase(node) => NonNull::from(*node).cast(),
            AnyNodeRef::Decorator(node) => NonNull::from(*node).cast(),
            AnyNodeRef::TypeParams(node) => NonNull::from(*node).cast(),
            AnyNodeRef::TypeParamTypeVar(node) => NonNull::from(*node).cast(),
            AnyNodeRef::TypeParamTypeVarTuple(node) => NonNull::from(*node).cast(),
            AnyNodeRef::TypeParamParamSpec(node) => NonNull::from(*node).cast(),
            AnyNodeRef::FString(node) => NonNull::from(*node).cast(),
            AnyNodeRef::StringLiteral(node) => NonNull::from(*node).cast(),
            AnyNodeRef::BytesLiteral(node) => NonNull::from(*node).cast(),
            AnyNodeRef::ElifElseClause(node) => NonNull::from(*node).cast(),
        }
    }

    /// Compares two any node refs by their pointers (referential equality).
    pub fn ptr_eq(self, other: AnyNodeRef) -> bool {
        self.as_ptr().eq(&other.as_ptr()) && self.kind() == other.kind()
    }

    /// Returns the node's [`kind`](NodeKind) that has no data associated and is [`Copy`].
    pub const fn kind(self) -> NodeKind {
        match self {
            AnyNodeRef::ModModule(_) => NodeKind::ModModule,
            AnyNodeRef::ModessionExpr(_) => NodeKind::ModessionExpr,
            AnyNodeRef::StmtFunctionDef(_) => NodeKind::StmtFunctionDef,
            AnyNodeRef::StmtClassDef(_) => NodeKind::StmtClassDef,
            AnyNodeRef::StmtReturn(_) => NodeKind::StmtReturn,
            AnyNodeRef::StmtDelete(_) => NodeKind::StmtDelete,
            AnyNodeRef::StmtTypeAlias(_) => NodeKind::StmtTypeAlias,
            AnyNodeRef::StmtAssign(_) => NodeKind::StmtAssign,
            AnyNodeRef::StmtAugAssign(_) => NodeKind::StmtAugAssign,
            AnyNodeRef::StmtAnnAssign(_) => NodeKind::StmtAnnAssign,
            AnyNodeRef::StmtFor(_) => NodeKind::StmtFor,
            AnyNodeRef::StmtWhile(_) => NodeKind::StmtWhile,
            AnyNodeRef::StmtIf(_) => NodeKind::StmtIf,
            AnyNodeRef::StmtWith(_) => NodeKind::StmtWith,
            AnyNodeRef::StmtMatch(_) => NodeKind::StmtMatch,
            AnyNodeRef::StmtRaise(_) => NodeKind::StmtRaise,
            AnyNodeRef::StmtTry(_) => NodeKind::StmtTry,
            AnyNodeRef::StmtAssert(_) => NodeKind::StmtAssert,
            AnyNodeRef::StmtImport(_) => NodeKind::StmtImport,
            AnyNodeRef::StmtImportFrom(_) => NodeKind::StmtImportFrom,
            AnyNodeRef::StmtGlobal(_) => NodeKind::StmtGlobal,
            AnyNodeRef::StmtNonlocal(_) => NodeKind::StmtNonlocal,
            AnyNodeRef::StmtExpr(_) => NodeKind::StmtExpr,
            AnyNodeRef::StmtPass(_) => NodeKind::StmtPass,
            AnyNodeRef::StmtBreak(_) => NodeKind::StmtBreak,
            AnyNodeRef::StmtContinue(_) => NodeKind::StmtContinue,
            AnyNodeRef::StmtIpyEscapeCommand(_) => NodeKind::StmtIpyEscapeCommand,
            AnyNodeRef::BoolOpExpr(_) => NodeKind::BoolOpExpr,
            AnyNodeRef::NamedExprExpr(_) => NodeKind::NamedExprExpr,
            AnyNodeRef::BinOpExpr(_) => NodeKind::BinOpExpr,
            AnyNodeRef::UnaryOpExpr(_) => NodeKind::UnaryOpExpr,
            AnyNodeRef::LambdaExpr(_) => NodeKind::LambdaExpr,
            AnyNodeRef::IfExpr(_) => NodeKind::IfExpr,
            AnyNodeRef::DictExpr(_) => NodeKind::DictExpr,
            AnyNodeRef::SetExpr(_) => NodeKind::SetExpr,
            AnyNodeRef::ListCompExpr(_) => NodeKind::ListCompExpr,
            AnyNodeRef::SetCompExpr(_) => NodeKind::SetCompExpr,
            AnyNodeRef::DictCompExpr(_) => NodeKind::DictCompExpr,
            AnyNodeRef::GeneratorExpExpr(_) => NodeKind::GeneratorExpExpr,
            AnyNodeRef::AwaitExpr(_) => NodeKind::AwaitExpr,
            AnyNodeRef::YieldExpr(_) => NodeKind::YieldExpr,
            AnyNodeRef::YieldFromExpr(_) => NodeKind::YieldFromExpr,
            AnyNodeRef::CompareExpr(_) => NodeKind::CompareExpr,
            AnyNodeRef::CallExpr(_) => NodeKind::CallExpr,
            AnyNodeRef::FStringessionElementExpr(_) => NodeKind::FStringessionElementExpr,
            AnyNodeRef::FStringLiteralElement(_) => NodeKind::FStringLiteralElement,
            AnyNodeRef::FStringInvalidElement(_) => NodeKind::FStringInvalidElement,
            AnyNodeRef::FStringExpr(_) => NodeKind::FStringExpr,
            AnyNodeRef::StringLiteralExpr(_) => NodeKind::StringLiteralExpr,
            AnyNodeRef::BytesLiteralExpr(_) => NodeKind::BytesLiteralExpr,
            AnyNodeRef::NumberLiteralExpr(_) => NodeKind::NumberLiteralExpr,
            AnyNodeRef::BooleanLiteralExpr(_) => NodeKind::BooleanLiteralExpr,
            AnyNodeRef::NoneLiteralExpr(_) => NodeKind::NoneLiteralExpr,
            AnyNodeRef::EllipsisLiteralExpr(_) => NodeKind::EllipsisLiteralExpr,
            AnyNodeRef::AttributeExpr(_) => NodeKind::AttributeExpr,
            AnyNodeRef::SubscriptExpr(_) => NodeKind::SubscriptExpr,
            AnyNodeRef::StarredExpr(_) => NodeKind::StarredExpr,
            AnyNodeRef::NameExpr(_) => NodeKind::NameExpr,
            AnyNodeRef::ListExpr(_) => NodeKind::ListExpr,
            AnyNodeRef::TupleExpr(_) => NodeKind::TupleExpr,
            AnyNodeRef::SliceExpr(_) => NodeKind::SliceExpr,
            AnyNodeRef::IpyEscapeCommandExpr(_) => NodeKind::IpyEscapeCommandExpr,
            AnyNodeRef::InvalidExpr(_) => NodeKind::InvalidExpr,
            AnyNodeRef::ExceptHandlerExceptHandler(_) => NodeKind::ExceptHandlerExceptHandler,
            AnyNodeRef::PatternMatchValue(_) => NodeKind::PatternMatchValue,
            AnyNodeRef::PatternMatchSingleton(_) => NodeKind::PatternMatchSingleton,
            AnyNodeRef::PatternMatchSequence(_) => NodeKind::PatternMatchSequence,
            AnyNodeRef::PatternMatchMapping(_) => NodeKind::PatternMatchMapping,
            AnyNodeRef::PatternMatchClass(_) => NodeKind::PatternMatchClass,
            AnyNodeRef::PatternMatchStar(_) => NodeKind::PatternMatchStar,
            AnyNodeRef::PatternMatchAs(_) => NodeKind::PatternMatchAs,
            AnyNodeRef::PatternMatchOr(_) => NodeKind::PatternMatchOr,
            AnyNodeRef::PatternArguments(_) => NodeKind::PatternArguments,
            AnyNodeRef::PatternKeyword(_) => NodeKind::PatternKeyword,
            AnyNodeRef::PatternMatchInvalid(_) => NodeKind::PatternInvalid,
            AnyNodeRef::Comprehension(_) => NodeKind::Comprehension,
            AnyNodeRef::Arguments(_) => NodeKind::Arguments,
            AnyNodeRef::Parameters(_) => NodeKind::Parameters,
            AnyNodeRef::Parameter(_) => NodeKind::Parameter,
            AnyNodeRef::ParameterWithDefault(_) => NodeKind::ParameterWithDefault,
            AnyNodeRef::Keyword(_) => NodeKind::Keyword,
            AnyNodeRef::Alias(_) => NodeKind::Alias,
            AnyNodeRef::WithItem(_) => NodeKind::WithItem,
            AnyNodeRef::MatchCase(_) => NodeKind::MatchCase,
            AnyNodeRef::Decorator(_) => NodeKind::Decorator,
            AnyNodeRef::TypeParams(_) => NodeKind::TypeParams,
            AnyNodeRef::TypeParamTypeVar(_) => NodeKind::TypeParamTypeVar,
            AnyNodeRef::TypeParamTypeVarTuple(_) => NodeKind::TypeParamTypeVarTuple,
            AnyNodeRef::TypeParamParamSpec(_) => NodeKind::TypeParamParamSpec,
            AnyNodeRef::FString(_) => NodeKind::FString,
            AnyNodeRef::StringLiteral(_) => NodeKind::StringLiteral,
            AnyNodeRef::BytesLiteral(_) => NodeKind::BytesLiteral,
            AnyNodeRef::ElifElseClause(_) => NodeKind::ElifElseClause,
        }
    }

    pub const fn is_statement(self) -> bool {
        match self {
            AnyNodeRef::StmtFunctionDef(_)
            | AnyNodeRef::StmtClassDef(_)
            | AnyNodeRef::StmtReturn(_)
            | AnyNodeRef::StmtDelete(_)
            | AnyNodeRef::StmtTypeAlias(_)
            | AnyNodeRef::StmtAssign(_)
            | AnyNodeRef::StmtAugAssign(_)
            | AnyNodeRef::StmtAnnAssign(_)
            | AnyNodeRef::StmtFor(_)
            | AnyNodeRef::StmtWhile(_)
            | AnyNodeRef::StmtIf(_)
            | AnyNodeRef::StmtWith(_)
            | AnyNodeRef::StmtMatch(_)
            | AnyNodeRef::StmtRaise(_)
            | AnyNodeRef::StmtTry(_)
            | AnyNodeRef::StmtAssert(_)
            | AnyNodeRef::StmtImport(_)
            | AnyNodeRef::StmtImportFrom(_)
            | AnyNodeRef::StmtGlobal(_)
            | AnyNodeRef::StmtNonlocal(_)
            | AnyNodeRef::StmtExpr(_)
            | AnyNodeRef::StmtPass(_)
            | AnyNodeRef::StmtBreak(_)
            | AnyNodeRef::StmtContinue(_)
            | AnyNodeRef::StmtIpyEscapeCommand(_) => true,

            AnyNodeRef::ModModule(_)
            | AnyNodeRef::ModessionExpr(_)
            | AnyNodeRef::BoolOpExpr(_)
            | AnyNodeRef::NamedExprExpr(_)
            | AnyNodeRef::BinOpExpr(_)
            | AnyNodeRef::UnaryOpExpr(_)
            | AnyNodeRef::LambdaExpr(_)
            | AnyNodeRef::IfExpr(_)
            | AnyNodeRef::DictExpr(_)
            | AnyNodeRef::SetExpr(_)
            | AnyNodeRef::ListCompExpr(_)
            | AnyNodeRef::SetCompExpr(_)
            | AnyNodeRef::DictCompExpr(_)
            | AnyNodeRef::GeneratorExpExpr(_)
            | AnyNodeRef::AwaitExpr(_)
            | AnyNodeRef::YieldExpr(_)
            | AnyNodeRef::YieldFromExpr(_)
            | AnyNodeRef::CompareExpr(_)
            | AnyNodeRef::CallExpr(_)
            | AnyNodeRef::FStringessionElementExpr(_)
            | AnyNodeRef::FStringLiteralElement(_)
            | AnyNodeRef::FStringInvalidElement(_)
            | AnyNodeRef::FStringExpr(_)
            | AnyNodeRef::StringLiteralExpr(_)
            | AnyNodeRef::BytesLiteralExpr(_)
            | AnyNodeRef::NumberLiteralExpr(_)
            | AnyNodeRef::BooleanLiteralExpr(_)
            | AnyNodeRef::NoneLiteralExpr(_)
            | AnyNodeRef::EllipsisLiteralExpr(_)
            | AnyNodeRef::AttributeExpr(_)
            | AnyNodeRef::SubscriptExpr(_)
            | AnyNodeRef::StarredExpr(_)
            | AnyNodeRef::NameExpr(_)
            | AnyNodeRef::ListExpr(_)
            | AnyNodeRef::TupleExpr(_)
            | AnyNodeRef::SliceExpr(_)
            | AnyNodeRef::IpyEscapeCommandExpr(_)
            | AnyNodeRef::InvalidExpr(_)
            | AnyNodeRef::ExceptHandlerExceptHandler(_)
            | AnyNodeRef::PatternMatchValue(_)
            | AnyNodeRef::PatternMatchSingleton(_)
            | AnyNodeRef::PatternMatchSequence(_)
            | AnyNodeRef::PatternMatchMapping(_)
            | AnyNodeRef::PatternMatchClass(_)
            | AnyNodeRef::PatternMatchStar(_)
            | AnyNodeRef::PatternMatchAs(_)
            | AnyNodeRef::PatternMatchOr(_)
            | AnyNodeRef::PatternArguments(_)
            | AnyNodeRef::PatternKeyword(_)
            | AnyNodeRef::PatternMatchInvalid(_)
            | AnyNodeRef::Comprehension(_)
            | AnyNodeRef::Arguments(_)
            | AnyNodeRef::Parameters(_)
            | AnyNodeRef::Parameter(_)
            | AnyNodeRef::ParameterWithDefault(_)
            | AnyNodeRef::Keyword(_)
            | AnyNodeRef::Alias(_)
            | AnyNodeRef::WithItem(_)
            | AnyNodeRef::MatchCase(_)
            | AnyNodeRef::Decorator(_)
            | AnyNodeRef::TypeParams(_)
            | AnyNodeRef::TypeParamTypeVar(_)
            | AnyNodeRef::TypeParamTypeVarTuple(_)
            | AnyNodeRef::TypeParamParamSpec(_)
            | AnyNodeRef::FString(_)
            | AnyNodeRef::StringLiteral(_)
            | AnyNodeRef::BytesLiteral(_)
            | AnyNodeRef::ElifElseClause(_) => false,
        }
    }

    pub const fn is_expression(self) -> bool {
        match self {
            AnyNodeRef::BoolOpExpr(_)
            | AnyNodeRef::NamedExprExpr(_)
            | AnyNodeRef::BinOpExpr(_)
            | AnyNodeRef::UnaryOpExpr(_)
            | AnyNodeRef::LambdaExpr(_)
            | AnyNodeRef::IfExpr(_)
            | AnyNodeRef::DictExpr(_)
            | AnyNodeRef::SetExpr(_)
            | AnyNodeRef::ListCompExpr(_)
            | AnyNodeRef::SetCompExpr(_)
            | AnyNodeRef::DictCompExpr(_)
            | AnyNodeRef::GeneratorExpExpr(_)
            | AnyNodeRef::AwaitExpr(_)
            | AnyNodeRef::YieldExpr(_)
            | AnyNodeRef::YieldFromExpr(_)
            | AnyNodeRef::CompareExpr(_)
            | AnyNodeRef::CallExpr(_)
            | AnyNodeRef::FStringExpr(_)
            | AnyNodeRef::StringLiteralExpr(_)
            | AnyNodeRef::BytesLiteralExpr(_)
            | AnyNodeRef::NumberLiteralExpr(_)
            | AnyNodeRef::BooleanLiteralExpr(_)
            | AnyNodeRef::NoneLiteralExpr(_)
            | AnyNodeRef::EllipsisLiteralExpr(_)
            | AnyNodeRef::AttributeExpr(_)
            | AnyNodeRef::SubscriptExpr(_)
            | AnyNodeRef::StarredExpr(_)
            | AnyNodeRef::NameExpr(_)
            | AnyNodeRef::ListExpr(_)
            | AnyNodeRef::TupleExpr(_)
            | AnyNodeRef::SliceExpr(_)
            | AnyNodeRef::IpyEscapeCommandExpr(_)
            | AnyNodeRef::InvalidExpr(_) => true,

            AnyNodeRef::ModModule(_)
            | AnyNodeRef::ModessionExpr(_)
            | AnyNodeRef::StmtFunctionDef(_)
            | AnyNodeRef::StmtClassDef(_)
            | AnyNodeRef::StmtReturn(_)
            | AnyNodeRef::StmtDelete(_)
            | AnyNodeRef::StmtTypeAlias(_)
            | AnyNodeRef::StmtAssign(_)
            | AnyNodeRef::StmtAugAssign(_)
            | AnyNodeRef::StmtAnnAssign(_)
            | AnyNodeRef::StmtFor(_)
            | AnyNodeRef::StmtWhile(_)
            | AnyNodeRef::StmtIf(_)
            | AnyNodeRef::StmtWith(_)
            | AnyNodeRef::StmtMatch(_)
            | AnyNodeRef::StmtRaise(_)
            | AnyNodeRef::StmtTry(_)
            | AnyNodeRef::StmtAssert(_)
            | AnyNodeRef::StmtImport(_)
            | AnyNodeRef::StmtImportFrom(_)
            | AnyNodeRef::StmtGlobal(_)
            | AnyNodeRef::StmtNonlocal(_)
            | AnyNodeRef::StmtExpr(_)
            | AnyNodeRef::StmtPass(_)
            | AnyNodeRef::StmtBreak(_)
            | AnyNodeRef::StmtContinue(_)
            | AnyNodeRef::StmtIpyEscapeCommand(_)
            | AnyNodeRef::ExceptHandlerExceptHandler(_)
            | AnyNodeRef::FStringessionElementExpr(_)
            | AnyNodeRef::FStringLiteralElement(_)
            | AnyNodeRef::FStringInvalidElement(_)
            | AnyNodeRef::PatternMatchValue(_)
            | AnyNodeRef::PatternMatchSingleton(_)
            | AnyNodeRef::PatternMatchSequence(_)
            | AnyNodeRef::PatternMatchMapping(_)
            | AnyNodeRef::PatternMatchClass(_)
            | AnyNodeRef::PatternMatchStar(_)
            | AnyNodeRef::PatternMatchAs(_)
            | AnyNodeRef::PatternMatchOr(_)
            | AnyNodeRef::PatternArguments(_)
            | AnyNodeRef::PatternKeyword(_)
            | AnyNodeRef::PatternMatchInvalid(_)
            | AnyNodeRef::Comprehension(_)
            | AnyNodeRef::Arguments(_)
            | AnyNodeRef::Parameters(_)
            | AnyNodeRef::Parameter(_)
            | AnyNodeRef::ParameterWithDefault(_)
            | AnyNodeRef::Keyword(_)
            | AnyNodeRef::Alias(_)
            | AnyNodeRef::WithItem(_)
            | AnyNodeRef::MatchCase(_)
            | AnyNodeRef::Decorator(_)
            | AnyNodeRef::TypeParams(_)
            | AnyNodeRef::TypeParamTypeVar(_)
            | AnyNodeRef::TypeParamTypeVarTuple(_)
            | AnyNodeRef::TypeParamParamSpec(_)
            | AnyNodeRef::FString(_)
            | AnyNodeRef::StringLiteral(_)
            | AnyNodeRef::BytesLiteral(_)
            | AnyNodeRef::ElifElseClause(_) => false,
        }
    }

    pub const fn is_module(self) -> bool {
        match self {
            AnyNodeRef::ModModule(_) | AnyNodeRef::ModessionExpr(_) => true,

            AnyNodeRef::StmtFunctionDef(_)
            | AnyNodeRef::StmtClassDef(_)
            | AnyNodeRef::StmtReturn(_)
            | AnyNodeRef::StmtDelete(_)
            | AnyNodeRef::StmtTypeAlias(_)
            | AnyNodeRef::StmtAssign(_)
            | AnyNodeRef::StmtAugAssign(_)
            | AnyNodeRef::StmtAnnAssign(_)
            | AnyNodeRef::StmtFor(_)
            | AnyNodeRef::StmtWhile(_)
            | AnyNodeRef::StmtIf(_)
            | AnyNodeRef::StmtWith(_)
            | AnyNodeRef::StmtMatch(_)
            | AnyNodeRef::StmtRaise(_)
            | AnyNodeRef::StmtTry(_)
            | AnyNodeRef::StmtAssert(_)
            | AnyNodeRef::StmtImport(_)
            | AnyNodeRef::StmtImportFrom(_)
            | AnyNodeRef::StmtGlobal(_)
            | AnyNodeRef::StmtNonlocal(_)
            | AnyNodeRef::StmtExpr(_)
            | AnyNodeRef::StmtPass(_)
            | AnyNodeRef::StmtBreak(_)
            | AnyNodeRef::StmtContinue(_)
            | AnyNodeRef::StmtIpyEscapeCommand(_)
            | AnyNodeRef::BoolOpExpr(_)
            | AnyNodeRef::NamedExprExpr(_)
            | AnyNodeRef::BinOpExpr(_)
            | AnyNodeRef::UnaryOpExpr(_)
            | AnyNodeRef::LambdaExpr(_)
            | AnyNodeRef::IfExpr(_)
            | AnyNodeRef::DictExpr(_)
            | AnyNodeRef::SetExpr(_)
            | AnyNodeRef::ListCompExpr(_)
            | AnyNodeRef::SetCompExpr(_)
            | AnyNodeRef::DictCompExpr(_)
            | AnyNodeRef::GeneratorExpExpr(_)
            | AnyNodeRef::AwaitExpr(_)
            | AnyNodeRef::YieldExpr(_)
            | AnyNodeRef::YieldFromExpr(_)
            | AnyNodeRef::CompareExpr(_)
            | AnyNodeRef::CallExpr(_)
            | AnyNodeRef::FStringessionElementExpr(_)
            | AnyNodeRef::FStringLiteralElement(_)
            | AnyNodeRef::FStringInvalidElement(_)
            | AnyNodeRef::FStringExpr(_)
            | AnyNodeRef::StringLiteralExpr(_)
            | AnyNodeRef::BytesLiteralExpr(_)
            | AnyNodeRef::NumberLiteralExpr(_)
            | AnyNodeRef::BooleanLiteralExpr(_)
            | AnyNodeRef::NoneLiteralExpr(_)
            | AnyNodeRef::EllipsisLiteralExpr(_)
            | AnyNodeRef::AttributeExpr(_)
            | AnyNodeRef::SubscriptExpr(_)
            | AnyNodeRef::StarredExpr(_)
            | AnyNodeRef::NameExpr(_)
            | AnyNodeRef::ListExpr(_)
            | AnyNodeRef::TupleExpr(_)
            | AnyNodeRef::SliceExpr(_)
            | AnyNodeRef::IpyEscapeCommandExpr(_)
            | AnyNodeRef::InvalidExpr(_)
            | AnyNodeRef::ExceptHandlerExceptHandler(_)
            | AnyNodeRef::PatternMatchValue(_)
            | AnyNodeRef::PatternMatchSingleton(_)
            | AnyNodeRef::PatternMatchSequence(_)
            | AnyNodeRef::PatternMatchMapping(_)
            | AnyNodeRef::PatternMatchClass(_)
            | AnyNodeRef::PatternMatchStar(_)
            | AnyNodeRef::PatternMatchAs(_)
            | AnyNodeRef::PatternMatchOr(_)
            | AnyNodeRef::PatternArguments(_)
            | AnyNodeRef::PatternKeyword(_)
            | AnyNodeRef::PatternMatchInvalid(_)
            | AnyNodeRef::Comprehension(_)
            | AnyNodeRef::Arguments(_)
            | AnyNodeRef::Parameters(_)
            | AnyNodeRef::Parameter(_)
            | AnyNodeRef::ParameterWithDefault(_)
            | AnyNodeRef::Keyword(_)
            | AnyNodeRef::Alias(_)
            | AnyNodeRef::WithItem(_)
            | AnyNodeRef::MatchCase(_)
            | AnyNodeRef::Decorator(_)
            | AnyNodeRef::TypeParams(_)
            | AnyNodeRef::TypeParamTypeVar(_)
            | AnyNodeRef::TypeParamTypeVarTuple(_)
            | AnyNodeRef::TypeParamParamSpec(_)
            | AnyNodeRef::FString(_)
            | AnyNodeRef::StringLiteral(_)
            | AnyNodeRef::BytesLiteral(_)
            | AnyNodeRef::ElifElseClause(_) => false,
        }
    }

    pub const fn is_pattern(self) -> bool {
        match self {
            AnyNodeRef::PatternMatchValue(_)
            | AnyNodeRef::PatternMatchSingleton(_)
            | AnyNodeRef::PatternMatchSequence(_)
            | AnyNodeRef::PatternMatchMapping(_)
            | AnyNodeRef::PatternMatchClass(_)
            | AnyNodeRef::PatternMatchStar(_)
            | AnyNodeRef::PatternMatchAs(_)
            | AnyNodeRef::PatternMatchOr(_)
            | AnyNodeRef::PatternMatchInvalid(_) => true,

            AnyNodeRef::ModModule(_)
            | AnyNodeRef::ModessionExpr(_)
            | AnyNodeRef::StmtFunctionDef(_)
            | AnyNodeRef::StmtClassDef(_)
            | AnyNodeRef::StmtReturn(_)
            | AnyNodeRef::StmtDelete(_)
            | AnyNodeRef::StmtTypeAlias(_)
            | AnyNodeRef::StmtAssign(_)
            | AnyNodeRef::StmtAugAssign(_)
            | AnyNodeRef::StmtAnnAssign(_)
            | AnyNodeRef::StmtFor(_)
            | AnyNodeRef::StmtWhile(_)
            | AnyNodeRef::StmtIf(_)
            | AnyNodeRef::StmtWith(_)
            | AnyNodeRef::StmtMatch(_)
            | AnyNodeRef::StmtRaise(_)
            | AnyNodeRef::StmtTry(_)
            | AnyNodeRef::StmtAssert(_)
            | AnyNodeRef::StmtImport(_)
            | AnyNodeRef::StmtImportFrom(_)
            | AnyNodeRef::StmtGlobal(_)
            | AnyNodeRef::StmtNonlocal(_)
            | AnyNodeRef::StmtExpr(_)
            | AnyNodeRef::StmtPass(_)
            | AnyNodeRef::StmtBreak(_)
            | AnyNodeRef::StmtContinue(_)
            | AnyNodeRef::StmtIpyEscapeCommand(_)
            | AnyNodeRef::BoolOpExpr(_)
            | AnyNodeRef::NamedExprExpr(_)
            | AnyNodeRef::BinOpExpr(_)
            | AnyNodeRef::UnaryOpExpr(_)
            | AnyNodeRef::LambdaExpr(_)
            | AnyNodeRef::IfExpr(_)
            | AnyNodeRef::DictExpr(_)
            | AnyNodeRef::SetExpr(_)
            | AnyNodeRef::ListCompExpr(_)
            | AnyNodeRef::SetCompExpr(_)
            | AnyNodeRef::DictCompExpr(_)
            | AnyNodeRef::GeneratorExpExpr(_)
            | AnyNodeRef::AwaitExpr(_)
            | AnyNodeRef::YieldExpr(_)
            | AnyNodeRef::YieldFromExpr(_)
            | AnyNodeRef::CompareExpr(_)
            | AnyNodeRef::CallExpr(_)
            | AnyNodeRef::FStringessionElementExpr(_)
            | AnyNodeRef::FStringLiteralElement(_)
            | AnyNodeRef::FStringInvalidElement(_)
            | AnyNodeRef::FStringExpr(_)
            | AnyNodeRef::StringLiteralExpr(_)
            | AnyNodeRef::BytesLiteralExpr(_)
            | AnyNodeRef::NumberLiteralExpr(_)
            | AnyNodeRef::BooleanLiteralExpr(_)
            | AnyNodeRef::NoneLiteralExpr(_)
            | AnyNodeRef::EllipsisLiteralExpr(_)
            | AnyNodeRef::AttributeExpr(_)
            | AnyNodeRef::SubscriptExpr(_)
            | AnyNodeRef::StarredExpr(_)
            | AnyNodeRef::NameExpr(_)
            | AnyNodeRef::ListExpr(_)
            | AnyNodeRef::TupleExpr(_)
            | AnyNodeRef::SliceExpr(_)
            | AnyNodeRef::IpyEscapeCommandExpr(_)
            | AnyNodeRef::InvalidExpr(_)
            | AnyNodeRef::PatternArguments(_)
            | AnyNodeRef::PatternKeyword(_)
            | AnyNodeRef::ExceptHandlerExceptHandler(_)
            | AnyNodeRef::Comprehension(_)
            | AnyNodeRef::Arguments(_)
            | AnyNodeRef::Parameters(_)
            | AnyNodeRef::Parameter(_)
            | AnyNodeRef::ParameterWithDefault(_)
            | AnyNodeRef::Keyword(_)
            | AnyNodeRef::Alias(_)
            | AnyNodeRef::WithItem(_)
            | AnyNodeRef::MatchCase(_)
            | AnyNodeRef::Decorator(_)
            | AnyNodeRef::TypeParams(_)
            | AnyNodeRef::TypeParamTypeVar(_)
            | AnyNodeRef::TypeParamTypeVarTuple(_)
            | AnyNodeRef::TypeParamParamSpec(_)
            | AnyNodeRef::FString(_)
            | AnyNodeRef::StringLiteral(_)
            | AnyNodeRef::BytesLiteral(_)
            | AnyNodeRef::ElifElseClause(_) => false,
        }
    }

    pub const fn is_except_handler(self) -> bool {
        match self {
            AnyNodeRef::ExceptHandlerExceptHandler(_) => true,

            AnyNodeRef::ModModule(_)
            | AnyNodeRef::ModessionExpr(_)
            | AnyNodeRef::StmtFunctionDef(_)
            | AnyNodeRef::StmtClassDef(_)
            | AnyNodeRef::StmtReturn(_)
            | AnyNodeRef::StmtDelete(_)
            | AnyNodeRef::StmtTypeAlias(_)
            | AnyNodeRef::StmtAssign(_)
            | AnyNodeRef::StmtAugAssign(_)
            | AnyNodeRef::StmtAnnAssign(_)
            | AnyNodeRef::StmtFor(_)
            | AnyNodeRef::StmtWhile(_)
            | AnyNodeRef::StmtIf(_)
            | AnyNodeRef::StmtWith(_)
            | AnyNodeRef::StmtMatch(_)
            | AnyNodeRef::StmtRaise(_)
            | AnyNodeRef::StmtTry(_)
            | AnyNodeRef::StmtAssert(_)
            | AnyNodeRef::StmtImport(_)
            | AnyNodeRef::StmtImportFrom(_)
            | AnyNodeRef::StmtGlobal(_)
            | AnyNodeRef::StmtNonlocal(_)
            | AnyNodeRef::StmtExpr(_)
            | AnyNodeRef::StmtPass(_)
            | AnyNodeRef::StmtBreak(_)
            | AnyNodeRef::StmtContinue(_)
            | AnyNodeRef::StmtIpyEscapeCommand(_)
            | AnyNodeRef::BoolOpExpr(_)
            | AnyNodeRef::NamedExprExpr(_)
            | AnyNodeRef::BinOpExpr(_)
            | AnyNodeRef::UnaryOpExpr(_)
            | AnyNodeRef::LambdaExpr(_)
            | AnyNodeRef::IfExpr(_)
            | AnyNodeRef::DictExpr(_)
            | AnyNodeRef::SetExpr(_)
            | AnyNodeRef::ListCompExpr(_)
            | AnyNodeRef::SetCompExpr(_)
            | AnyNodeRef::DictCompExpr(_)
            | AnyNodeRef::GeneratorExpExpr(_)
            | AnyNodeRef::AwaitExpr(_)
            | AnyNodeRef::YieldExpr(_)
            | AnyNodeRef::YieldFromExpr(_)
            | AnyNodeRef::CompareExpr(_)
            | AnyNodeRef::CallExpr(_)
            | AnyNodeRef::FStringessionElementExpr(_)
            | AnyNodeRef::FStringLiteralElement(_)
            | AnyNodeRef::FStringInvalidElement(_)
            | AnyNodeRef::FStringExpr(_)
            | AnyNodeRef::StringLiteralExpr(_)
            | AnyNodeRef::BytesLiteralExpr(_)
            | AnyNodeRef::NumberLiteralExpr(_)
            | AnyNodeRef::BooleanLiteralExpr(_)
            | AnyNodeRef::NoneLiteralExpr(_)
            | AnyNodeRef::EllipsisLiteralExpr(_)
            | AnyNodeRef::AttributeExpr(_)
            | AnyNodeRef::SubscriptExpr(_)
            | AnyNodeRef::StarredExpr(_)
            | AnyNodeRef::NameExpr(_)
            | AnyNodeRef::ListExpr(_)
            | AnyNodeRef::TupleExpr(_)
            | AnyNodeRef::SliceExpr(_)
            | AnyNodeRef::IpyEscapeCommandExpr(_)
            | AnyNodeRef::InvalidExpr(_)
            | AnyNodeRef::PatternMatchValue(_)
            | AnyNodeRef::PatternMatchSingleton(_)
            | AnyNodeRef::PatternMatchSequence(_)
            | AnyNodeRef::PatternMatchMapping(_)
            | AnyNodeRef::PatternMatchClass(_)
            | AnyNodeRef::PatternMatchStar(_)
            | AnyNodeRef::PatternMatchAs(_)
            | AnyNodeRef::PatternMatchOr(_)
            | AnyNodeRef::PatternArguments(_)
            | AnyNodeRef::PatternKeyword(_)
            | AnyNodeRef::PatternMatchInvalid(_)
            | AnyNodeRef::Comprehension(_)
            | AnyNodeRef::Arguments(_)
            | AnyNodeRef::Parameters(_)
            | AnyNodeRef::Parameter(_)
            | AnyNodeRef::ParameterWithDefault(_)
            | AnyNodeRef::Keyword(_)
            | AnyNodeRef::Alias(_)
            | AnyNodeRef::WithItem(_)
            | AnyNodeRef::MatchCase(_)
            | AnyNodeRef::Decorator(_)
            | AnyNodeRef::TypeParams(_)
            | AnyNodeRef::TypeParamTypeVar(_)
            | AnyNodeRef::TypeParamTypeVarTuple(_)
            | AnyNodeRef::TypeParamParamSpec(_)
            | AnyNodeRef::FString(_)
            | AnyNodeRef::StringLiteral(_)
            | AnyNodeRef::BytesLiteral(_)
            | AnyNodeRef::ElifElseClause(_) => false,
        }
    }

    pub const fn is_node_with_body(self) -> bool {
        matches!(
            self,
            AnyNodeRef::StmtIf(_)
                | AnyNodeRef::StmtFor(_)
                | AnyNodeRef::StmtWhile(_)
                | AnyNodeRef::StmtWith(_)
                | AnyNodeRef::StmtMatch(_)
                | AnyNodeRef::StmtFunctionDef(_)
                | AnyNodeRef::StmtClassDef(_)
                | AnyNodeRef::StmtTry(_)
                | AnyNodeRef::ExceptHandlerExceptHandler(_)
                | AnyNodeRef::ElifElseClause(_)
        )
    }

    /// In our AST, only some alternative branches are represented as a node. This has historical
    /// reasons, e.g. we added a node for elif/else in if statements which was not originally
    /// present in the parser.
    pub const fn is_alternative_branch_with_node(self) -> bool {
        matches!(
            self,
            AnyNodeRef::ExceptHandlerExceptHandler(_) | AnyNodeRef::ElifElseClause(_)
        )
    }

    pub fn visit_preorder<'b, V>(&'b self, visitor: &mut V)
    where
        V: PreorderVisitor<'b> + ?Sized,
    {
        match self {
            AnyNodeRef::ModModule(node) => node.visit_preorder(visitor),
            AnyNodeRef::ModessionExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::StmtFunctionDef(node) => node.visit_preorder(visitor),
            AnyNodeRef::StmtClassDef(node) => node.visit_preorder(visitor),
            AnyNodeRef::StmtReturn(node) => node.visit_preorder(visitor),
            AnyNodeRef::StmtDelete(node) => node.visit_preorder(visitor),
            AnyNodeRef::StmtTypeAlias(node) => node.visit_preorder(visitor),
            AnyNodeRef::StmtAssign(node) => node.visit_preorder(visitor),
            AnyNodeRef::StmtAugAssign(node) => node.visit_preorder(visitor),
            AnyNodeRef::StmtAnnAssign(node) => node.visit_preorder(visitor),
            AnyNodeRef::StmtFor(node) => node.visit_preorder(visitor),
            AnyNodeRef::StmtWhile(node) => node.visit_preorder(visitor),
            AnyNodeRef::StmtIf(node) => node.visit_preorder(visitor),
            AnyNodeRef::StmtWith(node) => node.visit_preorder(visitor),
            AnyNodeRef::StmtMatch(node) => node.visit_preorder(visitor),
            AnyNodeRef::StmtRaise(node) => node.visit_preorder(visitor),
            AnyNodeRef::StmtTry(node) => node.visit_preorder(visitor),
            AnyNodeRef::StmtAssert(node) => node.visit_preorder(visitor),
            AnyNodeRef::StmtImport(node) => node.visit_preorder(visitor),
            AnyNodeRef::StmtImportFrom(node) => node.visit_preorder(visitor),
            AnyNodeRef::StmtGlobal(node) => node.visit_preorder(visitor),
            AnyNodeRef::StmtNonlocal(node) => node.visit_preorder(visitor),
            AnyNodeRef::StmtExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::StmtPass(node) => node.visit_preorder(visitor),
            AnyNodeRef::StmtBreak(node) => node.visit_preorder(visitor),
            AnyNodeRef::StmtContinue(node) => node.visit_preorder(visitor),
            AnyNodeRef::StmtIpyEscapeCommand(node) => node.visit_preorder(visitor),
            AnyNodeRef::BoolOpExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::NamedExprExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::BinOpExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::UnaryOpExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::LambdaExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::IfExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::DictExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::SetExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::ListCompExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::SetCompExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::DictCompExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::GeneratorExpExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::AwaitExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::YieldExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::YieldFromExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::CompareExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::CallExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::FStringessionElementExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::FStringLiteralElement(node) => node.visit_preorder(visitor),
            AnyNodeRef::FStringExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::StringLiteralExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::BytesLiteralExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::NumberLiteralExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::BooleanLiteralExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::NoneLiteralExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::EllipsisLiteralExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::AttributeExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::SubscriptExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::StarredExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::NameExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::ListExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::TupleExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::SliceExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::IpyEscapeCommandExpr(node) => node.visit_preorder(visitor),
            AnyNodeRef::ExceptHandlerExceptHandler(node) => node.visit_preorder(visitor),
            AnyNodeRef::PatternMatchValue(node) => node.visit_preorder(visitor),
            AnyNodeRef::PatternMatchSingleton(node) => node.visit_preorder(visitor),
            AnyNodeRef::PatternMatchSequence(node) => node.visit_preorder(visitor),
            AnyNodeRef::PatternMatchMapping(node) => node.visit_preorder(visitor),
            AnyNodeRef::PatternMatchClass(node) => node.visit_preorder(visitor),
            AnyNodeRef::PatternMatchStar(node) => node.visit_preorder(visitor),
            AnyNodeRef::PatternMatchAs(node) => node.visit_preorder(visitor),
            AnyNodeRef::PatternMatchOr(node) => node.visit_preorder(visitor),
            AnyNodeRef::PatternArguments(node) => node.visit_preorder(visitor),
            AnyNodeRef::PatternKeyword(node) => node.visit_preorder(visitor),
            AnyNodeRef::PatternMatchInvalid(_) => {}
            AnyNodeRef::Comprehension(node) => node.visit_preorder(visitor),
            AnyNodeRef::Arguments(node) => node.visit_preorder(visitor),
            AnyNodeRef::Parameters(node) => node.visit_preorder(visitor),
            AnyNodeRef::Parameter(node) => node.visit_preorder(visitor),
            AnyNodeRef::ParameterWithDefault(node) => node.visit_preorder(visitor),
            AnyNodeRef::Keyword(node) => node.visit_preorder(visitor),
            AnyNodeRef::Alias(node) => node.visit_preorder(visitor),
            AnyNodeRef::WithItem(node) => node.visit_preorder(visitor),
            AnyNodeRef::MatchCase(node) => node.visit_preorder(visitor),
            AnyNodeRef::Decorator(node) => node.visit_preorder(visitor),
            AnyNodeRef::TypeParams(node) => node.visit_preorder(visitor),
            AnyNodeRef::TypeParamTypeVar(node) => node.visit_preorder(visitor),
            AnyNodeRef::TypeParamTypeVarTuple(node) => node.visit_preorder(visitor),
            AnyNodeRef::TypeParamParamSpec(node) => node.visit_preorder(visitor),
            AnyNodeRef::FString(node) => node.visit_preorder(visitor),
            AnyNodeRef::StringLiteral(node) => node.visit_preorder(visitor),
            AnyNodeRef::BytesLiteral(node) => node.visit_preorder(visitor),
            AnyNodeRef::ElifElseClause(node) => node.visit_preorder(visitor),
            AnyNodeRef::InvalidExpr(_) | AnyNodeRef::FStringInvalidElement(_) => {}
        }
    }

    /// The last child of the last branch, if the node has multiple branches.
    pub fn last_child_in_body(&self) -> Option<AnyNodeRef<'a>> {
        let body = match self {
            AnyNodeRef::StmtFunctionDef(ast::FunctionDefStmt { body, .. })
            | AnyNodeRef::StmtClassDef(ast::ClassDefStmt { body, .. })
            | AnyNodeRef::StmtWith(ast::WithStmt { body, .. })
            | AnyNodeRef::MatchCase(MatchCase { body, .. })
            | AnyNodeRef::ExceptHandlerExceptHandler(ast::ExceptHandlerExceptHandler {
                body,
                ..
            })
            | AnyNodeRef::ElifElseClause(ast::ElifElseClause { body, .. }) => body,
            AnyNodeRef::StmtIf(ast::IfStmt {
                body,
                elif_else_clauses,
                ..
            }) => elif_else_clauses.last().map_or(body, |clause| &clause.body),

            AnyNodeRef::StmtFor(ast::ForStmt { body, orelse, .. })
            | AnyNodeRef::StmtWhile(ast::WhileStmt { body, orelse, .. }) => {
                if orelse.is_empty() {
                    body
                } else {
                    orelse
                }
            }

            AnyNodeRef::StmtMatch(ast::MatchStmt { cases, .. }) => {
                return cases.last().map(AnyNodeRef::from);
            }

            AnyNodeRef::StmtTry(ast::TryStmt {
                body,
                handlers,
                orelse,
                finalbody,
                ..
            }) => {
                if finalbody.is_empty() {
                    if orelse.is_empty() {
                        if handlers.is_empty() {
                            body
                        } else {
                            return handlers.last().map(AnyNodeRef::from);
                        }
                    } else {
                        orelse
                    }
                } else {
                    finalbody
                }
            }

            // Not a node that contains an indented child node.
            _ => return None,
        };

        body.last().map(AnyNodeRef::from)
    }
}

impl<'a> From<&'a ast::ModModule> for AnyNodeRef<'a> {
    fn from(node: &'a ast::ModModule) -> Self {
        AnyNodeRef::ModModule(node)
    }
}

impl<'a> From<&'a ast::ModExpression> for AnyNodeRef<'a> {
    fn from(node: &'a ast::ModExpression) -> Self {
        AnyNodeRef::ModessionExpr(node)
    }
}

impl<'a> From<&'a ast::FunctionDefStmt> for AnyNodeRef<'a> {
    fn from(node: &'a ast::FunctionDefStmt) -> Self {
        AnyNodeRef::StmtFunctionDef(node)
    }
}

impl<'a> From<&'a ast::ClassDefStmt> for AnyNodeRef<'a> {
    fn from(node: &'a ast::ClassDefStmt) -> Self {
        AnyNodeRef::StmtClassDef(node)
    }
}

impl<'a> From<&'a ast::ReturnStmt> for AnyNodeRef<'a> {
    fn from(node: &'a ast::ReturnStmt) -> Self {
        AnyNodeRef::StmtReturn(node)
    }
}

impl<'a> From<&'a ast::DeleteStmt> for AnyNodeRef<'a> {
    fn from(node: &'a ast::DeleteStmt) -> Self {
        AnyNodeRef::StmtDelete(node)
    }
}

impl<'a> From<&'a ast::TypeAliasStmt> for AnyNodeRef<'a> {
    fn from(node: &'a ast::TypeAliasStmt) -> Self {
        AnyNodeRef::StmtTypeAlias(node)
    }
}

impl<'a> From<&'a ast::AssignStmt> for AnyNodeRef<'a> {
    fn from(node: &'a ast::AssignStmt) -> Self {
        AnyNodeRef::StmtAssign(node)
    }
}

impl<'a> From<&'a ast::AugAssignStmt> for AnyNodeRef<'a> {
    fn from(node: &'a ast::AugAssignStmt) -> Self {
        AnyNodeRef::StmtAugAssign(node)
    }
}

impl<'a> From<&'a ast::AnnAssignStmt> for AnyNodeRef<'a> {
    fn from(node: &'a ast::AnnAssignStmt) -> Self {
        AnyNodeRef::StmtAnnAssign(node)
    }
}

impl<'a> From<&'a ast::ForStmt> for AnyNodeRef<'a> {
    fn from(node: &'a ast::ForStmt) -> Self {
        AnyNodeRef::StmtFor(node)
    }
}

impl<'a> From<&'a ast::WhileStmt> for AnyNodeRef<'a> {
    fn from(node: &'a ast::WhileStmt) -> Self {
        AnyNodeRef::StmtWhile(node)
    }
}

impl<'a> From<&'a ast::IfStmt> for AnyNodeRef<'a> {
    fn from(node: &'a ast::IfStmt) -> Self {
        AnyNodeRef::StmtIf(node)
    }
}

impl<'a> From<&'a ast::ElifElseClause> for AnyNodeRef<'a> {
    fn from(node: &'a ast::ElifElseClause) -> Self {
        AnyNodeRef::ElifElseClause(node)
    }
}

impl<'a> From<&'a ast::WithStmt> for AnyNodeRef<'a> {
    fn from(node: &'a ast::WithStmt) -> Self {
        AnyNodeRef::StmtWith(node)
    }
}

impl<'a> From<&'a ast::MatchStmt> for AnyNodeRef<'a> {
    fn from(node: &'a ast::MatchStmt) -> Self {
        AnyNodeRef::StmtMatch(node)
    }
}

impl<'a> From<&'a ast::RaiseStmt> for AnyNodeRef<'a> {
    fn from(node: &'a ast::RaiseStmt) -> Self {
        AnyNodeRef::StmtRaise(node)
    }
}

impl<'a> From<&'a ast::TryStmt> for AnyNodeRef<'a> {
    fn from(node: &'a ast::TryStmt) -> Self {
        AnyNodeRef::StmtTry(node)
    }
}

impl<'a> From<&'a ast::AssertStmt> for AnyNodeRef<'a> {
    fn from(node: &'a ast::AssertStmt) -> Self {
        AnyNodeRef::StmtAssert(node)
    }
}

impl<'a> From<&'a ast::ImportStmt> for AnyNodeRef<'a> {
    fn from(node: &'a ast::ImportStmt) -> Self {
        AnyNodeRef::StmtImport(node)
    }
}

impl<'a> From<&'a ast::ImportFromStmt> for AnyNodeRef<'a> {
    fn from(node: &'a ast::ImportFromStmt) -> Self {
        AnyNodeRef::StmtImportFrom(node)
    }
}

impl<'a> From<&'a ast::GlobalStmt> for AnyNodeRef<'a> {
    fn from(node: &'a ast::GlobalStmt) -> Self {
        AnyNodeRef::StmtGlobal(node)
    }
}

impl<'a> From<&'a ast::NonlocalStmt> for AnyNodeRef<'a> {
    fn from(node: &'a ast::NonlocalStmt) -> Self {
        AnyNodeRef::StmtNonlocal(node)
    }
}

impl<'a> From<&'a ast::ExprStmt> for AnyNodeRef<'a> {
    fn from(node: &'a ast::ExprStmt) -> Self {
        AnyNodeRef::StmtExpr(node)
    }
}

impl<'a> From<&'a ast::PassStmt> for AnyNodeRef<'a> {
    fn from(node: &'a ast::PassStmt) -> Self {
        AnyNodeRef::StmtPass(node)
    }
}

impl<'a> From<&'a ast::BreakStmt> for AnyNodeRef<'a> {
    fn from(node: &'a ast::BreakStmt) -> Self {
        AnyNodeRef::StmtBreak(node)
    }
}

impl<'a> From<&'a ast::ContinueStmt> for AnyNodeRef<'a> {
    fn from(node: &'a ast::ContinueStmt) -> Self {
        AnyNodeRef::StmtContinue(node)
    }
}

impl<'a> From<&'a ast::IpyEscapeCommandStmt> for AnyNodeRef<'a> {
    fn from(node: &'a ast::IpyEscapeCommandStmt) -> Self {
        AnyNodeRef::StmtIpyEscapeCommand(node)
    }
}

impl<'a> From<&'a ast::BoolOpExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::BoolOpExpr) -> Self {
        AnyNodeRef::BoolOpExpr(node)
    }
}

impl<'a> From<&'a ast::NamedExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::NamedExpr) -> Self {
        AnyNodeRef::NamedExprExpr(node)
    }
}

impl<'a> From<&'a ast::BinOpExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::BinOpExpr) -> Self {
        AnyNodeRef::BinOpExpr(node)
    }
}

impl<'a> From<&'a ast::UnaryOpExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::UnaryOpExpr) -> Self {
        AnyNodeRef::UnaryOpExpr(node)
    }
}

impl<'a> From<&'a ast::LambdaExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::LambdaExpr) -> Self {
        AnyNodeRef::LambdaExpr(node)
    }
}

impl<'a> From<&'a ast::IfExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::IfExpr) -> Self {
        AnyNodeRef::IfExpr(node)
    }
}

impl<'a> From<&'a ast::DictExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::DictExpr) -> Self {
        AnyNodeRef::DictExpr(node)
    }
}

impl<'a> From<&'a ast::SetExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::SetExpr) -> Self {
        AnyNodeRef::SetExpr(node)
    }
}

impl<'a> From<&'a ast::ListCompExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::ListCompExpr) -> Self {
        AnyNodeRef::ListCompExpr(node)
    }
}

impl<'a> From<&'a ast::SetCompExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::SetCompExpr) -> Self {
        AnyNodeRef::SetCompExpr(node)
    }
}

impl<'a> From<&'a ast::DictCompExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::DictCompExpr) -> Self {
        AnyNodeRef::DictCompExpr(node)
    }
}

impl<'a> From<&'a ast::GeneratorExpExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::GeneratorExpExpr) -> Self {
        AnyNodeRef::GeneratorExpExpr(node)
    }
}

impl<'a> From<&'a ast::AwaitExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::AwaitExpr) -> Self {
        AnyNodeRef::AwaitExpr(node)
    }
}

impl<'a> From<&'a ast::YieldExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::YieldExpr) -> Self {
        AnyNodeRef::YieldExpr(node)
    }
}

impl<'a> From<&'a ast::YieldFromExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::YieldFromExpr) -> Self {
        AnyNodeRef::YieldFromExpr(node)
    }
}

impl<'a> From<&'a ast::CompareExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::CompareExpr) -> Self {
        AnyNodeRef::CompareExpr(node)
    }
}

impl<'a> From<&'a ast::CallExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::CallExpr) -> Self {
        AnyNodeRef::CallExpr(node)
    }
}

impl<'a> From<&'a ast::FStringExpressionElement> for AnyNodeRef<'a> {
    fn from(node: &'a ast::FStringExpressionElement) -> Self {
        AnyNodeRef::FStringessionElementExpr(node)
    }
}

impl<'a> From<&'a ast::FStringLiteralElement> for AnyNodeRef<'a> {
    fn from(node: &'a ast::FStringLiteralElement) -> Self {
        AnyNodeRef::FStringLiteralElement(node)
    }
}

impl<'a> From<&'a ast::FStringExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::FStringExpr) -> Self {
        AnyNodeRef::FStringExpr(node)
    }
}

impl<'a> From<&'a ast::StringLiteralExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::StringLiteralExpr) -> Self {
        AnyNodeRef::StringLiteralExpr(node)
    }
}

impl<'a> From<&'a ast::BytesLiteralExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::BytesLiteralExpr) -> Self {
        AnyNodeRef::BytesLiteralExpr(node)
    }
}

impl<'a> From<&'a ast::NumberLiteralExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::NumberLiteralExpr) -> Self {
        AnyNodeRef::NumberLiteralExpr(node)
    }
}

impl<'a> From<&'a ast::BooleanLiteralExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::BooleanLiteralExpr) -> Self {
        AnyNodeRef::BooleanLiteralExpr(node)
    }
}

impl<'a> From<&'a ast::NoneLiteralExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::NoneLiteralExpr) -> Self {
        AnyNodeRef::NoneLiteralExpr(node)
    }
}

impl<'a> From<&'a ast::EllipsisLiteralExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::EllipsisLiteralExpr) -> Self {
        AnyNodeRef::EllipsisLiteralExpr(node)
    }
}

impl<'a> From<&'a ast::AttributeExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::AttributeExpr) -> Self {
        AnyNodeRef::AttributeExpr(node)
    }
}

impl<'a> From<&'a ast::SubscriptExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::SubscriptExpr) -> Self {
        AnyNodeRef::SubscriptExpr(node)
    }
}

impl<'a> From<&'a ast::StarredExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::StarredExpr) -> Self {
        AnyNodeRef::StarredExpr(node)
    }
}

impl<'a> From<&'a ast::NameExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::NameExpr) -> Self {
        AnyNodeRef::NameExpr(node)
    }
}

impl<'a> From<&'a ast::ListExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::ListExpr) -> Self {
        AnyNodeRef::ListExpr(node)
    }
}

impl<'a> From<&'a ast::TupleExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::TupleExpr) -> Self {
        AnyNodeRef::TupleExpr(node)
    }
}

impl<'a> From<&'a ast::SliceExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::SliceExpr) -> Self {
        AnyNodeRef::SliceExpr(node)
    }
}

impl<'a> From<&'a ast::IpyEscapeCommandExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::IpyEscapeCommandExpr) -> Self {
        AnyNodeRef::IpyEscapeCommandExpr(node)
    }
}

impl<'a> From<&'a ast::InvalidExpr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::InvalidExpr) -> Self {
        AnyNodeRef::InvalidExpr(node)
    }
}

impl<'a> From<&'a ast::ExceptHandlerExceptHandler> for AnyNodeRef<'a> {
    fn from(node: &'a ast::ExceptHandlerExceptHandler) -> Self {
        AnyNodeRef::ExceptHandlerExceptHandler(node)
    }
}

impl<'a> From<&'a ast::PatternMatchValue> for AnyNodeRef<'a> {
    fn from(node: &'a ast::PatternMatchValue) -> Self {
        AnyNodeRef::PatternMatchValue(node)
    }
}

impl<'a> From<&'a ast::PatternMatchSingleton> for AnyNodeRef<'a> {
    fn from(node: &'a ast::PatternMatchSingleton) -> Self {
        AnyNodeRef::PatternMatchSingleton(node)
    }
}

impl<'a> From<&'a ast::PatternMatchSequence> for AnyNodeRef<'a> {
    fn from(node: &'a ast::PatternMatchSequence) -> Self {
        AnyNodeRef::PatternMatchSequence(node)
    }
}

impl<'a> From<&'a ast::PatternMatchMapping> for AnyNodeRef<'a> {
    fn from(node: &'a ast::PatternMatchMapping) -> Self {
        AnyNodeRef::PatternMatchMapping(node)
    }
}

impl<'a> From<&'a ast::PatternMatchClass> for AnyNodeRef<'a> {
    fn from(node: &'a ast::PatternMatchClass) -> Self {
        AnyNodeRef::PatternMatchClass(node)
    }
}

impl<'a> From<&'a ast::PatternMatchStar> for AnyNodeRef<'a> {
    fn from(node: &'a ast::PatternMatchStar) -> Self {
        AnyNodeRef::PatternMatchStar(node)
    }
}

impl<'a> From<&'a ast::PatternMatchAs> for AnyNodeRef<'a> {
    fn from(node: &'a ast::PatternMatchAs) -> Self {
        AnyNodeRef::PatternMatchAs(node)
    }
}

impl<'a> From<&'a ast::PatternMatchOr> for AnyNodeRef<'a> {
    fn from(node: &'a ast::PatternMatchOr) -> Self {
        AnyNodeRef::PatternMatchOr(node)
    }
}

impl<'a> From<&'a ast::PatternMatchInvalid> for AnyNodeRef<'a> {
    fn from(node: &'a ast::PatternMatchInvalid) -> Self {
        AnyNodeRef::PatternMatchInvalid(node)
    }
}

impl<'a> From<&'a ast::PatternArguments> for AnyNodeRef<'a> {
    fn from(node: &'a ast::PatternArguments) -> Self {
        AnyNodeRef::PatternArguments(node)
    }
}

impl<'a> From<&'a ast::PatternKeyword> for AnyNodeRef<'a> {
    fn from(node: &'a ast::PatternKeyword) -> Self {
        AnyNodeRef::PatternKeyword(node)
    }
}

impl<'a> From<&'a Decorator> for AnyNodeRef<'a> {
    fn from(node: &'a Decorator) -> Self {
        AnyNodeRef::Decorator(node)
    }
}

impl<'a> From<&'a ast::TypeParams> for AnyNodeRef<'a> {
    fn from(node: &'a ast::TypeParams) -> Self {
        AnyNodeRef::TypeParams(node)
    }
}
impl<'a> From<&'a TypeParamTypeVar> for AnyNodeRef<'a> {
    fn from(node: &'a TypeParamTypeVar) -> Self {
        AnyNodeRef::TypeParamTypeVar(node)
    }
}

impl<'a> From<&'a TypeParamTypeVarTuple> for AnyNodeRef<'a> {
    fn from(node: &'a TypeParamTypeVarTuple) -> Self {
        AnyNodeRef::TypeParamTypeVarTuple(node)
    }
}

impl<'a> From<&'a TypeParamParamSpec> for AnyNodeRef<'a> {
    fn from(node: &'a TypeParamParamSpec) -> Self {
        AnyNodeRef::TypeParamParamSpec(node)
    }
}

impl<'a> From<&'a ast::FString> for AnyNodeRef<'a> {
    fn from(node: &'a ast::FString) -> Self {
        AnyNodeRef::FString(node)
    }
}

impl<'a> From<&'a ast::StringLiteral> for AnyNodeRef<'a> {
    fn from(node: &'a ast::StringLiteral) -> Self {
        AnyNodeRef::StringLiteral(node)
    }
}

impl<'a> From<&'a ast::BytesLiteral> for AnyNodeRef<'a> {
    fn from(node: &'a ast::BytesLiteral) -> Self {
        AnyNodeRef::BytesLiteral(node)
    }
}

impl<'a> From<&'a Stmt> for AnyNodeRef<'a> {
    fn from(stmt: &'a Stmt) -> Self {
        match stmt {
            Stmt::FunctionDef(node) => AnyNodeRef::StmtFunctionDef(node),
            Stmt::ClassDef(node) => AnyNodeRef::StmtClassDef(node),
            Stmt::Return(node) => AnyNodeRef::StmtReturn(node),
            Stmt::Delete(node) => AnyNodeRef::StmtDelete(node),
            Stmt::TypeAlias(node) => AnyNodeRef::StmtTypeAlias(node),
            Stmt::Assign(node) => AnyNodeRef::StmtAssign(node),
            Stmt::AugAssign(node) => AnyNodeRef::StmtAugAssign(node),
            Stmt::AnnAssign(node) => AnyNodeRef::StmtAnnAssign(node),
            Stmt::For(node) => AnyNodeRef::StmtFor(node),
            Stmt::While(node) => AnyNodeRef::StmtWhile(node),
            Stmt::If(node) => AnyNodeRef::StmtIf(node),
            Stmt::With(node) => AnyNodeRef::StmtWith(node),
            Stmt::Match(node) => AnyNodeRef::StmtMatch(node),
            Stmt::Raise(node) => AnyNodeRef::StmtRaise(node),
            Stmt::Try(node) => AnyNodeRef::StmtTry(node),
            Stmt::Assert(node) => AnyNodeRef::StmtAssert(node),
            Stmt::Import(node) => AnyNodeRef::StmtImport(node),
            Stmt::ImportFrom(node) => AnyNodeRef::StmtImportFrom(node),
            Stmt::Global(node) => AnyNodeRef::StmtGlobal(node),
            Stmt::Nonlocal(node) => AnyNodeRef::StmtNonlocal(node),
            Stmt::Expr(node) => AnyNodeRef::StmtExpr(node),
            Stmt::Pass(node) => AnyNodeRef::StmtPass(node),
            Stmt::Break(node) => AnyNodeRef::StmtBreak(node),
            Stmt::Continue(node) => AnyNodeRef::StmtContinue(node),
            Stmt::IpyEscapeCommand(node) => AnyNodeRef::StmtIpyEscapeCommand(node),
        }
    }
}

impl<'a> From<&'a Expr> for AnyNodeRef<'a> {
    fn from(expr: &'a Expr) -> Self {
        match expr {
            Expr::BoolOp(node) => AnyNodeRef::BoolOpExpr(node),
            Expr::NamedExpr(node) => AnyNodeRef::NamedExprExpr(node),
            Expr::BinOp(node) => AnyNodeRef::BinOpExpr(node),
            Expr::UnaryOp(node) => AnyNodeRef::UnaryOpExpr(node),
            Expr::Lambda(node) => AnyNodeRef::LambdaExpr(node),
            Expr::IfExp(node) => AnyNodeRef::IfExpr(node),
            Expr::Dict(node) => AnyNodeRef::DictExpr(node),
            Expr::Set(node) => AnyNodeRef::SetExpr(node),
            Expr::ListComp(node) => AnyNodeRef::ListCompExpr(node),
            Expr::SetComp(node) => AnyNodeRef::SetCompExpr(node),
            Expr::DictComp(node) => AnyNodeRef::DictCompExpr(node),
            Expr::GeneratorExp(node) => AnyNodeRef::GeneratorExpExpr(node),
            Expr::Await(node) => AnyNodeRef::AwaitExpr(node),
            Expr::Yield(node) => AnyNodeRef::YieldExpr(node),
            Expr::YieldFrom(node) => AnyNodeRef::YieldFromExpr(node),
            Expr::Compare(node) => AnyNodeRef::CompareExpr(node),
            Expr::Call(node) => AnyNodeRef::CallExpr(node),
            Expr::FString(node) => AnyNodeRef::FStringExpr(node),
            Expr::StringLiteral(node) => AnyNodeRef::StringLiteralExpr(node),
            Expr::BytesLiteral(node) => AnyNodeRef::BytesLiteralExpr(node),
            Expr::NumberLiteral(node) => AnyNodeRef::NumberLiteralExpr(node),
            Expr::BooleanLiteral(node) => AnyNodeRef::BooleanLiteralExpr(node),
            Expr::NoneLiteral(node) => AnyNodeRef::NoneLiteralExpr(node),
            Expr::EllipsisLiteral(node) => AnyNodeRef::EllipsisLiteralExpr(node),
            Expr::Attribute(node) => AnyNodeRef::AttributeExpr(node),
            Expr::Subscript(node) => AnyNodeRef::SubscriptExpr(node),
            Expr::Starred(node) => AnyNodeRef::StarredExpr(node),
            Expr::Name(node) => AnyNodeRef::NameExpr(node),
            Expr::List(node) => AnyNodeRef::ListExpr(node),
            Expr::Tuple(node) => AnyNodeRef::TupleExpr(node),
            Expr::Slice(node) => AnyNodeRef::SliceExpr(node),
            Expr::IpyEscapeCommand(node) => AnyNodeRef::IpyEscapeCommandExpr(node),
            Expr::Invalid(node) => AnyNodeRef::InvalidExpr(node),
        }
    }
}

impl<'a> From<&'a Mod> for AnyNodeRef<'a> {
    fn from(module: &'a Mod) -> Self {
        match module {
            Mod::Module(node) => AnyNodeRef::ModModule(node),
            Mod::Expression(node) => AnyNodeRef::ModessionExpr(node),
        }
    }
}

impl<'a> From<&'a FStringElement> for AnyNodeRef<'a> {
    fn from(element: &'a FStringElement) -> Self {
        match element {
            FStringElement::Expression(node) => AnyNodeRef::FStringessionElementExpr(node),
            FStringElement::Literal(node) => AnyNodeRef::FStringLiteralElement(node),
            FStringElement::Invalid(node) => AnyNodeRef::FStringInvalidElement(node),
        }
    }
}

impl<'a> From<&'a Pattern> for AnyNodeRef<'a> {
    fn from(pattern: &'a Pattern) -> Self {
        match pattern {
            Pattern::MatchValue(node) => AnyNodeRef::PatternMatchValue(node),
            Pattern::MatchSingleton(node) => AnyNodeRef::PatternMatchSingleton(node),
            Pattern::MatchSequence(node) => AnyNodeRef::PatternMatchSequence(node),
            Pattern::MatchMapping(node) => AnyNodeRef::PatternMatchMapping(node),
            Pattern::MatchClass(node) => AnyNodeRef::PatternMatchClass(node),
            Pattern::MatchStar(node) => AnyNodeRef::PatternMatchStar(node),
            Pattern::MatchAs(node) => AnyNodeRef::PatternMatchAs(node),
            Pattern::MatchOr(node) => AnyNodeRef::PatternMatchOr(node),
            Pattern::Invalid(node) => AnyNodeRef::PatternMatchInvalid(node),
        }
    }
}

impl<'a> From<&'a TypeParam> for AnyNodeRef<'a> {
    fn from(type_param: &'a TypeParam) -> Self {
        match type_param {
            TypeParam::TypeVar(node) => AnyNodeRef::TypeParamTypeVar(node),
            TypeParam::TypeVarTuple(node) => AnyNodeRef::TypeParamTypeVarTuple(node),
            TypeParam::ParamSpec(node) => AnyNodeRef::TypeParamParamSpec(node),
        }
    }
}

impl<'a> From<&'a ExceptHandler> for AnyNodeRef<'a> {
    fn from(handler: &'a ExceptHandler) -> Self {
        match handler {
            ExceptHandler::ExceptHandler(handler) => {
                AnyNodeRef::ExceptHandlerExceptHandler(handler)
            }
        }
    }
}

impl<'a> From<&'a Comprehension> for AnyNodeRef<'a> {
    fn from(node: &'a Comprehension) -> Self {
        AnyNodeRef::Comprehension(node)
    }
}
impl<'a> From<&'a Arguments> for AnyNodeRef<'a> {
    fn from(node: &'a Arguments) -> Self {
        AnyNodeRef::Arguments(node)
    }
}
impl<'a> From<&'a Parameters> for AnyNodeRef<'a> {
    fn from(node: &'a Parameters) -> Self {
        AnyNodeRef::Parameters(node)
    }
}
impl<'a> From<&'a Parameter> for AnyNodeRef<'a> {
    fn from(node: &'a Parameter) -> Self {
        AnyNodeRef::Parameter(node)
    }
}
impl<'a> From<&'a ParameterWithDefault> for AnyNodeRef<'a> {
    fn from(node: &'a ParameterWithDefault) -> Self {
        AnyNodeRef::ParameterWithDefault(node)
    }
}
impl<'a> From<&'a Keyword> for AnyNodeRef<'a> {
    fn from(node: &'a Keyword) -> Self {
        AnyNodeRef::Keyword(node)
    }
}
impl<'a> From<&'a Alias> for AnyNodeRef<'a> {
    fn from(node: &'a Alias) -> Self {
        AnyNodeRef::Alias(node)
    }
}
impl<'a> From<&'a WithItem> for AnyNodeRef<'a> {
    fn from(node: &'a WithItem) -> Self {
        AnyNodeRef::WithItem(node)
    }
}
impl<'a> From<&'a MatchCase> for AnyNodeRef<'a> {
    fn from(node: &'a MatchCase) -> Self {
        AnyNodeRef::MatchCase(node)
    }
}

impl Ranged for AnyNodeRef<'_> {
    fn range(&self) -> TextRange {
        match self {
            AnyNodeRef::ModModule(node) => node.range(),
            AnyNodeRef::ModessionExpr(node) => node.range(),
            AnyNodeRef::StmtFunctionDef(node) => node.range(),
            AnyNodeRef::StmtClassDef(node) => node.range(),
            AnyNodeRef::StmtReturn(node) => node.range(),
            AnyNodeRef::StmtDelete(node) => node.range(),
            AnyNodeRef::StmtTypeAlias(node) => node.range(),
            AnyNodeRef::StmtAssign(node) => node.range(),
            AnyNodeRef::StmtAugAssign(node) => node.range(),
            AnyNodeRef::StmtAnnAssign(node) => node.range(),
            AnyNodeRef::StmtFor(node) => node.range(),
            AnyNodeRef::StmtWhile(node) => node.range(),
            AnyNodeRef::StmtIf(node) => node.range(),
            AnyNodeRef::StmtWith(node) => node.range(),
            AnyNodeRef::StmtMatch(node) => node.range(),
            AnyNodeRef::StmtRaise(node) => node.range(),
            AnyNodeRef::StmtTry(node) => node.range(),
            AnyNodeRef::StmtAssert(node) => node.range(),
            AnyNodeRef::StmtImport(node) => node.range(),
            AnyNodeRef::StmtImportFrom(node) => node.range(),
            AnyNodeRef::StmtGlobal(node) => node.range(),
            AnyNodeRef::StmtNonlocal(node) => node.range(),
            AnyNodeRef::StmtExpr(node) => node.range(),
            AnyNodeRef::StmtPass(node) => node.range(),
            AnyNodeRef::StmtBreak(node) => node.range(),
            AnyNodeRef::StmtContinue(node) => node.range(),
            AnyNodeRef::StmtIpyEscapeCommand(node) => node.range(),
            AnyNodeRef::BoolOpExpr(node) => node.range(),
            AnyNodeRef::NamedExprExpr(node) => node.range(),
            AnyNodeRef::BinOpExpr(node) => node.range(),
            AnyNodeRef::UnaryOpExpr(node) => node.range(),
            AnyNodeRef::LambdaExpr(node) => node.range(),
            AnyNodeRef::IfExpr(node) => node.range(),
            AnyNodeRef::DictExpr(node) => node.range(),
            AnyNodeRef::SetExpr(node) => node.range(),
            AnyNodeRef::ListCompExpr(node) => node.range(),
            AnyNodeRef::SetCompExpr(node) => node.range(),
            AnyNodeRef::DictCompExpr(node) => node.range(),
            AnyNodeRef::GeneratorExpExpr(node) => node.range(),
            AnyNodeRef::AwaitExpr(node) => node.range(),
            AnyNodeRef::YieldExpr(node) => node.range(),
            AnyNodeRef::YieldFromExpr(node) => node.range(),
            AnyNodeRef::CompareExpr(node) => node.range(),
            AnyNodeRef::CallExpr(node) => node.range(),
            AnyNodeRef::FStringessionElementExpr(node) => node.range(),
            AnyNodeRef::FStringLiteralElement(node) => node.range(),
            AnyNodeRef::FStringInvalidElement(node) => node.range(),
            AnyNodeRef::FStringExpr(node) => node.range(),
            AnyNodeRef::StringLiteralExpr(node) => node.range(),
            AnyNodeRef::BytesLiteralExpr(node) => node.range(),
            AnyNodeRef::NumberLiteralExpr(node) => node.range(),
            AnyNodeRef::BooleanLiteralExpr(node) => node.range(),
            AnyNodeRef::NoneLiteralExpr(node) => node.range(),
            AnyNodeRef::EllipsisLiteralExpr(node) => node.range(),
            AnyNodeRef::AttributeExpr(node) => node.range(),
            AnyNodeRef::SubscriptExpr(node) => node.range(),
            AnyNodeRef::StarredExpr(node) => node.range(),
            AnyNodeRef::NameExpr(node) => node.range(),
            AnyNodeRef::ListExpr(node) => node.range(),
            AnyNodeRef::TupleExpr(node) => node.range(),
            AnyNodeRef::SliceExpr(node) => node.range(),
            AnyNodeRef::IpyEscapeCommandExpr(node) => node.range(),
            AnyNodeRef::InvalidExpr(node) => node.range(),
            AnyNodeRef::ExceptHandlerExceptHandler(node) => node.range(),
            AnyNodeRef::PatternMatchValue(node) => node.range(),
            AnyNodeRef::PatternMatchSingleton(node) => node.range(),
            AnyNodeRef::PatternMatchSequence(node) => node.range(),
            AnyNodeRef::PatternMatchMapping(node) => node.range(),
            AnyNodeRef::PatternMatchClass(node) => node.range(),
            AnyNodeRef::PatternMatchStar(node) => node.range(),
            AnyNodeRef::PatternMatchAs(node) => node.range(),
            AnyNodeRef::PatternMatchOr(node) => node.range(),
            AnyNodeRef::PatternArguments(node) => node.range(),
            AnyNodeRef::PatternKeyword(node) => node.range(),
            AnyNodeRef::PatternMatchInvalid(node) => node.range(),
            AnyNodeRef::Comprehension(node) => node.range(),
            AnyNodeRef::Arguments(node) => node.range(),
            AnyNodeRef::Parameters(node) => node.range(),
            AnyNodeRef::Parameter(node) => node.range(),
            AnyNodeRef::ParameterWithDefault(node) => node.range(),
            AnyNodeRef::Keyword(node) => node.range(),
            AnyNodeRef::Alias(node) => node.range(),
            AnyNodeRef::WithItem(node) => node.range(),
            AnyNodeRef::MatchCase(node) => node.range(),
            AnyNodeRef::Decorator(node) => node.range(),
            AnyNodeRef::ElifElseClause(node) => node.range(),
            AnyNodeRef::TypeParams(node) => node.range(),
            AnyNodeRef::TypeParamTypeVar(node) => node.range(),
            AnyNodeRef::TypeParamTypeVarTuple(node) => node.range(),
            AnyNodeRef::TypeParamParamSpec(node) => node.range(),
            AnyNodeRef::FString(node) => node.range(),
            AnyNodeRef::StringLiteral(node) => node.range(),
            AnyNodeRef::BytesLiteral(node) => node.range(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum NodeKind {
    ModModule,
    ModInteractive,
    ModessionExpr,
    ModFunctionType,
    StmtFunctionDef,
    StmtClassDef,
    StmtReturn,
    StmtDelete,
    StmtTypeAlias,
    StmtAssign,
    StmtAugAssign,
    StmtAnnAssign,
    StmtFor,
    StmtWhile,
    StmtIf,
    StmtWith,
    StmtMatch,
    StmtRaise,
    StmtTry,
    StmtAssert,
    StmtImport,
    StmtImportFrom,
    StmtGlobal,
    StmtNonlocal,
    StmtIpyEscapeCommand,
    StmtExpr,
    StmtPass,
    StmtBreak,
    StmtContinue,
    BoolOpExpr,
    NamedExprExpr,
    BinOpExpr,
    UnaryOpExpr,
    LambdaExpr,
    IfExpr,
    DictExpr,
    SetExpr,
    ListCompExpr,
    SetCompExpr,
    DictCompExpr,
    GeneratorExpExpr,
    AwaitExpr,
    YieldExpr,
    YieldFromExpr,
    CompareExpr,
    CallExpr,
    FStringessionElementExpr,
    FStringLiteralElement,
    FStringInvalidElement,
    FStringExpr,
    StringLiteralExpr,
    BytesLiteralExpr,
    NumberLiteralExpr,
    BooleanLiteralExpr,
    NoneLiteralExpr,
    EllipsisLiteralExpr,
    AttributeExpr,
    SubscriptExpr,
    StarredExpr,
    NameExpr,
    ListExpr,
    TupleExpr,
    SliceExpr,
    IpyEscapeCommandExpr,
    InvalidExpr,
    ExceptHandlerExceptHandler,
    PatternMatchValue,
    PatternMatchSingleton,
    PatternMatchSequence,
    PatternMatchMapping,
    PatternMatchClass,
    PatternMatchStar,
    PatternMatchAs,
    PatternMatchOr,
    PatternArguments,
    PatternKeyword,
    PatternInvalid,
    TypeIgnoreTypeIgnore,
    Comprehension,
    Arguments,
    Parameters,
    Parameter,
    ParameterWithDefault,
    Keyword,
    Alias,
    WithItem,
    MatchCase,
    Decorator,
    ElifElseClause,
    TypeParams,
    TypeParamTypeVar,
    TypeParamTypeVarTuple,
    TypeParamParamSpec,
    FString,
    StringLiteral,
    BytesLiteral,
}
