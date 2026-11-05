use serde_json::Value;

use crate::schema::loose;

use super::helpers::for_each_product_identification_helper_via_as_object_mut;
use super::{Convert, Diagnostics, Settings};

pub struct LowercaseHashValue;

impl Convert for LowercaseHashValue {
    fn convert(&self, doc: &mut loose::Document, _settings: &Settings, _diagnostics: &mut dyn Diagnostics) {
        for_each_product_identification_helper_via_as_object_mut(doc, |helper| {
            let Some(Value::Array(hashes)) = helper.get_mut("hashes") else {
                return;
            };

            for hash in hashes {
                let Some(Value::Array(file_hashes)) = hash.get_mut("file_hashes") else {
                    continue;
                };

                for file_hash in file_hashes {
                    if let Some(Value::String(value)) = file_hash.get_mut("value") {
                        // CSAF 2.1 requires lowercase hexadecimal hash values
                        value.make_ascii_lowercase();
                    }
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use crate::converter::tests::NoDiagnostics;

    use super::*;

    const UPPERCASE_HASH: &str = "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF";
    const LOWERCASE_HASH: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    fn convert(input: Value) -> Value {
        let mut doc = loose::Document::new(input);
        LowercaseHashValue.convert(&mut doc, &Settings::default(), &mut NoDiagnostics);
        doc.into_raw()
    }

    fn product(product_id: &str, hash_value: Value) -> Value {
        json!({
            "name": product_id,
            "product_id": product_id,
            "product_identification_helper": {
                "hashes": [{
                    "filename": "example.bin",
                    "file_hashes": [{
                        "algorithm": "sha256",
                        "value": hash_value
                    }]
                }]
            }
        })
    }

    fn document_with_all_product_locations(hash_value: Value) -> Value {
        json!({
            "product_tree": {
                "branches": [{
                    "category": "vendor",
                    "name": "Vendor",
                    "branches": [{
                        "category": "product_name",
                        "name": "Product",
                        "product": product("branch-product", hash_value.clone())
                    }]
                }],
                "full_product_names": [
                    product("direct-product", hash_value.clone())
                ],
                "relationships": [{
                    "category": "default_component_of",
                    "full_product_name": product("relationship-product", hash_value),
                    "product_reference": "direct-product",
                    "relates_to_product_reference": "branch-product"
                }]
            }
        })
    }

    #[test]
    fn lowercases_hashes_in_every_full_product_name_location() {
        let input = document_with_all_product_locations(json!(UPPERCASE_HASH));
        let expected = document_with_all_product_locations(json!(LOWERCASE_HASH));

        assert_eq!(expected, convert(input));
    }
}
