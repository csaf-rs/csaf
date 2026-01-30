use crate::csaf::types::csaf_version_number::{CsafVersionNumber, ValidVersionNumber};
use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait};
use crate::validation::ValidationError;
use crate::csaf::aggregation::csaf_revision_history::validated_revision_history_numbers::ValidatedRevisionHistoryNumbers;

fn create_mixed_versioning_error(
    doc_version: &ValidVersionNumber,
    revision_number: &ValidVersionNumber,
) -> ValidationError {
    ValidationError {
        message: format!(
            "The document version '{doc_version}' and revision history number '{revision_number}' use different versioning schemes"
        ),
        instance_path: "/document/tracking/revision_history/0/number".to_string(),
    }
}

/// 6.1.30 Mixed Integer and Semantic Versioning
///
/// `/document/tracking/version` and `document/tracking/revision_history[]/number` need to use
/// the same versioning scheme (either integer versioning or semantic versioning) across the document.
/// For this test, we take the document version as authoritative for the versioning scheme used in the document.
pub fn test_6_1_30_mixed_integer_and_semantic_versioning(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let tracking = doc.get_document().get_tracking();

    // Get the document version
    let doc_version = match tracking.get_version() {
        CsafVersionNumber::Valid(doc_version) => doc_version,
        CsafVersionNumber::Invalid(err) => {
            // If the document version is invalid, return an error
            return Err(vec![err.get_validation_error("/document/version")]);
        },
    };

    // Get the revision history numbers
    match ValidatedRevisionHistoryNumbers::from(&tracking.get_revision_history()) {
        // If the revision history numbers are invalid for any reason, return the errors
        ValidatedRevisionHistoryNumbers::Invalid(err) => { Err(err.into()) }
        // If the revision history numbers are valid, check if the versioning scheme of
        // the first revision history number matches the document version.
        ValidatedRevisionHistoryNumbers::Valid(numbers) => {
            let first = numbers.first().unwrap_or_else(||
                unreachable!("At this point, the revision history numbers should contain at least one element, with all numbers being valid and of the same schema"));
            match (&first, &doc_version) {
                (ValidVersionNumber::IntVer(_), ValidVersionNumber::SemVer(_)) |
                (ValidVersionNumber::SemVer(_), ValidVersionNumber::IntVer(_)) => {
                    // If the versioning scheme of the first revision history number does not match the document version, add an error for the first revision history item and skip this test
                    Err(vec![create_mixed_versioning_error(&doc_version, first)])
                }
                _ => { Ok(()) }
            }
        }
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_30
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_30_mixed_integer_and_semantic_versioning(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_30
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_30_mixed_integer_and_semantic_versioning(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use std::str::FromStr;

    #[test]
    fn test_test_6_1_30() {
        let case_01 = Err(vec![create_mixed_versioning_error(
            &ValidVersionNumber::from_str("2").unwrap(),
            &ValidVersionNumber::from_str("1.0.0").unwrap(),
        )]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0.test_6_1_30.expect(case_01.clone(), Ok(()));
        TESTS_2_1.test_6_1_30.expect(case_01, Ok(()));
    }
}

