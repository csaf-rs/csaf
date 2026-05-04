use std::str::FromStr;

use crate::csaf_traits::{
    ContentTrait, CsafTrait, MetricTrait, ProductStatusAndPath, ProductStatusGroup, ProductStatusGroupMap,
    VulnerabilityTrait,
};
use crate::cvss::{deserialize_cvss, is_zero_score};
use crate::validation::ValidationError;
use cvss_rs::Cvss;
use cvss_rs::v2_0::{CvssV2, TargetDistribution};
use cvss_rs::v3::{CvssV3, Impact};

fn create_cvss_for_fixed_products_error(
    product_id: &str,
    statuses: &[ProductStatusAndPath],
    path: &str,
) -> ValidationError {
    let status_list: Vec<String> = statuses.iter().map(|s| s.status.to_string()).collect();
    ValidationError {
        message: format!(
            "Product '{}' is listed as fixed (status: {}) but has a CVSS score without an environmental score of 0",
            product_id,
            status_list.join(", ")
        ),
        instance_path: path.to_string(),
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

/// Returns true if all CVSS scores in this content have environmental score of 0.
fn content_has_env_score_zero(content: &impl ContentTrait) -> bool {
    // check if cvss_v2 prop is set
    if let Some(cvss_v2) = content.get_cvss_v2() {
        // deserialize cvss, we only care about result, not errors
        let Some(deserialized) = deserialize_cvss(cvss_v2, "", &mut None) else {
            // TODO: Nondeterminable #409, could not deserialize
            return false;
        };
        return match deserialized {
            Cvss::V2(v2) => cvss_v2_has_env_score_zero(v2),
            // TODO: Nondeterminable #409 - deserialized into wrong version
            _ => false,
        };
    }
    // check if the cvss_v3 prop is set
    if let Some(cvss_v3) = content.get_cvss_v3() {
        // deserialize cvss, we only care about result, not errors
        let Some(deserialized) = deserialize_cvss(cvss_v3, "", &mut None) else {
            // TODO: Nondeterminable #409, could not deserialize
            return false;
        };
        return match deserialized {
            Cvss::V3_0(v3) | Cvss::V3_1(v3) => cvss_v3_has_env_score_zero(v3),
            // TODO: Nondeterminable #409 - deserialized into wrong version
            _ => false,
        };
    }
    // There are no CVSS scores
    false
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
        let fixed_products = match vuln.get_product_status() {
            Some(product_status) => {
                let status_map = ProductStatusGroupMap::from(product_status);
                match status_map.get(&ProductStatusGroup::Fixed) {
                    Some(products) => products.clone(),
                    None => continue,
                }
            },
            None => continue,
        };

        // if there are no fixed products, we can skip early
        if fixed_products.is_empty() {
            continue;
        }

        // check each metric/score
        if let Some(metrics) = vuln.get_metrics() {
            let metrics_path = vuln.get_metrics_path();
            for (m_i, metric) in metrics.iter().enumerate() {
                let content = metric.get_content();
                for (p_i, product_id) in metric.get_products().enumerate() {
                    // if the metric/score is relevant to a product
                    if let Some(statuses) = fixed_products.get(product_id.as_str()) {
                        // and the product does not have an env score of zero, generate an error
                        if !content_has_env_score_zero(content) {
                            errors
                                .get_or_insert_default()
                                .push(create_cvss_for_fixed_products_error(
                                    product_id.as_str(),
                                    statuses,
                                    &format!("/vulnerabilities/{v_i}/{metrics_path}/{m_i}/products/{p_i}"),
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
    use crate::csaf_traits::ProductStatus;
    use crate::csaf2_0::testcases::TESTS_2_0;

    #[test]
    fn test_test_6_2_19() {
        // Test data only contains two paths, so we can share the error messages
        let err_fixed = Err(vec![create_cvss_for_fixed_products_error(
            "CSAFPID-9080700",
            &[ProductStatusAndPath {
                status: ProductStatus::Fixed,
                index: 0,
            }],
            "/vulnerabilities/0/scores/0/products/0",
        )]);
        let err_first_fixed = Err(vec![create_cvss_for_fixed_products_error(
            "CSAFPID-9080700",
            &[ProductStatusAndPath {
                status: ProductStatus::FirstFixed,
                index: 0,
            }],
            "/vulnerabilities/0/scores/0/products/0",
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
            err_fixed.clone(),
            err_fixed.clone(),
            err_fixed.clone(),
            err_fixed.clone(),
            err_first_fixed.clone(),
            err_fixed.clone(),
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
