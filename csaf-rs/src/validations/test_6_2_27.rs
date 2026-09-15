use crate::csaf_traits::{CsafTrait, ProductStatusGroup, ProductStatusGroupMap, RemediationTrait, VulnerabilityTrait};
use crate::schema::csaf2_1::schema::CategoryOfTheRemediation;
use crate::validation::{TestFinding, TestFindingData};

const NOT_AFFECTED_DISCOURAGED: &[CategoryOfTheRemediation] = &[
    CategoryOfTheRemediation::FixPlanned,
    CategoryOfTheRemediation::NoFixPlanned,
];

const FIXED_DISCOURAGED: &[CategoryOfTheRemediation] = &[CategoryOfTheRemediation::OptionalPatch];

const UNDER_INVESTIGATION_DISCOURAGED: &[CategoryOfTheRemediation] = &[
    CategoryOfTheRemediation::Workaround,
    CategoryOfTheRemediation::Mitigation,
    CategoryOfTheRemediation::VendorFix,
    CategoryOfTheRemediation::FixPlanned,
];

const UNKNOWN_DISCOURAGED: &[CategoryOfTheRemediation] = &[
    CategoryOfTheRemediation::Workaround,
    CategoryOfTheRemediation::Mitigation,
    CategoryOfTheRemediation::VendorFix,
    CategoryOfTheRemediation::FixPlanned,
];

const DISCOURAGED_COMBINATIONS: &[(ProductStatusGroup, &[CategoryOfTheRemediation])] = &[
    (ProductStatusGroup::NotAffected, NOT_AFFECTED_DISCOURAGED),
    (ProductStatusGroup::Fixed, FIXED_DISCOURAGED),
    (ProductStatusGroup::UnderInvestigation, UNDER_INVESTIGATION_DISCOURAGED),
    (ProductStatusGroup::Unknown, UNKNOWN_DISCOURAGED),
];

fn create_discouraged_combination_warning(
    product_id: &str,
    status_group: &ProductStatusGroup,
    category: &CategoryOfTheRemediation,
    vulnerability_index: usize,
    remediation_index: usize,
) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: format!(
            "Product {product_id} is listed as {status_group} but has discouraged remediation category: {category}"
        ),
        instance_path: format!("/vulnerabilities/{vulnerability_index}/remediations/{remediation_index}"),
    })
}

pub fn test_6_2_27_discouraged_product_status_remediation_combination(
    doc: &impl CsafTrait,
) -> Result<(), Vec<TestFinding>> {
    let mut warnings = Vec::new();

    // Iterate over vulnerabilities
    for (vulnerability_index, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        // Group the product IDs by their corresponding status group
        let products_by_status_group =
            // Only proceed if the vulnerability has a product status
            if let Some(product_status) = vulnerability.get_product_status() {
                ProductStatusGroupMap::from(product_status)
            } else {
                continue;
            };

        // Iterate over remediations
        for (remediation_index, remediation) in vulnerability.get_remediations().iter().enumerate() {
            // Only proceed if the remediation is associated with products, directly or via product groups
            let Some(remediation_product_ids) = remediation.get_all_product_ids(doc) else {
                continue;
            };

            let category = remediation.get_category();

            for product_id in remediation_product_ids {
                for (status_group, discouraged_categories) in DISCOURAGED_COMBINATIONS {
                    // If the product belongs to the current status group
                    // and the remediation category is discouraged for that group,
                    // add a warning.
                    if products_by_status_group.contains(status_group, &product_id)
                        && discouraged_categories.contains(&category)
                    {
                        warnings.push(create_discouraged_combination_warning(
                            &product_id,
                            status_group,
                            &category,
                            vulnerability_index,
                            remediation_index,
                        ));
                    }
                }
            }
        }
    }

    if warnings.is_empty() { Ok(()) } else { Err(warnings) }
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_27,
    test_6_2_27_discouraged_product_status_remediation_combination
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_27 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_27() {
        let case_01 = Err(vec![create_discouraged_combination_warning(
            "CSAFPID-9080700",
            &ProductStatusGroup::NotAffected,
            &CategoryOfTheRemediation::FixPlanned,
            0,
            0,
        )]);

        let case_02 = Err(vec![
            create_discouraged_combination_warning(
                "CSAFPID-9080701",
                &ProductStatusGroup::UnderInvestigation,
                &CategoryOfTheRemediation::FixPlanned,
                0,
                0,
            ),
            create_discouraged_combination_warning(
                "CSAFPID-9080702",
                &ProductStatusGroup::UnderInvestigation,
                &CategoryOfTheRemediation::FixPlanned,
                0,
                0,
            ),
            create_discouraged_combination_warning(
                "CSAFPID-9080700",
                &ProductStatusGroup::UnderInvestigation,
                &CategoryOfTheRemediation::Mitigation,
                0,
                1,
            ),
            create_discouraged_combination_warning(
                "CSAFPID-9080701",
                &ProductStatusGroup::UnderInvestigation,
                &CategoryOfTheRemediation::Mitigation,
                0,
                1,
            ),
            create_discouraged_combination_warning(
                "CSAFPID-9080702",
                &ProductStatusGroup::UnderInvestigation,
                &CategoryOfTheRemediation::Mitigation,
                0,
                1,
            ),
            create_discouraged_combination_warning(
                "CSAFPID-9080703",
                &ProductStatusGroup::Fixed,
                &CategoryOfTheRemediation::OptionalPatch,
                0,
                2,
            ),
        ]);

        let case_03 = Err(vec![create_discouraged_combination_warning(
            "CSAFPID-9080700",
            &ProductStatusGroup::Unknown,
            &CategoryOfTheRemediation::FixPlanned,
            0,
            0,
        )]);

        TESTS_2_1.test_6_2_27.expect(ExpectedResults {
            case_01,
            case_02,
            case_03,
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
        });
    }
}
