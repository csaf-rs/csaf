use serde_json::Value;

use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::schema::csaf2_1::schema::CategoryOfReference;
use crate::validation::TestFindingData;
use crate::validations::utils::raw_json_value_helper::property_string_value_is;

pub(crate) fn create_missing_reference_data(
    required_summary_prefix: &str,
    required_category: &CategoryOfReference,
    document_category: &CsafDocumentCategory,
) -> TestFindingData {
    TestFindingData {
        message: format!(
            "`{document_category}` must have at least one document reference with a summary starting with `{required_summary_prefix}` and category `{required_category}`"
        ),
        instance_path: "/document/references".to_string(),
    }
}

pub(crate) fn create_invalid_category_data(
    required_summary_prefix: &str,
    category: Option<&Value>,
    required_category: &CategoryOfReference,
    reference_index: usize,
) -> TestFindingData {
    let actual_category = match category {
        None => "not present".to_string(),
        Some(Value::String(category)) => format!("`{category}`"),
        Some(category) => format!("`{category}`"),
    };
    TestFindingData {
        message: format!(
            "The category of the `{required_summary_prefix}` reference must be present and contain the value `{required_category}` (was: {actual_category})."
        ),
        instance_path: format!("/document/references/{reference_index}"),
    }
}

/// Checks that at least one document reference has a summary starting with
/// `required_summary_prefix` and reports each matching reference whose category
/// is absent or not equal to `required_category`, in the
/// context of a document with `doc_category`. `doc_category` is forwarded to error generation
/// functions and not used in the error detection logic.
///
/// Raw references are used so an omitted `category` can be distinguished from
/// a category supplied by schema deserialization defaults.
///
/// The following findings are reported in that order / prioritization:
/// - If at least one reference with the required summary prefix is found but some have the
///   required category: wrong-category errors for those with wrong category.
/// - If there is no reference with the required summary prefix: a single missing-reference error.
/// - If there is at least one reference with both the required summary prefix AND all such
///   references have the required category: no findings.
///
/// Returns `None` if the check passes, or `Some` with a list of [`TestFindingData`]s otherwise.
pub(crate) fn check_references_with_summary_prefix_and_category(
    references: Option<&[Value]>,
    required_summary_prefix: &str,
    required_category: &CategoryOfReference,
    doc_category: &CsafDocumentCategory,
) -> Option<Vec<TestFindingData>> {
    let mut matching_reference_found = false;
    let mut findings = Vec::new();

    // if there are references
    if let Some(references) = references {
        // iterate each reference
        for (reference_index, reference) in references.iter().enumerate() {
            // if the reference summary does not start with the required prefix, continue
            if !matches!(
                reference.pointer("/summary"),
                Some(Value::String(summary)) if summary.starts_with(required_summary_prefix)
            ) {
                continue;
            }

            // a matching reference was found
            matching_reference_found = true;
            // check the category, it needs to be present and have the required value
            if !property_string_value_is("/category", &required_category.to_string(), reference) {
                findings.push(create_invalid_category_data(
                    required_summary_prefix,
                    reference.pointer("/category"),
                    required_category,
                    reference_index,
                ));
            }
        }
    }

    // there were wrong-category findings
    if !findings.is_empty() {
        Some(findings)
    }
    // there were no matching references
    else if !matching_reference_found {
        Some(vec![create_missing_reference_data(
            required_summary_prefix,
            required_category,
            doc_category,
        )])
    }
    // all matching references have the required category
    else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
    use rstest::rstest;

    #[test]
    fn create_missing_reference_data_uses_specification_wording() {
        assert_eq!(
            create_missing_reference_data(
                "Superseding Document",
                &CategoryOfReference::External,
                &CsafDocumentCategory::CsafSuperseded,
            )
            .message,
            "`csaf_superseded` must have at least one document reference with a summary starting with `Superseding Document` and category `external`"
        );
    }

    #[test]
    fn create_invalid_category_data_reports_self_category() {
        assert_eq!(
            create_invalid_category_data(
                "Superseding Document",
                Some(&Value::String("self".to_string())),
                &CategoryOfReference::External,
                0,
            )
            .message,
            "The category of the `Superseding Document` reference must be present and contain the value `external` (was: `self`)."
        );
    }

    #[rstest]
    #[case(None, "not present")]
    #[case(Some(Value::Null), "`null`")]
    #[case(Some(Value::String("self".to_string())), "`self`")]
    #[case(Some(Value::Bool(true)), "`true`")]
    #[case(Some(Value::Number(42.into())), "`42`")]
    #[case(Some(Value::Array(vec![])), "`[]`")]
    #[case(Some(Value::Object(Default::default())), "`{}`")]
    fn create_invalid_category_data_reports_actual_category(
        #[case] category: Option<Value>,
        #[case] expected_actual_category: &str,
    ) {
        let expected_category = CategoryOfReference::External;

        assert!(
            create_invalid_category_data("Superseding Document", category.as_ref(), &expected_category, 0)
                .message
                .contains(&format!("was: {expected_actual_category}"))
        );
    }
}
