use super::{
    not::Not, passthrough::Passthrough, ActiveFilter, DynamicFilter, FilterResult, GroupMatcher,
    LayoutFilter,
};
use crate::internals::{
    alloc_prelude::*, query::view::Fetch, storage::component::ComponentTypeId, world::WorldId,
};

/// A filter which always matches `true`.
#[derive(Debug, Clone, Default)]
pub struct Any;

impl GroupMatcher for Any {
    fn can_match_group() -> bool {
        false
    }
    fn group_components() -> Vec<ComponentTypeId> {
        vec![]
    }
}

impl LayoutFilter for Any {
    fn matches_layout(&self, _: &[ComponentTypeId]) -> FilterResult {
        FilterResult::Match(true)
    }
}

impl DynamicFilter for Any {
    fn prepare(&mut self, _: WorldId) {}

    fn matches_archetype<F: Fetch>(&mut self, _: &F) -> FilterResult {
        FilterResult::Match(true)
    }
}

impl core::ops::Not for Any {
    type Output = Not<Self>;

    #[inline]
    fn not(self) -> Self::Output {
        Not { filter: self }
    }
}

impl<Rhs: ActiveFilter> core::ops::BitAnd<Rhs> for Any {
    type Output = Rhs;

    #[inline]
    fn bitand(self, rhs: Rhs) -> Self::Output {
        rhs
    }
}

impl core::ops::BitAnd<Passthrough> for Any {
    type Output = Self;

    #[inline]
    fn bitand(self, _: Passthrough) -> Self::Output {
        self
    }
}

impl<Rhs: ActiveFilter> core::ops::BitOr<Rhs> for Any {
    type Output = Self;

    #[inline]
    fn bitor(self, _: Rhs) -> Self::Output {
        self
    }
}

impl core::ops::BitOr<Passthrough> for Any {
    type Output = Self;

    #[inline]
    fn bitor(self, _: Passthrough) -> Self::Output {
        self
    }
}
