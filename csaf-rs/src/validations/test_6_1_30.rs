use crate::csaf::types::version_number::CsafVersionNumber;
use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionHistorySortable, TrackingTrait};
use crate::validation::ValidationError;
use std::mem::discriminant;

fn create_mixed_versioning_error(
    doc_version: &CsafVersionNumber,
    revision_version: &CsafVersionNumber,
) -> ValidationError {
    ValidationError {
        message: format!(
            "The document version '{doc_version}' and revision history number '{revision_version}' use different versioning schemes"
        ),
        instance_path: "/document/tracking/version".to_string(),
    }
}

fn create_mixed_versioning_within_history_error(
    first_version: &CsafVersionNumber,
    second_version: &CsafVersionNumber,
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

fn get_version_type_name(v: &CsafVersionNumber) -> &'static str {
    match v {
        CsafVersionNumber::SemVer(_) => "semantic versioning",
        CsafVersionNumber::IntVer(_) => "integer versioning",
    }
}

/// 6.1.30 Mixed Integer and Semantic Versioning
///
/// `/document/tracking/version` and `document/tracking/revision_history[]/number` need to use
/// the same versioning scheme (either integer versioning or semantic versioning) across the document.
pub fn test_6_1_30_mixed_integer_and_semantic_versioning(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    // make sure revision history is consistent in itself
    let mut tuples = doc.get_document().get_tracking().get_revision_history_tuples();
    tuples.inplace_sort_by_date_then_number();
    let mut errors: Option<Vec<ValidationError>> = None;
    let mut previous_version: Option<(CsafVersionNumber, usize)> = None;
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
                    &current_version.path_index,
                ));
        }
        previous_version = Some((current_number, i_v));
    }

    // now make sure revision history matches document versioning
    let doc_version = doc.get_document().get_tracking().get_version();

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
            &CsafVersionNumber::from("1.0.0"),
            &CsafVersionNumber::from("2"),
            &1,
        )]);
        let case_intver_then_semver_in_history = Err(vec![create_mixed_versioning_within_history_error(
            &CsafVersionNumber::from("1"),
            &CsafVersionNumber::from("2.0.0"),
            &1,
        )]);
        let case_intver_history_semver_document = Err(vec![create_mixed_versioning_error(
            &CsafVersionNumber::from("3.0.0"),
            &CsafVersionNumber::from("2"),
        )]);
        let case_semver_history_intver_document = Err(vec![create_mixed_versioning_error(
            &CsafVersionNumber::from("3"),
            &CsafVersionNumber::from("2.0.0"),
        )]);

        let case_unordered_intver_semver_in_history_semver_in_document = Err(vec![
            create_mixed_versioning_within_history_error(
                &CsafVersionNumber::from("1"),
                &CsafVersionNumber::from("2.0.0"),
                &2,
            ),
            create_mixed_versioning_within_history_error(
                &CsafVersionNumber::from("2.0.0"),
                &CsafVersionNumber::from("3"),
                &0,
            ),
        ]);

        let case_intver_then_semver_then_intver_in_history_semver_in_document = Err(vec![
            create_mixed_versioning_within_history_error(
                &CsafVersionNumber::from("1"),
                &CsafVersionNumber::from("2.0.0"),
                &1,
            ),
            create_mixed_versioning_within_history_error(
                &CsafVersionNumber::from("2.0.0"),
                &CsafVersionNumber::from("3"),
                &2,
            ),
            create_mixed_versioning_error(&CsafVersionNumber::from("4.0.0"), &CsafVersionNumber::from("3")),
        ]);

        TESTS_2_0.test_6_1_30.expect(
            case_semver_then_intver_in_history.clone(),
            case_intver_then_semver_in_history.clone(),
            case_intver_history_semver_document.clone(),
            case_semver_history_intver_document.clone(),
            case_intver_then_semver_then_intver_in_history_semver_in_document.clone(),
            case_unordered_intver_semver_in_history_semver_in_document.clone(),
            Ok(()), // only semver versioning
            Ok(()), // only intver versioning
        );
        TESTS_2_1.test_6_1_30.expect(
            case_semver_then_intver_in_history,
            case_intver_then_semver_in_history,
            case_intver_history_semver_document,
            case_semver_history_intver_document,
            case_intver_then_semver_then_intver_in_history_semver_in_document,
            case_unordered_intver_semver_in_history_semver_in_document,
            Ok(()), // only semver versioning
            Ok(()), // only intver versioning
        );
    }
}
