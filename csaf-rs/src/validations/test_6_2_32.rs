use crate::csaf::traits::vulnerabilities::{
    file_hash_trait::FileHashTrait, hash_trait::HashTrait, product_ident_helper_trait::ProductIdentificationHelperTrait,
};
use crate::csaf_traits::{CsafTrait, ProductTrait, ProductTreeTrait};
use crate::validation::ValidationError;
use std::collections::HashMap;

fn generate_duplicate_helper_error(
    category: &str,
    value: &str,
    product_id_1: &str,
    product_id_2: &str,
) -> ValidationError {
    ValidationError {
        message: format!(
            "The Product Identification Helper property '{category}' contains a duplicate value '{value}' used across multiple distinct products ('{product_id_1}' and '{product_id_2}'). Properties must be pairwise disjoint."
        ),
        instance_path: format!("/product_tree/.../product_identification_helper/{category}"),
    }
}

/// Test 6.2.32: Use of Same Product Identification Helper for Different Products
pub fn test_6_2_32_duplicate_product_identification_helpers(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let Some(product_tree) = doc.get_product_tree() else {
        return Ok(());
    };

    let mut errors: Vec<ValidationError> = vec![];

    let mut seen_purls: HashMap<String, String> = HashMap::new();
    let mut seen_serial_numbers: HashMap<String, String> = HashMap::new();
    let mut seen_model_numbers: HashMap<String, String> = HashMap::new();
    let mut seen_skus: HashMap<String, String> = HashMap::new();
    let mut seen_hashes: HashMap<String, String> = HashMap::new();

    product_tree.visit_all_products_generic(&mut |product, _instance_path| {
        let product_id = product.get_product_id().to_string();

        if let Some(helper) = product.get_product_identification_helper() {
            // Check PURLs
            if let Some(purls) = helper.get_purls() {
                for purl in purls {
                    let purl_str = format!("{purl:?}");
                    if let Some(existing_prod) = seen_purls.get(&purl_str) {
                        if *existing_prod != product_id {
                            errors.push(generate_duplicate_helper_error(
                                "purls",
                                &purl_str,
                                existing_prod,
                                &product_id,
                            ));
                        }
                    } else {
                        seen_purls.insert(purl_str, product_id.clone());
                    }
                }
            }

            // Check SKUs
            for sku in helper.get_skus() {
                let sku_str = sku.to_string();
                if let Some(existing_prod) = seen_skus.get(&sku_str) {
                    if *existing_prod != product_id {
                        errors.push(generate_duplicate_helper_error(
                            "skus",
                            &sku_str,
                            existing_prod,
                            &product_id,
                        ));
                    }
                } else {
                    seen_skus.insert(sku_str, product_id.clone());
                }
            }

            // Check Serial Numbers
            if let Some(serial_numbers) = helper.get_serial_numbers() {
                for sn in serial_numbers {
                    let sn_str = sn.to_string();
                    if let Some(existing_prod) = seen_serial_numbers.get(&sn_str) {
                        if *existing_prod != product_id {
                            errors.push(generate_duplicate_helper_error(
                                "serial_numbers",
                                &sn_str,
                                existing_prod,
                                &product_id,
                            ));
                        }
                    } else {
                        seen_serial_numbers.insert(sn_str, product_id.clone());
                    }
                }
            }

            // Check Model Numbers
            if let Some(model_numbers) = helper.get_model_numbers() {
                for mn in model_numbers {
                    let mn_str = mn.to_string();
                    if let Some(existing_prod) = seen_model_numbers.get(&mn_str) {
                        if *existing_prod != product_id {
                            errors.push(generate_duplicate_helper_error(
                                "model_numbers",
                                &mn_str,
                                existing_prod,
                                &product_id,
                            ));
                        }
                    } else {
                        seen_model_numbers.insert(mn_str, product_id.clone());
                    }
                }
            }

            // Check Hashes
            for hash_obj in helper.get_hashes() {
                let filename = hash_obj.get_filename();
                let mut inner_signatures: Vec<String> = hash_obj
                    .get_file_hashes()
                    .iter()
                    .map(|fh| format!("{:?}:{}", fh.get_algorithm(), fh.get_hash()))
                    .collect();
                inner_signatures.sort();

                let hash_str = format!("file:{filename};hashes:{}", inner_signatures.join(","));

                if let Some(existing_prod) = seen_hashes.get(&hash_str) {
                    if *existing_prod != product_id {
                        errors.push(generate_duplicate_helper_error(
                            "hashes",
                            &hash_str,
                            existing_prod,
                            &product_id,
                        ));
                    }
                } else {
                    seen_hashes.insert(hash_str, product_id.clone());
                }
            }
        }
    });

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_32,
    test_6_2_32_duplicate_product_identification_helpers
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_32() {
        // Case 01 contains a duplicate serial number
        let case_01_failing_example_1 = Err(vec![generate_duplicate_helper_error(
            "serial_numbers",
            "143-D-354",
            "CSAFPID-908070601",
            "CSAFPID-908070602",
        )]);

        // Case 02 contains a duplicate model number
        let case_02_failing_example_2 = Err(vec![generate_duplicate_helper_error(
            "model_numbers",
            "143-D-354",
            "CSAFPID-908070601",
            "CSAFPID-908070602",
        )]);

        // Case 03 is an invalid example that yields 2 model number validation errors
        let case_03_failing_example_3 = Err(vec![
            generate_duplicate_helper_error("model_numbers", "143-D-354", "CSAFPID-908070602", "CSAFPID-908070603"),
            generate_duplicate_helper_error("model_numbers", "143-D-354", "CSAFPID-908070602", "CSAFPID-908070605"),
        ]);

        let case_11_valid_example_1 = Ok(());
        let case_12_valid_example_2 = Ok(());

        TESTS_2_1.test_6_2_32.expect(
            case_01_failing_example_1,
            case_02_failing_example_2,
            case_03_failing_example_3,
            case_11_valid_example_1,
            case_12_valid_example_2,
        );
    }
}
