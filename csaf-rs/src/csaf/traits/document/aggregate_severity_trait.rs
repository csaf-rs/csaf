use crate::schema::csaf2_0::schema::AggregateSeverity as AggregateSeverity20;
use crate::schema::csaf2_1::schema::AggregateSeverity as AggregateSeverity21;
use crate::csaf::traits::util::impl_macros::{impl_optional_str_field_getter, impl_str_field_getter};

pub trait AggregateSeverityTrait {
    fn get_namespace(&self) -> Option<&str>;
    fn get_text(&self) -> &str;
}

impl AggregateSeverityTrait for AggregateSeverity20 {
    impl_optional_str_field_getter!(get_namespace, namespace);
    impl_str_field_getter!(get_text, text);
}

impl AggregateSeverityTrait for AggregateSeverity21 {
    impl_optional_str_field_getter!(get_namespace, namespace);
    impl_str_field_getter!(get_text, text);
}
