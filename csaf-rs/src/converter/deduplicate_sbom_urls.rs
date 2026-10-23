use super::{Convert, Diagnostics, Settings};
use crate::converter::helpers::for_each_product_identification_helper_via_as_object_mut;
use crate::schema::loose;

pub struct DeduplicateSbomUrls;

impl Convert for DeduplicateSbomUrls {
    fn convert(&self, doc: &mut loose::Document, _settings: &Settings, _diagnostics: &mut dyn Diagnostics) {
        for_each_product_identification_helper_via_as_object_mut(doc, |helper| {
            let Some(serde_json::Value::Array(sbom_urls)) = helper.get_mut("sbom_urls") else {
                return;
            };

            let mut seen = Vec::new();
            sbom_urls.retain(|sbom_url| {
                if seen.contains(sbom_url) {
                    false
                } else {
                    seen.push(sbom_url.clone());
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
    fn keeps_first_occurrence_of_each_sbom_url_in_order() {
        let input = json!({
            "product_tree": {
                "full_product_names": [{
                    "name": "product",
                    "product_id": "product",
                    "product_identification_helper": {
                        "sbom_urls": [
                            "https://example.com/sbom-b.json",
                            "https://example.com/sbom-a.json",
                            "https://example.com/sbom-b.json",
                            "https://example.com/sbom-c.json",
                            "https://example.com/sbom-a.json"
                        ]
                    }
                }]
            }
        });
        let mut doc = loose::Document::new(input);

        DeduplicateSbomUrls.convert(&mut doc, &Settings::default(), &mut NoDiagnostics);

        assert_eq!(
            json!({
                "product_tree": {
                    "full_product_names": [{
                        "name": "product",
                        "product_id": "product",
                        "product_identification_helper": {
                            "sbom_urls": [
                                "https://example.com/sbom-b.json",
                                "https://example.com/sbom-a.json",
                                "https://example.com/sbom-c.json"
                            ]
                        }
                    }]
                }
            }),
            doc.into_raw()
        );
    }
}
