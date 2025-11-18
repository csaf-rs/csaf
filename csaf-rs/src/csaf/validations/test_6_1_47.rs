use crate::csaf::csaf_traits::{ContentTrait, CsafTrait, DocumentTrait, MetricTrait, TrackingTrait, VulnerabilityIdTrait, VulnerabilityTrait};
use crate::csaf::validation::ValidationError;

pub fn test_6_1_47_inconsistent_ssvc_id(
    doc: &impl CsafTrait,
) -> Result<(), Vec<ValidationError>> {
    let vulnerabilities = doc.get_vulnerabilities();

    for (i_v, v) in vulnerabilities.iter().enumerate() {
        if let Some(metrics) = v.get_metrics() {
            for (i_m, m) in metrics.iter().enumerate() {
                if m.get_content().has_ssvc() {
                    match m.get_content().get_ssvc() {
                        Ok(ssvc) => {
                            // Get the SSVC target_ids if they exist
                            if let Some(target_ids) = &ssvc.target_ids {
                                let document_id = doc.get_document().get_tracking().get_id();

                                // Check each target ID
                                for (i_t, target_id) in target_ids.iter().enumerate() {
                                    // Check if target ID equals document ID
                                    if target_id == document_id {
                                        // If there are multiple vulnerabilities, the validation must fail here.
                                        if vulnerabilities.len() > 1 {
                                            return Err(vec![ValidationError {
                                                message: format!("The SSVC target ID equals the document ID '{}' and the document contains multiple vulnerabilities", document_id),
                                                instance_path: format!("/vulnerabilities/{}/metrics/{}/content/ssvc_v2/target_ids/{}", i_v, i_m, i_t),
                                            }]);
                                        }
                                        // Target ID is valid, continue to next
                                        continue;
                                    }

                                    // Check if it matches CVE
                                    if let Some(cve) = v.get_cve() {
                                        if target_id == cve {
                                            continue;
                                        }
                                    }

                                    // Check if it matches any ID in id array
                                    if let Some(ids) = v.get_ids() {
                                        if ids.iter().any(|id| id.get_text() == target_id) {
                                            continue;
                                        }
                                    }

                                    // Return error if target ID is not valid
                                    return Err(vec![ValidationError {
                                        message: format!("The SSVC target ID '{}' does not match the document ID, the CVE ID or any ID in the IDs array of the vulnerability", target_id),
                                        instance_path: format!("/vulnerabilities/{}/metrics/{}/content/ssvc_v2/target_ids/{}", i_v, i_m, i_t),
                                    }]);
                                }
                            }
                        },
                        Err(err) => {
                            return Err(vec![ValidationError {
                                message: format!("Invalid SSVC object: {}", err),
                                instance_path: format!("/vulnerabilities/{}/metrics/{}/content/ssvc_v2", i_v, i_m),
                            }]);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::csaf::test_helper::run_csaf21_tests;
    use crate::csaf::validation::ValidationError;
    use crate::csaf::validations::test_6_1_47::test_6_1_47_inconsistent_ssvc_id;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_47() {
        let instance_path = "/vulnerabilities/0/metrics/0/content/ssvc_v2/target_ids/0".to_string();

        run_csaf21_tests(
            "47",
            test_6_1_47_inconsistent_ssvc_id,
            &HashMap::from([
                ("01", &ValidationError {
                    message: "The SSVC target ID 'CVE-1900-0002' does not match the document ID, the CVE ID or any ID in the IDs array of the vulnerability".to_string(),
                    instance_path: instance_path.clone(),
                }),
                ("02", &ValidationError {
                    message: "The SSVC target ID 'CVE-1900-0001' does not match the document ID, the CVE ID or any ID in the IDs array of the vulnerability".to_string(),
                    instance_path: instance_path.clone(),
                }),
                ("03", &ValidationError {
                    message: "The SSVC target ID '2723' does not match the document ID, the CVE ID or any ID in the IDs array of the vulnerability".to_string(),
                    instance_path: instance_path.clone(),
                }),
                ("04", &ValidationError {
                    message: "The SSVC target ID 'Bug#2723' does not match the document ID, the CVE ID or any ID in the IDs array of the vulnerability".to_string(),
                    instance_path: instance_path.clone(),
                }),
                ("05", &ValidationError {
                    message: "The SSVC target ID 'OASIS_CSAF_TC-CSAF_2.1-2024-6-1-47-15' does not match the document ID, the CVE ID or any ID in the IDs array of the vulnerability".to_string(),
                    instance_path: instance_path.clone(),
                }),
                ("06", &ValidationError {
                    message: "The SSVC target ID equals the document ID 'OASIS_CSAF_TC-CSAF_2.1-2024-6-1-47-06' and the document contains multiple vulnerabilities".to_string(),
                    instance_path: "/vulnerabilities/1/metrics/0/content/ssvc_v2/target_ids/0".to_string(),
                }),
            ])
        );
    }
}