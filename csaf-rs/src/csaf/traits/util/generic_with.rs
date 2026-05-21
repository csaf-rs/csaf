use crate::csaf::types::csaf_datetime::CsafDateTime;

pub trait WithOptionalGroupIds {
    /// Returns the product group IDs associated with this entity
    fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_>;
}

/// Implements `WithOptionalGroupIds` by delegating to `self.group_ids`.
macro_rules! impl_with_optional_group_ids {
    ($type:ty) => {
        impl $crate::csaf_traits::WithOptionalGroupIds for $type {
            fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
                self.group_ids
                    .as_ref()
                    .map(|g| g.iter().map(|x| ::std::ops::Deref::deref(x)))
            }
        }
    };
}

pub(crate) use impl_with_optional_group_ids;

/// Implements `WithOptionalGroupIds` as always returning `None`.
macro_rules! impl_without_group_ids {
    ($type:ty) => {
        impl $crate::csaf_traits::WithOptionalGroupIds for $type {
            fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
                None::<std::iter::Empty<&String>>
            }
        }
    };
}

pub(crate) use impl_without_group_ids;

pub trait WithOptionalProductIds {
    /// Returns the product IDs associated with this entity
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_>;
}

/// Implements `WithOptionalProductIds` by delegating to `self.product_ids`.
macro_rules! impl_with_optional_product_ids {
    ($type:ty) => {
        impl $crate::csaf_traits::WithOptionalProductIds for $type {
            fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
                self.product_ids
                    .as_ref()
                    .map(|p| p.iter().map(|x| ::std::ops::Deref::deref(x)))
            }
        }
    };
}

pub(crate) use impl_with_optional_product_ids;

/// Implements `WithOptionalProductIds` as always returning `None`.
macro_rules! impl_without_product_ids {
    ($type:ty) => {
        impl $crate::csaf_traits::WithOptionalProductIds for $type {
            fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
                None::<std::iter::Empty<&String>>
            }
        }
    };
}

pub(crate) use impl_without_product_ids;

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

pub(crate) use impl_with_date;

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

pub(crate) use impl_with_optional_date;
