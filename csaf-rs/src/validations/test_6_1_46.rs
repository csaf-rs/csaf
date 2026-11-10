use ssvc::ParsedNamespace;

use crate::csaf::types::language::CsafLanguage;
use crate::csaf_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait};
use crate::validation::{IntoTestFindingError, TestFinding, TestFindingData};

#[jsonschema::validator(path = "assets/SelectionList_2_0_0.schema.json", draft = Draft202012)]
struct SsvcValidator;

fn create_invalid_ssvc_error(
    error_message: &str,
    error_path: &str,
    vulnerability_index: usize,
    metric_index: usize,
) -> TestFinding {
    let path_info = if error_path.is_empty() {
        String::new()
    } else {
        format!(" at {error_path}")
    };
    TestFinding::Error(TestFindingData {
        message: format!("Invalid SSVC object{path_info}: {error_message}"),
        instance_path: format!(
            "/vulnerabilities/{vulnerability_index}/metrics/{metric_index}/content/ssvc_v2{error_path}"
        ),
    })
}

/// Validate a language code and append the validation error to the errors vector.
///
/// If the given language is [`CsafLanguage::Invalid`], a [`TestFinding`] is created
/// with the specified JSON path and added to the errors collection.
fn validate_language(
    lang: &str,
    vuln_idx: usize,
    metric_idx: usize,
    selection_idx: usize,
    errors: &mut Option<Vec<TestFinding>>,
) {
    if let Some(CsafLanguage::Invalid(_, err)) = Some(CsafLanguage::from(lang)) {
        errors
            .get_or_insert_default()
            .push(err.into_test_finding_error(&format!(
                "/vulnerabilities/{vuln_idx}/metrics/{metric_idx}/content/ssvc_v2/selections/{selection_idx}/namespace"
            )));
    }
}

/// 6.1.46 Invalid SSVC
///
/// It MUST be tested that the given SSVC object is valid according to the referenced schema.
pub fn test_6_1_46_invalid_ssvc(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let mut errors: Option<Vec<TestFinding>> = None;
    // look for ssvc_v2 metrics in all vulnerabilities and metrics
    for (vulnerability_idx, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(metrics) = vulnerability.get_metrics() {
            for (metric_idx, metric) in metrics.iter().enumerate() {
                let content = metric.get_content();
                // if there is a ssvc_v2 metric
                if let Some(ssvc) = content.get_ssvc_v2_raw() {
                    // schema validation
                    // depending on how we implement lenient parsing, this might need to be
                    // prefaced with json format check
                    for error in SsvcValidator::iter_errors(&serde_json::Value::Object(ssvc.clone())) {
                        errors.get_or_insert_default().push(create_invalid_ssvc_error(
                            &error.to_string(),
                            error.instance_path().as_str(),
                            vulnerability_idx,
                            metric_idx,
                        ));
                    }

                    if let Some(selections) = ssvc.get("selections").and_then(|selections| selections.as_array()) {
                        for (selection_idx, selection) in selections.iter().enumerate() {
                            if let Some(namespace) =
                                selection.pointer("/namespace").and_then(|namespace| namespace.as_str())
                                && let Ok(namespace) = ParsedNamespace::parse_allow_test(namespace)
                                && let Some(extensions) = namespace.extensions
                            {
                                for extension in extensions {
                                    match extension {
                                        ssvc::Extension::EmptyDefaultLanguage | ssvc::Extension::Domain { .. } => {
                                            continue;
                                        },
                                        ssvc::Extension::Language(language)
                                        | ssvc::Extension::Translation { language, .. } => {
                                            validate_language(
                                                &language,
                                                vulnerability_idx,
                                                metric_idx,
                                                selection_idx,
                                                &mut errors,
                                            );
                                        },
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_46, test_6_1_46_invalid_ssvc);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf::types::language::CsafLanguage::Invalid;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_46 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_46() {
        // Case 01: selections object is missing
        // Case 02: key in selections object is missing
        // Case 03: invalid bcp47 tag
        // Case 11: minimal valid ssvc
        // Case 12: valid ssvc
        // Case 13: valid bcp47 tag
        let selection_object_missing = Err(vec![create_invalid_ssvc_error(
            "\"selections\" is a required property",
            "",
            0,
            0,
        )]);
        let key_in_selections_object_is_missing = Err(vec![
            create_invalid_ssvc_error("\"key\" is a required property", "/selections/0", 0, 0),
            create_invalid_ssvc_error("\"key\" is a required property", "/selections/0/values/0", 0, 0),
        ]);
        let Invalid(_, invalid_language_subtag) = CsafLanguage::from("EZ-nn") else {
            unreachable!()
        };

        TESTS_2_1.test_6_1_46.expect(ExpectedResults {
            case_01: selection_object_missing,
            case_02: key_in_selections_object_is_missing,
            // TODO update when new requirements are added #379
            case_03: Err(vec![invalid_language_subtag.into_test_finding_error(
                "/vulnerabilities/0/metrics/0/content/ssvc_v2/selections/0/namespace",
            )]),
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
        });
    }
}
