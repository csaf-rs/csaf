use crate::csaf_traits::{
    ContentTrait, CsafTrait, DocumentTrait, MetricTrait, TrackingTrait, VulnerabilityIdTrait, VulnerabilityTrait,
};
use crate::validation::ValidationError;

fn create_document_id_multiple_vulnerabilities_error(
    document_id: &str,
    i_v: usize,
    i_m: usize,
    i_t: usize,
) -> ValidationError {
    ValidationError {
        message: format!(
            "The SSVC target ID equals the document ID '{}' and the document contains multiple vulnerabilities",
            document_id
        ),
        instance_path: format!(
            "/vulnerabilities/{}/metrics/{}/content/ssvc_v2/target_ids/{}",
            i_v, i_m, i_t
        ),
    }
}

fn create_target_id_mismatch_error(target_id: &str, i_v: usize, i_m: usize, i_t: usize) -> ValidationError {
    ValidationError {
        message: format!(
            "The SSVC target ID '{}' does not match the document ID, the CVE ID or any ID in the IDs array of the vulnerability",
            target_id
        ),
        instance_path: format!(
            "/vulnerabilities/{}/metrics/{}/content/ssvc_v2/target_ids/{}",
            i_v, i_m, i_t
        ),
    }
}

fn create_invalid_ssvc_error(error: impl std::fmt::Display, i_v: usize, i_m: usize) -> ValidationError {
    ValidationError {
        message: format!("Invalid SSVC object: {}", error),
        instance_path: format!("/vulnerabilities/{}/metrics/{}/content/ssvc_v2", i_v, i_m),
    }
}

pub fn test_6_1_47_inconsistent_ssvc_id(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
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
                                            return Err(vec![create_document_id_multiple_vulnerabilities_error(
                                                document_id,
                                                i_v,
                                                i_m,
                                                i_t,
                                            )]);
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
                                    return Err(vec![create_target_id_mismatch_error(target_id, i_v, i_m, i_t)]);
                                }
                            }
                        },
                        Err(err) => {
                            return Err(vec![create_invalid_ssvc_error(err, i_v, i_m)]);
                        },
                    }
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::run_csaf21_tests;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_47() {
        run_csaf21_tests(
            "47",
            test_6_1_47_inconsistent_ssvc_id,
            HashMap::from([
                ("01", vec![create_target_id_mismatch_error("CVE-1900-0002", 0, 0, 0)]),
                ("02", vec![create_target_id_mismatch_error("CVE-1900-0001", 0, 0, 0)]),
                ("03", vec![create_target_id_mismatch_error("2723", 0, 0, 0)]),
                ("04", vec![create_target_id_mismatch_error("Bug#2723", 0, 0, 0)]),
                (
                    "05",
                    vec![create_target_id_mismatch_error(
                        "OASIS_CSAF_TC-CSAF_2.1-2024-6-1-47-15",
                        0,
                        0,
                        0,
                    )],
                ),
                (
                    "06",
                    vec![create_document_id_multiple_vulnerabilities_error(
                        "OASIS_CSAF_TC-CSAF_2.1-2024-6-1-47-06",
                        1,
                        0,
                        0,
                    )],
                ),
            ]),
        );
    }
}
