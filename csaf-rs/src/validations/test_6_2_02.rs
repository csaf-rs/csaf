use crate::csaf_traits::{
    CsafTrait, ProductStatusAndPath, ProductStatusGroup, ProductStatusGroupMap, RemediationTrait, VulnerabilityTrait,
    WithOptionalProductIds,
};
use crate::schema::csaf2_1::schema::CategoryOfTheRemediation;
use crate::validation::{TestFinding, TestFindingData};
use std::collections::HashSet;

fn create_missing_remediation_error(
    vulnerability_index: usize,
    product_id: &str,
    status_path: ProductStatusAndPath,
) -> TestFinding {
    let status_group_name = status_path.status.to_string();
    let none_available_name = CategoryOfTheRemediation::NoneAvailable.to_string();
    let no_fix_planned_name = CategoryOfTheRemediation::NoFixPlanned.to_string();
    TestFinding::Warning(TestFindingData {
        message: format!(
            "Missing at least a remediation of category '{none_available_name}' or '{no_fix_planned_name}' for product ID '{product_id}' with product status '{status_group_name}'",
        ),
        instance_path: status_path.json_path(vulnerability_index),
    })
}

/// 6.2.2 Missing Remediation
///
/// For each product in status groups "affected" or "under investigation", a remediation of category
/// `none_available` or `no_fix_planned` must exist.
pub fn test_6_2_02_missing_remediations(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let mut errors: Option<Vec<TestFinding>> = None;

    // for each vulnerability
    for (v_i, vuln) in doc.get_vulnerabilities().iter().enumerate() {
        // if there are product statuses
        if let Some(product_status) = vuln.get_product_status() {
            let product_to_groups = ProductStatusGroupMap::from(product_status);

            let relevant_groups = product_to_groups
                .into_iter()
                .filter(|g| g.0 == ProductStatusGroup::Affected || g.0 == ProductStatusGroup::UnderInvestigation);

            // collect all product IDs referenced in remediations of category none_available or no_fix_planned
            let mut remediation_product_ids = HashSet::<String>::new();
            for remediation in vuln.get_remediations() {
                if (remediation.get_category() == CategoryOfTheRemediation::NoneAvailable
                    || remediation.get_category() == CategoryOfTheRemediation::NoFixPlanned)
                    && let Some(product_ids) = remediation.get_product_ids()
                {
                    for product_id in product_ids {
                        remediation_product_ids.insert(product_id.to_owned());
                    }
                }
            }

            for (_, product_ids_map) in relevant_groups {
                for (product_id, status_with_path) in product_ids_map {
                    if !remediation_product_ids.contains(&product_id) {
                        for status_path in status_with_path {
                            errors.get_or_insert_default().push(create_missing_remediation_error(
                                v_i,
                                &product_id,
                                status_path,
                            ));
                        }
                    }
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(ValidatorForTest6_2_2, test_6_2_02_missing_remediations);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf_traits::ProductStatus;
    use crate::csaf2_0::testcases::ExpectedResults_6_2_2 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_2 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_02() {
        let case_01 = Err(vec![create_missing_remediation_error(
            0,
            "CSAFPID-9080700",
            ProductStatusAndPath {
                status: ProductStatus::LastAffected,
                index: 0,
            },
        )]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0.test_6_2_2.expect(ExpectedResults_2_0 {
            case_01: case_01.clone(),
        });
        TESTS_2_1.test_6_2_2.expect(ExpectedResults_2_1 { case_01 });
    }
}
