use crate::csaf_traits::{CsafTrait, ProductGroupTrait, ProductTreeTrait};
use crate::validation::ValidationError;
use std::collections::HashSet;

fn generate_err_msg(ref_id: &str, ref_path: &str) -> ValidationError {
    ValidationError {
        message: format!("Missing definition of product_group_id: {ref_id}"),
        instance_path: ref_path.to_owned(),
    }
}

pub fn test_6_1_04_missing_definition_of_product_group_id(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = Option::None;
    if let Some(tree) = doc.get_product_tree() {
        let mut known_groups = HashSet::<String>::new();
        for g in tree.get_product_groups().iter() {
            known_groups.insert(g.get_group_id().to_owned());
        }

        let product_group_references = doc.get_all_group_references();
        for (ref_id, ref_path) in product_group_references.iter() {
            if !known_groups.contains(ref_id) {
                errors
                    .get_or_insert_default()
                    .push(generate_err_msg(ref_id, ref_path));
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    ValidatorForTest6_1_4,
    test_6_1_04_missing_definition_of_product_group_id
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_04() {
        let case_threats = Err(vec![generate_err_msg(
            "CSAFGID-1020301",
            "/vulnerabilities/0/threats/0/group_ids/0",
        )]);
        let case_flags = Err(vec![
            generate_err_msg("CSAFGID-1020300", "/vulnerabilities/0/flags/0/group_ids/0"),
            generate_err_msg("CSAFGID-1020301", "/vulnerabilities/1/flags/0/group_ids/0"),
        ]);
        let case_remediations = Err(vec![
            generate_err_msg("CSAFGID-1020300", "/vulnerabilities/0/remediations/0/group_ids/0"),
            generate_err_msg("CSAFGID-1020301", "/vulnerabilities/1/remediations/0/group_ids/0"),
        ]);
        let case_vulnerabilities = Err(vec![
            generate_err_msg("CSAFGID-1020304", "/document/notes/0/group_ids/0"),
            generate_err_msg("CSAFGID-1020303", "/vulnerabilities/0/flags/0/group_ids/0"),
            generate_err_msg("CSAFGID-1020302", "/vulnerabilities/0/involvements/0/group_ids/0"),
            generate_err_msg("CSAFGID-1020301", "/vulnerabilities/0/involvements/1/group_ids/0"),
            generate_err_msg("CSAFGID-1020302", "/vulnerabilities/0/involvements/2/group_ids/0"),
            generate_err_msg("CSAFGID-1020304", "/vulnerabilities/0/involvements/3/group_ids/0"),
            generate_err_msg("CSAFGID-1020302", "/vulnerabilities/0/notes/0/group_ids/0"),
            generate_err_msg("CSAFGID-1020302", "/vulnerabilities/0/remediations/0/group_ids/0"),
            generate_err_msg("CSAFGID-1020301", "/vulnerabilities/0/threats/0/group_ids/0"),
            generate_err_msg(
                "CSAFGID-1020301",
                "/vulnerabilities/0/first_known_exploitation_dates/0/group_ids/0",
            ),
        ]);

        TESTS_2_0.test_6_1_4.expect(
            case_threats.clone(),      // threats
            case_flags.clone(),        // flags
            case_remediations.clone(), // remediations
            Ok(()),
            Ok(()),
        );
        TESTS_2_1.test_6_1_4.expect(
            case_threats,         // threats
            case_vulnerabilities, // vulnerabilities
            Ok(()),
            Ok(()),
        );
    }
}
