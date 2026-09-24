use crate::{
    csaf::types::language::CsafLanguage,
    csaf_traits::{CsafTrait, DocumentTrait, NoteTrait, WithOptionalGroupIds, WithOptionalProductIds},
    schema::csaf2_1::schema::NoteCategory,
    validation::{TestFinding, TestFindingData},
    validations::utils::language_specific_translations::{
        create_no_translation_known_info, get_translation_for_term_product_description,
    },
};

fn create_missing_product_reference_in_description(note_idx: usize) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: String::from("Product description is missing a product reference via product id or group id."),
        instance_path: format!("/document/notes/{note_idx}"),
    })
}

// TODO: Define this in translation files see #1111
const PRODUCT_DESCRIPTION: &str = "Product Description";

/// 6.2.40 Product Description without Product Reference
///
/// The test checks that each description of a product contains a reference to a product with
/// `group_ids` or `product_ids`.
pub fn test_6_2_40_product_description_without_product_reference(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let product_description_term = match doc.get_document().get_lang() {
        Some(CsafLanguage::Invalid(_, _)) => return Ok(()), // TODO: Where are invalid languages handled?
        Some(CsafLanguage::Valid(lang)) if lang.is_english() => PRODUCT_DESCRIPTION,
        Some(CsafLanguage::Valid(lang)) => {
            if let Some(term) = get_translation_for_term_product_description(lang.primary_language()) {
                term
            } else {
                // If the translation is unknown it will be for all occurrences. Reporting for each
                // individual note does not make sense and is time inefficient
                return Err(vec![create_no_translation_known_info(
                    PRODUCT_DESCRIPTION,
                    &lang,
                    "/document/notes/*/title",
                )]);
            }
        },
        None => PRODUCT_DESCRIPTION, // Assume default English
    };

    // TODO: Refactor if #1112 chooses to make this access easier
    let Some(notes) = doc.get_document().get_notes() else {
        return Ok(()); // No notes to check
    };

    let findings = notes
        .iter()
        .enumerate()
        .filter(|(_, note)| {
            note.get_category() == NoteCategory::Description
                && note
                    .get_title()
                    .is_some_and(|title| title.contains(product_description_term))
        })
        .filter(|(_, note)| note.get_group_ids().is_none() && note.get_product_ids().is_none())
        .map(|(note_idx, _)| create_missing_product_reference_in_description(note_idx))
        .collect::<Vec<_>>();

    if findings.is_empty() { Ok(()) } else { Err(findings) }
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_40,
    test_6_2_40_product_description_without_product_reference
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_40 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_40() {
        let simple_title_product_description_with_missing_reference =
            Err(vec![create_missing_product_reference_in_description(0)]);
        let title_containing_term_is_product_description_with_missing_reference =
            Err(vec![create_missing_product_reference_in_description(0)]);
        let title_containing_german_term_is_product_description_with_missing_reference =
            Err(vec![create_missing_product_reference_in_description(0)]);

        TESTS_2_1.test_6_2_40.expect(ExpectedResults {
            case_01: simple_title_product_description_with_missing_reference,
            case_02: title_containing_term_is_product_description_with_missing_reference,
            case_03: title_containing_german_term_is_product_description_with_missing_reference,
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
            // Test if group IDs also don't cause an error to be emitted
            case_s11: Ok(()),
        });
    }
}
