use crate::csaf::types::csaf_datetime::CsafDateTime;

pub trait WithOptionalGroupIds {
    /// Returns the product group IDs associated with this entity
    fn get_group_ids(&self) -> Option<impl Iterator<Item = &str> + '_>;
}

pub trait WithOptionalProductIds {
    /// Returns the product IDs associated with this entity
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &str> + '_>;
}

/// Implements `WithOptionalGroupIds` or `WithOptionalProductIds` for a type.
///
/// If `ReturnsValues` is specified, the implementation expects `group_ids` or `product_ids`
/// to be an `Option<T>` field where T derefs to a Vec of items that have `as_str()`.
///
/// # Usage
/// - `impl_optional_ids!(MyType, WithOptionalGroupIds, ReturnsValues)` — delegates to `self.group_ids`
/// - `impl_optional_ids!(MyType, WithOptionalGroupIds, ReturnsEmpty)` — always returns `None`
/// - `impl_optional_ids!(MyType, WithOptionalProductIds, ReturnsValues)` — delegates to `self.product_ids`
/// - `impl_optional_ids!(MyType, WithOptionalProductIds, ReturnsEmpty)` — always returns `None`
macro_rules! impl_optional_ids {
    ($type:ty, WithOptionalGroupIds, ReturnsValues) => {
        impl $crate::csaf_traits::WithOptionalGroupIds for $type {
            fn get_group_ids(&self) -> Option<impl Iterator<Item = &str> + '_> {
                self.group_ids
                    .as_ref()
                    .map(|items| items.iter().map(|x| x.as_str()))
            }
        }
    };
    ($type:ty, WithOptionalGroupIds, ReturnsEmpty) => {
        impl $crate::csaf_traits::WithOptionalGroupIds for $type {
            fn get_group_ids(&self) -> Option<impl Iterator<Item = &str> + '_> {
                None::<::std::iter::Empty<&str>>
            }
        }
    };
    ($type:ty, WithOptionalProductIds, ReturnsValues) => {
        impl $crate::csaf_traits::WithOptionalProductIds for $type {
            fn get_product_ids(&self) -> Option<impl Iterator<Item = &str> + '_> {
                self.product_ids
                    .as_ref()
                    .map(|items| items.iter().map(|x| x.as_str()))
            }
        }
    };
    ($type:ty, WithOptionalProductIds, ReturnsEmpty) => {
        impl $crate::csaf_traits::WithOptionalProductIds for $type {
            fn get_product_ids(&self) -> Option<impl Iterator<Item = &str> + '_> {
                None::<::std::iter::Empty<&str>>
            }
        }
    };
}

pub trait WithDate {
    /// Returns the date associated with this entity
    fn get_date(&self) -> CsafDateTime;
}

/// Implements `WithDate` by delegating to `self.date`.
macro_rules! impl_with_date {
    ($type:ty) => {
        impl $crate::csaf_traits::WithDate for $type {
            fn get_date(&self) -> $crate::csaf::types::csaf_datetime::CsafDateTime {
                $crate::csaf::types::csaf_datetime::CsafDateTime::from(&self.date)
            }
        }
    };
}

pub trait WithOptionalDate {
    /// Returns the date associated with this entity
    fn get_date(&self) -> Option<CsafDateTime>;
}

/// Implements `WithOptionalDate` by delegating to `self.date`.
macro_rules! impl_with_optional_date {
    ($type:ty) => {
        impl $crate::csaf_traits::WithOptionalDate for $type {
            fn get_date(&self) -> Option<$crate::csaf::types::csaf_datetime::CsafDateTime> {
                self.date
                    .as_ref()
                    .map($crate::csaf::types::csaf_datetime::CsafDateTime::from)
            }
        }
    };
}

pub(crate) use impl_optional_ids;
pub(crate) use impl_with_date;
pub(crate) use impl_with_optional_date;
