use crate::csaf::types::csaf_hash_algo::CsafHashAlgorithm;
use crate::csaf_traits::{
    CsafTrait, FileHashTrait, HashTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait,
};
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::unknown_hash_algorithm_finding::create_unknown_hash_algorithm_finding_data;

fn create_inconsistent_hash_length_error(
    algorithm: &CsafHashAlgorithm,
    expected_length: usize,
    actual_length: usize,
    base_path: &str,
    hash_index: usize,
    file_hash_index: usize,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "Hash algorithm '{algorithm}' requires a hash value length of {expected_length} characters, \
             but the value has length {actual_length}"
        ),
        instance_path: format!(
            "{base_path}/product_identification_helper/hashes/{hash_index}/file_hashes/{file_hash_index}/value"
        ),
    })
}

/// 6.1.63 Inconsistent Hash Value Length
///
/// For each file hash object, it SHALL be tested that the value length aligns with the algorithm.
///
/// The test SHALL be skipped for algorithms with variable length output.
///
/// Algorithms not supported by the implementation SHALL result in a warning which SHALL include the value of algorithm.
/// The warning SHALL differentiate between the values mentioned in section 3.1.10.3.2 and those not mentioned there.
pub fn test_6_1_63_inconsistent_hash_value_length(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let mut findings: Option<Vec<TestFinding>> = None;

    // if there is no product tree, we can skip
    let Some(product_tree) = doc.get_product_tree() else {
        return Ok(()); // #409 wasSkipped
    };

    // visit all products, if they have PIHs with hashes
    product_tree.visit_all_products(&mut |product, path| {
        if let Some(helper) = product.get_product_identification_helper()
            && let Some(hashes) = helper.get_hashes()
        {
            // iterate over hashes and file hashes
            for (hash_index, hash) in hashes.iter().enumerate() {
                for (file_hash_index, file_hash) in hash.get_file_hashes().iter().enumerate() {
                    let algorithm = file_hash.get_algorithm();

                    // check if the algorithm is supported
                    if !algorithm.is_supported_algorithm() {
                        findings.get_or_insert_default().push(TestFinding::Warning(
                            create_unknown_hash_algorithm_finding_data(&algorithm, path, hash_index, file_hash_index),
                        ));
                        continue;
                    }

                    // check if the hash is the right length for the algorithm
                    let value_length = file_hash.get_value_length();
                    // some algorithms have variable length output, so we skip those (e.g. SHAKE algos)
                    // algorithms not supported (CsafHashAlgorithm::Other]) are already handled above
                    if let Some(expected_length) = algorithm.expected_length()
                        && value_length != expected_length
                    {
                        findings
                            .get_or_insert_default()
                            .push(create_inconsistent_hash_length_error(
                                &algorithm,
                                expected_length,
                                value_length,
                                path,
                                hash_index,
                                file_hash_index,
                            ));
                    }
                }
            }
        }
    });

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_63,
    test_6_1_63_inconsistent_hash_value_length
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf::types::csaf_hash_algo::CsafHashAlgorithm;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_63 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_63_inconsistent_hash_value_length() {
        let case_s01_simple = Err(vec![create_inconsistent_hash_length_error(
            &CsafHashAlgorithm::Md4,
            32,
            128,
            "/product_tree/full_product_names/0",
            0,
            0,
        )]);

        TESTS_2_1.test_6_1_63.expect(ExpectedResults {
            case_s01: case_s01_simple,
        });
    }
}
