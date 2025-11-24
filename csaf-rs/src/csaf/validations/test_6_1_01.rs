use crate::csaf::csaf_traits::{CsafTrait, ProductTrait, ProductTreeTrait};
use crate::csaf::product_helpers::gather_product_references;
use crate::csaf::validation::ValidationError;
use std::collections::HashSet;

pub fn test_6_1_01_missing_definition_of_product_id(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut definitions_set = HashSet::<String>::new();
    if let Some(tree) = doc.get_product_tree().as_ref() {
        tree.visit_all_products(&mut |fpn, _path| {
            definitions_set.insert(fpn.get_product_id().to_owned());
        });
    }

    let references = gather_product_references(doc);
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

#[cfg(test)]
mod tests {
    use crate::csaf::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::csaf::validation::ValidationError;
    use crate::csaf::validations::test_6_1_01::test_6_1_01_missing_definition_of_product_id;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_01() {
        let errors_20 = HashMap::from([
            (
                "01",
                vec![
                    ValidationError {
                        message: "Missing definition of product_id: CSAFPID-9080700".to_string(),
                        instance_path: "/product_tree/product_groups/0/product_ids/0".to_string(),
                    },
                    ValidationError {
                        message: "Missing definition of product_id: CSAFPID-9080701".to_string(),
                        instance_path: "/product_tree/product_groups/0/product_ids/1".to_string(),
                    },
                ],
            ),
            (
                "02",
                vec![
                    ValidationError {
                        message: "Missing definition of product_id: CSAFPID-9080701".to_string(),
                        instance_path: "/vulnerabilities/0/flags/0/product_ids/1".to_string(),
                    },
                    ValidationError {
                        message: "Missing definition of product_id: CSAFPID-9080702".to_string(),
                        instance_path: "/vulnerabilities/1/flags/0/product_ids/0".to_string(),
                    },
                ],
            ),
        ]);
        run_csaf20_tests("01", test_6_1_01_missing_definition_of_product_id, errors_20);

        let errors_21 = HashMap::from([(
            "01",
            vec![
                ValidationError {
                    message: "Missing definition of product_id: CSAFPID-9080700".to_string(),
                    instance_path: "/product_tree/product_groups/0/product_ids/0".to_string(),
                },
                ValidationError {
                    message: "Missing definition of product_id: CSAFPID-9080701".to_string(),
                    instance_path: "/product_tree/product_groups/0/product_ids/1".to_string(),
                },
            ],
        )]);
        run_csaf21_tests("01", test_6_1_01_missing_definition_of_product_id, errors_21);
    }
}
