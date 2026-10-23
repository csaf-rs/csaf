use super::{Convert, Diagnostics, Settings};
use crate::converter::helpers::for_each_product_identification_helper_via_as_object_mut;
use crate::schema::loose;

pub struct DeduplicateXGenericUris;

impl Convert for DeduplicateXGenericUris {
    fn convert(&self, doc: &mut loose::Document, _settings: &Settings, _diagnostics: &mut dyn Diagnostics) {
        for_each_product_identification_helper_via_as_object_mut(doc, |helper| {
            let Some(serde_json::Value::Array(uris)) = helper.get_mut("x_generic_uris") else {
                return;
            };

            let mut seen = Vec::new();
            uris.retain(|uri| {
                if seen.contains(uri) {
                    false
                } else {
                    seen.push(uri.clone());
                    true
                }
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::converter::tests::NoDiagnostics;

    use super::*;

    #[test]
    fn keeps_first_occurrence_of_each_generic_uri_in_order() {
        let input = json!({
            "product_tree": {
                "full_product_names": [{
                    "name": "product",
                    "product_id": "product",
                    "product_identification_helper": {
                        "x_generic_uris": [
                            {
                                "namespace": "https://example.com",
                                "uri": "https://example.com/uri-b"
                            },
                            {
                                "namespace": "https://example.com",
                                "uri": "https://example.com/uri-a"
                            },
                            {
                                "namespace": "https://example.com",
                                "uri": "https://example.com/uri-b"
                            },
                            {
                                "namespace": "https://example.com",
                                "uri": "https://example.com/uri-c"
                            },
                            {
                                "namespace": "https://example.com",
                                "uri": "https://example.com/uri-a"
                            },
                            {
                                "namespace": "https://not.example.com",
                                "uri": "https://example.com/uri-a"
                            },
                        ]
                    }
                }]
            }
        });
        let mut doc = loose::Document::new(input);

        DeduplicateXGenericUris.convert(&mut doc, &Settings::default(), &mut NoDiagnostics);

        assert_eq!(
            json!({
                "product_tree": {
                    "full_product_names": [{
                        "name": "product",
                        "product_id": "product",
                        "product_identification_helper": {
                            "x_generic_uris": [
                                {"namespace": "https://example.com", "uri": "https://example.com/uri-b"},
                                {"namespace": "https://example.com", "uri": "https://example.com/uri-a"},
                                {"namespace": "https://example.com", "uri": "https://example.com/uri-c"},
                                {"namespace": "https://not.example.com", "uri": "https://example.com/uri-a"}
                            ]
                        }
                    }]
                }
            }),
            doc.into_raw()
        );
    }
}
