use std::str::FromStr;

use crate::csaf_traits::{
    ContentTrait, CsafTrait, MetricTrait, ProductStatusGroup, ProductStatusGroupMap, VulnerabilityTrait,
};
use crate::cvss::{deserialize_cvss, is_zero_score};
use crate::validation::ValidationError;
use cvss_rs::Cvss;
use cvss_rs::v2_0::CvssV2;
use cvss_rs::v3::CvssV3;
use cvss_rs::v4_0::CvssV4;

fn create_cvss_for_fixed_products_error(product_id: &str, instance_path: String) -> ValidationError {
    ValidationError {
        message: format!("environmental score should be 0 since {product_id} is listed as fixed"),
        instance_path,
    }
}

/// Checks if a CVSS v2 score has an environmental score of 0.
///
/// Per the CVSS v2 specification, an environmental metric that is not defined (either absent
/// from the vector string or absent from the JSON object) does not simply count as "not set" for
/// scoring purposes: it falls back to its default (`targetDistribution` defaults to `ND`, i.e. a
/// multiplier of 1.0). To correctly honor this, the effective score is calculated (using the
/// library's own scoring implementation) from a CVSS object seeded with the base metrics parsed
/// from `vectorString`, with any environmental metric given explicitly in JSON overlaid on top
/// (JSON properties take precedence over the vector string, mirroring how CSAF documents are
/// allowed to convey them either way).
fn cvss_v2_has_env_score_zero(cvss_v2: CvssV2) -> bool {
    // an explicitly reported environmental score is authoritative
    if let Some(env_score) = cvss_v2.environmental_score
        && is_zero_score(env_score)
    {
        return true;
    }

    // seed the effective CVSS object with the base (and any vector-encoded environmental) metrics
    let Ok(mut effective) = CvssV2::from_str(&cvss_v2.vector_string) else {
        return false; // #409 nondeterminable
    };

    // JSON-provided environmental properties take precedence over the vector string
    if cvss_v2.target_distribution.is_some() {
        effective.target_distribution = cvss_v2.target_distribution;
    }

    // let the library calculate the actual environmental score, correctly handling metrics
    // that are inherited from their base counterpart when not explicitly defined
    effective.calculated_environmental_score().is_some_and(is_zero_score)
}

/// Checks if a CVSS v3 score has an environmental score of 0.
///
/// See [`cvss_v2_has_env_score_zero`] for why the effective score is (re)calculated instead of
/// only checking whether the modified impact metrics are explicitly `NONE`: an undefined modified
/// impact metric inherits the value of its corresponding base impact metric, which can also
/// result in an environmental score of 0 (e.g. `.../A:N/MC:N/MI:N` with no `MA` at all).
fn cvss_v3_has_env_score_zero(cvss_v3: CvssV3) -> bool {
    // an explicitly reported environmental score is authoritative
    if let Some(env_score) = cvss_v3.environmental_score
        && is_zero_score(env_score)
    {
        return true;
    }

    // seed the effective CVSS object with the base (and any vector-encoded environmental) metrics
    let Ok(mut effective) = CvssV3::from_str(&cvss_v3.vector_string) else {
        return false; // #409 nondeterminable
    };

    // JSON-provided environmental properties take precedence over the vector string
    if cvss_v3.modified_confidentiality_impact.is_some() {
        effective.modified_confidentiality_impact = cvss_v3.modified_confidentiality_impact;
    }
    if cvss_v3.modified_integrity_impact.is_some() {
        effective.modified_integrity_impact = cvss_v3.modified_integrity_impact;
    }
    if cvss_v3.modified_availability_impact.is_some() {
        effective.modified_availability_impact = cvss_v3.modified_availability_impact;
    }

    // let the library calculate the actual environmental score, correctly handling metrics
    // that are inherited from their base counterpart when not explicitly defined
    effective.calculated_environmental_score().is_some_and(is_zero_score)
}

