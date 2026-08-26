use std::str::FromStr;

use crate::csaf_traits::{
    ContentTrait, CsafTrait, MetricTrait, ProductStatusGroup, ProductStatusGroupMap, VulnerabilityTrait,
};
use crate::cvss::{deserialize_cvss, is_not_defined, is_zero_score};
use crate::validation::{TestFinding, TestFindingData};
use cvss_rs::Cvss;
use cvss_rs::v2_0::CvssV2;
use cvss_rs::v3::CvssV3;
use cvss_rs::v4_0::CvssV4;

fn create_cvss_property_score_nonzero_for_fixed_products_warning(
    product_id: &str,
    metric_type: &TestSpecificCvssKind,
    score: &f64,
    content_path: &str,
) -> TestFinding {
    let (metric_key, score_key, score_name) = metric_type.get_version_specific_finding_strings();
    TestFinding::Warning(TestFindingData {
        message: format!("{score_name} should be 0.0 since {product_id} is listed as fixed (it is {score})"),
        instance_path: format!("{content_path}/{metric_key}/{score_key}"),
    })
}

fn create_cvss_calced_score_nonzero_for_fixed_products_warning(
    product_id: &str,
    metric_type: &TestSpecificCvssKind,
    score: &f64,
    content_path: &str,
) -> TestFinding {
    let (metric_key, _, score_name) = metric_type.get_version_specific_finding_strings();
    TestFinding::Warning(TestFindingData {
        message: format!(
            "{score_name} should be 0.0 since {product_id} is listed as fixed (it is calculated to be {score})"
        ),
        instance_path: format!("{content_path}/{metric_key}"),
    })
}

/// 6.2.19 test specific marker enum for CVSS versions.
/// Not using `CsafVulnerabilityMetric` as we don't care about / don't need the inner CVSS version string
#[cfg_attr(test, derive(Clone, Copy))]
enum TestSpecificCvssKind {
    CvssV2,
    CvssV3,
    CvssV4,
}

impl TestSpecificCvssKind {
    fn get_version_specific_finding_strings(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::CvssV2 => ("cvss_v2", "environmentalScore", "Environmental score"),
            Self::CvssV3 => ("cvss_v3", "environmentalScore", "Environmental score"),
            Self::CvssV4 => ("cvss_v4", "baseScore", "Score"),
        }
    }
}

