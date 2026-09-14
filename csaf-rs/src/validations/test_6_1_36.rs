use crate::csaf_traits::{CsafTrait, ProductStatusGroup, ProductStatusGroupMap, RemediationTrait, VulnerabilityTrait};
use crate::schema::csaf2_1::schema::CategoryOfTheRemediation;
use crate::validation::{TestFinding, TestFindingData};

/// Remediation categories that conflict with the product status "not affected".
const NOT_AFFECTED_PROHIBITED: &[CategoryOfTheRemediation] = &[
    CategoryOfTheRemediation::Workaround,
    CategoryOfTheRemediation::Mitigation,
    CategoryOfTheRemediation::VendorFix,
    CategoryOfTheRemediation::NoneAvailable,
];

/// Remediation categories that conflict with "fixed" product statuses.
const FIXED_PROHIBITED: &[CategoryOfTheRemediation] = &[
    CategoryOfTheRemediation::NoneAvailable,
    CategoryOfTheRemediation::FixPlanned,
    CategoryOfTheRemediation::NoFixPlanned,
    CategoryOfTheRemediation::VendorFix,
    CategoryOfTheRemediation::Mitigation,
    CategoryOfTheRemediation::Workaround,
];

fn create_prohibited_combination_error(
    product_id: &str,
    status_group: &ProductStatusGroup,
    category: &CategoryOfTheRemediation,
    vulnerability_index: usize,
    remediation_index: usize,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "Product {product_id} is listed as {status_group} but has prohibited remediation category: {category}"
        ),
        instance_path: format!("/vulnerabilities/{vulnerability_index}/remediations/{remediation_index}"),
    })
}

pub fn test_6_1_36_status_group_contradicting_remediation_categories(
    doc: &impl CsafTrait,
) -> Result<(), Vec<TestFinding>> {
    let mut errors = Vec::new();

    for (vulnerability_index, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(product_status) = vulnerability.get_product_status() {
            // Group the product IDs by their corresponding status group
            let status_map = ProductStatusGroupMap::from(product_status);

            // Iterate over remediations
            for (remediation_index, remediation) in vulnerability.get_remediations().iter().enumerate() {
                // Only handle remediations associated with products, directly or via product groups
                if let Some(remediation_product_ids) = remediation.get_all_product_ids(doc) {
                    // Category of current remediation
                    let category = remediation.get_category();

                    // Iterate over product IDs
                    for product_id in remediation_product_ids {
                        if status_map.contains(&ProductStatusGroup::Affected, &product_id)
                            && category == CategoryOfTheRemediation::OptionalPatch
                        {
                            errors.push(create_prohibited_combination_error(
                                &product_id,
                                &ProductStatusGroup::Affected,
                                &category,
                                vulnerability_index,
                                remediation_index,
                            ));
                        }

                        if status_map.contains(&ProductStatusGroup::NotAffected, &product_id)
                            && NOT_AFFECTED_PROHIBITED.contains(&category)
                        {
                            errors.push(create_prohibited_combination_error(
                                &product_id,
                                &ProductStatusGroup::NotAffected,
                                &category,
                                vulnerability_index,
                                remediation_index,
                            ));
                        }

                        if status_map.contains(&ProductStatusGroup::Fixed, &product_id)
                            && FIXED_PROHIBITED.contains(&category)
                        {
                            errors.push(create_prohibited_combination_error(
                                &product_id,
                                &ProductStatusGroup::Fixed,
                                &category,
                                vulnerability_index,
                                remediation_index,
                            ));
                        }
                    }
                }
            }
        }
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_36,
    test_6_1_36_status_group_contradicting_remediation_categories
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_36 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_36() {
        let case_01 = Err(vec![create_prohibited_combination_error(
            "CSAFPID-9080700",
            &ProductStatusGroup::NotAffected,
            &CategoryOfTheRemediation::VendorFix,
            0,
            0,
        )]);

        let case_02 = Err(vec![
            create_prohibited_combination_error(
                "CSAFPID-9080703",
                &ProductStatusGroup::Fixed,
                &CategoryOfTheRemediation::NoneAvailable,
                0,
                0,
            ),
            create_prohibited_combination_error(
                "CSAFPID-9080700",
                &ProductStatusGroup::Fixed,
                &CategoryOfTheRemediation::Mitigation,
                0,
                1,
            ),
            create_prohibited_combination_error(
                "CSAFPID-9080701",
                &ProductStatusGroup::Fixed,
                &CategoryOfTheRemediation::Mitigation,
                0,
                1,
            ),
            create_prohibited_combination_error(
                "CSAFPID-9080702",
                &ProductStatusGroup::Fixed,
                &CategoryOfTheRemediation::Mitigation,
                0,
                1,
            ),
            create_prohibited_combination_error(
                "CSAFPID-9080701",
                &ProductStatusGroup::Fixed,
                &CategoryOfTheRemediation::VendorFix,
                0,
                2,
            ),
            create_prohibited_combination_error(
                "CSAFPID-9080702",
                &ProductStatusGroup::Fixed,
                &CategoryOfTheRemediation::VendorFix,
                0,
                2,
            ),
        ]);

        let case_03 = Err(vec![create_prohibited_combination_error(
            "CSAFPID-9080700",
            &ProductStatusGroup::Affected,
            &CategoryOfTheRemediation::OptionalPatch,
            0,
            0,
        )]);

        let case_04 = Err(vec![create_prohibited_combination_error(
            "CSAFPID-9080700",
            &ProductStatusGroup::Fixed,
            &CategoryOfTheRemediation::NoFixPlanned,
            0,
            0,
        )]);

        // Only CSAF 2.1 has this test with 8 test cases (4 error cases, 4 success cases)
        TESTS_2_1.test_6_1_36.expect(ExpectedResults {
            case_01,
            case_02,
            case_03,
            case_04,
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
            case_14: Ok(()),
        });
    }
}
