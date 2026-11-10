use crate::csaf::traits::util::impl_into_any_getter;
use crate::csaf::traits::util::not_present_20::NotPresentInCsaf20;
use crate::schema::csaf2_1::schema::ExtensionsT as Extensions21;
use serde_json::{Map, Value};

pub trait ExtensionsTrait {
    fn get_raw(&self) -> &Vec<Map<String, Value>>;
}

impl ExtensionsTrait for NotPresentInCsaf20 {
    impl_into_any_getter!(get_raw, &Vec<Map<String, Value>>);
}

impl ExtensionsTrait for Extensions21 {
    fn get_raw(&self) -> &Vec<Map<String, Value>> {
        &self.0
    }
}
