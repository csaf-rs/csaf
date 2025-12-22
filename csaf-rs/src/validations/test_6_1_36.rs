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

fn create_affected_conflict_error(
    product_id: &str,
    category: &CategoryOfTheRemediation,
    v_i: usize,
    r_i: usize,
) -> ValidationError {
    ValidationError {
        message: format!(
            "Product {} is listed as affected but has conflicting remediation category {}",
            product_id, category
        ),
        instance_path: format!("/vulnerabilities/{}/remediations/{}", v_i, r_i),
    }
}

fn create_not_affected_conflict_error(
    product_id: &str,
    category: &CategoryOfTheRemediation,
    v_i: usize,
    r_i: usize,
) -> ValidationError {
    ValidationError {
        message: format!(
            "Product {} is listed as not affected but has conflicting remediation category {}",
            product_id, category
        ),
        instance_path: format!("/vulnerabilities/{}/remediations/{}", v_i, r_i),
    }
}

fn create_fixed_conflict_error(
    product_id: &str,
    category: &CategoryOfTheRemediation,
    v_i: usize,
    r_i: usize,
) -> ValidationError {
    ValidationError {
        message: format!(
            "Product {} is listed as fixed but has conflicting remediation category {}",
            product_id, category
        ),
        instance_path: format!("/vulnerabilities/{}/remediations/{}", v_i, r_i),
    }
}

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
                                return Err(vec![create_affected_conflict_error(&p, &cat, v_i, r_i)]);
                            }
                        }
                        if let Some(not_affected_products) = not_affected_products {
                            if not_affected_products.contains(&p) && NOT_AFFECTED_CONFLICTS.contains(&cat) {
                                return Err(vec![create_not_affected_conflict_error(&p, &cat, v_i, r_i)]);
                            }
                        }
                        if let Some(fixed_products) = fixed_products {
                            if fixed_products.contains(&p) && FIXED_CONFLICTS.contains(&cat) {
                                return Err(vec![create_fixed_conflict_error(&p, &cat, v_i, r_i)]);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_36
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_36_status_group_contradicting_remediation_categories(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_36() {
        let case_01 = Err(vec![create_not_affected_conflict_error(
            "CSAFPID-9080700",
            &CategoryOfTheRemediation::VendorFix,
            0,
            0,
        )]);
        let case_02 = Err(vec![create_fixed_conflict_error(
            "CSAFPID-9080703",
            &CategoryOfTheRemediation::NoneAvailable,
            0,
            0,
        )]);
        let case_03 = Err(vec![create_affected_conflict_error(
            "CSAFPID-9080700",
            &CategoryOfTheRemediation::OptionalPatch,
            0,
            0,
        )]);
        let case_04 = Err(vec![create_fixed_conflict_error(
            "CSAFPID-9080700",
            &CategoryOfTheRemediation::NoFixPlanned,
            0,
            0,
        )]);

        // Only CSAF 2.1 has this test with 8 test cases (4 error cases, 4 success cases)
        TESTS_2_1
            .test_6_1_36
            .expect(case_01, case_02, case_03, case_04, Ok(()), Ok(()), Ok(()), Ok(()));
    }
}
