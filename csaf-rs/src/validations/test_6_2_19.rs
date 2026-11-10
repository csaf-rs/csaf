use crate::csaf_traits::{
    ContentTrait, CsafTrait, MetricTrait, ProductStatusGroup, ProductStatusGroupMap, VulnerabilityTrait,
};
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::cvss::{CvssScoreNonZeroCheckResult, get_content_nonzero};
use cvss_rs::Version;

fn get_version_specific_finding_strings(version: &Version) -> (&'static str, &'static str, &'static str) {
    match version {
        Version::V2 => ("cvss_v2", "environmentalScore", "Environmental score"),
        Version::V3_0 | Version::V3_1 => ("cvss_v3", "environmentalScore", "Environmental score"),
        Version::V4 => ("cvss_v4", "baseScore", "Score"),
    }
}

fn create_cvss_property_score_nonzero_for_fixed_products_warning(
    product_id: &str,
    metric_type: &Version,
    score: &f64,
    content_path: &str,
) -> TestFinding {
    let (metric_key, score_key, score_name) = get_version_specific_finding_strings(metric_type);
    TestFinding::Warning(TestFindingData {
        message: format!("{score_name} should be 0.0 since {product_id} is listed as fixed (it is {score})"),
        instance_path: format!("{content_path}/{metric_key}/{score_key}"),
    })
}

fn create_cvss_calced_score_nonzero_for_fixed_products_warning(
    product_id: &str,
    metric_type: &Version,
    score: &f64,
    content_path: &str,
) -> TestFinding {
    let (metric_key, _, score_name) = get_version_specific_finding_strings(metric_type);
    TestFinding::Warning(TestFindingData {
        message: format!(
            "{score_name} should be 0.0 since {product_id} is listed as fixed (it is calculated to be {score})"
        ),
        instance_path: format!("{content_path}/{metric_key}"),
    })
}

