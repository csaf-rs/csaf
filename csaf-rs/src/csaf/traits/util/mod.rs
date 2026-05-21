pub mod extract_references;
pub mod generic_with;
pub mod not_present_20;
pub mod resolve_product_groups;

pub(crate) use generic_with::{
    impl_with_date, impl_with_optional_date, impl_with_optional_group_ids, impl_with_optional_product_ids,
    impl_without_group_ids, impl_without_product_ids,
};
