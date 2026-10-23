use super::{Convert, Diagnostics, Settings};
use crate::converter::helpers::for_each_product_identification_helper_via_as_object_mut;
use crate::schema::loose;

pub struct DeduplicateSkus;

impl Convert for DeduplicateSkus {
    fn convert(&self, doc: &mut loose::Document, _settings: &Settings, _diagnostics: &mut dyn Diagnostics) {
        for_each_product_identification_helper_via_as_object_mut(doc, |helper| {
            let Some(serde_json::Value::Array(skus)) = helper.get_mut("skus") else {
                return;
            };

            let mut seen = Vec::new();
            skus.retain(|sku| {
                if seen.contains(sku) {
                    false
                } else {
                    seen.push(sku.clone());
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
    fn keeps_first_occurrence_of_each_sku_in_order() {
        let input = json!({
            "product_tree": {
                "full_product_names": [{
                    "name": "product",
                    "product_id": "product",
                    "product_identification_helper": {
                        "skus": ["BBB", "AAA", "BBB", "CCC", "AAA"]
                    }
                }]
            }
        });
        let mut doc = loose::Document::new(input);

        DeduplicateSkus.convert(&mut doc, &Settings::default(), &mut NoDiagnostics);

        assert_eq!(
            json!({
                "product_tree": {
                    "full_product_names": [{
                        "name": "product",
                        "product_id": "product",
                        "product_identification_helper": {
                            "skus": ["BBB", "AAA", "CCC"]
                        }
                    }]
                }
            }),
            doc.into_raw()
        );
    }
}