/// 6.2.19 CVSS for Fixed Products
///
/// For each item in the fixed products group (first_fixed and fixed) it MUST be tested that
/// a CVSS applying to this product has an environmental score of 0.
/// The test SHALL pass if none of the Product IDs listed within product status fixed or
/// first_fixed is found in products of any item of the scores element.
pub fn test_6_2_19_cvss_for_fixed_products(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let mut errors: Option<Vec<TestFinding>> = None;

    for (vuln_index, vuln) in doc.get_vulnerabilities().iter().enumerate() {
        // collect fixed product IDs using the aggregation map
        let status_map = match vuln.get_product_status() {
            Some(product_status) => ProductStatusGroupMap::from(product_status),
            // there are no product statuses (TODO #409)
            None => continue,
        };
        let fixed_products = match status_map.get(&ProductStatusGroup::Fixed) {
            Some(products) => products,
            // there are no products with status group fixed (TODO #409)
            None => continue,
        };

        // check each metric/score for a reference to a fixed product with a
        // non-zero environmental score, reporting the offending CVSS object's path
        if let Some(metrics) = vuln.get_metrics() {
            for (metric_index, metric) in metrics.iter().enumerate() {
                let content = metric.get_content();
                let failing_check_results = get_content_nonzero(content);

                if failing_check_results.is_empty() {
                    continue;
                }

                let content_path = content.get_content_json_path(vuln_index, metric_index);
                for product_id in metric.get_products() {
                    if fixed_products.contains_key(product_id) {
                        for (metric_type, failed_check_result) in &failing_check_results {
                            match failed_check_result {
                                CvssScoreNonZeroCheckResult::PropertyCvssScoreIsZero
                                | CvssScoreNonZeroCheckResult::CalculatedCvssScoreIsZero
                                | CvssScoreNonZeroCheckResult::ParsingFailed
                                | CvssScoreNonZeroCheckResult::CalculationFailed => {
                                    // TODO #409
                                },
                                CvssScoreNonZeroCheckResult::PropertyCvssScoreNonZero(score) => {
                                    errors.get_or_insert_default().push(
                                        create_cvss_property_score_nonzero_for_fixed_products_warning(
                                            product_id,
                                            metric_type,
                                            score,
                                            content_path.as_str(),
                                        ),
                                    );
                                },
                                CvssScoreNonZeroCheckResult::CalculatedCvssScoreNonZero(score) => {
                                    errors.get_or_insert_default().push(
                                        create_cvss_calced_score_nonzero_for_fixed_products_warning(
                                            product_id,
                                            metric_type,
                                            score,
                                            content_path.as_str(),
                                        ),
                                    );
                                },
                            }
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
    use crate::csaf2_0::testcases::ExpectedResults_6_2_19 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_19 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    // helper to build the "calculated" warning for a given metric/score/path combination
    fn calc_error(metric_type: Version, score: f64, content_path: &str) -> Result<(), Vec<TestFinding>> {
        Err(vec![create_cvss_calced_score_nonzero_for_fixed_products_warning(
            "CSAFPID-9080700",
            &metric_type,
            &score,
            content_path,
        )])
    }
    // helper to build the "property" warning for a given metric/score/path combination
    fn prop_error(metric_type: Version, score: f64, content_path: &str) -> Result<(), Vec<TestFinding>> {
        Err(vec![create_cvss_property_score_nonzero_for_fixed_products_warning(
            "CSAFPID-9080700",
            &metric_type,
            &score,
            content_path,
        )])
    }
    // helper to build both the "property" and "calculated" warnings, in that order, for a
    // given metric/score/path combination
    fn calc_and_prop_errors(
        metric_type: Version,
        prop_score: f64,
        calc_score: f64,
        content_path: &str,
    ) -> Result<(), Vec<TestFinding>> {
        let Err(mut findings) = prop_error(metric_type, prop_score, content_path) else {
            unreachable!()
        };
        let Err(calc_findings) = calc_error(metric_type, calc_score, content_path) else {
            unreachable!()
        };
        findings.extend(calc_findings);
        Err(findings)
    }

    #[test]
    fn test_test_6_2_19() {
        let calc = |metric_type: Version, score: f64| calc_error(metric_type, score, "/vulnerabilities/0/scores/0");
        let prop = |metric_type: Version, score: f64| prop_error(metric_type, score, "/vulnerabilities/0/scores/0");
        let both = |metric_type: Version, prop_score: f64, calc_score: f64| {
            calc_and_prop_errors(metric_type, prop_score, calc_score, "/vulnerabilities/0/scores/0")
        };

        // Case 01: CVSS v3.1, no metric that sets to 0, status fixed
        // Case 02: CVSS v3.1, JSON modifiedAvailabilityImpact is not set to None, status fixed
        // Case 03: CVSS v2, JSON targetDistribution is not set to None, status fixed
        // Case 04: CVSS v2, no metric that sets to 0, status fixed
        // Case 05: CVSS v3.0, no metric that sets to 0, status first_fixed
        // Case 06: CVSS v3.0, JSON modifiedAvailabilityImpact is not set to None, status fixed

        // Case s01: CVSS v3.1, MC/MI set to None in vector but MA omitted while base A:H
        //           (MA inherits the non-zero base value, so the score is not zero), status fixed
        // Case s02: same as case_01 (CVSS v3.1, no metric that sets to 0), but with an explicit
        //           environmentalScore matching the vector's calculated non-zero score, status fixed
        //           (both the explicit property and the recalculated score are reported)
        // Case s03: same as case_04 (CVSS v2, no metric that sets to 0), but with an explicit
        //           environmentalScore matching the vector's calculated non-zero score, status fixed
        //           (both the explicit property and the recalculated score are reported)
        // Case s04: same as case_05 (CVSS v3.0, no metric that sets to 0), but with an explicit
        //           environmentalScore matching the vector's calculated non-zero score, status first_fixed
        //           (both the explicit property and the recalculated score are reported)
        // Case s05: CVSS v2, explicit environmentalScore of 0 without targetDistribution NONE, status fixed
        //           (the explicit 0 is not reported, but the recalculated score using the
        //           JSON-provided targetDistribution is non-zero, so it is still reported)
        // Case s06: CVSS v3.1, explicit environmentalScore of 0 without all modified impacts NONE, status fixed
        //           (the explicit 0 is not reported, but the recalculated score is non-zero,
        //           so it is still reported)
        // Case s07: CVSS v3.0, explicit environmentalScore of 0 without all modified impacts NONE, status fixed
        //           (the explicit 0 is not reported, but the recalculated score is non-zero,
        //           so it is still reported)
        // Case s08: CVSS v3.1, all modifiedImpact metrics NONE in JSON, so the
        //           recalculated score is correctly 0, but with an explicit environmentalScore
        //           that does not match, status fixed
        // Case s09: CVSS v2, targetDistribution NONE in JSON, so the
        //           recalculated score is correctly 0, but with an explicit environmentalScore
        //           that does not match, status fixed
        // Case s21: CVSS v3.0, all modifiedImpact metrics NONE in JSON, so the
        //           recalculated score is correctly 0, but with an explicit environmentalScore
        //           that does not match, status fixed
        // Case 11: CVSS v3.1, all modifiedImpact metrics are None in vector, status fixed
        // Case 12: CVSS v3.1, all modifiedImpact metrics are None in JSON, status fixed
        // Case 13: CVSS v2, targetDistribution is None in JSON, status fixed
        // Case 14: CVSS v2, targetDistribution is None in vector, status fixed
        // Case 15: CVSS v3.0, all modifiedImpact metrics are None in vector, status first_fixed
        // Case 16: CVSS v3.1, all modifiedImpact metrics are None in JSON, status fixed
        // Case 17: product status known_affected
        // Case s11: CVSS v3.1, MC/MI set to None in vector, MA omitted while base A:N
        //           (MA inherits the zero-valued base, so the score is zero), status fixed

        TESTS_2_0.test_6_2_19.expect(ExpectedResults_2_0 {
            case_01: calc(Version::V3_0, 6.5),
            case_02: calc(Version::V3_0, 4.2),
            case_03: calc(Version::V2, 1.7),
            case_04: calc(Version::V2, 6.8),
            case_05: calc(Version::V3_0, 6.5),
            case_06: calc(Version::V3_0, 4.2),
            case_s01: calc(Version::V3_0, 4.2),
            case_s02: both(Version::V3_0, 6.5, 6.5),
            case_s03: both(Version::V2, 6.8, 6.8),
            case_s04: both(Version::V3_0, 6.5, 6.5),
            case_s05: calc(Version::V2, 1.7),
            case_s06: calc(Version::V3_0, 6.5),
            case_s07: calc(Version::V3_0, 6.5),
            case_s08: prop(Version::V3_0, 6.5),
            case_s09: prop(Version::V2, 6.8),
            case_s21: prop(Version::V3_0, 6.5),
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
            case_14: Ok(()),
            case_15: Ok(()),
            case_16: Ok(()),
            case_17: Ok(()),
            case_s11: Ok(()),
        });
    }

    #[test]
    fn test_test_6_2_19_2_1() {
        // CSAF 2.1 uses /metrics/{m}/content instead of /scores/{m}
        let calc =
            |metric_type: Version, score: f64| calc_error(metric_type, score, "/vulnerabilities/0/metrics/0/content");
        let prop =
            |metric_type: Version, score: f64| prop_error(metric_type, score, "/vulnerabilities/0/metrics/0/content");
        let both = |metric_type: Version, prop_score: f64, calc_score: f64| {
            calc_and_prop_errors(
                metric_type,
                prop_score,
                calc_score,
                "/vulnerabilities/0/metrics/0/content",
            )
        };

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
        // Case s02: same as case_01 (CVSS v3.1, no metric that sets to 0), but with an explicit
        //           environmentalScore matching the vector's calculated non-zero score, status fixed
        //           (both the explicit property and the recalculated score are reported)
        // Case s03: same as case_04 (CVSS v2, no metric that sets to 0), but with an explicit
        //           environmentalScore matching the vector's calculated non-zero score, status fixed
        //           (both the explicit property and the recalculated score are reported)
        // Case s04: same as case_05 (CVSS v3.0, no metric that sets to 0), but with an explicit
        //           environmentalScore matching the vector's calculated non-zero score, status first_fixed
        //           (both the explicit property and the recalculated score are reported)
        // Case s05: CVSS v2, explicit environmentalScore of 0 without targetDistribution NONE, status fixed
        //           (the explicit 0 is not reported, but the recalculated score using the
        //           JSON-provided targetDistribution is non-zero, so it is still reported)
        // Case s06: CVSS v3.1, explicit environmentalScore of 0 without all modified impacts NONE, status fixed
        //           (the explicit 0 is not reported, but the recalculated score is non-zero,
        //           so it is still reported)
        // Case s07: CVSS v3.0, explicit environmentalScore of 0 without all modified impacts NONE, status fixed
        //           (the explicit 0 is not reported, but the recalculated score is non-zero,
        //           so it is still reported)
        // Case s08: CVSS v3.1, all modifiedImpact metrics NONE in JSON, so the
        //           recalculated score is correctly 0, but with an explicit environmentalScore
        //           that does not match, status fixed
        // Case s09: CVSS v2, targetDistribution NONE in JSON, so the
        //           recalculated score is correctly 0, but with an explicit environmentalScore
        //           that does not match, status fixed
        // Case s21: CVSS v3.0, all modifiedImpact metrics NONE in JSON, so the
        //           recalculated score is correctly 0, but with an explicit environmentalScore
        //           that does not match, status fixed
        // Case s22: CVSS v4.0, explicit baseScore of 0 without all six modified impacts set, status fixed
        //           (the explicit 0 is not a violation, but the recalculated score is non-zero,
        //           so it is still reported)
        // Case s23: CVSS v4.0, all six modified impacts set in JSON, so the recalculated score is
        //           correctly 0, but with an explicit baseScore that does not match, status fixed

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

        TESTS_2_1.test_6_2_19.expect(ExpectedResults_2_1 {
            case_01: calc(Version::V3_0, 6.5),
            case_02: calc(Version::V3_0, 4.2),
            case_03: calc(Version::V2, 1.7),
            case_04: calc(Version::V2, 6.8),
            case_05: calc(Version::V3_0, 6.5),
            case_06: calc(Version::V3_0, 4.2),
            case_07: both(Version::V4, 7.3, 7.3),
            case_08: both(Version::V4, 7.3, 1.8),
            case_s01: calc(Version::V3_0, 4.2),
            case_s02: both(Version::V3_0, 6.5, 6.5),
            case_s03: both(Version::V2, 6.8, 6.8),
            case_s04: both(Version::V3_0, 6.5, 6.5),
            case_s05: calc(Version::V2, 1.7),
            case_s06: calc(Version::V3_0, 6.5),
            case_s07: calc(Version::V3_0, 6.5),
            case_s08: prop(Version::V3_0, 6.5),
            case_s09: prop(Version::V2, 6.8),
            case_s21: prop(Version::V3_0, 6.5),
            case_s22: calc(Version::V4, 7.3),
            case_s23: prop(Version::V4, 7.3),
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
            case_14: Ok(()),
            case_15: Ok(()),
            case_16: Ok(()),
            case_17: Ok(()),
            case_18: Ok(()),
            case_19: Ok(()),
            case_s11: Ok(()),
        });
    }
}
