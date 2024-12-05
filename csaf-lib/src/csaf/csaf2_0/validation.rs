use super::product_helper::*;
use super::schema::CommonSecurityAdvisoryFramework;
use crate::csaf::validation::Validate;
use std::collections::{HashMap, HashSet};

impl Validate for CommonSecurityAdvisoryFramework {
    fn validate(&self) {
        println!("Validating document... \n");

        println!("Executing Test 6.1.1... ");

        let _ = match test_6_01_01_missing_definition_of_product_id(self) {
            Ok(()) => println!("> Test Success"),
            Err(e) => println!("> Error: {}", e),
        };

        println!("Executing Test 6.1.2... ");

        let _ = match test_6_01_02_multiple_definition_of_product_id(self) {
            Ok(()) => println!("> Test Success"),
            Err(e) => println!("> Error: {}", e),
        };
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

fn find_duplicates<T: std::hash::Hash + Eq + Clone>(vec: Vec<T>) -> Vec<T> {
    let mut occurrences = HashMap::new();
    let mut duplicates = Vec::new();

    for item in vec.iter() {
        let count = occurrences.entry(item.clone()).or_insert(0);
        *count += 1;
    }

    for (item, count) in occurrences {
        if count > 1 {
            duplicates.push(item);
        }
    }

    duplicates
}

#[cfg(test)]
mod tests {
    use crate::csaf::csaf2_0::validation::test_6_01_02_multiple_definition_of_product_id;
    use crate::csaf::csaf2_0::{
        loader::load_document, validation::test_6_01_01_missing_definition_of_product_id,
    };

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
            Err(String::from(
                "Duplicate definitions: [ProductIdT(\"CSAFPID-9080700\")]"
            ))
        )
    }
}
