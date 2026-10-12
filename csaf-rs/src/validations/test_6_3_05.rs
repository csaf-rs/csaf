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
                        let file_hash_len = file_hash.get_value_length();
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
        let case_01_simple_32_length_hash = Err(vec![create_short_hash_error(
            "/product_tree/full_product_names/0",
            0,
            0,
            32,
        )]);

        let case_02_all_paths_mixed = Err(vec![
            create_short_hash_error("/product_tree/full_product_names/0", 0, 0, 32),
            create_short_hash_error("/product_tree/full_product_names/1", 1, 0, 40),
            create_short_hash_error("/product_tree/product_paths/0/full_product_name", 1, 0, 40),
            create_short_hash_error("/product_tree/product_paths/1/full_product_name", 0, 1, 56),
            create_short_hash_error("/product_tree/branches/0/branches/0/branches/0/product", 0, 0, 32),
            create_short_hash_error("/product_tree/branches/0/branches/0/branches/1/product", 1, 0, 32),
        ]);

        let case_03_hash_length_63 = Err(vec![create_short_hash_error(
            "/product_tree/full_product_names/0",
            0,
            0,
            63,
        )]);

        let case_s01_branch_depth_exhaustive = Err(vec![
            create_short_hash_error("/product_tree/branches/0/branches/0/branches/0/product", 0, 1, 32),
            create_short_hash_error("/product_tree/branches/0/branches/1/product", 0, 1, 40),
            create_short_hash_error("/product_tree/branches/1/product", 1, 0, 56),
        ]);

        // TODO: Add a test case for an empty hash value once lenient parsing is supported

        TESTS_2_0.test_6_3_5.expect(ExpectedResults_2_0 {
            case_01: case_01_simple_32_length_hash.clone(),
        });

        // Valid test cases:
        // 11, 12, 13 are 01, 02, 03 with all hashes >= 64 characters

        TESTS_2_1.test_6_3_5.expect(ExpectedResults_2_1 {
            case_01: case_01_simple_32_length_hash,
            case_02: case_02_all_paths_mixed,
            case_03: case_03_hash_length_63,
            case_s01: case_s01_branch_depth_exhaustive,
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
        });
    }
}
