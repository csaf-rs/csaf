use crate::csaf::schema::ProductsT;

use super::schema::{CommonSecurityAdvisoryFramework, ProductIdT};

pub fn validate_document(doc: &CommonSecurityAdvisoryFramework) {
    println!("Validating document");

    println!("Executing Test 6.1.1...");
    test_6_01_01_missing_definition_of_product_id(doc);
}

pub fn test_6_01_01_missing_definition_of_product_id(doc: &CommonSecurityAdvisoryFramework) {
    let mut ids = Vec::<&ProductIdT>::new();

    if let Some(x) = doc.product_tree.as_ref() {
        //  /product_tree/product_groups[]/product_ids[]
        ids.extend(x.product_groups.iter().flat_map(|x| &x.product_ids));

        // /product_tree/relationships[]/product_reference
        ids.extend(x.relationships.iter().map(|x| &x.product_reference));

        //   /product_tree/relationships[]/relates_to_product_reference
        ids.extend(x.relationships.iter().map(|x| &x.product_reference));
    }

    for vuln in doc.vulnerabilities.iter() {
        if let Some(x) = vuln.product_status.as_ref() {
            ids.extend(x.first_affected.as_ref().unwrap().iter());
            ids.extend(x.first_fixed.as_ref().unwrap().iter());
            ids.extend(x.fixed.as_ref().unwrap().iter());
            ids.extend(x.known_affected.as_ref().unwrap().iter());
            ids.extend(x.last_affected.as_ref().unwrap().iter());
            ids.extend(x.recommended.as_ref().unwrap().iter());
            ids.extend(x.under_investigation.as_ref().unwrap().iter());
        }
    }

    println!("Product references: {:?}", ids);
}

#[cfg(test)]
mod tests {
    use crate::csaf::{loader::load_document, validation::test_6_01_01_missing_definition_of_product_id};
    
    #[test]
    fn it_works() {
        let doc = load_document("../csaf/csaf_2.0/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_0-2021-6-1-01-01.json").unwrap();
        assert_eq!(
            doc.document.title.as_str(),
            "Mandatory test: Missing Definition of Product ID (failing example 1)"
        );

        test_6_01_01_missing_definition_of_product_id(&doc);
    }
}
