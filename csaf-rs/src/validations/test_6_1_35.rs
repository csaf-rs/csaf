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

/// Creates a ValidationError for contradicting remediations
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
        instance_path: format!(
            "/vulnerabilities/{}/remediations/{}",
            vulnerability_index, remediation_index
        ),
    }
}

pub fn test_6_1_35_contradicting_remediations(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    for (v_i, v) in doc.get_vulnerabilities().iter().enumerate() {
        // Data struct to store observed remediation categories per product IT
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
                            return Err(vec![create_contradicting_remediations_error(
                                &p,
                                exist_cat_set,
                                cat,
                                v_i,
                                r_i,
                            )]);
                        }
                        exist_cat_set.push(cat);
                    } else {
                        product_categories.insert(p, Vec::from([cat]));
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
    fn test_test_6_1_35() {
        run_csaf21_tests(
            "35",
            test_6_1_35_contradicting_remediations,
            HashMap::from([
                (
                    "01",
                    vec![create_contradicting_remediations_error(
                        "CSAFPID-9080700",
                        &[CategoryOfTheRemediation::NoFixPlanned],
                        CategoryOfTheRemediation::VendorFix,
                        0,
                        1,
                    )],
                ),
                (
                    "02",
                    vec![create_contradicting_remediations_error(
                        "CSAFPID-9080700",
                        &[CategoryOfTheRemediation::NoneAvailable],
                        CategoryOfTheRemediation::Mitigation,
                        0,
                        1,
                    )],
                ),
                (
                    "03",
                    vec![create_contradicting_remediations_error(
                        "CSAFPID-9080702",
                        &[
                            CategoryOfTheRemediation::Workaround,
                            CategoryOfTheRemediation::FixPlanned,
                        ],
                        CategoryOfTheRemediation::OptionalPatch,
                        0,
                        2,
                    )],
                ),
                (
                    "04",
                    vec![create_contradicting_remediations_error(
                        "CSAFPID-9080701",
                        &[
                            CategoryOfTheRemediation::Mitigation,
                            CategoryOfTheRemediation::FixPlanned,
                        ],
                        CategoryOfTheRemediation::OptionalPatch,
                        0,
                        2,
                    )],
                ),
            ]),
        );
    }
}
