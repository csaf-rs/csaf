use crate::validations::utils::cvss::CvssScoreNonZeroCheckResult;
use crate::validations::utils::cvss::check_nonzero::utils::check_cvss_score_non_zero;
use crate::validations::utils::cvss::deserialize_vector::deserialize_vector;
use crate::validations::utils::cvss::merge_vector_with_json::{merge_v2, merge_v3, merge_v4};
use cvss_rs::Version;
use cvss_rs::v2_0::CvssV2;
use cvss_rs::v3::CvssV3;
use cvss_rs::v4_0::CvssV4;

/// Checks if a CVSS v2 score has an environmental score of 0 in the property or
/// calculated from the merged properties of vector and JSON.
pub(crate) fn cvss_v2_has_env_score_zero(cvss_v2: CvssV2) -> Vec<CvssScoreNonZeroCheckResult> {
    check_cvss_score_non_zero(cvss_v2.environmental_score, || {
        let parsed = deserialize_vector::<CvssV2>(&cvss_v2.vector_string, Version::V2, "").map_err(|_| ())?;
        Ok(merge_v2(parsed, cvss_v2).0.calculated_environmental_score())
    })
}

/// Checks if a CVSS v2 score has an environmental score of 0 in the property or
/// calculated from the merged properties of vector and JSON.
pub(crate) fn cvss_v3_has_env_score_zero(cvss_v3: CvssV3) -> Vec<CvssScoreNonZeroCheckResult> {
    check_cvss_score_non_zero(cvss_v3.environmental_score, || {
        let parsed = deserialize_vector::<CvssV3>(&cvss_v3.vector_string, Version::V3_0, "").map_err(|_| ())?;
        Ok(merge_v3(parsed, cvss_v3).0.calculated_environmental_score())
    })
}

/// Checks if a CVSS v4 score has an overall score of 0, either in the property or
/// calculated from the merged properties of vector and JSON.
pub(crate) fn cvss_v4_has_score_zero(cvss_v4: CvssV4) -> Vec<CvssScoreNonZeroCheckResult> {
    check_cvss_score_non_zero(Some(cvss_v4.base_score), || {
        let parsed = deserialize_vector::<CvssV4>(&cvss_v4.vector_string, Version::V4, "").map_err(|_| ())?;
        Ok(merge_v4(cvss_v4, parsed).0.calculated_score().map(|(score, _)| score))
    })
}