enum CvssScoreNonZeroCheckResult {
    PropertyCvssScoreIsZero,
    PropertyCvssScoreNonZero(f64),
    CalculatedCvssScoreIsZero,
    CalculatedCvssScoreNonZero(f64),
    ParsingFailed,
    CalculationFailed,
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
fn cvss_v2_has_env_score_zero(cvss_v2: CvssV2) -> Vec<CvssScoreNonZeroCheckResult> {
    let mut results: Vec<CvssScoreNonZeroCheckResult> = Vec::new();
    // an explicitly reported environmental score is authoritative
    if let Some(env_score) = cvss_v2.environmental_score {
        if is_zero_score(env_score) {
            results.push(CvssScoreNonZeroCheckResult::PropertyCvssScoreIsZero)
        } else {
            results.push(CvssScoreNonZeroCheckResult::PropertyCvssScoreNonZero(env_score))
        };
    }

    // seed the effective CVSS object with the base (and any vector-encoded environmental) metrics
    let Ok(mut effective) = CvssV2::from_str(&cvss_v2.vector_string) else {
        results.push(CvssScoreNonZeroCheckResult::ParsingFailed);
        return results;
    };

    // JSON-provided environmental properties supplement the vector, if the vector does not have
    // the value or has it as NotDefined, and the JSON has the value defined (i.e. not NotDefined)
    if effective.target_distribution.as_ref().is_none_or(is_not_defined)
        && cvss_v2.target_distribution.as_ref().is_some_and(|v| !is_not_defined(v))
    {
        effective.target_distribution = cvss_v2.target_distribution;
    }

    // calculate the environmental score based on merged vector and JSON
    if let Some(calced_env_score) = effective.calculated_environmental_score() {
        if is_zero_score(calced_env_score) {
            results.push(CvssScoreNonZeroCheckResult::CalculatedCvssScoreIsZero);
        } else {
            results.push(CvssScoreNonZeroCheckResult::CalculatedCvssScoreNonZero(
                calced_env_score,
            ));
        }
    } else {
        results.push(CvssScoreNonZeroCheckResult::CalculationFailed);
    }
    results
}

/// Checks if a CVSS v3 score has an environmental score of 0.
///
/// See [`cvss_v2_has_env_score_zero`] for why the effective score is (re)calculated instead of
/// only checking whether the modified impact metrics are explicitly `NONE`: an undefined modified
/// impact metric inherits the value of its corresponding base impact metric, which can also
/// result in an environmental score of 0 (e.g. `.../A:N/MC:N/MI:N` with no `MA` at all).
fn cvss_v3_has_env_score_zero(cvss_v3: CvssV3) -> Vec<CvssScoreNonZeroCheckResult> {
    let mut results = Vec::new();
    // an explicitly reported environmental score is authoritative
    if let Some(env_score) = cvss_v3.environmental_score {
        if is_zero_score(env_score) {
            results.push(CvssScoreNonZeroCheckResult::PropertyCvssScoreIsZero);
        } else {
            results.push(CvssScoreNonZeroCheckResult::PropertyCvssScoreNonZero(env_score));
        }
    }

    // seed the effective CVSS object with the base (and any vector-encoded environmental) metrics
    let Ok(mut effective) = CvssV3::from_str(&cvss_v3.vector_string) else {
        results.push(CvssScoreNonZeroCheckResult::ParsingFailed);
        return results;
    };

    // JSON-provided environmental properties supplement the vector, if the vector does not have
    // the value or has it as NotDefined, and the JSON has the value defined (i.e. not NotDefined)
    if effective
        .modified_confidentiality_impact
        .as_ref()
        .is_none_or(is_not_defined)
        && cvss_v3
            .modified_confidentiality_impact
            .as_ref()
            .is_some_and(|v| !is_not_defined(v))
    {
        effective.modified_confidentiality_impact = cvss_v3.modified_confidentiality_impact;
    }
    if effective.modified_integrity_impact.as_ref().is_none_or(is_not_defined)
        && cvss_v3
            .modified_integrity_impact
            .as_ref()
            .is_some_and(|v| !is_not_defined(v))
    {
        effective.modified_integrity_impact = cvss_v3.modified_integrity_impact;
    }
    if effective
        .modified_availability_impact
        .as_ref()
        .is_none_or(is_not_defined)
        && cvss_v3
            .modified_availability_impact
            .as_ref()
            .is_some_and(|v| !is_not_defined(v))
    {
        effective.modified_availability_impact = cvss_v3.modified_availability_impact;
    }

    // let the library calculate the actual environmental score, correctly handling metrics
    // that are inherited from their base counterpart when not explicitly defined
    if let Some(calced_env_score) = effective.calculated_environmental_score() {
        if is_zero_score(calced_env_score) {
            results.push(CvssScoreNonZeroCheckResult::CalculatedCvssScoreIsZero);
        } else {
            results.push(CvssScoreNonZeroCheckResult::CalculatedCvssScoreNonZero(
                calced_env_score,
            ));
        }
    } else {
        results.push(CvssScoreNonZeroCheckResult::CalculationFailed);
    }
    results
}

/// Checks if a CVSS v4 score has an overall score of 0.
///
/// CVSS v4 has no separate environmental score; the modified environmental metrics directly
/// influence the single overall score. See [`cvss_v2_has_env_score_zero`] for why the effective
/// score is (re)calculated instead of only checking whether the modified impact metrics are
/// explicitly set: an undefined modified impact metric inherits the value of its corresponding
/// base impact metric.
fn cvss_v4_has_score_zero(cvss_v4: CvssV4) -> Vec<CvssScoreNonZeroCheckResult> {
    let mut results = Vec::new();

    // an explicitly reported overall score is authoritative
    if is_zero_score(cvss_v4.base_score) {
        results.push(CvssScoreNonZeroCheckResult::PropertyCvssScoreIsZero);
    } else {
        results.push(CvssScoreNonZeroCheckResult::PropertyCvssScoreNonZero(
            cvss_v4.base_score,
        ));
    }

    // seed the effective CVSS object with the base (and any vector-encoded environmental) metrics
    let Ok(mut effective) = CvssV4::from_str(&cvss_v4.vector_string) else {
        results.push(CvssScoreNonZeroCheckResult::ParsingFailed);
        return results;
    };

    // JSON-provided environmental properties supplement the vector, if the vector does not have
    // the value or has it as NotDefined, and the JSON has the value defined (i.e. not NotDefined)
    if effective
        .modified_vuln_confidentiality_impact
        .as_ref()
        .is_none_or(is_not_defined)
        && cvss_v4
            .modified_vuln_confidentiality_impact
            .as_ref()
            .is_some_and(|v| !is_not_defined(v))
    {
        effective.modified_vuln_confidentiality_impact = cvss_v4.modified_vuln_confidentiality_impact;
    }
    if effective
        .modified_vuln_integrity_impact
        .as_ref()
        .is_none_or(is_not_defined)
        && cvss_v4
            .modified_vuln_integrity_impact
            .as_ref()
            .is_some_and(|v| !is_not_defined(v))
    {
        effective.modified_vuln_integrity_impact = cvss_v4.modified_vuln_integrity_impact;
    }
    if effective
        .modified_vuln_availability_impact
        .as_ref()
        .is_none_or(is_not_defined)
        && cvss_v4
            .modified_vuln_availability_impact
            .as_ref()
            .is_some_and(|v| !is_not_defined(v))
    {
        effective.modified_vuln_availability_impact = cvss_v4.modified_vuln_availability_impact;
    }
    if effective
        .modified_sub_confidentiality_impact
        .as_ref()
        .is_none_or(is_not_defined)
        && cvss_v4
            .modified_sub_confidentiality_impact
            .as_ref()
            .is_some_and(|v| !is_not_defined(v))
    {
        effective.modified_sub_confidentiality_impact = cvss_v4.modified_sub_confidentiality_impact;
    }
    if effective
        .modified_sub_integrity_impact
        .as_ref()
        .is_none_or(is_not_defined)
        && cvss_v4
            .modified_sub_integrity_impact
            .as_ref()
            .is_some_and(|v| !is_not_defined(v))
    {
        effective.modified_sub_integrity_impact = cvss_v4.modified_sub_integrity_impact;
    }
    if effective
        .modified_sub_availability_impact
        .as_ref()
        .is_none_or(is_not_defined)
        && cvss_v4
            .modified_sub_availability_impact
            .as_ref()
            .is_some_and(|v| !is_not_defined(v))
    {
        effective.modified_sub_availability_impact = cvss_v4.modified_sub_availability_impact;
    }

    // let the library calculate the actual overall score, correctly handling metrics that are
    // inherited from their base counterpart when not explicitly defined
    if let Some((calced_score, _)) = effective.calculated_score() {
        if is_zero_score(calced_score) {
            results.push(CvssScoreNonZeroCheckResult::PropertyCvssScoreIsZero);
        } else {
            results.push(CvssScoreNonZeroCheckResult::CalculatedCvssScoreNonZero(calced_score));
        }
    } else {
        results.push(CvssScoreNonZeroCheckResult::CalculationFailed);
    }
    results
}

/// Returns the JSON keys (`cvss_v2`, `cvss_v3`, and/or `cvss_v4`) of the CVSS objects in this
/// content whose environmental (resp. overall) score is not 0. An empty result means every
/// present CVSS score (if any) has a score of 0.
fn failing_cvss_keys(content: &impl ContentTrait) -> Vec<(TestSpecificCvssKind, CvssScoreNonZeroCheckResult)> {
    let mut failing_check_results: Vec<(TestSpecificCvssKind, CvssScoreNonZeroCheckResult)> = Vec::new();

    // check if cvss_v2 prop is set
    if let Some(cvss_v2) = content.get_cvss_v2() {
        // deserialize cvss, we only care about result, not errors
        let v2_result = match deserialize_cvss(cvss_v2, "", &mut None) {
            Some(Cvss::V2(v2)) => cvss_v2_has_env_score_zero(v2),
            // TODO: Nondeterminable #409 - deserialized into wrong version
            Some(_) => vec![CvssScoreNonZeroCheckResult::ParsingFailed],
            // TODO: Nondeterminable #409, could not deserialize
            None => vec![CvssScoreNonZeroCheckResult::ParsingFailed],
        };
        for result in v2_result {
            if matches!(
                result,
                CvssScoreNonZeroCheckResult::CalculatedCvssScoreNonZero(_)
                    | CvssScoreNonZeroCheckResult::PropertyCvssScoreNonZero(_)
            ) {
                failing_check_results.push((TestSpecificCvssKind::CvssV2, result));
            }
        }
    }

    // check if the cvss_v3 prop is set
    if let Some(cvss_v3) = content.get_cvss_v3() {
        // deserialize cvss, we only care about result, not errors
        let v3_result = match deserialize_cvss(cvss_v3, "", &mut None) {
            Some(Cvss::V3_0(v3)) | Some(Cvss::V3_1(v3)) => cvss_v3_has_env_score_zero(v3),
            // TODO: Nondeterminable #409, could not deserialize
            None => vec![CvssScoreNonZeroCheckResult::ParsingFailed],
            // TODO: Nondeterminable #409 - deserialized into wrong version
            Some(_) => vec![CvssScoreNonZeroCheckResult::ParsingFailed],
        };
        for result in v3_result {
            if matches!(
                result,
                CvssScoreNonZeroCheckResult::CalculatedCvssScoreNonZero(_)
                    | CvssScoreNonZeroCheckResult::PropertyCvssScoreNonZero(_)
            ) {
                failing_check_results.push((TestSpecificCvssKind::CvssV3, result));
            }
        }
    }

    // check if the cvss_v4 prop is set (CSAF 2.1 only)
    if let Some(cvss_v4) = content.get_cvss_v4() {
        // deserialize cvss, we only care about result, not errors
        let v4_result = match deserialize_cvss(cvss_v4, "", &mut None) {
            Some(Cvss::V4(v4)) => cvss_v4_has_score_zero(v4),
            // TODO: Nondeterminable #409, could not deserialize
            None => vec![CvssScoreNonZeroCheckResult::ParsingFailed],
            // TODO: Nondeterminable #409 - deserialized into wrong version
            Some(_) => vec![CvssScoreNonZeroCheckResult::ParsingFailed],
        };
        for result in v4_result {
            if matches!(
                result,
                CvssScoreNonZeroCheckResult::CalculatedCvssScoreNonZero(_)
                    | CvssScoreNonZeroCheckResult::PropertyCvssScoreNonZero(_)
            ) {
                failing_check_results.push((TestSpecificCvssKind::CvssV4, result));
            }
        }
    }

    failing_check_results
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
                let failing_check_results = failing_cvss_keys(content);

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
    fn calc_error(metric_type: TestSpecificCvssKind, score: f64, content_path: &str) -> Result<(), Vec<TestFinding>> {
        Err(vec![create_cvss_calced_score_nonzero_for_fixed_products_warning(
            "CSAFPID-9080700",
            &metric_type,
            &score,
            content_path,
        )])
    }
    // helper to build the "property" warning for a given metric/score/path combination
    fn prop_error(metric_type: TestSpecificCvssKind, score: f64, content_path: &str) -> Result<(), Vec<TestFinding>> {
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
        metric_type: TestSpecificCvssKind,
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
        let calc = |metric_type: TestSpecificCvssKind, score: f64| {
            calc_error(metric_type, score, "/vulnerabilities/0/scores/0")
        };
        let prop = |metric_type: TestSpecificCvssKind, score: f64| {
            prop_error(metric_type, score, "/vulnerabilities/0/scores/0")
        };
        let both = |metric_type: TestSpecificCvssKind, prop_score: f64, calc_score: f64| {
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
            case_01: calc(TestSpecificCvssKind::CvssV3, 6.5),
            case_02: calc(TestSpecificCvssKind::CvssV3, 4.2),
            case_03: calc(TestSpecificCvssKind::CvssV2, 1.7),
            case_04: calc(TestSpecificCvssKind::CvssV2, 6.8),
            case_05: calc(TestSpecificCvssKind::CvssV3, 6.5),
            case_06: calc(TestSpecificCvssKind::CvssV3, 4.2),
            case_s01: calc(TestSpecificCvssKind::CvssV3, 4.2),
            case_s02: both(TestSpecificCvssKind::CvssV3, 6.5, 6.5),
            case_s03: both(TestSpecificCvssKind::CvssV2, 6.8, 6.8),
            case_s04: both(TestSpecificCvssKind::CvssV3, 6.5, 6.5),
            case_s05: calc(TestSpecificCvssKind::CvssV2, 1.7),
            case_s06: calc(TestSpecificCvssKind::CvssV3, 6.5),
            case_s07: calc(TestSpecificCvssKind::CvssV3, 6.5),
            case_s08: prop(TestSpecificCvssKind::CvssV3, 6.5),
            case_s09: prop(TestSpecificCvssKind::CvssV2, 6.8),
            case_s21: prop(TestSpecificCvssKind::CvssV3, 6.5),
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
        let calc = |metric_type: TestSpecificCvssKind, score: f64| {
            Err(vec![create_cvss_calced_score_nonzero_for_fixed_products_warning(
                "CSAFPID-9080700",
                &metric_type,
                &score,
                "/vulnerabilities/0/metrics/0/content",
            )])
        };
        // helper to build the "property" warning for a given metric/score/path combination
        let prop = |metric_type: TestSpecificCvssKind, score: f64| {
            Err(vec![create_cvss_property_score_nonzero_for_fixed_products_warning(
                "CSAFPID-9080700",
                &metric_type,
                &score,
                "/vulnerabilities/0/metrics/0/content",
            )])
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
        // Case s03: same as case_04 (CVSS v2, no metric that sets to 0), but with an explicit
        //           environmentalScore matching the vector's calculated non-zero score, status fixed
        // Case s04: same as case_05 (CVSS v3.0, no metric that sets to 0), but with an explicit
        //           environmentalScore matching the vector's calculated non-zero score, status first_fixed

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

        TESTS_2_1.test_6_2_19.expect(ExpectedResults_2_1 {
            case_01: calc(TestSpecificCvssKind::CvssV3, 6.5),
            case_02: calc(TestSpecificCvssKind::CvssV3, 4.2),
            case_03: calc(TestSpecificCvssKind::CvssV2, 1.7),
            case_04: calc(TestSpecificCvssKind::CvssV2, 6.8),
            case_05: calc(TestSpecificCvssKind::CvssV3, 6.5),
            case_06: calc(TestSpecificCvssKind::CvssV3, 4.2),
            case_07: calc(TestSpecificCvssKind::CvssV4, 7.3),
            case_08: calc(TestSpecificCvssKind::CvssV4, 1.8),
            case_s01: calc(TestSpecificCvssKind::CvssV3, 4.2),
            case_s02: prop(TestSpecificCvssKind::CvssV3, 6.5),
            case_s03: prop(TestSpecificCvssKind::CvssV2, 6.8),
            case_s04: prop(TestSpecificCvssKind::CvssV3, 6.5),
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
            case_s12: Ok(()),
            case_s13: Ok(()),
            case_s14: Ok(()),
        });
    }
}
