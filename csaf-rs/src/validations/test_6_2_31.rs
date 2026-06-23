use crate::csaf_traits::{CsafTrait, ProductTreeTrait, ProductTrait, ProductPathTrait};
use crate::csaf::traits::vulnerabilities::product_ident_helper_trait::ProductIdentificationHelperTrait;
use crate::validation::ValidationError;

fn generate_hardware_software_mix_error(product_id: &str) -> ValidationError {
    ValidationError {
        message: format!(
            "Product '{product_id}' contains serial_numbers or model_numbers but lacks a valid product path. This indicates a potential hardware and software mix in the product tree."
        ),
        instance_path: format!("/product_tree/.../product/id='{product_id}'"),
    }
}

/// Test 6.2.31: Hardware and Software
pub fn test_6_2_31_hardware_and_software(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let Some(product_tree) = doc.get_product_tree() else {
        return Ok(());
    };

    // 1. Gather all string references from the product paths cleanly
    let mut valid_path_references: Vec<&str> = Vec::new();
    let paths = product_tree.get_product_paths();

    for path in paths {
        let beg_ref = path.get_beginning_product_reference();
        valid_path_references.push(beg_ref);

        let full_prod = path.get_full_product_name();
        valid_path_references.push(full_prod.get_product_id());

        for sub_ref in path.get_subpath_product_references() {
            valid_path_references.push(sub_ref);
        }
    }

    let mut errors: Vec<ValidationError> = vec![];

    // 2. Use the built-in visitor pattern to safely check all full products across the entire tree
    product_tree.visit_all_products_generic(&mut |product, _instance_path| {
        if let Some(helper) = product.get_product_identification_helper() {
            // now check if serial_nubers or model_numbers exist
            if helper.get_serial_numbers().is_some() || helper.get_model_numbers().is_some() {
                let product_id = product.get_product_id();
                // if product_id is not in valid_path_references add an error
                if !valid_path_references.contains(&product_id) {
                    errors.push(generate_hardware_software_mix_error(product_id));
                }
            }
        }
    });

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_31,
    test_6_2_31_hardware_and_software
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_31() {
        let case_01_failing_example_1 = Err(vec![
            generate_hardware_software_mix_error("CSAFPID-908070601"),
        ]);

        let case_11_valid_example_1 = Ok(());
        let case_12_valid_example_2 = Ok(());
        let case_13_valid_example_3 = Ok(());

        TESTS_2_1.test_6_2_31.expect(
            case_01_failing_example_1,
            case_11_valid_example_1,
            case_12_valid_example_2,
            case_13_valid_example_3,
        );
    }
}