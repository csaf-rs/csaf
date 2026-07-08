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

    // 2. Visit standard nested branch layouts
    product_tree.visit_all_products(&mut |product, instance_path| {
        let product_id = product.get_product_id();

        if let Some(helper) = product.get_product_identification_helper() {
            let has_serial = helper.get_serial_numbers().map_or(false, |sn: Vec<_>| !sn.is_empty());
            let has_model = helper.get_model_numbers().map_or(false, |mn: Vec<_>| !mn.is_empty());

            if has_serial || has_model {
                if !valid_path_references.contains(product_id) {
                    errors.push(generate_hardware_software_mix_error(product_id, instance_path));
                }
            }
        }
    });

    // 3. Fall back to checking full_product_names array items
    for (idx, product) in product_tree.get_full_product_names().iter().enumerate() {
        let product_id = product.get_product_id();

        if let Some(helper) = product.get_product_identification_helper() {
            let has_serial = helper.get_serial_numbers().map_or(false, |sn: Vec<_>| !sn.is_empty());
            let has_model = helper.get_model_numbers().map_or(false, |mn: Vec<_>| !mn.is_empty());

            if has_serial || has_model {
                if !valid_path_references.contains(product_id) {
                    let instance_path = format!("/product_tree/full_product_names/{idx}");
                    errors.push(generate_hardware_software_mix_error(product_id, &instance_path));
                }
            }
        }
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_2_31, test_6_2_31_hardware_software_mix);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_31() {
        // Case 01: Baseline failure from OASIS specification
        let case_01 = vec![generate_hardware_software_mix_error(
            "CSAFPID-908070601",
            "/product_tree/branches/0/branches/0/branches/0/product",
        )];

        // S01: Edge cases for 6.2.31
        let s01 = vec![
            // 1. Missing Product Path: Fails because the branch structure does not define a valid product path
            generate_hardware_software_mix_error("CSAFPID-908070601", "/product_tree/branches/0/branches/0/product"),
        ];

        // Case 01: Invalid - Product has serial numbers but no matching path entry or inline declaration
        // Case S01: Supplementary edge case from csaf-rs_csaf-csaf_2_1-6-2-31-s01.json (Missing Product Path)
        // Case 11: Valid - Product has hardware tags but is fully anchored by a product path mapping
        // Case 12: Valid - Product has no identification helper field configuration
        // Case 13: Valid - Pure software components without model or serial signatures

        // Sequence matches macro definition: 01, s01, 11, 12, 13
        TESTS_2_1
            .test_6_2_31
            .expect(Err(case_01), Err(s01), Ok(()), Ok(()), Ok(()));
    }
}
