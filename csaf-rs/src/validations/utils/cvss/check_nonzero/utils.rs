use crate::validations::utils::cvss::CvssScoreNonZeroCheckResult;

/// Returns true if a score is zero when truncated to CVSS's one-decimal precision.
pub fn is_zero_score(actual: f64) -> bool {
    !actual.is_finite() || (actual * 10.0).trunc() as i64 == 0
}

/// Checks a CVSS score and a score calculated from the merged vector/json for a non-zero score,
/// returning a vector of check results.
pub fn check_cvss_score_non_zero(
    property_score: Option<f64>,
    calculated_score: Result<Option<f64>, ()>,
) -> Vec<CvssScoreNonZeroCheckResult> {
    let mut results = Vec::new();

    // check if the property score is zero, if it exists
    if let Some(score) = property_score {
        results.push(if is_zero_score(score) {
            CvssScoreNonZeroCheckResult::PropertyCvssScoreIsZero
        } else {
            CvssScoreNonZeroCheckResult::PropertyCvssScoreNonZero(score)
        });
    }

    let calculated_score = match calculated_score {
        Ok(score) => score,
        Err(_) => {
            results.push(CvssScoreNonZeroCheckResult::ParsingFailed);
            return results;
        },
    };

    // check if the calculated score is zero, if it was calculated
    match calculated_score {
        Some(score) if is_zero_score(score) => {
            results.push(CvssScoreNonZeroCheckResult::CalculatedCvssScoreIsZero);
        },
        Some(score) => {
            results.push(CvssScoreNonZeroCheckResult::CalculatedCvssScoreNonZero(score));
        },
        None => results.push(CvssScoreNonZeroCheckResult::CalculationFailed),
    }

    results
}

#[cfg(test)]
mod tests {
    use super::is_zero_score;

    #[test]
    fn zero_score_uses_truncation_at_one_decimal_precision() {
        assert!(is_zero_score(0.0));
        assert!(is_zero_score(0.09));
        assert!(!is_zero_score(0.1));
    }

    #[test]
    fn non_finite_scores_are_zero() {
        assert!(is_zero_score(f64::NAN));
        assert!(is_zero_score(f64::INFINITY));
        assert!(is_zero_score(f64::NEG_INFINITY));
    }
}
