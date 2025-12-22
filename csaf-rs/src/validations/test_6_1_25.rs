use crate::csaf_traits::{
    CsafTrait, FileHashTrait, HashTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait,
};
use crate::validation::ValidationError;
use std::collections::HashMap;

/// Test 6.1.25: Multiple Use of the Same Hash Algorithm
///
/// For `*/file_hashes[]`, each `file_hashes` item need to use different
/// values in their `algorithm` fields.
///
/// This checks in:
/// `/product_tree/branches[](/branches[])*/product/product_identification_helper/hashes[]/file_hashes[]`
/// `/product_tree/full_product_names[]/product_identification_helper/hashes[]/file_hashes[]`
/// `/product_tree/relationships[]/full_product_name/product_identification_helper/hashes[]/file_hashes[]`
pub fn test_6_1_25_multiple_use_of_same_hash_algorithm(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;
    // Visit all products in the product tree
    if let Some(product_tree) = doc.get_product_tree() {
        product_tree.visit_all_products(&mut |product, path| {
            // Check all file_hashes in all hashes in all product identification helpers
            if let Some(helper) = product.get_product_identification_helper() {
                for (hash_i, hash) in helper.get_hashes().iter().enumerate() {
                    // Iterate over file_hashes, build hashmap of all encountered algos and their indices
                    let mut algorithms = HashMap::<String, Vec<usize>>::new();
                    for (file_hash_i, file_hash) in hash.get_file_hashes().iter().enumerate() {
                        let file_hash_is = algorithms.entry(file_hash.get_algorithm().to_string()).or_default();
                        file_hash_is.push(file_hash_i);
                    }
                    // For each algo found multiple times, generate error message for all indices with the algo
                    for (algo, file_hash_is) in &algorithms {
                        if file_hash_is.len() > 1 {
                            for file_hash_i in file_hash_is.iter() {
                                errors.get_or_insert_with(Vec::new).push(test_6_1_25_err_generator(
                                    algo.to_string(),
                                    path.to_string(),
                                    hash_i.to_string(),
                                    file_hash_i.to_string(),
                                ));
                            }
                        }
                    }
                }
            }
        });
    }
    errors.map_or(Ok(()), Err)
}

fn test_6_1_25_err_generator(algorithm: String, path: String, hash_i: String, file_hash_i: String) -> ValidationError {
    ValidationError {
        message: format!("Multiple use of the same hash algorithm '{}' in file_hashes", algorithm),
        instance_path: format!(
            "{}/product_identification_helper/hashes/{}/file_hashes/{} /algorithm",
            path, hash_i, file_hash_i
        ),
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_25
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_25_multiple_use_of_same_hash_algorithm(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_25
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_25_multiple_use_of_same_hash_algorithm(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_25() {
        let case_01 = Err(vec![
            test_6_1_25_err_generator(
                "sha256".to_string(),
                "/product_tree/full_product_names/0".to_string(),
                "0".to_string(),
                "0".to_string(),
            ),
            test_6_1_25_err_generator(
                "sha256".to_string(),
                "/product_tree/full_product_names/0".to_string(),
                "0".to_string(),
                "1".to_string(),
            ),
        ]);

        // Both CSAF 2.0 and 2.1 have 1 test case
        TESTS_2_0.test_6_1_25.expect(case_01.clone());
        TESTS_2_1.test_6_1_25.expect(case_01);
    }
}
