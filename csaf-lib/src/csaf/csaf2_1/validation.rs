use super::product_helper::*;
use super::schema::CommonSecurityAdvisoryFramework;
use crate::csaf::helpers::find_duplicates;
use crate::csaf::validation::{test_6_01_35_contradicting_remediations, Test, Validatable, ValidationPreset};
use std::collections::{HashMap, HashSet};

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

pub fn test_6_01_01_missing_definition_of_product_id(
    doc: &CommonSecurityAdvisoryFramework,
) -> Result<(), String> {
    let definitions = HashSet::from_iter(gather_product_definitions(doc).iter().copied());
    let references = gather_product_references(&doc);

    let mut missing = references.difference(&definitions).collect::<Vec<_>>();
    missing.sort();

    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!("Missing definitions: {:?}", missing))
    }
}

pub fn test_6_01_02_multiple_definition_of_product_id(
    doc: &CommonSecurityAdvisoryFramework,
) -> Result<(), String> {
    let definitions = gather_product_definitions(doc);
    let duplicates = find_duplicates(definitions);

    if duplicates.is_empty() {
        Ok(())
    } else {
        Err(format!("Duplicate definitions: {:?}", duplicates))
    }
}

pub fn test_6_01_34_branches_recursion_depth(
    doc: &CommonSecurityAdvisoryFramework,
) -> Result<(), String> {
    let depth = if let Some(x) = doc.product_tree.as_ref() {
        count_branch_depth_tree(x)
    } else {
        0
    };

    if depth < 30 {
        Ok(())
    } else {
        Err(format!("Recursion depth too big: {:?}", depth))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::csaf::csaf2_1::{
        loader::load_document, validation::test_6_01_01_missing_definition_of_product_id,
        validation::test_6_01_02_multiple_definition_of_product_id,
        validation::test_6_01_34_branches_recursion_depth,
    };
    use crate::csaf::validation::test_6_01_35_contradicting_remediations;

    #[test]
    fn test_test_6_01_01() {
        let doc = load_document("../csaf/csaf_2.1/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_1-2024-6-1-01-01.json").unwrap();
        assert_eq!(
            test_6_01_01_missing_definition_of_product_id(&doc),
            Err(String::from("Missing definitions: [ProductIdT(\"CSAFPID-9080700\"), ProductIdT(\"CSAFPID-9080701\")]"))
        )
    }

    #[test]
    fn test_test_6_01_02() {
        let doc = load_document("../csaf/csaf_2.1/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_1-2024-6-1-02-01.json").unwrap();
        assert_eq!(
            test_6_01_02_multiple_definition_of_product_id(&doc),
            Err(String::from(
                "Duplicate definitions: [ProductIdT(\"CSAFPID-9080700\")]"
            ))
        )
    }

    #[test]
    fn test_test_6_01_34() {
        let doc = load_document("../csaf/csaf_2.1/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_1-2024-6-1-34-01.json").unwrap();
        assert_eq!(
            test_6_01_34_branches_recursion_depth(&doc),
            Err(String::from("Recursion depth too big: 31"))
        )
    }

    #[test]
    fn test_test_6_01_35() {
        for x in ["11", "12", "13", "14"].iter() {
            let doc = load_document(format!("../csaf/csaf_2.1/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_1-2024-6-1-35-{}.json", x).as_str()).unwrap();
            assert_eq!(
                test_6_01_35_contradicting_remediations(&doc),
                Ok(())
            )
        }
        HashMap::from([
            ("01", "Product CSAFPID-9080700 has contradicting remediations: no_fix_planned and vendor_fix"),
            ("02", "Product CSAFPID-9080700 has contradicting remediations: no_fix_planned and vendor_fix"),
            ("03", "Product CSAFPID-9080700 has contradicting remediations: no_fix_planned and vendor_fix"),
            ("04", "Product CSAFPID-9080700 has contradicting remediations: no_fix_planned and vendor_fix"),
        ]).iter().for_each(|(x, err)| {
            let doc = load_document(format!("../csaf/csaf_2.1/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_1-2024-6-1-35-{}.json", x).as_str()).unwrap();
            assert_eq!(
                test_6_01_35_contradicting_remediations(&doc),
                Err(format!("{}", err))
            )
        });
    }
}