/// Checks if a CVSS v4 score has an overall score of 0.
///
/// CVSS v4 has no separate environmental score; the modified environmental metrics directly
/// influence the single overall score. See [`cvss_v2_has_env_score_zero`] for why the effective
/// score is (re)calculated instead of only checking whether the modified impact metrics are
/// explicitly set: an undefined modified impact metric inherits the value of its corresponding
/// base impact metric.
fn cvss_v4_has_env_score_zero(cvss_v4: CvssV4) -> bool {
    // an explicitly reported overall score is authoritative
    if is_zero_score(cvss_v4.base_score) {
        return true;
    }

    // seed the effective CVSS object with the base (and any vector-encoded environmental) metrics
    let Ok(mut effective) = CvssV4::from_str(&cvss_v4.vector_string) else {
        return false; // #409 nondeterminable
    };

    // JSON-provided environmental properties take precedence over the vector string
    if cvss_v4.modified_vuln_confidentiality_impact.is_some() {
        effective.modified_vuln_confidentiality_impact = cvss_v4.modified_vuln_confidentiality_impact;
    }
    if cvss_v4.modified_vuln_integrity_impact.is_some() {
        effective.modified_vuln_integrity_impact = cvss_v4.modified_vuln_integrity_impact;
    }
    if cvss_v4.modified_vuln_availability_impact.is_some() {
        effective.modified_vuln_availability_impact = cvss_v4.modified_vuln_availability_impact;
    }
    if cvss_v4.modified_sub_confidentiality_impact.is_some() {
        effective.modified_sub_confidentiality_impact = cvss_v4.modified_sub_confidentiality_impact;
    }
    if cvss_v4.modified_sub_integrity_impact.is_some() {
        effective.modified_sub_integrity_impact = cvss_v4.modified_sub_integrity_impact;
    }
    if cvss_v4.modified_sub_availability_impact.is_some() {
        effective.modified_sub_availability_impact = cvss_v4.modified_sub_availability_impact;
    }

    // let the library calculate the actual overall score, correctly handling metrics that are
    // inherited from their base counterpart when not explicitly defined
    effective
        .calculated_score()
        .is_some_and(|(score, _)| is_zero_score(score))
}

