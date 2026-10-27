use crate::csaf_traits::{ActionTrait, Csaf21InvolvementTrait, CsafTrait, DocumentTrait, VulnerabilityTrait};
use crate::validation::{TestFinding, TestFindingData};

fn create_uncovered_vulnerability_info(vulnerability_index: usize) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: "Vulnerability is not covered by any involvement action".to_owned(),
        instance_path: format!("/vulnerabilities/{vulnerability_index}"),
    })
}

/// 6.3.26.1 Vulnerability Not Covered by Involvement
///
/// Each vulnerability must be covered by at least one involvement action.
/// An action without `dl_vuln_ids` implicitly covers every vulnerability.
pub fn test_6_3_26_1_vulnerability_not_covered_by_involvement(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // check if involvement is present, if not, skip the test
    let Some(involvement) = doc.get_document().get_involvement() else {
        return Ok(());
    };

    // if there is any action without `dl_vuln_ids`, then all vulnerabilities are implicitly covered
    let implicitly_covered = involvement
        .get_actions()
        .any(|action| action.get_document_local_vuln_ids().is_none());
    if implicitly_covered {
        return Ok(());
    }

    let mut findings: Option<Vec<TestFinding>> = None;
    // check each vulnerability
    for (vulnerability_index, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        // if it does not define a document_local_vuln_id, and if it's not implicitly covered, then it is uncovered
        // implicit coverage is already checked above, so we generate a finding here
        let Some(vuln_dl_vuln_id) = vulnerability.get_document_local_vuln_id() else {
            findings
                .get_or_insert_default()
                .push(create_uncovered_vulnerability_info(vulnerability_index));
            continue;
        };

        // check if any action covers this vulnerability, if not, generate a finding
        if !involvement.get_actions().any(|action| {
            action
                .get_document_local_vuln_ids()
                .into_iter()
                .flatten()
                .any(|action_dl_vuln_id| action_dl_vuln_id == vuln_dl_vuln_id)
        }) {
            findings
                .get_or_insert_default()
                .push(create_uncovered_vulnerability_info(vulnerability_index));
        }
    }

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_3_26_1,
    test_6_3_26_1_vulnerability_not_covered_by_involvement
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::{ExpectedResults_6_3_26_1, TESTS_2_1};

    #[test]
    fn test_test_6_3_26_1() {
        // TODO: Evaluate test coverage
        // TODO: Document test cases properly

        TESTS_2_1.test_6_3_26_1.expect(ExpectedResults_6_3_26_1 {
            case_01: Err(vec![create_uncovered_vulnerability_info(1)]),
            case_02: Err(vec![create_uncovered_vulnerability_info(5)]),
            case_11: Ok(()),
            case_12: Ok(()),
        });
    }
}
