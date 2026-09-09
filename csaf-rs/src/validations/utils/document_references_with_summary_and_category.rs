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
/// - If there is no reference with summary starting with `required_summary_prefix`: a single
///   missing-reference error.
/// - If at least one reference with summary starting with `required_summary_prefix` is found
///   but one has the wrong category: wrong-category errors for those with wrong category.
///
/// Returns `None` if the check passes, or `Some` with a list of [`TestFindingData`]s otherwise.
pub(crate) fn check_references_with_summary_prefix_and_category<Ref: ReferenceTrait>(
    references: Option<&[Ref]>,
    required_summary_prefix: &str,
    required_category: &CategoryOfReference,
    doc_category: &CsafDocumentCategory,
) -> Option<Vec<TestFindingData>> {
    let mut errors: Option<Vec<TestFindingData>> = None;
    let mut matching_indices = Vec::new();

    // filter references for required summary prefix and category
    // collect correct summary prefix, wrong category errors
    if let Some(references) = references {
        for (i_r, reference) in references.iter().enumerate() {
            if reference.get_summary().starts_with(required_summary_prefix) {
                if reference.get_category() != *required_category {
                    errors.get_or_insert_default().push(create_incorrect_category_data(
                        required_summary_prefix,
                        &reference.get_category(),
                        required_category,
                        doc_category,
                        i_r,
                    ));
                }
                matching_indices.push(i_r);
            }
        }
    }

    // If no reference with the required summary prefix is found, report missing reference.
    // If at least one is found, report only the category errors (if any).
    if matching_indices.is_empty() {
        return Some(vec![create_missing_reference_data(
            required_summary_prefix,
            required_category,
            doc_category,
        )]);
    }

    errors
}
