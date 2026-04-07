use crate::csaf_traits::{CsafTrait, RemediationTrait, VulnerabilityTrait};
use crate::schema::csaf2_1::schema::CategoryOfTheRemediation;
use crate::validation::ValidationError;
use std::collections::BTreeMap;

/// Totally exclusive categories that cannot be combined with any other category.
static EX_STATES: &[CategoryOfTheRemediation] = &[
    CategoryOfTheRemediation::NoneAvailable,
    CategoryOfTheRemediation::OptionalPatch,
];

/// Mutually exclusive states that cannot apply at the same time.
static MUT_EX_STATES: &[CategoryOfTheRemediation] = &[
    CategoryOfTheRemediation::NoFixPlanned,
    CategoryOfTheRemediation::FixPlanned,
    CategoryOfTheRemediation::VendorFix,
];

fn create_contradicting_remediations_error(
    product_id: &str,
    existing_categories: &[CategoryOfTheRemediation],
    new_category: CategoryOfTheRemediation,
    vulnerability_index: usize,
    remediation_index: usize,
) -> ValidationError {
    ValidationError {
        message: format!(
            "Product {} has contradicting remediations: {} and {}",
            product_id,
            existing_categories
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            new_category
        ),
        instance_path: format!("/vulnerabilities/{vulnerability_index}/remediations/{remediation_index}"),
    }
}

pub fn test_6_1_35_contradicting_remediations(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    // TODO #409 early return + no data
    let mut errors: Option<Vec<ValidationError>> = None;
    for (v_i, v) in doc.get_vulnerabilities().iter().enumerate() {
        // Data struct to store observed remediation categories per product ID
        let mut product_categories: BTreeMap<String, Vec<CategoryOfTheRemediation>> = BTreeMap::new();
        for (r_i, r) in v.get_remediations().iter().enumerate() {
            // Only handle Remediations having product IDs associated
            if let Some(product_ids) = r.get_all_product_ids(doc) {
                // Category of current remediation
                let cat = r.get_category();
                // Iterate over product IDs
                for p in product_ids {
                    // Check if product ID has categories associated
                    if let Some(exist_cat_set) = product_categories.get_mut(&p) {
                        // Checks if current category is exclusive and a non-equal previous category was found.
                        if EX_STATES.contains(&cat) && exist_cat_set.first().is_some_and(|e_cat| e_cat != &cat)
                            // Checks if the (only) previous category is exclusive.
                            || exist_cat_set.first().is_some_and(|e_cat| EX_STATES.contains(e_cat))
                            // Checks if the current category conflicts with any other in the group of mutually exclusive ones.
                            || MUT_EX_STATES.contains(&cat) && exist_cat_set.iter().any(|e_cat| MUT_EX_STATES.contains(e_cat))
                        {
                            errors.get_or_insert_default().push(create_contradicting_remediations_error(
                                &p,
                                exist_cat_set,
                                cat,
                                v_i,
                                r_i,
                            ));
                        } else {
                            exist_cat_set.push(cat);
                        }
                    } else {
                        product_categories.insert(p, Vec::from([cat]));
                    }
                }
            }
        }
    }
    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_35, test_6_1_35_contradicting_remediations);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_35() {

        // TODO: Improve test coverage (issue #526)

        let case_01_mutually_exclusive_via_product = Err(vec![create_contradicting_remediations_error(
            "CSAFPID-9080700",
            &[CategoryOfTheRemediation::NoFixPlanned],
            CategoryOfTheRemediation::VendorFix,
            0,
            1,
        )]);

        let case_02_exclusive_none_available_via_group = Err(vec![create_contradicting_remediations_error(
            "CSAFPID-9080700",
            &[CategoryOfTheRemediation::NoneAvailable],
            CategoryOfTheRemediation::Mitigation,
            0,
            1,
        )]);

        let case_03_exclusive_optional_path_via_group = Err(vec![create_contradicting_remediations_error(
            "CSAFPID-9080702",
            &[
                CategoryOfTheRemediation::Workaround,
                CategoryOfTheRemediation::FixPlanned,
            ],
            CategoryOfTheRemediation::OptionalPatch,
            0,
            2,
        )]);

        let case_04_exclusive_optional_patch_via_groups_multiple_products = Err(vec![
            create_contradicting_remediations_error(
                "CSAFPID-9080701",
                &[
                    CategoryOfTheRemediation::Mitigation,
                    CategoryOfTheRemediation::FixPlanned,
                ],
                CategoryOfTheRemediation::OptionalPatch,
                0,
                2,
            ),
            create_contradicting_remediations_error(
                "CSAFPID-9080702",
                &[
                    CategoryOfTheRemediation::Mitigation,
                    CategoryOfTheRemediation::FixPlanned,
                ],
                CategoryOfTheRemediation::OptionalPatch,
                0,
                2,
            ),
        ]);

        // Case 01: One product, one remediation
        // Case 02: One product, one group, exclusive optional patch only on the product
        // Case 03: One product, one group, exlusive optional patch only on the group
        // Case 04: Two groups, exclusive optional patch applies only to one group

        TESTS_2_1.test_6_1_35.expect(
            case_01_mutually_exclusive_via_product,
            case_02_exclusive_none_available_via_group,
            case_03_exclusive_optional_path_via_group,
            case_04_exclusive_optional_patch_via_groups_multiple_products,
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
