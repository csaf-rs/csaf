pub mod extract_references;
pub mod generic_with;
pub mod impl_macros;
pub mod not_present_20;
pub mod resolve_product_groups;

pub(crate) use generic_with::{impl_optional_ids, impl_with_date, impl_with_optional_date};
pub(crate) use impl_macros::{
    impl_optional_str_field_getter, impl_optional_str_iter_field_getter, impl_str_field_getter,
    impl_str_iter_field_getter,
};
