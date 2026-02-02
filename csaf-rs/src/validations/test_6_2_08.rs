use crate::csaf_traits::{
    CsafTrait, CsafHashAlgorithm, HashTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait,
};
use crate::validation::ValidationError;

/// 6.2.8 Use of MD5 as the only Hash Algorithm
///
/// When hashes are provided as product identification helpers for a product, another hash
/// besides a MD5 hash must be provided.
pub fn test_6_2_08_use_of_md5_as_only_hash_algo(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    // for each product in the product tree, check all product identification helper hashes for MD5 as the only hash algorithm
    if let Some(tree) = doc.get_product_tree().as_ref() {
        tree.visit_all_products(&mut |fpn, path| {
            if let Some(helper) = fpn.get_product_identification_helper() {
                for (h_i, hash) in helper.get_hashes().iter().enumerate() {
                    if hash.contains_only_hash_algorithm(CsafHashAlgorithm::Md5) {
                        errors
                            .get_or_insert_with(Vec::new)
                            .push(create_md5_only_hash_error(path, h_i));
                    }
                }
            }
        });
    }

    errors.map_or(Ok(()), Err)
}

fn create_md5_only_hash_error(path: &str, hash_index: usize) -> ValidationError {
    ValidationError {
        message: "hashes product identification helper uses MD5 as the only hash algorithm".to_string(),
        instance_path: format!(
            "{}/product_identification_helper/hashes/{}/file_hashes",
            path, hash_index
        ),
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_2_8
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_08_use_of_md5_as_only_hash_algo(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_2_8
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_08_use_of_md5_as_only_hash_algo(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_08() {
        let case_01_and_02 = Err(vec![create_md5_only_hash_error(
            "/product_tree/full_product_names/0",
            0,
        )]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0
            .test_6_2_8
            .expect(case_01_and_02.clone(), case_01_and_02.clone());
        TESTS_2_1.test_6_2_8.expect(case_01_and_02.clone(), case_01_and_02);
    }
}
