use std::ops::Index;

use python_ast::{AnyNodeRef, Expr, Stmt};
use ruff_index::{newtype_index, IndexVec};
use ruff_text_size::Ranged;

#[newtype_index]
pub struct NodeId;

#[derive(Debug)]
pub struct NodeWithParent<'a> {
    pub node: AnyNodeRef<'a>,
    parent: Option<NodeId>,
}

impl Ranged for NodeWithParent<'_> {
    fn range(&self) -> ruff_text_size::TextRange {
        self.node.range()
    }
}

#[derive(Debug, Default)]
pub struct Nodes<'a>(IndexVec<NodeId, NodeWithParent<'a>>);

impl<'a> Nodes<'a> {
    pub fn insert(&mut self, node: AnyNodeRef<'a>, parent: Option<NodeId>) -> NodeId {
        self.0.push(NodeWithParent { node, parent })
    }

    #[inline]
    pub fn parend_id(&self, node_id: NodeId) -> Option<NodeId> {
        self.0[node_id].parent
    }

    pub fn ancestor_ids(&self, node_id: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        std::iter::successors(Some(node_id), |&node_id| self.0[node_id].parent)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, NodeWithParent<'_>> {
        self.0.iter()
    }
}

impl<'a> Index<NodeId> for Nodes<'a> {
    type Output = AnyNodeRef<'a>;

    fn index(&self, index: NodeId) -> &Self::Output {
        &self.0[index].node
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NodeRef<'a> {
    Stmt(&'a Stmt),
    Expr(&'a Expr),
}

impl<'a> NodeRef<'a> {
    pub fn as_stmt(&self) -> Option<&Stmt> {
        match self {
            NodeRef::Stmt(stmt) => Some(stmt),
            _ => None,
        }
    }

    pub fn as_expr(&self) -> Option<&Expr> {
        match self {
            NodeRef::Expr(expr) => Some(expr),
            _ => None,
        }
    }
}

impl<'a> From<&'a Expr> for NodeRef<'a> {
    fn from(value: &'a Expr) -> Self {
        NodeRef::Expr(value)
    }
}

impl<'a> From<&'a Stmt> for NodeRef<'a> {
    fn from(value: &'a Stmt) -> Self {
        NodeRef::Stmt(value)
    }
}

impl Ranged for NodeRef<'_> {
    fn range(&self) -> ruff_text_size::TextRange {
        match self {
            NodeRef::Stmt(stmt) => stmt.range(),
            NodeRef::Expr(expr) => expr.range(),
        }
    }
}
