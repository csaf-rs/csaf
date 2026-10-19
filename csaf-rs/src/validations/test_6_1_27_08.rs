use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{
    CsafTrait, CsafVersion, DocumentTrait, ProductStatusTrait, VulnerabilityTrait, WithOptionalGroupIds,
    WithOptionalProductIds, resolve_product_groups,
};
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::document_category_test_config::DocumentCategoryTestConfig;
use std::collections::HashSet;

fn create_no_cve_no_ids_error(document_category: &CsafDocumentCategory, vuln_path_index: &usize) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "Document with category '{document_category}' must provide at least either cve or ids in each vulnerability"
        ),
        instance_path: format!("/vulnerabilities/{vuln_path_index}"),
    })
}

fn create_product_status_without_cve_or_refed_ids_error(product_id: &str, vuln_path_index: usize) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Product '{product_id}' in product_status is not assigned to a vulnerability ID"),
        instance_path: format!("/vulnerabilities/{vuln_path_index}/product_status"),
    })
}

/// 6.1.27.8 Vulnerability ID
///
/// This test only applies to documents with `/document/category` with value `csaf_vex` on CSAF 2.0 and
/// additionally to `csaf_security_advisory` and `csaf_vulnerability_report` on CSAF 2.1.
///
/// In documents with this category each `/vulnerabilities[]` item must have at the `cve` or the `ids`
/// element.
///
/// For CSAF 2.1, this test additionally checks:
/// If no `cve`  is present and all items in `ids` contain `group_ids` or `product_ids`, it SHALL be tested
/// that each product mentioned in `product_status[*][*]` is assigned at least one item in `ids`.
/// This is independent from whether the product is referenced directly or indirectly through a product group.
pub fn test_6_1_27_08_vulnerability_id(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let document = doc.get_document();
    let doc_category = document.get_category();

    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(document.get_csaf_version(), &doc_category) {
        return Ok(()); // ToDo generate skipped https://github.com/csaf-rs/csaf/issues/409
    }

    let mut errors: Option<Vec<TestFinding>> = None;
    // return error if there are vulnerabilities without either cve or ids
    for (v_i, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if vulnerability.get_cve().is_none() && vulnerability.get_ids().is_none() {
            errors
                .get_or_insert_default()
                .push(create_no_cve_no_ids_error(&doc_category, &v_i));
        }
    }

    if document.get_csaf_version() == CsafVersion::X21 {
        for (v_i, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
            // Check preconditions:
            // if cve is present, we can skip
            if vulnerability.get_cve().is_some() {
                continue;
            }
            // if ids is not present, we can skip
            let Some(ids) = vulnerability.get_ids() else {
                continue;
            };
            // if every id has not either group_ids or product_ids, we can skip
            if !ids
                .iter()
                .all(|id| id.get_group_ids().is_some() || id.get_product_ids().is_some())
            {
                continue;
            }

            // collect all product_ids and resolve all group_ids from ids
            let mut assigned_products = HashSet::new();
            let mut group_ids = Vec::new();
            for id in ids {
                if let Some(product_ids) = id.get_product_ids() {
                    assigned_products.extend(product_ids.map(str::to_owned));
                }
                if let Some(ids) = id.get_group_ids() {
                    group_ids.extend(ids.map(str::to_owned));
                }
            }
            if let Some(product_ids) = resolve_product_groups(doc, group_ids) {
                assigned_products.extend(product_ids);
            }

            // check that every product in product_status is assigned at least one id
            if let Some(product_status) = vulnerability.get_product_status() {
                for product_id in product_status
                    .get_products_by_status()
                    .into_iter()
                    .flat_map(|(_, product_ids)| product_ids)
                    .filter(|product_id| !assigned_products.contains(product_id))
                {
                    errors
                        .get_or_insert_default()
                        .push(create_product_status_without_cve_or_refed_ids_error(&product_id, v_i));
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig = DocumentCategoryTestConfig::new()
    .shared(&[CsafDocumentCategory::CsafVex])
    .csaf21(&[
        CsafDocumentCategory::CsafSecurityAdvisory,
        CsafDocumentCategory::CsafVulnerabilityReport,
    ]);

crate::test_validation::impl_validator!(ValidatorForTest6_1_27_8, test_6_1_27_08_vulnerability_id);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_1_27_8 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_27_8 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_27_08() {
        let case_vex_without_cve_or_id = Err(vec![create_no_cve_no_ids_error(&CsafDocumentCategory::CsafVex, &0)]);
        let case_vuln_report_without_cve_or_id = Err(vec![create_no_cve_no_ids_error(
            &CsafDocumentCategory::CsafVulnerabilityReport,
            &0,
        )]);
        let case_security_advisory_without_cve_or_id = Err(vec![create_no_cve_no_ids_error(
            &CsafDocumentCategory::CsafSecurityAdvisory,
            &0,
        )]);
        let case_products_without_vulnerability_ids = Err(vec![
            create_product_status_without_cve_or_refed_ids_error("CSAFPID-9080700", 0),
            create_product_status_without_cve_or_refed_ids_error("CSAFPID-9080701", 0),
            create_product_status_without_cve_or_refed_ids_error("CSAFPID-9080703", 1),
            create_product_status_without_cve_or_refed_ids_error("CSAFPID-9080703", 2),
        ]);

        // Note: For simplicity, this file does not contain product tree or notes elements for the two vulns, which violates 6.1.27.4 and 6.1.27.5
        // Case S11: two vulns, one with cve, one with ids

        TESTS_2_0.test_6_1_27_8.expect(ExpectedResults_2_0 {
            case_01: case_vex_without_cve_or_id.clone(),
            case_s11: Ok(()),
        });
        TESTS_2_1.test_6_1_27_8.expect(ExpectedResults_2_1 {
            case_01: case_vex_without_cve_or_id,
            case_02: case_products_without_vulnerability_ids,
            case_03: case_vuln_report_without_cve_or_id,
            case_04: case_security_advisory_without_cve_or_id,
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
            case_14: Ok(()),
            case_s11: Ok(()),
        });
    }
}
