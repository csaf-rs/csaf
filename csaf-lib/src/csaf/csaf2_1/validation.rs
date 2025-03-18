use super::schema::CommonSecurityAdvisoryFramework;
use crate::csaf::validation::{test_6_01_01_missing_definition_of_product_id, test_6_01_02_multiple_definition_of_product_id, test_6_01_34_branches_recursion_depth, test_6_01_35_contradicting_remediations, Test, Validatable, ValidationPreset};
use std::collections::HashMap;

impl Validatable<CommonSecurityAdvisoryFramework> for CommonSecurityAdvisoryFramework {
    fn presets(&self) -> HashMap<ValidationPreset, Vec<&str>> {
        let basic_tests = Vec::from(["6.1.1", "6.1.2", "6.1.34", "6.1.35"]);
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
            ("6.1.34", test_6_01_34_branches_recursion_depth as CsafTest),
            ("6.1.35", test_6_01_35_contradicting_remediations as CsafTest),
        ])
    }

    fn doc(&self) -> &CommonSecurityAdvisoryFramework {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::csaf::csaf2_1::loader::load_document;
    use crate::csaf::validation::{test_6_01_01_missing_definition_of_product_id, test_6_01_02_multiple_definition_of_product_id, test_6_01_34_branches_recursion_depth, test_6_01_35_contradicting_remediations};

    #[test]
    fn test_test_6_01_01() {
        let doc = load_document("../csaf/csaf_2.1/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_1-2024-6-1-01-01.json").unwrap();
        assert_eq!(
            test_6_01_01_missing_definition_of_product_id(&doc),
            Err(String::from("Missing definitions: [\"CSAFPID-9080700\", \"CSAFPID-9080701\"]"))
        )
    }

    #[test]
    fn test_test_6_01_02() {
        let doc = load_document("../csaf/csaf_2.1/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_1-2024-6-1-02-01.json").unwrap();
        assert_eq!(
            test_6_01_02_multiple_definition_of_product_id(&doc),
            Err(String::from(
                "Duplicate definitions: [\"CSAFPID-9080700\"]"
            ))
        )
    }

    #[test]
    fn test_test_6_01_34() {
        for x in ["11"].iter() {
            let doc = load_document(format!("../csaf/csaf_2.1/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_1-2024-6-1-34-{}.json", x).as_str()).unwrap();
            assert_eq!(
                Ok(()),
                test_6_01_35_contradicting_remediations(&doc)
            )
        }
        for (x, err) in [
            ("01", "Branches recursion depth too big (> 30)"),
            ("02", "Branches recursion depth too big (> 30)"),
        ].iter() {
            let doc = load_document(format!("../csaf/csaf_2.1/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_1-2024-6-1-34-{}.json", x).as_str()).unwrap();
            assert_eq!(
                Err(format!("{}", err)),
                test_6_01_34_branches_recursion_depth(&doc)
            )
        }
    }

    #[test]
    fn test_test_6_01_35() {
        for x in ["11", "12", "13", "14"].iter() {
            let doc = load_document(format!("../csaf/csaf_2.1/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_1-2024-6-1-35-{}.json", x).as_str()).unwrap();
            assert_eq!(
                Ok(()),
                test_6_01_35_contradicting_remediations(&doc)
            )
        }
        for (x, err) in [
            ("01", "Product CSAFPID-9080700 has contradicting remediations: no_fix_planned and vendor_fix"),
            ("02", "Product CSAFPID-9080700 has contradicting remediations: none_available and mitigation"),
            ("03", "Product CSAFPID-9080702 has contradicting remediations: workaround, fix_planned and optional_patch"),
            ("04", "Product CSAFPID-9080701 has contradicting remediations: mitigation, fix_planned and optional_patch"),
        ].iter() {
            let doc = load_document(format!("../csaf/csaf_2.1/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_1-2024-6-1-35-{}.json", x).as_str()).unwrap();
            assert_eq!(
                Err(format!("{}", err)),
                test_6_01_35_contradicting_remediations(&doc)
            )
        }
    }
}
