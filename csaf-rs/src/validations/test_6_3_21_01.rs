use crate::csaf_traits::{CsafTrait, ExtensionsTrait};
use crate::validation::{TestFinding, TestFindingData};
use serde_json::Value;

fn essential_extension_finding(base_path: &str, extension_index: usize) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: "The extension category is essential. A reader that does not understand this extension might miss information that is essential to understand the content of the CSAF document.".to_string(),
        instance_path: format!("{base_path}/{extension_index}/category"),
    })
}

/// 6.3.21.1 Extension Category Essential
///
/// For each item in an element of type `$['$defs'].extensions_t` it SHALL be tested that
/// the extension `category` of the item is not `essential`.
pub fn test_6_3_21_1_extension_category_essential(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let mut findings: Option<Vec<TestFinding>> = None;

    doc.visit_all_extensions(&mut |extensions, base_path| {
        for (extension_index, extension) in extensions.get_raw().iter().enumerate() {
            if matches!(extension.get("category"), Some(Value::String(category)) if category == "essential") {
                findings
                    .get_or_insert_default()
                    .push(essential_extension_finding(base_path, extension_index));
            }
        }
    });

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_3_21_1,
    test_6_3_21_1_extension_category_essential
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_3_21_1 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_21_1() {
        let essential_category_error = Err(vec![essential_extension_finding("/x_extensions", 0)]);

        // Case 01: essential category in the root extension, extension prop critical is false
        // Case 02: essential category in the root extension, extension prop critical is true
        // Case 11: supplementary category in the root extension, extension prop critical is false
        // Case 12: significant category in the root extension, extension prop critical is true

        TESTS_2_1.test_6_3_21_1.expect(ExpectedResults {
            case_01: essential_category_error.clone(),
            case_02: essential_category_error,
            case_11: Ok(()),
            case_12: Ok(()),
        });
    }
}
