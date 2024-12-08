use super::product_helper::*;
use super::schema::CommonSecurityAdvisoryFramework;
use crate::csaf::helpers::find_duplicates;
use crate::csaf::validation::{Test, Validatable, ValidationProfile};
use std::collections::{HashMap, HashSet};

impl Validatable<CommonSecurityAdvisoryFramework> for CommonSecurityAdvisoryFramework {
    fn profiles(&self) -> HashMap<ValidationProfile, Vec<&str>> {
        HashMap::from([
            (
                ValidationProfile::Basic,
                Vec::from(["6.1.1", "6.1.2", "6.1.34"]),
            ),
            (ValidationProfile::Extended, Vec::from(["6.1.1", "6.1.2"])),
            (ValidationProfile::Full, Vec::from(["6.1.1", "6.1.2"])),
        ])
    }

    fn tests(&self) -> HashMap<&str, Test<CommonSecurityAdvisoryFramework>> {
        HashMap::<&str, Test<CommonSecurityAdvisoryFramework>>::from([
            ("6.1.1", test_6_01_01_missing_definition_of_product_id),
            ("6.1.2", test_6_01_02_multiple_definition_of_product_id),
            ("6.1.34", test_6_01_34_branches_recursion_depth),
        ]
            as [(&str, Test<CommonSecurityAdvisoryFramework>); 3])
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
    use crate::csaf::csaf2_1::{
        loader::load_document, validation::test_6_01_01_missing_definition_of_product_id,
        validation::test_6_01_02_multiple_definition_of_product_id,
        validation::test_6_01_34_branches_recursion_depth,
    };

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
}
