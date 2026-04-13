use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait};
use crate::validation::ValidationError;

fn create_tracking_id_in_title_error(title: &str, tracking_id: &str) -> ValidationError {
    ValidationError {
        message: format!("The document title '{title}' contains the document tracking id '{tracking_id}'."),
        instance_path: "/document/title".to_string(),
    }
}

/// 6.2.22 Document Tracking ID in Title
///
/// It MUST be tested that the /document/title does not contain the /document/tracking/id.
pub fn test_6_2_22_document_tracking_id_in_title(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();
    let title = document.get_title();
    let tracking_id = document.get_tracking().get_id();

    if title.contains(tracking_id.as_str()) {
        return Err(vec![create_tracking_id_in_title_error(title, tracking_id)]);
    }

    Ok(())
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_22,
    test_6_2_22_document_tracking_id_in_title
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_22() {
        let case_01_starts_with_id_colon_as_sep = Err(vec![create_tracking_id_in_title_error(
            "OASIS_CSAF_TC-CSAF_2.1-2024-6-2-22-01: Recommended Test: Document Tracking ID in Title (failing example 1)",
            "OASIS_CSAF_TC-CSAF_2.1-2024-6-2-22-01",
        )]);
        let case_02_ends_with_id_in_parenthesis = Err(vec![create_tracking_id_in_title_error(
            "Recommended Test: Document Tracking ID in Title (failing example 2) (OASIS_CSAF_TC-CSAF_2.1-2024-6-2-22-02)",
            "OASIS_CSAF_TC-CSAF_2.1-2024-6-2-22-02",
        )]);
        let case_03_ends_with_id_colon_sep = Err(vec![create_tracking_id_in_title_error(
            "Recommended Test: Document Tracking ID in Title (failing example 3) - OASIS_CSAF_TC-CSAF_2.1-2024-6-2-22-03",
            "OASIS_CSAF_TC-CSAF_2.1-2024-6-2-22-03",
        )]);
        let case_04_starts_with_id_dash_sep = Err(vec![create_tracking_id_in_title_error(
            "OASIS_CSAF_TC-CSAF_2.1-2024-6-2-22-04 - Recommended Test: Document Tracking ID in Title (failing example 4)",
            "OASIS_CSAF_TC-CSAF_2.1-2024-6-2-22-04",
        )]);

        // Case 11: title does not contain its own tracking ID
        // Case 12: title contains a different tracking ID (not its own)

        TESTS_2_1.test_6_2_22.expect(
            case_01_starts_with_id_colon_as_sep,
            case_02_ends_with_id_in_parenthesis,
            case_03_ends_with_id_colon_sep,
            case_04_starts_with_id_dash_sep,
            Ok(()),
            Ok(()),
        );
    }
}
