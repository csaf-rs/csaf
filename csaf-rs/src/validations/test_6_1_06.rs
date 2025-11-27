use crate::csaf_traits::{CsafTrait, ProductStatusGroup, ProductStatusTrait, VulnerabilityTrait};
use crate::validation::ValidationError;
use std::collections::HashMap;

pub fn test_6_1_06_contradicting_product_status(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;
    for (vulnerability_index, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(product_status) = vulnerability.get_product_status() {
            let product_status_map = product_status.get_all_by_product_status();

            // Invert the map: product_id -> list of ProductStatusGroups
            let mut product_to_groups: HashMap<String, Vec<ProductStatusGroup>> = HashMap::new();

            for (group, product_ids) in product_status_map {
                if group == ProductStatusGroup::Recommended {
                    // recommended products must not be checked for contradictions
                    continue;
                }
                for product_id in product_ids {
                    product_to_groups
                        .entry(product_id.to_owned())
                        .or_insert_with(Vec::new)
                        .push(group.clone());
                }
            }

            // Check for products with multiple status groups (contradictions)
            for (product_id, groups) in product_to_groups {
                if groups.len() > 1 {
                    let mut affected_groups = groups;
                    affected_groups.sort();
                    errors.get_or_insert_with(Vec::new).push(ValidationError {
                        message: create_error_message(&product_id, &affected_groups),
                        instance_path: format!("/vulnerabilities/{}/product_status", vulnerability_index),
                    });
                }
            }
        }
    }
    errors.map_or(Ok(()), Err)
}

fn create_error_message(product_id: &str, groups: &[ProductStatusGroup]) -> String {
    let group_names: Vec<String> = groups.iter().map(|g| format!("'{}'", g.to_string())).collect();
    format!(
        "Product {} is member of contradicting product status groups: {}",
        product_id,
        group_names.join(", ")
    )
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::validation::ValidationError;
    use crate::validations::test_6_1_06::{
        ProductStatusGroup, create_error_message, test_6_1_06_contradicting_product_status,
    };
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_06() {
        let expected_errors = HashMap::from([
            (
                "01",
                vec![ValidationError {
                    message: create_error_message(
                        "CSAFPID-9080700",
                        &[ProductStatusGroup::Affected, ProductStatusGroup::NotAffected],
                    ),
                    instance_path: "/vulnerabilities/0/product_status".to_string(),
                }],
            ),
            (
                "02",
                vec![ValidationError {
                    message: create_error_message(
                        "CSAFPID-9080700",
                        &[ProductStatusGroup::Affected, ProductStatusGroup::NotAffected],
                    ),
                    instance_path: "/vulnerabilities/0/product_status".to_string(),
                }],
            ),
            (
                "03",
                vec![ValidationError {
                    message: create_error_message(
                        "CSAFPID-9080700",
                        &[ProductStatusGroup::Affected, ProductStatusGroup::NotAffected],
                    ),
                    instance_path: "/vulnerabilities/0/product_status".to_string(),
                }],
            ),
            (
                "04",
                vec![
                    ValidationError {
                        message: create_error_message(
                            "CSAFPID-9080700",
                            &[ProductStatusGroup::Affected, ProductStatusGroup::UnderInvestigation],
                        ),
                        instance_path: "/vulnerabilities/0/product_status".to_string(),
                    },
                    ValidationError {
                        message: create_error_message(
                            "CSAFPID-9080701",
                            &[ProductStatusGroup::NotAffected, ProductStatusGroup::Fixed],
                        ),
                        instance_path: "/vulnerabilities/0/product_status".to_string(),
                    },
                ],
            ),
            (
                "05",
                vec![
                    ValidationError {
                        message: create_error_message(
                            "CSAFPID-9080700",
                            &[ProductStatusGroup::Affected, ProductStatusGroup::UnderInvestigation],
                        ),
                        instance_path: "/vulnerabilities/0/product_status".to_string(),
                    },
                    ValidationError {
                        message: create_error_message(
                            "CSAFPID-9080701",
                            &[ProductStatusGroup::NotAffected, ProductStatusGroup::UnderInvestigation],
                        ),
                        instance_path: "/vulnerabilities/0/product_status".to_string(),
                    },
                    ValidationError {
                        message: create_error_message(
                            "CSAFPID-9080702",
                            &[ProductStatusGroup::Affected, ProductStatusGroup::Fixed],
                        ),
                        instance_path: "/vulnerabilities/0/product_status".to_string(),
                    },
                ],
            ),
            (
                "06",
                vec![ValidationError {
                    message: create_error_message(
                        "CSAFPID-9080700",
                        &[ProductStatusGroup::Affected, ProductStatusGroup::Unknown],
                    ),
                    instance_path: "/vulnerabilities/0/product_status".to_string(),
                }],
            ),
        ]);
        run_csaf20_tests("06", test_6_1_06_contradicting_product_status, expected_errors.clone());
        run_csaf21_tests("06", test_6_1_06_contradicting_product_status, expected_errors);
    }
}
