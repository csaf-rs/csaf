use crate::csaf_traits::{CsafTrait, ProductStatusGroup, ProductStatusTrait, RemediationTrait, VulnerabilityTrait};
use crate::schema::csaf2_1::schema::CategoryOfTheRemediation;
use crate::validation::ValidationError;

/// Remediation categories that conflict with the product status "not affected".
const NOT_AFFECTED_CONFLICTS: &[CategoryOfTheRemediation] = &[
    CategoryOfTheRemediation::Workaround,
    CategoryOfTheRemediation::Mitigation,
    CategoryOfTheRemediation::VendorFix,
    CategoryOfTheRemediation::NoneAvailable,
];

/// Remediation categories that conflict with "fixed" product statuses.
const FIXED_CONFLICTS: &[CategoryOfTheRemediation] = &[
    CategoryOfTheRemediation::NoneAvailable,
    CategoryOfTheRemediation::FixPlanned,
    CategoryOfTheRemediation::NoFixPlanned,
    CategoryOfTheRemediation::VendorFix,
    CategoryOfTheRemediation::Mitigation,
    CategoryOfTheRemediation::Workaround,
];

pub fn test_6_1_36_status_group_contradicting_remediation_categories(
    doc: &impl CsafTrait,
) -> Result<(), Vec<ValidationError>> {
    for (v_i, v) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(product_status) = v.get_product_status() {
            let all_by_status = product_status.get_all_by_product_status();
            // Collect Product IDs that may cause conflicts
            let affected_products = all_by_status.get(&ProductStatusGroup::Affected);
            let not_affected_products = all_by_status.get(&ProductStatusGroup::NotAffected);
            let fixed_products = all_by_status.get(&ProductStatusGroup::Fixed);
            // Iterate over remediations
            for (r_i, r) in v.get_remediations().iter().enumerate() {
                // Only handle Remediations having product IDs associated
                if let Some(product_ids) = r.get_all_product_ids(doc) {
                    // Category of current remediation
                    let cat = r.get_category();
                    // Iterate over product IDs
                    for p in product_ids {
                        if let Some(affected_products) = affected_products {
                            if affected_products.contains(&p) && cat == CategoryOfTheRemediation::OptionalPatch {
                                return Err(vec![ValidationError {
                                    message: format!(
                                        "Product {} is listed as affected but has conflicting remediation category {}",
                                        p, cat
                                    ),
                                    instance_path: format!("/vulnerabilities/{}/remediations/{}", v_i, r_i),
                                }]);
                            }
                        }
                        if let Some(not_affected_products) = not_affected_products {
                            if not_affected_products.contains(&p) && NOT_AFFECTED_CONFLICTS.contains(&cat) {
                                return Err(vec![ValidationError {
                                    message: format!(
                                        "Product {} is listed as not affected but has conflicting remediation category {}",
                                        p, cat
                                    ),
                                    instance_path: format!("/vulnerabilities/{}/remediations/{}", v_i, r_i),
                                }]);
                            }
                        }
                        if let Some(fixed_products) = fixed_products {
                            if fixed_products.contains(&p) && FIXED_CONFLICTS.contains(&cat) {
                                return Err(vec![ValidationError {
                                    message: format!(
                                        "Product {} is listed as fixed but has conflicting remediation category {}",
                                        p, cat
                                    ),
                                    instance_path: format!("/vulnerabilities/{}/remediations/{}", v_i, r_i),
                                }]);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_helper::run_csaf21_tests;
    use crate::validation::ValidationError;
    use crate::validations::test_6_1_36::test_6_1_36_status_group_contradicting_remediation_categories;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_36() {
        run_csaf21_tests(
            "36",
            test_6_1_36_status_group_contradicting_remediation_categories,
            HashMap::from([
                ("01", vec![ValidationError {
                    message: "Product CSAFPID-9080700 is listed as not affected but has conflicting remediation category vendor_fix".to_string(),
                    instance_path: "/vulnerabilities/0/remediations/0".to_string()
                }]),
                ("02", vec![ValidationError {
                    message: "Product CSAFPID-9080703 is listed as fixed but has conflicting remediation category none_available".to_string(),
                    instance_path: "/vulnerabilities/0/remediations/0".to_string()
                }]),
                ("03", vec![ValidationError {
                    message: "Product CSAFPID-9080700 is listed as affected but has conflicting remediation category optional_patch".to_string(),
                    instance_path: "/vulnerabilities/0/remediations/0".to_string(),
                }]),
                ("04", vec![ValidationError {
                    message: "Product CSAFPID-9080700 is listed as fixed but has conflicting remediation category no_fix_planned".to_string(),
                    instance_path: "/vulnerabilities/0/remediations/0".to_string(),
                }]),
            ]),
        );
    }
}
