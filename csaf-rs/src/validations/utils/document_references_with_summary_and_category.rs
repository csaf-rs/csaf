use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::ReferenceTrait;
use crate::schema::csaf2_1::schema::CategoryOfReference;
use crate::validation::TestFindingData;

pub(crate) fn create_missing_reference_data(
    required_summary_prefix: &str,
    required_category: &CategoryOfReference,
    document_category: &CsafDocumentCategory,
) -> TestFindingData {
    TestFindingData {
        message: format!(
            "The document does not contain a reference with summary starting with `{required_summary_prefix}` and category `{required_category}` which is required for documents with category `{document_category}`"
        ),
        instance_path: "/document/references".to_string(),
    }
}

pub(crate) fn create_incorrect_category_data(
    required_summary_prefix: &str,
    wrong_category: &CategoryOfReference,
    required_category: &CategoryOfReference,
    doc_category: &CsafDocumentCategory,
    reference_index: usize,
) -> TestFindingData {
    TestFindingData {
        message: format!(
            "The document contains a reference with summary starting with `{required_summary_prefix}`, but it uses the wrong category `{wrong_category}` for documents with category `{doc_category}` (should be `{required_category}`)."
        ),
        instance_path: format!("/document/references/{reference_index}"),
    }
}

/// Checks that at least one document reference exists with a summary starting with the given
/// `required_summary_prefix` and `required_category` among the provided `references`, in the
/// context of a document with `doc_category`. `doc_category` is forwarded to error generation
/// functions and not used in the error detection logic.
///
/// The following findings are reported in that order / prioritization:
/// - If there is at least one reference with both the required summary prefix AND required
///   category: no error.
/// - If at least one reference with the required summary prefix is found but none have the
///   required category: wrong-category errors for those with wrong category.
/// - If there is no reference with the required summary prefix: a single missing-reference error.
///
/// Returns `None` if the check passes, or `Some` with a list of [`TestFindingData`]s otherwise.
pub(crate) fn check_references_with_summary_prefix_and_category<Ref: ReferenceTrait>(
    references: Option<&[Ref]>,
    required_summary_prefix: &str,
    required_category: &CategoryOfReference,
    doc_category: &CsafDocumentCategory,
) -> Option<Vec<TestFindingData>> {
    let mut wrong_category_errors = Vec::new();
    let mut has_correct_match = false;

    if let Some(references) = references {
        for (i_r, reference) in references.iter().enumerate() {
            if reference.get_summary().starts_with(required_summary_prefix) {
                if reference.get_category() == *required_category {
                    has_correct_match = true;
                } else {
                    wrong_category_errors.push(create_incorrect_category_data(
                        required_summary_prefix,
                        &reference.get_category(),
                        required_category,
                        doc_category,
                        i_r,
                    ));
                }
            }
        }
    }

    // a reference with both correct prefix and correct category was found
    if has_correct_match {
        return None;
    }

    // references with matching prefix but wrong category were found, report those errors
    if !wrong_category_errors.is_empty() {
        return Some(wrong_category_errors);
    }

    // no references with matching prefix at all, report missing reference
    Some(vec![create_missing_reference_data(
        required_summary_prefix,
        required_category,
        doc_category,
    )])
}
