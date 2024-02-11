use std::ops::{Deref, DerefMut};

use ruff_index::{newtype_index, IndexSlice, IndexVec};
use ruff_text_size::TextRange;

use super::ScopeId;

#[newtype_index]
pub struct ResolvedReferenceId;

#[derive(Debug)]
pub struct ResolvedReference {
    pub scope_id: ScopeId,
    pub range: TextRange,
}

#[derive(Debug, Default)]
pub struct ResolvedReferences(IndexVec<ResolvedReferenceId, ResolvedReference>);

impl ResolvedReferences {
    pub fn push(&mut self, scope_id: ScopeId, range: TextRange) -> ResolvedReferenceId {
        let id = self.len();
        self.0.push(ResolvedReference { scope_id, range });
        id.into()
    }
}

impl<'a> Deref for ResolvedReferences {
    type Target = IndexSlice<ResolvedReferenceId, ResolvedReference>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for ResolvedReferences {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
