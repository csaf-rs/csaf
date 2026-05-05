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
            "The SSVC target ID equals the document ID '{document_id}' and the document contains multiple vulnerabilities"
        ),
        instance_path: format!("/vulnerabilities/{i_v}/metrics/{i_m}/content/ssvc_v2/target_ids/{i_t}"),
    }
}

fn create_target_id_mismatch_error(target_id: &str, i_v: usize, i_m: usize, i_t: usize) -> ValidationError {
    ValidationError {
        message: format!(
            "The SSVC target ID '{target_id}' does not match the document ID, the CVE ID or any ID in the IDs array of the vulnerability"
        ),
        instance_path: format!("/vulnerabilities/{i_v}/metrics/{i_m}/content/ssvc_v2/target_ids/{i_t}"),
    }
}

fn create_invalid_ssvc_error(error: impl std::fmt::Display, i_v: usize, i_m: usize) -> ValidationError {
    ValidationError {
        message: format!("Invalid SSVC object: {error}"),
        instance_path: format!("/vulnerabilities/{i_v}/metrics/{i_m}/content/ssvc_v2"),
    }
}

/// 6.1.47 Inconsistent SSVC Target IDs
///
/// For each ssvc_v2 object it MUST be tested that each item in target_ids is either
/// the CVE of the vulnerability given in cve or the text of an item in the ids array of the vulnerability.
/// The test MUST fail, if the target ID equals the /document/tracking/id and the CSAF document
/// contains more than one vulnerability.
pub fn test_6_1_47_inconsistent_ssvc_id(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    let vulnerabilities = doc.get_vulnerabilities();

    // for each vulnerability, and its metrics, if they contain ssvc_v2
    for (i_v, v) in vulnerabilities.iter().enumerate() {
        if let Some(metrics) = v.get_metrics() {
            for (i_m, m) in metrics.iter().enumerate() {
                if m.get_content().has_ssvc() {
                    // try to parse ssvc_v2 content as SSVC
                    match m.get_content().get_ssvc() {
                        // parsing succeeded
                        Ok(ssvc) => {
                            // get the SSVC target_ids if they exist
                            if let Some(target_ids) = &ssvc.target_ids {
                                let document_id = doc.get_document().get_tracking().get_id();

                                // check each target ID
                                for (i_t, target_id) in target_ids.iter().enumerate() {
                                    // check if target ID equals document ID
                                    if target_id == document_id {
                                        // if there are multiple vulnerabilities, add an error
                                        if vulnerabilities.len() > 1 {
                                            errors.get_or_insert_default().push(
                                                create_document_id_multiple_vulnerabilities_error(
                                                    document_id,
                                                    i_v,
                                                    i_m,
                                                    i_t,
                                                ),
                                            );
                                        }
                                        // target ID is valid, continue to next
                                        continue;
                                    }

                                    // check if it matches CVE
                                    if let Some(cve) = v.get_cve()
                                        && target_id == cve
                                    {
                                        continue;
                                    }

                                    // check if it matches any ID in id array
                                    if let Some(ids) = v.get_ids()
                                        && ids.iter().any(|id| id.get_text() == target_id)
                                    {
                                        continue;
                                    }

                                    // none of the above criteria were met, so the target ID is invalid
                                    errors
                                        .get_or_insert_default()
                                        .push(create_target_id_mismatch_error(target_id, i_v, i_m, i_t));
                                }
                            }
                        },
                        // parsing failed
                        Err(err) => {
                            // TODO #409 this will nondeterminable later
                            errors
                                .get_or_insert_default()
                                .push(create_invalid_ssvc_error(err, i_v, i_m));
                        },
                    }
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_47, test_6_1_47_inconsistent_ssvc_id);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_47() {
        let case_01_target_id_cve_mismatch = Err(vec![create_target_id_mismatch_error("CVE-1900-0002", 0, 0, 0)]);
        let case_02_target_id_vuln_id_mismatch = Err(vec![create_target_id_mismatch_error("CVE-1900-0001", 0, 0, 0)]);
        let case_03_target_id_vuln_id_partial_mismatch = Err(vec![create_target_id_mismatch_error("2723", 0, 0, 0)]);
        let case_04_target_id_vuln_id_swapped_mismatch = Err(vec![
            create_target_id_mismatch_error("Bug#2723", 0, 0, 0),
            create_target_id_mismatch_error("Bug#3272", 1, 0, 0),
        ]);
        let case_05_target_id_document_id_mismatch = Err(vec![create_target_id_mismatch_error(
            "OASIS_CSAF_TC-CSAF_2.1-2024-6-1-47-15",
            0,
            0,
            0,
        )]);
        let case_06_target_id_document_id_match_multi_vuln =
            Err(vec![create_document_id_multiple_vulnerabilities_error(
                "OASIS_CSAF_TC-CSAF_2.1-2024-6-1-47-06",
                1,
                0,
                0,
            )]);

        // Case 01: target ID / CVE mismatch (CVE-1900-0002 vs CVE-1900-0001)
        // Case 02: target ID / vuln IDs mismatch (CVE-1900-0001 vs [Bug#2723])
        // Case 03: target ID / vuln IDs partial match, but still mismatch (2723 vs [Bug#2723])
        // Case 04: 2 vulns, with target ID and vuln IDs, but the IDs are swapped (Bug#2723 vs [Bug#3272])
        // Case 05: target ID / document ID mismatch (OASIS_CSAF_TC-CSAF_2.1-2024-6-1-47-15 vs OASIS_CSAF_TC-CSAF_2.1-2024-6-1-47-05)
        // Case 06: target ID matches vuln IDs, but also document ID, and there are multiple vulns

        // Case 11: target ID equals CVE
        // Case 12: target ID equals CVE, there is also a vuln ID
        // Case 13: target ID equals a vuln ID
        // Case 14: 2 vulns, target ID equals vuln ID, there is also a CVE in the second vuln
        // Case 15: target ID matches both document ID and vuln ID

        TESTS_2_1.test_6_1_47.expect(
            case_01_target_id_cve_mismatch,
            case_02_target_id_vuln_id_mismatch,
            case_03_target_id_vuln_id_partial_mismatch,
            case_04_target_id_vuln_id_swapped_mismatch,
            case_05_target_id_document_id_mismatch,
            case_06_target_id_document_id_match_multi_vuln,
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
