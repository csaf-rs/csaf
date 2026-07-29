use std::str::FromStr;

use crate::csaf_traits::{
    ContentTrait, CsafTrait, MetricTrait, ProductStatusGroup, ProductStatusGroupMap, VulnerabilityTrait,
};
use crate::cvss::{deserialize_cvss, is_zero_score};
use crate::validation::ValidationError;
use cvss_rs::Cvss;
use cvss_rs::v2_0::{CvssV2, TargetDistribution};
use cvss_rs::v3::{CvssV3, Impact};

fn create_cvss_for_fixed_products_error(product_id: &str, instance_path: String) -> ValidationError {
    ValidationError {
        message: format!("environmental score should be 0 since {product_id} is listed as fixed"),
        instance_path,
    }
}

/// Checks if a CVSS v2 score has an environmental score of 0.
fn cvss_v2_has_env_score_zero(cvss_v2: CvssV2) -> bool {
    let has_target_distribution_none =
        |cvss_v2: &CvssV2| -> bool { matches!(cvss_v2.target_distribution, Some(TargetDistribution::None)) };

    // check env score provided in json
    if let Some(env_score) = cvss_v2.environmental_score
        && is_zero_score(env_score)
    {
        return true;
    }

    // check if json contains prop that would set env score to zero
    if has_target_distribution_none(&cvss_v2) {
        return true;
    }

    // generate cvss object from vector
    match CvssV2::from_str(&cvss_v2.vector_string) {
        Err(_) => false, // #409 nondeterminable
        // check if vector contains prop that would set env score to zero
        Ok(from_vector) => has_target_distribution_none(&from_vector),
    }
}

/// Checks if a CVSS v3 score has an environmental score of 0.
fn cvss_v3_has_env_score_zero(cvss_v3: CvssV3) -> bool {
    let has_all_modified_impacts_none = |cvss_v3: &CvssV3| -> bool {
        matches!(
            (
                &cvss_v3.modified_availability_impact,
                &cvss_v3.modified_confidentiality_impact,
                &cvss_v3.modified_integrity_impact
            ),
            (Some(Impact::None), Some(Impact::None), Some(Impact::None))
        )
    };

    // check env score provided in json
    if let Some(env_score) = cvss_v3.environmental_score
        && is_zero_score(env_score)
    {
        return true;
    }

    // check if json contains prop that would set env score to zero
    if has_all_modified_impacts_none(&cvss_v3) {
        return true;
    }

    // generate cvss object from vector
    match CvssV3::from_str(&cvss_v3.vector_string) {
        Err(_) => false, // #409 nondeterminable
        // check if vector contains prop that would set env score to zero
        Ok(from_vector) => has_all_modified_impacts_none(&from_vector),
    }
}

/// Returns the JSON keys (`cvss_v2` and/or `cvss_v3`) of the CVSS objects in this content whose
/// environmental score is not 0. An empty result means every present CVSS score (if any) has an
/// environmental score of 0.
fn failing_cvss_keys(content: &impl ContentTrait) -> Vec<&'static str> {
    let mut failing_keys = Vec::new();

    // check if cvss_v2 prop is set
    if let Some(cvss_v2) = content.get_cvss_v2() {
        // deserialize cvss, we only care about result, not errors
        let v2_is_zero = match deserialize_cvss(cvss_v2, "", &mut None) {
            // TODO: Nondeterminable #409, could not deserialize
            None => false,
            Some(Cvss::V2(v2)) => cvss_v2_has_env_score_zero(v2),
            // TODO: Nondeterminable #409 - deserialized into wrong version
            Some(_) => false,
        };
        if !v2_is_zero {
            failing_keys.push("cvss_v2");
        }
    }

    // check if the cvss_v3 prop is set
    if let Some(cvss_v3) = content.get_cvss_v3() {
        // deserialize cvss, we only care about result, not errors
        let v3_is_zero = match deserialize_cvss(cvss_v3, "", &mut None) {
            // TODO: Nondeterminable #409, could not deserialize
            None => false,
            Some(Cvss::V3_0(v3)) | Some(Cvss::V3_1(v3)) => cvss_v3_has_env_score_zero(v3),
            // TODO: Nondeterminable #409 - deserialized into wrong version
            Some(_) => false,
        };
        if !v3_is_zero {
            failing_keys.push("cvss_v3");
        }
    }

    failing_keys
}

