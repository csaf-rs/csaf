use crate::csaf_traits::{
    CsafTrait, FileHashTrait, HashTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait,
};
use crate::validation::{TestFinding, TestFindingData};

fn create_short_hash_error(path: &str, hash_index: usize, file_hash_index: usize, hash_length: usize) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: format!("Too short hash found (length: {hash_length}), expected to be >= 64 chars"),
        instance_path: format!(
            "{path}/product_identification_helper/hashes/{hash_index}/file_hashes/{file_hash_index}/value"
        ),
    })
}

/// 6.3.5 Use of Short Hash
///
/// Each product in the product tree that contains a `product_identification_helper` via `hashes` must
/// provide hashes with a length of at least 64 characters.
///
/// Hint: This will fail for algorithms like SHA-1 (40 characters) or MD5 (32 characters), which are also
/// discouraged by 6.2.8 and 6.2.9.
pub fn test_6_3_5_use_of_short_hash(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let mut errors: Option<Vec<TestFinding>> = None;

    if let Some(tree) = doc.get_product_tree() {
        tree.visit_all_products(&mut |fpn, path| {
            if let Some(helper) = fpn.get_product_identification_helper()
                && let Some(hashes) = helper.get_hashes()
            {
                for (h_i, hash) in hashes.iter().enumerate() {
                    for (fh_i, file_hash) in hash.get_file_hashes().iter().enumerate() {
                        let file_hash_len = file_hash.get_hash().len();
                        if file_hash_len < 64 {
                            errors.get_or_insert_default().push(create_short_hash_error(
                                path,
                                h_i,
                                fh_i,
                                file_hash_len,
                            ));
                        }
                    }
                }
            }
        });
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(ValidatorForTest6_3_5, test_6_3_5_use_of_short_hash);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_3_5 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_3_5 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_5() {
        let case_01 = Err(vec![create_short_hash_error(
            "/product_tree/full_product_names/0",
            0,
            0,
            32,
        )]);

        let case_s01 = Err(vec![
            create_short_hash_error("/product_tree/branches/0/branches/0/branches/0/product", 0, 1, 32),
            create_short_hash_error("/product_tree/branches/0/branches/1/product", 0, 1, 40),
            create_short_hash_error("/product_tree/branches/1/product", 1, 0, 56),
        ]);

        let case_s02 = Err(vec![
            create_short_hash_error("/product_tree/full_product_names/0", 0, 1, 32),
            create_short_hash_error("/product_tree/full_product_names/1", 1, 0, 40),
        ]);

        let case_s03 = Err(vec![
            create_short_hash_error("/product_tree/product_paths/0/full_product_name", 0, 1, 32),
            create_short_hash_error("/product_tree/product_paths/1/full_product_name", 1, 0, 40),
        ]);

        let case_s04 = Err(vec![
            create_short_hash_error("/product_tree/branches/0/product", 0, 0, 32),
            create_short_hash_error("/product_tree/full_product_names/0", 0, 0, 40),
            create_short_hash_error("/product_tree/product_paths/0/full_product_name", 0, 0, 56),
        ]);

        let case_s05 = Err(vec![create_short_hash_error(
            "/product_tree/full_product_names/0",
            0,
            0,
            63,
        )]);

        // TODO: Add a test case for an empty hash value once lenient parsing is supported

        let case_s11 = Ok(());

        TESTS_2_0.test_6_3_5.expect(ExpectedResults_2_0 {
            case_01: case_01.clone(),
        });

        // Failing test cases:
        // 01  - hash value with exactly 32 characters
        // s01 - multiple short hashes in nested branches, mixed with valid hashes
        // s02 - multiple short hashes in full product names, mixed with valid hashes
        // s03 - multiple short hashes in product paths, mixed with valid hashes
        // s04 - multiple short hashes across all three product tree locations
        // s05 - hash value with exactly 63 characters

        // Valid test cases:
        // s11 - hash value with exactly 64 characters

        TESTS_2_1.test_6_3_5.expect(ExpectedResults_2_1 {
            case_01,
            case_s01,
            case_s02,
            case_s03,
            case_s04,
            case_s05,
            case_s11,
        });
    }
}
