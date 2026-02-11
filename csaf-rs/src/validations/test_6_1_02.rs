use crate::csaf_traits::{CsafTrait, ProductTrait, ProductTreeTrait};
use crate::validation::ValidationError;
use std::collections::HashMap;

pub fn test_6_1_02_multiple_definition_of_product_id(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    if let Some(tree) = doc.get_product_tree().as_ref() {
        // Map to store each key with all of its paths
        let mut products_with_paths: HashMap<String, Vec<String>> = HashMap::new();
        tree.visit_all_products(&mut |product, path| {
            products_with_paths
                .entry(product.get_product_id().to_owned())
                .or_default()
                .push(path.to_owned());
        });
        for (product_id, paths) in products_with_paths {
            if paths.len() > 1 {
                for path in paths {
                    errors
                        .get_or_insert_with(Vec::new)
                        .push(generate_err_msg(&product_id, &path));
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

fn generate_err_msg(product_id: &str, path: &str) -> ValidationError {
    ValidationError {
        message: format!("Duplicate definition for product ID {product_id}"),
        instance_path: format!("{path}/product_id"),
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_2
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_02_multiple_definition_of_product_id(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_2
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_02_multiple_definition_of_product_id(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_02() {
        let shared_error_01 = Err(vec![
            generate_err_msg("CSAFPID-9080700", "/product_tree/full_product_names/0"),
            generate_err_msg("CSAFPID-9080700", "/product_tree/full_product_names/1"),
        ]);
        let shared_error_02 = Err(vec![
            generate_err_msg("CSAFPID-9080700", "/product_tree/full_product_names/0"),
            generate_err_msg("CSAFPID-9080700", "/product_tree/full_product_names/1"),
            generate_err_msg("CSAFPID-9080701", "/product_tree/branches/0/product"),
            generate_err_msg("CSAFPID-9080701", "/product_tree/branches/1/branches/0/product"),
            generate_err_msg("CSAFPID-9080702", "/product_tree/relationships/0/full_product_name"),
            generate_err_msg("CSAFPID-9080702", "/product_tree/relationships/1/full_product_name"),
            generate_err_msg("CSAFPID-9080700", "/product_tree/branches/1/branches/1/product"),
            generate_err_msg("CSAFPID-9080701", "/product_tree/relationships/2/full_product_name"),
        ]);
        TESTS_2_0
            .test_6_1_2
            .expect(shared_error_01.clone(), shared_error_02.clone());
        TESTS_2_1
            .test_6_1_2
            .expect(shared_error_01.clone(), shared_error_02.clone());
    }
}
