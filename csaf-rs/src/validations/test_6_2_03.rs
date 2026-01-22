use crate::csaf_traits::{CsafTrait, MetricTrait, ProductStatusTrait, VulnerabilityTrait};
use crate::validation::ValidationError;
use std::collections::HashSet;

fn create_missing_metric_error(
    vulnerability_index: usize,
    status_group_name: &str,
    status_group_product_index: usize,
    product_id: &str,
) -> ValidationError {
    ValidationError {
        message: format!(
            "Missing at least one metric for product ID '{product_id}' in product status group '{status_group_name}'",
        ),
        instance_path: format!(
            "/vulnerabilities/{vulnerability_index}/product_status/{status_group_name}/{status_group_product_index}"
        ),
    }
}

fn check_product_status_group_for_missing_metrics<'a>(
    errors: &mut Option<Vec<ValidationError>>,
    status_group_product_ids: impl Iterator<Item = &'a String>,
    remediation_product_ids: &HashSet<String>,
    vulnerability_index: usize,
    status_group_name: &str,
) {
    // for each product ID in the status group, check if a metric exists
    // if not, generate an error
    for (sg_p_i, product_id) in status_group_product_ids.enumerate() {
        if !remediation_product_ids.contains(product_id) {
            errors.get_or_insert_with(Vec::new).push(create_missing_metric_error(
                vulnerability_index,
                status_group_name,
                sg_p_i,
                product_id,
            ));
        }
    }
}

/// 6.2.3 Missing Metric
///
/// For each product in status groups "affected", a metric must exist.
pub fn test_6_2_03_missing_metric(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    // for each vulnerability
    for (v_i, vuln) in doc.get_vulnerabilities().iter().enumerate() {
        // if there are product statuses
        if let Some(product_status) = vuln.get_product_status() {
            // check if there are products with the relevant product status groups
            if product_status.get_first_affected().is_none()
                && product_status.get_known_affected().is_none()
                && product_status.get_last_affected().is_none()
            {
                continue;
            }

            // collect all product IDs referenced in metrics
            let mut metric_product_ids = HashSet::<String>::new();
            if let Some(metrics) = vuln.get_metrics() {
                for metric in metrics {
                    for product_id in metric.get_products() {
                        metric_product_ids.insert(product_id.clone());
                    }
                }
            }

            // check each relevant product status group for missing metrics
            if let Some(first_affected) = product_status.get_first_affected() {
                check_product_status_group_for_missing_metrics(
                    &mut errors,
                    first_affected,
                    &metric_product_ids,
                    v_i,
                    "first_affected",
                );
            }
            if let Some(known_affected) = product_status.get_known_affected() {
                check_product_status_group_for_missing_metrics(
                    &mut errors,
                    known_affected,
                    &metric_product_ids,
                    v_i,
                    "known_affected",
                );
            }
            if let Some(last_affected) = product_status.get_last_affected() {
                check_product_status_group_for_missing_metrics(
                    &mut errors,
                    last_affected,
                    &metric_product_ids,
                    v_i,
                    "last_affected",
                );
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_2_3
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_03_missing_metric(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_2_3
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_03_missing_metric(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_03() {
        let case_01 = Err(vec![create_missing_metric_error(
            0,
            "first_affected",
            0,
            "CSAFPID-9080700",
        )]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0.test_6_2_3.expect(case_01.clone());
        TESTS_2_1.test_6_2_3.expect(case_01);
    }
}
