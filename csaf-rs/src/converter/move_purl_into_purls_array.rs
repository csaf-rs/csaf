use super::{Convert, Diagnostics, Settings};
use crate::converter::helpers::for_each_product_identification_helper_via_as_object_mut;
use crate::schema::loose;

pub struct MoveSinglePurlIntoPurlsArray;

impl Convert for MoveSinglePurlIntoPurlsArray {
    fn convert(&self, doc: &mut loose::Document, _settings: &Settings, _diagnostics: &mut dyn Diagnostics) {
        for_each_product_identification_helper_via_as_object_mut(doc, |helper| {
            if let Some(purl) = helper.remove("purl") {
                match helper.get_mut("purls") {
                    Some(serde_json::Value::Array(purls)) => purls.insert(0, purl),
                    _ => {
                        helper.insert("purls".to_string(), serde_json::Value::Array(vec![purl]));
                    },
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::converter::tests::NoDiagnostics;

    use super::*;

    #[test]
    fn converts_purl_for_every_product_tree_full_product_name() {
        let input = json!({
            "product_tree": {
                "full_product_names": [{
                    "name": "Product A",
                    "product_id": "CSAFPID-0001",
                    "product_identification_helper": {
                        "purl": "pkg:maven/com.example/product-A"
                    }
                }],
                "branches": [{
                    "category": "product_name",
                    "name": "Product B",
                    "branches": [
                        {
                            "category": "product_version",
                            "name": "1.0.0",
                           "product": {
                                "name": "Product B 1.0.0",
                                "product_id": "CSAFPID-0002",
                                "product_identification_helper": {
                                    "purl": "pkg:maven/com.example/product-B"
                                }
                            }
                        }
                    ]
                }],
                "relationships": [{
                    "category": "default_component_of",
                    "full_product_name": {
                        "name": "Product C",
                        "product_id": "CSAFPID-0003",
                        "product_identification_helper": {
                            "purl": "pkg:maven/com.example/product-C"
                        }
                    },
                    "product_reference": "CSAFPID-0001",
                    "relates_to_product_reference": "CSAFPID-0002"
                }]
            }
        });
        let expected = json!({
            "product_tree": {
                "full_product_names": [{
                    "name": "Product A",
                    "product_id": "CSAFPID-0001",
                    "product_identification_helper": {
                        "purls": ["pkg:maven/com.example/product-A"]
                    }
                }],
                "branches": [{
                    "category": "product_name",
                    "name": "Product B",
                    "branches": [
                        {
                            "category": "product_version",
                            "name": "1.0.0",
                            "product": {
                                "name": "Product B 1.0.0",
                                "product_id": "CSAFPID-0002",
                                "product_identification_helper": {
                                    "purls": ["pkg:maven/com.example/product-B"]
                                }
                            }
                        }
                    ]
                }],
                "relationships": [{
                    "category": "default_component_of",
                    "full_product_name": {
                        "name": "Product C",
                        "product_id": "CSAFPID-0003",
                        "product_identification_helper": {
                            "purls": ["pkg:maven/com.example/product-C"]
                        }
                    },
                    "product_reference": "CSAFPID-0001",
                    "relates_to_product_reference": "CSAFPID-0002"
                }]
            }
        });

        let mut doc = loose::Document::new(input);
        MoveSinglePurlIntoPurlsArray.convert(&mut doc, &Settings::default(), &mut NoDiagnostics);

        assert_eq!(expected, doc.into_raw());
    }
}
