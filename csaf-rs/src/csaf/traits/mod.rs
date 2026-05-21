pub mod csaf_trait;
pub mod document;
pub mod document_trait;
pub mod product_tree;
pub mod product_tree_trait;
pub mod shared;
pub mod util;
pub mod vulnerabilities;
pub mod vulnerabilities_trait;

pub(crate) use util::{
    impl_with_date, impl_with_optional_date, impl_with_optional_group_ids, impl_with_optional_product_ids,
    impl_without_group_ids, impl_without_product_ids,
};
