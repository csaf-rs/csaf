use crate::csaf::traits::util::generic_with::{WithDate, WithOptionalGroupIds, WithOptionalProductIds};
use crate::csaf::types::csaf_datetime::CsafDateTime;

/// Marker type for features that are not present in CSAF 2.1.
/// It cannot be instantiated since it is an empty enum.
#[derive(Debug, Clone, Copy)]
pub enum NotPresentInCsaf21 {}

impl NotPresentInCsaf21 {
    /// Converts `self` into any type. `NotPresentInCsaf21` can never be instantiated,
    /// so this method can never actually be called.
    #[inline]
    pub fn into_any<T>(self) -> T {
        match self {}
    }

    /// Converts `self` into an empty iterator option of any type. `NotPresentInCsaf21` can never be
    /// instantiated, so this method can never actually be called.
    #[inline]
    pub fn into_any_iter<T>(self) -> Option<std::iter::Empty<T>> {
        match self {}
    }
}

impl WithOptionalProductIds for NotPresentInCsaf21 {
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &str> + '_> {
        self.into_any_iter()
    }
}

impl WithOptionalGroupIds for NotPresentInCsaf21 {
    fn get_group_ids(&self) -> Option<impl Iterator<Item = &str> + '_> {
        self.into_any_iter()
    }
}

impl WithDate for NotPresentInCsaf21 {
    fn get_date(&self) -> CsafDateTime {
        self.into_any()
    }
}
