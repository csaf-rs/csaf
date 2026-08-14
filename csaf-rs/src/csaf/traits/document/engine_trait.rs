use crate::csaf::traits::util::{impl_optional_str_field_getter, impl_str_field_getter};
use crate::schema::csaf2_0::schema::EngineOfDocumentGeneration as Engine20;
use crate::schema::csaf2_1::schema::EngineOfDocumentGeneration as Engine21;

/// Trait for accessing document generation engine information
pub trait EngineTrait {
    fn get_name(&self) -> &str;
    fn get_version(&self) -> Option<&str>;
}

impl EngineTrait for Engine20 {
    impl_str_field_getter!(get_name, name);
    impl_optional_str_field_getter!(get_version, version);
}

impl EngineTrait for Engine21 {
    impl_str_field_getter!(get_name, name);
    impl_optional_str_field_getter!(get_version, version);
}
