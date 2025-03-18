use super::schema::CommonSecurityAdvisoryFramework;
use crate::csaf::validation::{test_6_01_01_missing_definition_of_product_id, test_6_01_02_multiple_definition_of_product_id, Test, Validatable, ValidationPreset};
use std::collections::HashMap;

impl Validatable<CommonSecurityAdvisoryFramework> for CommonSecurityAdvisoryFramework {
    fn presets(&self) -> HashMap<ValidationPreset, Vec<&str>> {
        let basic_tests = Vec::from(["6.1.1", "6.1.2"]);
        // More tests may be added in extend() here later
        let extended_tests: Vec<&str> = basic_tests.clone();
        // extended_tests.extend(["foo"].iter());
        let full_tests: Vec<&str> = extended_tests.clone();
        // full_tests.extend(["bar"].iter());
        HashMap::from([
            (ValidationPreset::Basic, basic_tests),
            (ValidationPreset::Extended, extended_tests),
            (ValidationPreset::Full, full_tests),
        ])
    }

    fn tests(&self) -> HashMap<&str, Test<CommonSecurityAdvisoryFramework>> {
        type CsafTest = Test<CommonSecurityAdvisoryFramework>;
        HashMap::from([
            ("6.1.1", test_6_01_01_missing_definition_of_product_id as CsafTest),
            ("6.1.2", test_6_01_02_multiple_definition_of_product_id as CsafTest),
        ])
    }

    fn doc(&self) -> &CommonSecurityAdvisoryFramework {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::csaf::csaf2_0::loader::load_document;
    use crate::csaf::validation::{test_6_01_01_missing_definition_of_product_id, test_6_01_02_multiple_definition_of_product_id};

    #[test]
    fn test_test_6_01_01() {
        let doc = load_document("../csaf/csaf_2.0/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_0-2021-6-1-01-01.json").unwrap();
        assert_eq!(
            test_6_01_01_missing_definition_of_product_id(&doc),
            Err(String::from("Missing definitions: [\"CSAFPID-9080700\", \"CSAFPID-9080701\"]"))
        )
    }

    #[test]
    fn test_test_6_01_02() {
        let doc = load_document("../csaf/csaf_2.0/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_0-2021-6-1-02-01.json").unwrap();
        assert_eq!(
            test_6_01_02_multiple_definition_of_product_id(&doc),
            Err(String::from(
                "Duplicate definitions: [\"CSAFPID-9080700\"]"
            ))
        )
    }
}
