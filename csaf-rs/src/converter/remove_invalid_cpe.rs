use crate::schema;
use crate::schema::{csaf2_1, loose};

use super::helpers::for_each_product_identification_helper_with_path_via_as_object_mut;
use super::{Convert, Diagnostics, Settings, Warning};

pub struct RemoveInvalidCpe;

impl Convert for RemoveInvalidCpe {
    fn convert(&self, doc: &mut loose::Document, _settings: &Settings, diagnostics: &mut dyn Diagnostics) {
        for_each_product_identification_helper_with_path_via_as_object_mut(doc, |helper, helper_path| {
            // only if there is a CPE
            let Some(cpe) = helper.get("cpe").and_then(serde_json::Value::as_str) else {
                return;
            };

            // try to cast it to CSAF 2.1, which validates against the updated regex
            if csaf2_1::schema::CommonPlatformEnumerationRepresentation::try_from(cpe).is_err() {
                let cpe_str = cpe.to_string();
                helper.remove("cpe");
                diagnostics.warning(format!("{helper_path}/cpe").into(), Warning::InvalidCpeRemoved(cpe_str));
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::converter::tests::OnlyWarnings;

    use super::*;

    fn convert(input: serde_json::Value) -> (serde_json::Value, Vec<(String, Warning)>) {
        let mut doc = loose::Document::new(input);
        let mut diagnostics = OnlyWarnings::default();
        RemoveInvalidCpe.convert(&mut doc, &Settings::default(), &mut diagnostics);
        let warnings = diagnostics
            .warnings()
            .iter()
            .map(|(path, warning)| (path.to_string(), warning.clone()))
            .collect();
        (doc.into_raw(), warnings)
    }

    #[test]
    fn removes_cpes_accepted_only_by_the_csaf_2_0_pattern() {
        let cpe = "NOTAVALIDCPEcpe:/o:example:a:42::anyother";
        let parsed = serde_json::from_value::<crate::schema::csaf2_0::schema::CommonPlatformEnumerationRepresentation>(
            json!(cpe),
        );

        assert!(parsed.is_ok());

        let (output, warnings) = convert(json!({
            "product_tree": {
                "full_product_names": [{
                    "name": "Product A",
                    "product_id": "CSAFPID-0001",
                    "product_identification_helper": {
                        "cpe": "NOTAVALIDCPEcpe:/o:example:a:42::anyother"
                    }
                }]
            }
        }));

        assert_eq!(
            json!({
                "product_tree": {
                    "full_product_names": [{
                        "name": "Product A",
                        "product_id": "CSAFPID-0001",
                        "product_identification_helper": {}
                    }]
                }
            }),
            output
        );
        assert_eq!(
            vec![(
                "/product_tree/full_product_names/0/product_identification_helper/cpe".to_string(),
                Warning::InvalidCpeRemoved("NOTAVALIDCPEcpe:/o:example:a:42::anyother".to_string())
            )],
            warnings
        );
    }

    #[test]
    fn keeps_cpes_accepted_by_the_csaf_2_1_pattern() {
        let input = json!({
            "product_tree": {
                "full_product_names": [{
                    "product_identification_helper": {
                        "cpe": "cpe:/o:example:a:42::anyother"
                    }
                }]
            }
        });

        let (output, warnings) = convert(input.clone());
        assert_eq!(input, output);
        assert!(warnings.is_empty());
    }
}
