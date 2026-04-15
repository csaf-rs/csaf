use crate::csaf_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait};
use crate::validation::ValidationError;

fn create_cvss_v3_0_used_error(content_path: &String) -> ValidationError {
    ValidationError {
        message: "CVSS v3.0 is used (version is '3.0').".to_string(),
        instance_path: format!("{content_path}/cvss_v3/version"),
    }
}

fn create_cvss_v3_0_vector_string_error(content_path: &String) -> ValidationError {
    ValidationError {
        message: "CVSS v3.0 is used (vectorString prefix is 'CVSS:3.0/').".to_string(),
        instance_path: format!("{content_path}/cvss_v3/vectorString"),
    }
}

/// 6.3.2 Use of CVSS v3.0
///
/// For each item in the list of metrics which contains the cvss_v3 object under content
/// it MUST be tested that CVSS v3.0 is not used.
///
/// Using the cvss deserialization here is to resource intensive for this simple test.
/// We only need to check if the two relevant fields `cvss_v3/version` and `cvss_v3/vectorString`
/// indicate CVSS v3.0, which can be done with simple string comparisons.
///
/// Returns up to two errors, if `version` is `3.0` and / or `vectorString` starts with `CVSS:3.0/`.
pub fn test_6_3_2_use_of_cvss_v3_0(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    for (v_i, vuln) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(metrics) = vuln.get_metrics() {
            for (m_i, metric) in metrics.iter().enumerate() {
                let content = metric.get_content();
                if let Some(cvss_v3_map) = content.get_cvss_v3() {
                    let content_path = content.get_content_json_path(v_i, m_i);

                    // if version is "3.0", add an error
                    if cvss_v3_map
                        .get("version")
                        .and_then(|v| v.as_str())
                        .is_some_and(|v| v == "3.0")
                    {
                        errors
                            .get_or_insert_default()
                            .push(create_cvss_v3_0_used_error(&content_path));
                    }

                    // if vectorString starts with "CVSS:3.0/", add an error
                    if cvss_v3_map
                        .get("vectorString")
                        .and_then(|v| v.as_str())
                        .is_some_and(|v| v.starts_with("CVSS:3.0/"))
                    {
                        errors
                            .get_or_insert_default()
                            .push(create_cvss_v3_0_vector_string_error(&content_path));
                    }
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(ValidatorForTest6_3_2, test_6_3_2_use_of_cvss_v3_0);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_2() {
        let case_01_v3_0_used_csaf_20 = Err(vec![
            create_cvss_v3_0_used_error(&"/vulnerabilities/0/scores/0".to_string()),
            create_cvss_v3_0_vector_string_error(&"/vulnerabilities/0/scores/0".to_string()),
        ]);
        let case_01_v3_0_used_csaf_21 = Err(vec![
            create_cvss_v3_0_used_error(&"/vulnerabilities/0/metrics/0/content".to_string()),
            create_cvss_v3_0_vector_string_error(&"/vulnerabilities/0/metrics/0/content".to_string()),
        ]);

        let case_02_mixed_some_with_v3_0_csaf_20 = Err(vec![
            create_cvss_v3_0_used_error(&"/vulnerabilities/0/scores/0".to_string()),
            create_cvss_v3_0_vector_string_error(&"/vulnerabilities/0/scores/0".to_string()),
            create_cvss_v3_0_used_error(&"/vulnerabilities/2/scores/0".to_string()),
            create_cvss_v3_0_vector_string_error(&"/vulnerabilities/2/scores/0".to_string()),
        ]);
        let case_02_mixed_some_with_v3_0_csaf_21 = Err(vec![
            create_cvss_v3_0_used_error(&"/vulnerabilities/0/metrics/0/content".to_string()),
            create_cvss_v3_0_vector_string_error(&"/vulnerabilities/0/metrics/0/content".to_string()),
            create_cvss_v3_0_used_error(&"/vulnerabilities/2/metrics/0/content".to_string()),
            create_cvss_v3_0_vector_string_error(&"/vulnerabilities/2/metrics/0/content".to_string()),
        ]);

        // Case 11: 1 vuln with v3.1
        // Case 12: 3 vulns with v3.1

        TESTS_2_0.test_6_3_2.expect(
            case_01_v3_0_used_csaf_20,
            case_02_mixed_some_with_v3_0_csaf_20,
            Ok(()),
            Ok(()),
        );
        TESTS_2_1.test_6_3_2.expect(
            case_01_v3_0_used_csaf_21,
            case_02_mixed_some_with_v3_0_csaf_21,
            Ok(()),
            Ok(()),
        );
    }
}
