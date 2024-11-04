use crate::csaf::product_helper::*;
use crate::csaf::schema::CommonSecurityAdvisoryFramework;
use std::collections::HashSet;

pub fn validate_document(doc: &CommonSecurityAdvisoryFramework) {
    println!("Validating document... \n");

    println!("Executing Test 6.1.1... ");

    let _ = match test_6_01_01_missing_definition_of_product_id(doc) {
        Ok(()) => println!("> Test Success"),
        Err(e) => println!("> Error: {}", e)
    };
}

pub fn test_6_01_01_missing_definition_of_product_id(doc: &CommonSecurityAdvisoryFramework) -> Result<(), String> {
    let definitions = HashSet::from_iter(gather_product_definitions(doc).iter().copied());
    let references = gather_product_references(&doc);

    let missing =references.difference(&definitions).collect::<Vec<_>>();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!("Missing definitions: {:?}", missing))
    }
}

pub fn test_6_01_02_multiple_definition_of_product_id(doc: &CommonSecurityAdvisoryFramework) -> Result<(), String> {
    let mut definitions = gather_product_definitions(doc);

    definitions.sort_unstable();
    definitions.dedup();

    if definitions.is_empty() {
        Ok(())
    } else {
        Err(format!("Duplicate definitions: {:?}", definitions))
    }
}

#[cfg(test)]
mod tests {
    use crate::csaf::{loader::load_document, validation::test_6_01_01_missing_definition_of_product_id};
    use crate::csaf::validation::test_6_01_02_multiple_definition_of_product_id;

    #[test]
    fn test_test_6_01_01() {
        let doc = load_document("../csaf/csaf_2.0/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_0-2021-6-1-01-01.json").unwrap();
        assert_eq!(
            test_6_01_01_missing_definition_of_product_id(&doc),
            Err(String::from("Missing definitions: [ProductIdT(\"CSAFPID-9080700\"), ProductIdT(\"CSAFPID-9080701\")]"))
        )
    }

    #[test]
    fn test_test_6_01_02() {
        let doc = load_document("../csaf/csaf_2.0/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_0-2021-6-1-02-01.json").unwrap();
        assert_eq!(
            test_6_01_02_multiple_definition_of_product_id(&doc),
            Err(String::from("Duplicate definitions: [ProductIdT(\"CSAFPID-9080700\")]"))
        )
    }
}
