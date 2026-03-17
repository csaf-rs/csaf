use crate::csaf::types::version_number::CsafVersionNumber;
use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionTrait, TrackingTrait};
use crate::validation::ValidationError;
use std::mem::discriminant;

fn create_mixed_versioning_error(
    doc_version: &CsafVersionNumber,
    revision_number: &CsafVersionNumber,
    revision_index: &usize,
) -> ValidationError {
    ValidationError {
        message: format!(
            "The document version '{doc_version}' and revision history number '{revision_number}' use different versioning schemes"
        ),
        instance_path: format!("/document/tracking/revision_history/{revision_index}/number"),
    }
}

/// 6.1.30 Mixed Integer and Semantic Versioning
///
/// `/document/tracking/version` and `document/tracking/revision_history[]/number` need to use
/// the same versioning scheme (either integer versioning or semantic versioning) across the document.
/// For this test, we take the document version as authoritative for the versioning scheme used in the document.
pub fn test_6_1_30_mixed_integer_and_semantic_versioning(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let tracking = doc.get_document().get_tracking();

    let doc_version = tracking.get_version();
    let doc_version_discriminant = discriminant(&doc_version);

    let mut errors: Option<Vec<ValidationError>> = None;
    for (revision_index, revision) in tracking.get_revision_history().iter().enumerate() {
        let number = revision.get_number();

        if doc_version_discriminant != discriminant(&number) {
            errors
                .get_or_insert_default()
                .push(create_mixed_versioning_error(&doc_version, &number, &revision_index));
        }
    }

    errors.map_or(Ok(()), Err)
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

    #[test]
    fn test_test_6_1_30() {
        let case_01 = Err(vec![create_mixed_versioning_error(
            &CsafVersionNumber::from("2"),
            &CsafVersionNumber::from("1.0.0"),
            &0,
        )]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0.test_6_1_30.expect(case_01.clone(), Ok(()));
        TESTS_2_1.test_6_1_30.expect(case_01, Ok(()));
    }
}
