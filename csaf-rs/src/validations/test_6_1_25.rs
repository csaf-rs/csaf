use crate::csaf::types::csaf_hash_algo::CsafHashAlgorithm;
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
    // TODO: This can be a wasSkipped later
    let Some(product_tree) = doc.get_product_tree() else {
        return Ok(());
    };

    let mut errors: Option<Vec<ValidationError>> = None;
    // Visit all products in the product tree
    product_tree.visit_all_products(&mut |product, path| {
        // Check all file_hashes in all hashes in all product identification helpers
        if let Some(helper) = product.get_product_identification_helper() {
            for (hash_i, hash) in helper.get_hashes().iter().enumerate() {
                // Iterate over file_hashes, build hashmap of all encountered algos and their indices
                let mut algorithms = HashMap::<CsafHashAlgorithm, Vec<(Option<CsafHashAlgorithm>, usize)>>::new();
                for (file_hash_i, file_hash) in hash.get_file_hashes().iter().enumerate() {
                    let original_algorithm = file_hash.get_algorithm();
                    let normalized_algorithm = original_algorithm.normalize();
                    let original_algo_if_normalized = if original_algorithm != normalized_algorithm {
                        Some(original_algorithm)
                    } else {
                        None
                    };
                    let file_hash_is = algorithms.entry(normalized_algorithm).or_default();
                    file_hash_is.push((original_algo_if_normalized, file_hash_i));
                }
                // For each algo found multiple times, generate error message for all indices with the algo
                for (normalized_algo, file_hash_is) in &algorithms {
                    if file_hash_is.len() > 1 {
                        for (original_algo, file_hash_i) in file_hash_is.iter() {
                            errors.get_or_insert_default().push(test_6_1_25_err_generator(
                                normalized_algo,
                                original_algo.as_ref(),
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
    errors.map_or(Ok(()), Err)
}

fn test_6_1_25_err_generator(
    normalized_algo: &CsafHashAlgorithm,
    original_algo: Option<&CsafHashAlgorithm>,
    path: String,
    hash_i: String,
    file_hash_i: String,
) -> ValidationError {
    let message = match original_algo {
        Some(original) => format!(
            "Multiple use of the same hash algorithm '{normalized_algo}' (written as '{original}') in file_hashes"
        ),
        None => format!("Multiple use of the same hash algorithm '{normalized_algo}' in file_hashes"),
    };
    ValidationError {
        message,
        instance_path: format!(
            "{path}/product_identification_helper/hashes/{hash_i}/file_hashes/{file_hash_i}/algorithm"
        ),
    }
}

crate::test_validation::impl_validator!(ValidatorForTest6_1_25, test_6_1_25_multiple_use_of_same_hash_algorithm);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_25() {
        // Case 01: one file_hashes, with two elements, both with same algorithm
        // Case S01: one file_hashes, with three elements, two with same algorithm
        // Case S02: one file_hashes, with three elements, all with same algorithm
        // Case S03: (CSAF 2.0 only) one file_hashes, with two elements, same algorithm in different casing ("MD5" -> Other("MD5"), "md5" -> Md5)
        // Case S11: one file_hashes, with two elements, both with different algorithms
        // Case S12: two file_hashes, each with one element, both with same algorithm

        let one_file_hash_two_hashes_same_algo = Err(vec![
            test_6_1_25_err_generator(
                &CsafHashAlgorithm::Sha256,
                None,
                "/product_tree/full_product_names/0".to_string(),
                "0".to_string(),
                "0".to_string(),
            ),
            test_6_1_25_err_generator(
                &CsafHashAlgorithm::Sha256,
                None,
                "/product_tree/full_product_names/0".to_string(),
                "0".to_string(),
                "1".to_string(),
            ),
        ]);

        let three_elements_two_same_algo = Err(vec![
            test_6_1_25_err_generator(
                &CsafHashAlgorithm::Sha256,
                None,
                "/product_tree/full_product_names/0".to_string(),
                "0".to_string(),
                "0".to_string(),
            ),
            test_6_1_25_err_generator(
                &CsafHashAlgorithm::Sha256,
                None,
                "/product_tree/full_product_names/0".to_string(),
                "0".to_string(),
                "2".to_string(),
            ),
        ]);

        let three_elements_all_same_algo = Err(vec![
            test_6_1_25_err_generator(
                &CsafHashAlgorithm::Sha256,
                None,
                "/product_tree/full_product_names/0".to_string(),
                "0".to_string(),
                "0".to_string(),
            ),
            test_6_1_25_err_generator(
                &CsafHashAlgorithm::Sha256,
                None,
                "/product_tree/full_product_names/0".to_string(),
                "0".to_string(),
                "1".to_string(),
            ),
            test_6_1_25_err_generator(
                &CsafHashAlgorithm::Sha256,
                None,
                "/product_tree/full_product_names/0".to_string(),
                "0".to_string(),
                "2".to_string(),
            ),
        ]);

        // S03: "MD5" (Other("MD5")) and "md5" (Md5) are the same algorithm with different casing
        let two_elements_same_algo_different_casing = Err(vec![
            test_6_1_25_err_generator(
                &CsafHashAlgorithm::Md5,
                Some(&CsafHashAlgorithm::Other("MD5".to_string())),
                "/product_tree/full_product_names/0".to_string(),
                "0".to_string(),
                "0".to_string(),
            ),
            test_6_1_25_err_generator(
                &CsafHashAlgorithm::Md5,
                None,
                "/product_tree/full_product_names/0".to_string(),
                "0".to_string(),
                "1".to_string(),
            ),
        ]);

        TESTS_2_0.test_6_1_25.expect(
            one_file_hash_two_hashes_same_algo.clone(),
            three_elements_two_same_algo.clone(),
            three_elements_all_same_algo.clone(),
            two_elements_same_algo_different_casing,
            Ok(()),
            Ok(()),
        );
        TESTS_2_1.test_6_1_25.expect(
            one_file_hash_two_hashes_same_algo,
            three_elements_two_same_algo,
            three_elements_all_same_algo,
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
