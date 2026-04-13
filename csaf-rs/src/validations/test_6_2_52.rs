use crate::csaf::types::csaf_hash_algo::CsafHashAlgorithm;
use crate::csaf_traits::{
    CsafTrait, FileHashTrait, HashTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait,
};
use crate::validation::ValidationError;

/// 6.2.52 Unknown Hash Algorithm
///
/// For each element of type `/$defs/full_product_name_t/product_identification_helper/hashes[]/file_hashes[]/algorithm`,
/// it MUST be tested that the hash algorithm is supported by the implementation.
/// The warning MUST differentiate between the values mentioned in section 3.1.4.3.2
/// and those not mentioned there.
///
/// TODO: For now, we do not distinguish between algorithms that are supported and mentioned in the spec,
/// but the methods are already there.
pub fn test_6_2_52_unknown_hash_algorithm(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    if let Some(tree) = doc.get_product_tree() {
        tree.visit_all_products(&mut |fpn, path| {
            if let Some(helper) = fpn.get_product_identification_helper() {
                for (h_i, hash) in helper.get_hashes().iter().enumerate() {
                    for (fh_i, file_hash) in hash.get_file_hashes().iter().enumerate() {
                        let algorithm = file_hash.get_algorithm();
                        if !algorithm.is_supported_algorithm() {
                            errors
                                .get_or_insert_default()
                                .push(create_unknown_hash_algorithm_error(&algorithm, path, h_i, fh_i));
                        }
                    }
                }
            }
        });
    }

    errors.map_or(Ok(()), Err)
}

fn create_unknown_hash_algorithm_error(
    algorithm: &CsafHashAlgorithm,
    path: &str,
    hash_index: usize,
    file_hash_index: usize,
) -> ValidationError {
    let message = match algorithm.is_mentioned_in_spec() {
        // TODO Right now, is_supported == is_mentioned_in_spec, and this only gets called when
        // !is_supported, so this can't be reached.
        true => format!(
            "Hash algorithm '{algorithm}' is listed in the specification but not supported by this implementation"
        ),
        false => format!(
            "Hash algorithm '{algorithm}' is not listed in section 3.1.4.3.2 of the CSAF 2.1 specification and not supported by this implementation"
        ),
    };

    ValidationError {
        message,
        instance_path: format!(
            "{path}/product_identification_helper/hashes/{hash_index}/file_hashes/{file_hash_index}/algorithm"
        ),
    }
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_2_52, test_6_2_52_unknown_hash_algorithm);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_52() {
        let case_01 = Err(vec![create_unknown_hash_algorithm_error(
            &CsafHashAlgorithm::Other("unknown-algorithm".to_string()),
            "/product_tree/full_product_names/0",
            0,
            0,
        )]);

        // Case 11: "blake2s256" is mentioned in the spec and should be supported

        TESTS_2_1.test_6_2_52.expect(case_01, Ok(()));
    }
}
