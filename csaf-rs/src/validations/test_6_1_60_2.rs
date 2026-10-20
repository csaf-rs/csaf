use crate::csaf_traits::{CsafTrait, ExtensionsTrait};
use crate::validation::{TestFinding, TestFindingData};

const EXTENSION_SCHEMA_WARNING: &str = "There is currently no supported extension schema defined for CSAF v2.1";

/// 6.1.60.2 Extension Schema
///
/// For each item in an element of type $['$defs'].extensions_t it SHALL be tested that the item is valid against the declared CSAF Extension Schema.
/// CSAF Extensions not supported by the implementation SHALL result in a warning which SHALL include the value of $schema of the extension.
/// Such warning SHALL differentiate between the different classes of extensions.
pub fn test_6_1_60_2_extension_schema(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let mut warnings: Option<Vec<TestFinding>> = None;
    doc.visit_all_extensions(&mut |extensions, base_path| {
        if !extensions.get_raw().is_empty() {
            warnings
                .get_or_insert_default()
                .push(extension_schema_warning(base_path));
        }
    });

    warnings.map_or(Ok(()), Err)
}

fn extension_schema_warning(instance_path: &str) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: EXTENSION_SCHEMA_WARNING.to_string(),
        instance_path: instance_path.to_string(),
    })
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_60_2, test_6_1_60_2_extension_schema);

#[cfg(test)]
mod tests {
    use crate::{
        csaf2_1::testcases::{ExpectedResults_6_1_60_2 as ExpectedResults, TESTS_2_1},
        validations::test_6_1_60_2::extension_schema_warning,
    };

    #[test]
    fn test_6_1_60_1() {
        let expected = Err(vec![extension_schema_warning("/x_extensions")]);

        TESTS_2_1.test_6_1_60_2.expect(ExpectedResults {
            case_01: expected.clone(),
            case_02: expected.clone(),
            case_11: expected.clone(),
            case_12: expected,
        });
    }
}
