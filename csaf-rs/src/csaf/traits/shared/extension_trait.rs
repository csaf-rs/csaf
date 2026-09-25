use crate::csaf::traits::util::not_present_20::NotPresentInCsaf20;
use crate::schema::csaf2_1::schema::ExtensionsT as Extensions21;
use serde_json::{Map, Value};

pub trait ExtensionsTrait {
    fn get_content(&self) -> &Vec<Map<String, Value>>;
}

impl ExtensionsTrait for NotPresentInCsaf20 {
    fn get_content(&self) -> &Vec<Map<String, Value>> {
        self.into_any()
    }
}

impl ExtensionsTrait for Extensions21 {
    fn get_content(&self) -> &Vec<Map<String, Value>> {
        &self.0
    }
}