/// Returns the JSON keys (`cvss_v2`, `cvss_v3`, and/or `cvss_v4`) of the CVSS objects in this
/// content whose environmental (resp. overall) score is not 0. An empty result means every
/// present CVSS score (if any) has a score of 0.
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

    // check if the cvss_v4 prop is set (CSAF 2.1 only)
    if let Some(cvss_v4) = content.get_cvss_v4() {
        // deserialize cvss, we only care about result, not errors
        let v4_is_zero = match deserialize_cvss(cvss_v4, "", &mut None) {
            // TODO: Nondeterminable #409, could not deserialize
            None => false,
            Some(Cvss::V4(v4)) => cvss_v4_has_env_score_zero(v4),
            // TODO: Nondeterminable #409 - deserialized into wrong version
            Some(_) => false,
        };
        if !v4_is_zero {
            failing_keys.push("cvss_v4");
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
    use crate::csaf2_1::testcases::TESTS_2_1;

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

        // Case s01: CVSS v3.1, MC/MI set to None in vector but MA omitted while base A:H
        //           (MA inherits the non-zero base value, so the score is not zero), status fixed

        // Case 11: CVSS v3.1, all modifiedImpact metrics are None in vector, status fixed
        // Case 12: CVSS v3.1, all modifiedImpact metrics are None in JSON, status fixed
        // Case 13: CVSS v2, targetDistribution is None in JSON, status fixed
        // Case 14: CVSS v2, targetDistribution is None in vector, status fixed
        // Case 15: CVSS v3.0, all modifiedImpact metrics are None in vector, status first_fixed
        // Case 16: CVSS v3.1, all modifiedImpact metrics are None in JSON, status fixed
        // Case 17: product status known_affected
        // Case s11: CVSS v3.1, MC/MI set to None in vector, MA omitted while base A:N
        //           (MA inherits the zero-valued base, so the score is zero), status fixed
        // Case s12: CVSS v2, explicit environmentalScore of 0 without targetDistribution NONE, status fixed
        // Case s13: CVSS v3.1, explicit environmentalScore of 0 without all modified impacts NONE, status fixed

        TESTS_2_0.test_6_2_19.expect(
            err_v3.clone(),
            err_v3.clone(),
            err_v2.clone(),
            err_v2,
            err_v3.clone(),
            err_v3.clone(),
            err_v3,
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }

    #[test]
    fn test_test_6_2_19_2_1() {
        // CSAF 2.1 uses /metrics/{m}/content instead of /scores/{m}
        let err_v3 = Err(vec![create_cvss_for_fixed_products_error(
            "CSAFPID-9080700",
            "/vulnerabilities/0/metrics/0/content/cvss_v3".to_string(),
        )]);
        let err_v2 = Err(vec![create_cvss_for_fixed_products_error(
            "CSAFPID-9080700",
            "/vulnerabilities/0/metrics/0/content/cvss_v2".to_string(),
        )]);
        let err_v4 = Err(vec![create_cvss_for_fixed_products_error(
            "CSAFPID-9080700",
            "/vulnerabilities/0/metrics/0/content/cvss_v4".to_string(),
        )]);

        // Case 01: CVSS v3.1, no metric that sets to 0, status fixed
        // Case 02: CVSS v3.1, JSON modifiedAvailabilityImpact is not set to None, status fixed
        // Case 03: CVSS v2, JSON targetDistribution is not set to None, status fixed
        // Case 04: CVSS v2, no metric that sets to 0, status fixed
        // Case 05: CVSS v3.0, no metric that sets to 0, status first_fixed
        // Case 06: CVSS v3.0, JSON modifiedAvailabilityImpact is not set to None, status fixed
        // Case 07: CVSS v4.0, no metric that sets to 0, status first_fixed
        // Case 08: CVSS v4.0, JSON modifiedSubAvailabilityImpact is not set to NEGLIGIBLE, status first_fixed
        // Case s01: CVSS v3.1, MC/MI set to None in vector but MA omitted while base A:H
        //           (MA inherits the non-zero base value, so the score is not zero), status fixed

        // Case 11: CVSS v3.1, all modifiedImpact metrics are None in vector, status fixed
        // Case 12: CVSS v3.1, all modifiedImpact metrics are None in JSON, status fixed
        // Case 13: CVSS v2, targetDistribution is None in JSON, status fixed
        // Case 14: CVSS v2, targetDistribution is None in vector, status fixed
        // Case 15: CVSS v3.0, all modifiedImpact metrics are None in vector, status first_fixed
        // Case 16: CVSS v3.1, all modifiedImpact metrics are None in JSON, status fixed
        // Case 17: product status known_affected
        // Case 18: CVSS v4.0, all modified* metrics are None/NEGLIGIBLE in vector, status first_fixed
        // Case 19: CVSS v4.0, all modified* metrics are None/NEGLIGIBLE in JSON, status first_fixed
        // Case s11: CVSS v3.1, MC/MI set to None in vector, MA omitted while base A:N
        //           (MA inherits the zero-valued base, so the score is zero), status fixed
        // Case s12: CVSS v2, explicit environmentalScore of 0 without targetDistribution NONE, status fixed
        // Case s13: CVSS v3.1, explicit environmentalScore of 0 without all modified impacts NONE, status fixed
        // Case s14: CVSS v4.0, explicit baseScore of 0 without all six modified impacts set, status fixed

        TESTS_2_1.test_6_2_19.expect(
            err_v3.clone(),
            err_v3.clone(),
            err_v2.clone(),
            err_v2,
            err_v3.clone(),
            err_v3.clone(),
            err_v4.clone(),
            err_v4,
            err_v3,
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
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
