use crate::csaf_traits::{
    CsafTrait, FileHashTrait, HashTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait,
};
use crate::validation::ValidationError;

fn create_short_hash_error(
    path: &str,
    hash_index: usize,
    file_hash_index: usize,
    hash_length: usize,
) -> ValidationError {
    ValidationError {
        message: format!(
            "Too short hash found (length: {}), expected to be >= 64 chars",
            hash_length
        ),
        instance_path: format!(
            "{}/product_identification_helper/hashes/{}/file_hashes/{}",
            path, hash_index, file_hash_index
        ),
    }
}

/// 6.3.5 Use of Short Hash
///
/// Each product in the product tree that contains a `product_identification_helper` via `hashes` must
/// provide hashes with a length of at least 64 characters.
///
/// Hint: This will fail for algorithms like SHA-1 (40 characters) or MD5 (32 characters), which are also
/// discouraged by 6.2.8 and 6.2.9.
pub fn test_6_3_5_use_of_short_hash(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    if let Some(tree) = doc.get_product_tree().as_ref() {
        tree.visit_all_products(&mut |fpn, path| {
            if let Some(helper) = fpn.get_product_identification_helper() {
                for (h_i, hash) in helper.get_hashes().iter().enumerate() {
                    for (fh_i, file_hash) in hash.get_file_hashes().iter().enumerate() {
                        let file_hash_len = file_hash.get_hash().len();
                        if file_hash_len < 64 {
                            errors.get_or_insert_with(Vec::new).push(create_short_hash_error(
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

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_3_5
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_3_5_use_of_short_hash(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_3_5
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_3_5_use_of_short_hash(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_3() {
        let case_01 = Err(vec![create_short_hash_error(
            "/product_tree/full_product_names/0",
            0,
            0,
            32,
        )]);

        // Both CSAF 2.0 and 2.1 have 1 test case
        TESTS_2_0.test_6_3_5.expect(case_01.clone());
        TESTS_2_1.test_6_3_5.expect(case_01);
    }
}
