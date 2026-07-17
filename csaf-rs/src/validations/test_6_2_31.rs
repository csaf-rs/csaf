use crate::csaf::traits::product_tree::product_path_trait::ProductPathTrait;
use crate::csaf::traits::product_tree_trait::BranchTrait;
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

fn check_product<P: ProductTrait>(
    product: &P,
    instance_path: &str,
    valid_path_references: &HashSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let product_id = product.get_product_id();

    if let Some(helper) = product.get_product_identification_helper() {
        let has_serial = helper.get_serial_numbers().is_some_and(|sn| !sn.is_empty());
        let has_model = helper.get_model_numbers().is_some_and(|mn| !mn.is_empty());

        // If it has hardware ID but is NOT in the paths registry -> Error
        if (has_serial || has_model) && !valid_path_references.contains(product_id) {
            errors.push(generate_hardware_software_mix_error(product_id, instance_path));
        }
    }
}

/// Test 6.2.31: Hardware and Software Mix
pub fn test_6_2_31_hardware_software_mix(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let Some(product_tree) = doc.get_product_tree() else {
        return Ok(());
    };
    let mut errors: Vec<ValidationError> = vec![];

    let mut valid_path_references = HashSet::new();

    for path in product_tree.get_product_paths() {
        valid_path_references.insert(path.get_beginning_product_reference().to_owned());

        for next in path.get_subpath_product_references() {
            valid_path_references.insert(next.to_owned());
        }
    }

    // Check Branches
    product_tree.visit_all_branches(&mut |branch, path| {
        if let Some(product) = branch.get_product() {
            check_product(product, &format!("{path}/product"), &valid_path_references, &mut errors);
        }
    });

    // Check Full Product Names
    for (i, fpn) in product_tree.get_full_product_names().iter().enumerate() {
        check_product(
            fpn,
            &format!("/product_tree/full_product_names/{i}"),
            &valid_path_references,
            &mut errors,
        );
    }

    // Check Product Paths anchors
    let prefix = product_tree.get_product_path_prefix();
    for (i, rel) in product_tree.get_product_paths().iter().enumerate() {
        check_product(
            rel.get_full_product_name(),
            &format!("{prefix}/{i}/full_product_name"),
            &valid_path_references,
            &mut errors,
        );
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

        // S01: Edge case for 6.2.31
        // These are the 6 failing cases defined in your updated JSON
        let s01_errors = vec![
            generate_hardware_software_mix_error("CSAFPID-B1", "/product_tree/branches/0/product"),
            generate_hardware_software_mix_error("CSAFPID-B2", "/product_tree/branches/1/product"),
            generate_hardware_software_mix_error("CSAFPID-F1", "/product_tree/full_product_names/0"),
            generate_hardware_software_mix_error("CSAFPID-F2", "/product_tree/full_product_names/1"),
            generate_hardware_software_mix_error("CSAFPID-F3", "/product_tree/full_product_names/2"),
            generate_hardware_software_mix_error("CSAFPID-F4", "/product_tree/full_product_names/3"),
            generate_hardware_software_mix_error("CSAFPID-F5", "/product_tree/product_paths/0/full_product_name"),
        ];

        // Case 01: Invalid - Product has serial numbers but no matching path entry or inline declaration
        // Case S01: Supplementary edge case from csaf-rs_csaf-csaf_2_1-6-2-31-s01.json (Missing Product Path)
        // Case 11: Valid - Product has hardware tags but is fully anchored by a product path mapping
        // Case 12: Valid - Product has no identification helper field configuration
        // Case 13: Valid - Pure software components without model or serial signatures

        // Sequence matches macro definition: 01, s01, 11, 12, 13
        TESTS_2_1
            .test_6_2_31
            .expect(Err(case_01), Err(s01_errors), Ok(()), Ok(()), Ok(()));
    }
}
