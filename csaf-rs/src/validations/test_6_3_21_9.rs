use crate::csaf_traits::CsafTrait;
use crate::validation::{TestFinding, TestFindingData};
use std::sync::LazyLock;

static ROOT_EXTENSION_INFO: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Information(TestFindingData {
        message: "The document uses a CSAF Extension.".to_string(),
        instance_path: "/x_extensions".to_string(),
    })
});

/// 6.3.21.9 Usage of Extension at Root Level
///
/// It SHALL be tested that the element `$.x_extensions` does not exist.
pub fn test_6_3_21_9_usage_of_extension_at_root_level(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    if doc.get_extensions().is_some() {
        Err(vec![ROOT_EXTENSION_INFO.clone()])
    } else {
        Ok(())
    }
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_3_21_9,
    test_6_3_21_9_usage_of_extension_at_root_level
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_3_21_9 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_21_9() {
        // Case 11: extension at document level, but none at root
        TESTS_2_1.test_6_3_21_9.expect(ExpectedResults {
            case_01: Err(vec![ROOT_EXTENSION_INFO.clone()]),
            case_11: Ok(()),
        });
    }
}
