use crate::csaf_traits::{
    CsafTrait, FileHashTrait, HashTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait,
};
use crate::validation::TestFinding;
use crate::validations::utils::unknown_hash_algorithm_finding::create_unknown_hash_algorithm_finding_data;

/// 6.2.52 Unknown Hash Algorithm
///
/// For each element of type `/$defs/full_product_name_t/product_identification_helper/hashes[]/file_hashes[]/algorithm`,
/// it MUST be tested that the hash algorithm is supported by the implementation.
/// The warning MUST differentiate between the values mentioned in section 3.1.4.3.2
/// and those not mentioned there.
///
/// TODO: For now, we do not distinguish between algorithms that are supported and mentioned in the spec,
/// but the methods are already there.
pub fn test_6_2_52_unknown_hash_algorithm(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let mut errors: Option<Vec<TestFinding>> = None;

    if let Some(tree) = doc.get_product_tree() {
        tree.visit_all_products(&mut |fpn, path| {
            if let Some(helper) = fpn.get_product_identification_helper()
                && let Some(hashes) = helper.get_hashes()
            {
                for (h_i, hash) in hashes.iter().enumerate() {
                    for (fh_i, file_hash) in hash.get_file_hashes().iter().enumerate() {
                        let algorithm = file_hash.get_algorithm();
                        if !algorithm.is_supported_algorithm() {
                            errors
                                .get_or_insert_default()
                                .push(TestFinding::Warning(create_unknown_hash_algorithm_finding_data(&algorithm, path, h_i, fh_i)));
                        }
                    }
                }
            }
        });
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_2_52, test_6_2_52_unknown_hash_algorithm);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf::types::csaf_hash_algo::CsafHashAlgorithm;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_52 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_52() {
        let case_01 = Err(vec![TestFinding::Warning(create_unknown_hash_algorithm_finding_data(
            &CsafHashAlgorithm::Other("unknown-algorithm".to_string()),
            "/product_tree/full_product_names/0",
            0,
            0,
        ))]);

        // Case 11: "blake2s256" is mentioned in the spec and should be supported

        TESTS_2_1.test_6_2_52.expect(ExpectedResults {
            case_01,
            case_11: Ok(()),
        });
    }
}
