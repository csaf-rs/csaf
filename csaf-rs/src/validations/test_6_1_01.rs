use crate::csaf_traits::{CsafTrait, ProductTrait, ProductTreeTrait};
use crate::validation::ValidationError;
use std::collections::HashSet;

fn validate_missing_product_id<Doc: CsafTrait>(doc: &Doc) -> Result<(), Vec<ValidationError>> {
    let mut definitions_set = HashSet::<String>::new();
    if let Some(tree) = doc.get_product_tree().as_ref() {
        tree.visit_all_products(&mut |fpn, _path| {
            definitions_set.insert(fpn.get_product_id().to_owned());
        });
    }

    let references = doc.get_all_product_references();
    let mut errors: Option<Vec<ValidationError>> = Option::None;
    for (ref_id, ref_path) in references.iter() {
        if !definitions_set.contains(ref_id) {
            errors.get_or_insert_with(Vec::new).push(ValidationError {
                message: format!("Missing definition of product_id: {}", ref_id),
                instance_path: ref_path.to_string(),
            });
        }
    }
    errors.map_or(Ok(()), Err)
}

/// Validation implementation for test 6.1.1 (CSAF 2.0)
impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_1
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        validate_missing_product_id(doc)
    }
}

/// Validation implementation for test 6.1.1 (CSAF 2.1)
impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_1
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        validate_missing_product_id(doc)
    }
}

#[cfg(test)]
mod tests {
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use crate::validation::ValidationError;

    #[test]
    fn test_6_1_01() {
        TESTS_2_0.test_6_1_1.expect(
            Err(vec![
                ValidationError {
                    message: "Missing definition of product_id: CSAFPID-9080700".to_string(),
                    instance_path: "/product_tree/product_groups/0/product_ids/0".to_string(),
                },
                ValidationError {
                    message: "Missing definition of product_id: CSAFPID-9080701".to_string(),
                    instance_path: "/product_tree/product_groups/0/product_ids/1".to_string(),
                },
            ]),
            Err(vec![
                ValidationError {
                    message: "Missing definition of product_id: CSAFPID-9080701".to_string(),
                    instance_path: "/vulnerabilities/0/flags/0/product_ids/1".to_string(),
                },
                ValidationError {
                    message: "Missing definition of product_id: CSAFPID-9080702".to_string(),
                    instance_path: "/vulnerabilities/1/flags/0/product_ids/0".to_string(),
                },
            ]),
            Ok(()),
            Ok(()),
        );

        TESTS_2_1.test_6_1_1.expect(Err(vec![
            ValidationError {
                message: "Missing definition of product_id: CSAFPID-9080700".to_string(),
                instance_path: "/product_tree/product_groups/0/product_ids/0".to_string(),
            },
            ValidationError {
                message: "Missing definition of product_id: CSAFPID-9080701".to_string(),
                instance_path: "/product_tree/product_groups/0/product_ids/1".to_string(),
            },
        ]));
    }
}
