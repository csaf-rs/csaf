use crate::csaf_traits::{CsafTrait, ProductStatusTrait, RemediationTrait, VulnerabilityTrait, WithOptionalProductIds};
use crate::schema::csaf2_1::schema::CategoryOfTheRemediation;
use crate::validation::ValidationError;
use std::collections::HashSet;

fn create_missing_remediation_error(
    vulnerability_index: usize,
    status_group_name: &str,
    status_group_product_index: usize,
    product_id: &str,
) -> ValidationError {
    ValidationError {
        message: format!(
            "Missing at least a remediation of category 'none_available' or 'no_fix_planned' for product ID '{product_id}' in product status group '{status_group_name}'",
        ),
        instance_path: format!(
            "/vulnerabilities/{vulnerability_index}/product_status/{status_group_name}/{status_group_product_index}"
        ),
    }
}

fn check_product_status_group_for_missing_remediations<'a>(
    errors: &mut Option<Vec<ValidationError>>,
    status_group_product_ids: impl Iterator<Item = &'a String>,
    remediation_product_ids: &HashSet<String>,
    vulnerability_index: usize,
    status_group_name: &str,
) {
    // for each product ID in the status group, check if a relevant remediation exists
    // if not, generate an error
    for (sg_p_i, product_id) in status_group_product_ids.enumerate() {
        if !remediation_product_ids.contains(product_id) {
            errors
                .get_or_insert_with(Vec::new)
                .push(create_missing_remediation_error(
                    vulnerability_index,
                    status_group_name,
                    sg_p_i,
                    product_id,
                ));
        }
    }
}

/// 6.2.2 Missing Remediation
///
/// For each product in status groups "affected" or "under investigation", a remediation of category
/// `none_available` or `no_fix_planned` must exist.
pub fn test_6_2_02_missing_remediations(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    // for each vulnerability
    for (v_i, vuln) in doc.get_vulnerabilities().iter().enumerate() {
        // if there are product statuses
        if let Some(product_status) = vuln.get_product_status() {
            // check if there are products with the relevant product status groups
            if product_status.get_first_affected().is_none()
                && product_status.get_known_affected().is_none()
                && product_status.get_last_affected().is_none()
                && product_status.get_under_investigation().is_none()
            {
                continue;
            }

            // collect all product IDs referenced in remediations of category none_available or no_fix_planned
            let mut remediation_product_ids = HashSet::<String>::new();
            for remediation in vuln.get_remediations() {
                if (remediation.get_category() == CategoryOfTheRemediation::NoneAvailable
                    || remediation.get_category() == CategoryOfTheRemediation::NoFixPlanned)
                    && let Some(product_ids) = remediation.get_product_ids()
                {
                    for product_id in product_ids {
                        remediation_product_ids.insert(product_id.clone());
                    }
                }
            }

            // check each relevant product status group for missing remediations
            if let Some(first_affected) = product_status.get_first_affected() {
                check_product_status_group_for_missing_remediations(
                    &mut errors,
                    first_affected,
                    &remediation_product_ids,
                    v_i,
                    "first_affected",
                );
            }
            if let Some(known_affected) = product_status.get_known_affected() {
                check_product_status_group_for_missing_remediations(
                    &mut errors,
                    known_affected,
                    &remediation_product_ids,
                    v_i,
                    "known_affected",
                );
            }
            if let Some(last_affected) = product_status.get_last_affected() {
                check_product_status_group_for_missing_remediations(
                    &mut errors,
                    last_affected,
                    &remediation_product_ids,
                    v_i,
                    "last_affected",
                );
            }
            if let Some(under_investigation) = product_status.get_under_investigation() {
                check_product_status_group_for_missing_remediations(
                    &mut errors,
                    under_investigation,
                    &remediation_product_ids,
                    v_i,
                    "under_investigation",
                );
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_2_2
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_02_missing_remediations(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_2_2
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_02_missing_remediations(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_02() {
        let case_01 = Err(vec![create_missing_remediation_error(
            0,
            "last_affected",
            0,
            "CSAFPID-9080700",
        )]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0.test_6_2_2.expect(case_01.clone());
        TESTS_2_1.test_6_2_2.expect(case_01);
    }
}
