use crate::csaf::types::csaf_version_number::{CsafVersionNumber, ValidVersionNumber};
use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionHistorySortable, TrackingTrait};
use crate::validation::ValidationError;
use std::mem::discriminant;

fn create_mixed_versioning_error(
    doc_version: &ValidVersionNumber,
    revision_version: &ValidVersionNumber,
) -> ValidationError {
    ValidationError {
        message: format!(
            "The document version '{doc_version}' and revision history number '{revision_version}' use different versioning schemes"
        ),
        instance_path: "/document/tracking/version".to_string(),
    }
}

fn create_mixed_versioning_within_history_error(
    first_version: &ValidVersionNumber,
    second_version: &ValidVersionNumber,
    revision_index: &usize,
) -> ValidationError {
    let first_version_type = get_version_type_name(first_version);
    let second_version_type = get_version_type_name(second_version);
    ValidationError {
        message: format!(
            "The versioning started with {first_version_type} ('{first_version}') and switched to {second_version_type} ('{second_version}')"
        ),
        instance_path: format!("/document/tracking/revision_history/{revision_index}/number"),
    }
}

fn get_version_type_name(v: &ValidVersionNumber) -> &'static str {
    match v {
        ValidVersionNumber::SemVer(_) => "semantic versioning",
        ValidVersionNumber::IntVer(_) => "integer versioning",
    }
}

/// 6.1.30 Mixed Integer and Semantic Versioning
///
/// `/document/tracking/version` and `document/tracking/revision_history[]/number` need to use
/// the same versioning scheme (either integer versioning or semantic versioning) across the document.
/// For this test, we take the document version as authoritative for the versioning scheme used in the document.
pub fn test_6_1_30_mixed_integer_and_semantic_versioning(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    // make sure revision history is consistent in itself
    let mut tuples = doc.get_document().get_tracking().get_revision_history_tuples();
    tuples.inplace_sort_by_date_then_number();
    let mut errors: Option<Vec<ValidationError>> = None;
    let mut previous_version: Option<(ValidVersionNumber, usize)> = None;
    for (i_v, current_version) in tuples.iter().enumerate() {
        let current_number = current_version.number.clone();
        let current_type = discriminant(&current_number);

        if let Some((previous_number, _)) = previous_version
            && discriminant(&previous_number) != current_type
        {
            errors
                .get_or_insert_default()
                .push(create_mixed_versioning_within_history_error(
                    &previous_number,
                    &current_number,
                    &i_v,
                ));
        }
        previous_version = Some((current_number, i_v));
    }

    // now make sure revision history matches document versioning
    let tracking = doc.get_document().get_tracking();
    let doc_version = match tracking.get_version() {
        CsafVersionNumber::Valid(doc_version) => doc_version,
        CsafVersionNumber::Invalid(err) => {
            return Err(vec![err.get_validation_error("/document/version")]);
            // ToDo generate warning https://github.com/csaf-rs/csaf/issues/409
        },
    };
    if let Some((last_history_revision_number, _)) = previous_version
        && discriminant(&last_history_revision_number) != discriminant(&doc_version)
    {
        errors.get_or_insert_default().push(create_mixed_versioning_error(
            &doc_version,
            &last_history_revision_number,
        ));
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
        let case_semver_then_intver_in_history = Err(vec![create_mixed_versioning_within_history_error(
            &ValidVersionNumber::from_str("1.0.0").unwrap(),
            &ValidVersionNumber::from_str("2").unwrap(),
            &1,
        )]);
        let case_intver_then_semver_in_history = Err(vec![create_mixed_versioning_within_history_error(
            &ValidVersionNumber::from_str("1").unwrap(),
            &ValidVersionNumber::from_str("2.0.0").unwrap(),
            &1,
        )]);
        let case_intver_history_semver_document = Err(vec![create_mixed_versioning_error(
            &ValidVersionNumber::from_str("3.0.0").unwrap(),
            &ValidVersionNumber::from_str("2").unwrap(),
        )]);
        let case_semver_history_intver_document = Err(vec![create_mixed_versioning_error(
            &ValidVersionNumber::from_str("3").unwrap(),
            &ValidVersionNumber::from_str("2.0.0").unwrap(),
        )]);

        let case_intver_then_semver_then_intver_in_history_semver_in_document = Err(vec![
            create_mixed_versioning_within_history_error(
                &ValidVersionNumber::from_str("1").unwrap(),
                &ValidVersionNumber::from_str("2.0.0").unwrap(),
                &1,
            ),
            create_mixed_versioning_within_history_error(
                &ValidVersionNumber::from_str("2.0.0").unwrap(),
                &ValidVersionNumber::from_str("3").unwrap(),
                &2,
            ),
            create_mixed_versioning_error(
                &ValidVersionNumber::from_str("4.0.0").unwrap(),
                &ValidVersionNumber::from_str("3").unwrap(),
            ),
        ]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0.test_6_1_30.expect(
            case_semver_then_intver_in_history.clone(),
            case_intver_then_semver_in_history.clone(),
            case_intver_history_semver_document.clone(),
            case_semver_history_intver_document.clone(),
            case_intver_then_semver_then_intver_in_history_semver_in_document.clone(),
            Ok(()), // only semver versioning
            Ok(()), // only intver versioning
        );
        TESTS_2_1.test_6_1_30.expect(
            case_semver_then_intver_in_history,
            case_intver_then_semver_in_history,
            case_intver_history_semver_document,
            case_semver_history_intver_document,
            case_intver_then_semver_then_intver_in_history_semver_in_document,
            Ok(()), // only semver versioning
            Ok(()), // only intver versioning
        );
    }
}
