use std::ops::Deref;
use crate::csaf::traits::util::generic_with::{WithDate, WithOptionalGroupIds, WithOptionalProductIds};
use crate::csaf::types::csaf_datetime::CsafDateTime;

/// Marker type for features that are not present in CSAF 2.0.
///
/// This type is used as an associated type when a feature does not exist in CSAF 2.0
/// (so far: `FirstKnownExploitationDates`, `SharingGroup`). It cannot be instantiated
/// since it is an empty enum.
#[derive(Debug, Clone, Copy)]
pub enum NotPresentInCsaf20 {}

impl NotPresentInCsaf20 {
    /// Converts `self` into any type. `NotPresentInCsaf20` can never be instantiated,
    /// so this method can never actually be called.
    #[inline]
    pub fn into_any<T>(self) -> T {
        match self {}
    }

    /// Converts `self` into an empty iterator option of any type. `NotPresentInCsaf20` can never be
    /// instantiated, so this method can never actually be called.
    #[inline]
    pub fn into_any_iter<T>(self) -> Option<std::iter::Empty<T>> {
        match self {}
    }
}

impl WithOptionalProductIds for NotPresentInCsaf20 {
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.into_any_iter()
    }
}

impl WithOptionalGroupIds for NotPresentInCsaf20 {
    fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.into_any_iter()
    }
}

impl WithDate for NotPresentInCsaf20 {
    fn get_date(&self) -> CsafDateTime {
        self.into_any()
    }
}

impl Deref for NotPresentInCsaf20 {
    type Target = String;

    fn deref(&self) -> &String {
        self.into_any()
    }
}