/// 6.2.19 CVSS for Fixed Products
///
/// For each item in the fixed products group (first_fixed and fixed) it MUST be tested that
/// a CVSS applying to this product has an environmental score of 0.
/// The test SHALL pass if none of the Product IDs listed within product status fixed or
/// first_fixed is found in products of any item of the scores element.
pub fn test_6_2_19_cvss_for_fixed_products(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;
    for (v_i, vuln) in doc.get_vulnerabilities().iter().enumerate() {
        // collect fixed product IDs using the aggregation map
        let status_map = match vuln.get_product_status() {
            Some(product_status) => ProductStatusGroupMap::from(product_status),
            // there are no product statuses
            None => continue,
        };
        let fixed_products = match status_map.get(&ProductStatusGroup::Fixed) {
            Some(products) => products,
            // there are no products with status group fixed
            None => continue,
        };

        // check each metric/score for a reference to a fixed product with a
        // non-zero environmental score, reporting the offending CVSS object's path
        if let Some(metrics) = vuln.get_metrics() {
            for (m_i, metric) in metrics.iter().enumerate() {
                let content = metric.get_content();
                let failing_keys = failing_cvss_keys(content);
                if failing_keys.is_empty() {
                    continue;
                }

                let content_path = content.get_content_json_path(v_i, m_i);
                for product_id in metric.get_products() {
                    if fixed_products.contains_key(product_id) {
                        for key in &failing_keys {
                            errors
                                .get_or_insert_default()
                                .push(create_cvss_for_fixed_products_error(
                                    product_id,
                                    format!("{content_path}/{key}"),
                                ));
                        }
                    }
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(ValidatorForTest6_2_19, test_6_2_19_cvss_for_fixed_products);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;

    #[test]
    fn test_test_6_2_19() {
        // Test data only contains two CVSS keys, so we can share the error messages
        let err_v3 = Err(vec![create_cvss_for_fixed_products_error(
            "CSAFPID-9080700",
            "/vulnerabilities/0/scores/0/cvss_v3".to_string(),
        )]);
        let err_v2 = Err(vec![create_cvss_for_fixed_products_error(
            "CSAFPID-9080700",
            "/vulnerabilities/0/scores/0/cvss_v2".to_string(),
        )]);

        // Case 01: CVSS v3.1, no metric that sets to 0, status fixed
        // Case 02: CVSS v3.1, JSON modifiedAvailabilityImpact is not set to None, status fixed
        // Case 03: CVSS v2, JSON targetDistribution is not set to None, status fixed
        // Case 04: CVSS v2, no metric that sets to 0, status fixed
        // Case 05: CVSS v3.0, no metric that sets to 0, status first_fixed
        // Case 06: CVSS v3.0, JSON modifiedAvailabilityImpact is not set to None, status fixed

        // Case 11: CVSS v3.1, all modifiedImpact metrics are None in vector, status fixed
        // Case 12: CVSS v3.1, all modifiedImpact metrics are None in JSON, status fixed
        // Case 13: CVSS v2, targetDistribution is None in JSON, status fixed
        // Case 14: CVSS v2, targetDistribution is None in vector, status fixed
        // Case 15: CVSS v3.0, all modifiedImpact metrics are None in vector, status first_fixed
        // Case 16: CVSS v3.1, all modifiedImpact metrics are None in JSON, status fixed
        // Case 17: product status known_affected

        TESTS_2_0.test_6_2_19.expect(
            err_v3.clone(),
            err_v3.clone(),
            err_v2.clone(),
            err_v2,
            err_v3.clone(),
            err_v3,
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
