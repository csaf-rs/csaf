use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::validation::{TestFinding, TestFindingData};
use std::sync::LazyLock;

static DOCUMENT_EXTENSION_INFO: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Information(TestFindingData {
        message: "The document contains a CSAF Extension.".to_string(),
        instance_path: "/document/x_extensions".to_string(),
    })
});

/// 6.3.21.3 Usage of Extension at Document Level
///
/// It SHALL be tested that the element `$.document.x_extensions` does not exist.
pub fn test_6_3_21_3_usage_of_extension_at_document_level(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    if doc.get_document().get_extensions().is_some() {
        Err(vec![DOCUMENT_EXTENSION_INFO.clone()])
    } else {
        Ok(())
    }
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_3_21_3,
    test_6_3_21_3_usage_of_extension_at_document_level
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_3_21_3 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_21_3() {
        // Case 11: extension at root level, but none at document level
        TESTS_2_1.test_6_3_21_3.expect(ExpectedResults {
            case_01: Err(vec![DOCUMENT_EXTENSION_INFO.clone()]),
            case_11: Ok(()),
        });
    }
}
