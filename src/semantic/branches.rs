use std::ops::Index;

use python_ast::{ElifElseClause, IfStmt};
use ruff_index::{newtype_index, IndexVec};

#[newtype_index]
pub struct BranchId;

#[derive(Debug)]
pub struct Branch<'a> {
    pub parent: Option<BranchId>,
    pub kind: BranchKind<'a>,
}

#[derive(Debug, Clone, Copy)]
pub enum BranchKind<'a> {
    If(&'a IfStmt),
    ElifElse(&'a ElifElseClause),
}

#[derive(Debug, Default)]
pub struct Branches<'a>(IndexVec<BranchId, Branch<'a>>);

impl<'a> Branches<'a> {
    pub fn insert(&mut self, parent: Branch<'a>) -> BranchId {
        self.0.push(parent)
    }

    pub fn parent_id(&self, node_id: BranchId) -> Option<BranchId> {
        self.0[node_id].parent
    }
}

impl<'a> Index<BranchId> for Branches<'a> {
    type Output = Branch<'a>;

    #[inline]
    fn index(&self, index: BranchId) -> &Self::Output {
        &self.0[index]
    }
}
