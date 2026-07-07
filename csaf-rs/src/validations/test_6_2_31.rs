use crate::csaf::traits::vulnerabilities::product_ident_helper_trait::ProductIdentificationHelperTrait;
use crate::csaf_traits::{CsafTrait, ProductTrait, ProductTreeTrait};
use crate::validation::ValidationError;
use std::collections::HashSet;

fn generate_hardware_software_mix_error(product_id: &str, base_path: &str) -> ValidationError {
    ValidationError {
        message: format!(
            "Product '{product_id}' contains serial_numbers or model_numbers but lacks a valid product path. This indicates a potential hardware and software mix in the product tree."
        ),
        instance_path: base_path.to_string(),
    }
}

/// Test 6.2.31: Hardware and Software Mix
pub fn test_6_2_31_hardware_software_mix(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let Some(product_tree) = doc.get_product_tree() else {
        return Ok(());
    };

    let mut errors: Vec<ValidationError> = vec![];

    // 1. Gather all legitimate string references from groups and relationship paths
    let mut valid_path_references: HashSet<String> = HashSet::new();

    for (id, _) in product_tree.get_relationships_product_references() {
        valid_path_references.insert(id);
    }

    // 2. Iterate using the version-specific product visitor pattern
    product_tree.visit_all_products(&mut |product, instance_path| {
        let product_id = product.get_product_id();

        if let Some(helper) = product.get_product_identification_helper() {
            let has_serial = helper.get_serial_numbers().map_or(false, |sn: Vec<_>| !sn.is_empty());
            let has_model = helper.get_model_numbers().map_or(false, |mn: Vec<_>| !mn.is_empty());

            if has_serial || has_model {
                // If it claims hardware components but isn't anchored by a relationship or group reference target
                if !valid_path_references.contains(product_id) {
                    errors.push(generate_hardware_software_mix_error(product_id, instance_path));
                }
            }
        }
    });

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_2_31, test_6_2_31_hardware_software_mix);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_31() {
        // Case 01: Invalid - Product has serial numbers but no matching path entry or inline declaration
        let case_01_errors = vec![generate_hardware_software_mix_error(
            "CSAFPID-908070601",
            "/product_tree/branches/0/branches/0/branches/0/product",
        )];

        TESTS_2_1.test_6_2_31.expect(
            Err(case_01_errors),
            // Case 11: Valid - Product has hardware tags but is fully anchored by a product path mapping
            Ok(()),
            // Case 12: Valid - Product has no identification helper field configuration
            Ok(()),
            // Case 13: Valid - Pure software components without model or serial signatures
            Ok(()),
        );
    }

    #[test]
    fn test_test_6_2_31_edge_cases() {
        use crate::schema::csaf2_1::schema::{
            CommonSecurityAdvisoryFramework, DocumentLevelMetaData, FullProductNameT, HelperToIdentifyTheProduct,
            JsonSchema, ModelNumber, ProductIdT, ProductTree, SerialNumber, TextualDescriptionOfTheProduct,
        };
        use std::str::FromStr;

        // Case 01: Product tree missing entirely (should pass cleanly without crashing)
        let doc01 = CommonSecurityAdvisoryFramework {
            schema: JsonSchema::HttpsDocsOasisOpenOrgCsafCsafV21SchemaCsafJson,
            product_tree: None,
            document: unsafe { std::mem::transmute([0u8; std::mem::size_of::<DocumentLevelMetaData>()]) },
            vulnerabilities: vec![],
            x_extensions: None,
        };

        // Case 02: Product attempts to implicitly self-validate without external references
        let standalone_product = FullProductNameT {
            product_id: "CSAFPID-SELF-VAL".parse::<ProductIdT>().unwrap(),
            name: "Isolated Component".parse::<TextualDescriptionOfTheProduct>().unwrap(),
            product_identification_helper: Some(HelperToIdentifyTheProduct {
                serial_numbers: Some(vec![
                    SerialNumber::from_str("SN-12345").expect("Could not parse SerialNumber"),
                ]),
                model_numbers: Some(vec![
                    ModelNumber::from_str("MOD-99X").expect("Could not parse ModelNumber"),
                ]),
                ..Default::default()
            }),
            x_extensions: None,
        };

        let doc02 = CommonSecurityAdvisoryFramework {
            schema: JsonSchema::HttpsDocsOasisOpenOrgCsafCsafV21SchemaCsafJson,
            product_tree: Some(ProductTree {
                full_product_names: vec![standalone_product],
                ..Default::default()
            }),
            document: unsafe { std::mem::transmute([0u8; std::mem::size_of::<DocumentLevelMetaData>()]) },
            vulnerabilities: vec![],
            x_extensions: None,
        };

        // Case 03: Pure model_numbers code branch validation coverage
        let model_only_product = FullProductNameT {
            product_id: "CSAFPID-MODEL-ONLY".parse::<ProductIdT>().unwrap(),
            name: "Model Target Component"
                .parse::<TextualDescriptionOfTheProduct>()
                .unwrap(),
            product_identification_helper: Some(HelperToIdentifyTheProduct {
                model_numbers: Some(vec![
                    ModelNumber::from_str("MOD-ABCDE").expect("Could not parse ModelNumber"),
                ]),
                ..Default::default()
            }),
            x_extensions: None,
        };

        let doc03 = CommonSecurityAdvisoryFramework {
            schema: JsonSchema::HttpsDocsOasisOpenOrgCsafCsafV21SchemaCsafJson,
            product_tree: Some(ProductTree {
                full_product_names: vec![model_only_product],
                ..Default::default()
            }),
            document: unsafe { std::mem::transmute([0u8; std::mem::size_of::<DocumentLevelMetaData>()]) },
            vulnerabilities: vec![],
            x_extensions: None,
        };

        // Case 04: Hardware and software helper fields present concurrently in one declaration block
        let mixed_helpers_product = FullProductNameT {
            product_id: "CSAFPID-SIMULTANEOUS".parse::<ProductIdT>().unwrap(),
            name: "Simultaneous Component"
                .parse::<TextualDescriptionOfTheProduct>()
                .unwrap(),
            product_identification_helper: Some(HelperToIdentifyTheProduct {
                serial_numbers: Some(vec![
                    SerialNumber::from_str("SN-99999").expect("Could not parse SerialNumber"),
                ]),
                model_numbers: Some(vec![
                    ModelNumber::from_str("MOD-8888").expect("Could not parse ModelNumber"),
                ]),
                ..Default::default()
            }),
            x_extensions: None,
        };

        let doc04 = CommonSecurityAdvisoryFramework {
            schema: JsonSchema::HttpsDocsOasisOpenOrgCsafCsafV21SchemaCsafJson,
            product_tree: Some(ProductTree {
                full_product_names: vec![mixed_helpers_product],
                ..Default::default()
            }),
            document: unsafe { std::mem::transmute([0u8; std::mem::size_of::<DocumentLevelMetaData>()]) },
            vulnerabilities: vec![],
            x_extensions: None,
        };

        // Case 05: Multiple individual invalid products present concurrently to test error accumulation
        let bad_prod_a = FullProductNameT {
            product_id: "CSAFPID-MULTIPLE-A".parse::<ProductIdT>().unwrap(),
            name: "Bad Item A".parse::<TextualDescriptionOfTheProduct>().unwrap(),
            product_identification_helper: Some(HelperToIdentifyTheProduct {
                serial_numbers: Some(vec![
                    SerialNumber::from_str("SN-A").expect("Could not parse SerialNumber"),
                ]),
                ..Default::default()
            }),
            x_extensions: None,
        };

        let bad_prod_b = FullProductNameT {
            product_id: "CSAFPID-MULTIPLE-B".parse::<ProductIdT>().unwrap(),
            name: "Bad Item B".parse::<TextualDescriptionOfTheProduct>().unwrap(),
            product_identification_helper: Some(HelperToIdentifyTheProduct {
                model_numbers: Some(vec![
                    ModelNumber::from_str("MOD-B").expect("Could not parse ModelNumber"),
                ]),
                ..Default::default()
            }),
            x_extensions: None,
        };

        let doc05 = CommonSecurityAdvisoryFramework {
            schema: JsonSchema::HttpsDocsOasisOpenOrgCsafCsafV21SchemaCsafJson,
            product_tree: Some(ProductTree {
                full_product_names: vec![bad_prod_a, bad_prod_b],
                ..Default::default()
            }),
            document: unsafe { std::mem::transmute([0u8; std::mem::size_of::<DocumentLevelMetaData>()]) },
            vulnerabilities: vec![],
            x_extensions: None,
        };

        test_6_2_31_hardware_software_mix(&doc01).expect("Case 01 failed: Missing product tree should pass cleanly");
        test_6_2_31_hardware_software_mix(&doc02)
            .err()
            .expect("Case 02 failed: Product should flag an error and not self-validate");
        test_6_2_31_hardware_software_mix(&doc03)
            .err()
            .expect("Case 03 failed: Pure model_numbers branch execution path should be hit and evaluated");
        test_6_2_31_hardware_software_mix(&doc04)
            .err()
            .expect("Case 04 failed: Simultaneous presence of serial and model fields must be processed safely");
        let errors_case05 = test_6_2_31_hardware_software_mix(&doc05)
            .err()
            .expect("Case 05 failed: Multiple validation errors should be captured");
        assert_eq!(
            errors_case05.len(),
            2,
            "Case 05 failed: Engine should iterate completely and accumulate both product violations"
        );
    }
}
