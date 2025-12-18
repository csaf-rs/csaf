use crate::csaf_traits::{CsafTrait, ProductGroupTrait, ProductTreeTrait};
use crate::validation::ValidationError;
use std::collections::HashSet;

fn generate_err_msg(ref_id: &str, ref_path: &str) -> ValidationError {
    ValidationError {
        message: format!("Missing definition of product_group_id: {}", ref_id),
        instance_path: ref_path.to_owned(),
    }
}

pub fn test_6_1_04_missing_definition_of_product_group_id(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = Option::None;
    if let Some(tree) = doc.get_product_tree().as_ref() {
        let mut known_groups = HashSet::<String>::new();
        for g in tree.get_product_groups().iter() {
            known_groups.insert(g.get_group_id().to_owned());
        }

        let product_group_references = doc.get_all_group_references();
        for (ref_id, ref_path) in product_group_references.iter() {
            if !known_groups.contains(ref_id) {
                errors
                    .get_or_insert_with(Vec::new)
                    .push(generate_err_msg(ref_id, ref_path));
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_04() {
        let errors = HashMap::from([
            (
                "01",
                vec![generate_err_msg(
                    "CSAFGID-1020301",
                    "/vulnerabilities/0/threats/0/group_ids/0",
                )],
            ),
            (
                "02",
                vec![
                    generate_err_msg("CSAFGID-1020300", "/vulnerabilities/0/flags/0/group_ids/0"),
                    generate_err_msg("CSAFGID-1020301", "/vulnerabilities/1/flags/0/group_ids/0"),
                ],
            ),
        ]);
        run_csaf20_tests("04", test_6_1_04_missing_definition_of_product_group_id, errors.clone());
        run_csaf21_tests("04", test_6_1_04_missing_definition_of_product_group_id, errors);
    }
}
