use super::schema::{CommonSecurityAdvisoryFramework, ProductIdT, ProductTree};

pub fn validate_document(doc: &CommonSecurityAdvisoryFramework) {
    println!("Validating document");

    println!("Executing Test 6.1.1...");
    test_6_01_01_missing_definition_of_product_id(doc);
}

pub fn test_6_01_01_missing_definition_of_product_id(doc: &CommonSecurityAdvisoryFramework) {
    let mut ids = Vec::<ProductIdT>::new();

    let product_tree = doc.product_tree.as_ref();
    if let Some(x) = product_tree {

        //  /product_tree/product_groups[]/product_ids[]
        ids.extend(x.product_groups.iter().flat_map(|x| x.product_ids.clone()));

        // /product_tree/relationships[]/product_reference
        ids.extend(x.relationships.iter().map(|x|x.product_reference.clone()));

        //   /product_tree/relationships[]/relates_to_product_reference
        ids.extend(x.relationships.iter().map(|x|x.product_reference.clone()));
    }

    println!("Product references: {:?}", ids);
}


#[cfg(test)]
mod tests {
    use crate::csaf::loader::load_document;
    use crate::csaf::validation::test_6_01_01_missing_definition_of_product_id;

    #[test]
    fn it_works() {
        let result = load_document("../csaf/csaf_2.0/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_0-2021-6-1-01-01.json").unwrap();
        assert_eq!(result.document.title.as_str(), "Mandatory test: Missing Definition of Product ID (failing example 1)")
    }
}
