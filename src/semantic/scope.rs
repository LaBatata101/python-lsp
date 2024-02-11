use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use ruff_index::{newtype_index, IndexSlice, IndexVec};

use super::SymbolId;

#[newtype_index]
pub struct ScopeId;

impl ScopeId {
    pub fn global() -> Self {
        ScopeId::from_u32(0)
    }

    pub fn is_global(&self) -> bool {
        self.index() == 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScopeKind {
    Global,
    Class,
    Function,
    Lambda,
}

impl ScopeKind {
    pub fn is_class(&self) -> bool {
        matches!(self, ScopeKind::Class)
    }
}

#[derive(Debug)]
pub struct Scope<'a> {
    pub kind: ScopeKind,
    pub parent: Option<ScopeId>,
    pub symbols: HashMap<&'a str, SymbolId>,
}

impl<'a> Scope<'a> {
    pub fn global() -> Self {
        Self {
            parent: None,
            kind: ScopeKind::Global,
            symbols: HashMap::default(),
        }
    }

    pub fn local(kind: ScopeKind, parent: ScopeId) -> Self {
        Self {
            kind,
            parent: Some(parent),
            symbols: HashMap::default(),
        }
    }

    pub fn add_symbol(&mut self, name: &'a str, id: SymbolId) {
        self.symbols.insert(name, id);
    }

    pub fn get_symbol(&self, name: &str) -> Option<SymbolId> {
        self.symbols.get(name).copied()
    }

    pub fn symbols(&self) -> impl Iterator<Item = (&str, SymbolId)> + '_ {
        self.symbols
            .iter()
            .map(|(&name, &symbol_id)| (name, symbol_id))
    }
}

#[derive(Debug)]
pub struct Scopes<'a>(IndexVec<ScopeId, Scope<'a>>);

impl<'a> Scopes<'a> {
    pub fn push_scope(&mut self, kind: ScopeKind, parent: ScopeId) -> ScopeId {
        self.0.push(Scope::local(kind, parent))
    }

    pub fn global(&self) -> &Scope {
        &self[ScopeId::global()]
    }

    pub fn global_mut(&mut self) -> &mut Scope<'a> {
        &mut self[ScopeId::global()]
    }

    pub fn ancestor_ids(&self, scope_id: ScopeId) -> impl Iterator<Item = ScopeId> + '_ {
        std::iter::successors(Some(scope_id), |&scope_id| self[scope_id].parent)
    }
}

impl Default for Scopes<'_> {
    fn default() -> Self {
        Self(IndexVec::from_raw(vec![Scope::global()]))
    }
}

impl<'a> Deref for Scopes<'a> {
    type Target = IndexSlice<ScopeId, Scope<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Scopes<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
