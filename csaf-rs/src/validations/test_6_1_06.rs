use crate::csaf_traits::{CsafTrait, ProductGroupsByIdMap, ProductStatusGroup, VulnerabilityTrait};
use crate::validation::ValidationError;

pub fn test_6_1_06_contradicting_product_status(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;
    for (vulnerability_index, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(product_status) = vulnerability.get_product_status() {
            let product_to_groups = ProductGroupsByIdMap::from(product_status);

            // TODO improve error messages now that the offending product paths are available
            // Check for products with multiple status groups (contradictions)
            for (product_id, groups) in product_to_groups {
                let mut affected_groups: Vec<ProductStatusGroup> = groups
                    .into_keys()
                    .filter(|g| *g != ProductStatusGroup::Recommended)
                    .collect();
                // sort for deterministic errors
                affected_groups.sort();
                if affected_groups.len() > 1 {
                    errors.get_or_insert_default().push(generate_err_msg(
                        &product_id,
                        &affected_groups,
                        vulnerability_index,
                    ));
                }
            }
        }
    }
    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(ValidatorForTest6_1_6, test_6_1_06_contradicting_product_status);

fn generate_err_msg(product_id: &str, groups: &[ProductStatusGroup], vulnerability_index: usize) -> ValidationError {
    let group_names: Vec<String> = groups.iter().map(|g| format!("'{g}'")).collect();
    ValidationError {
        message: format!(
            "Product {} is member of contradicting product status groups: {}",
            product_id,
            group_names.join(", ")
        ),
        instance_path: format!("/vulnerabilities/{vulnerability_index}/product_status"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_06() {
        let case_affected_not_affected = Err(vec![generate_err_msg(
            "CSAFPID-9080700",
            &[ProductStatusGroup::Affected, ProductStatusGroup::NotAffected],
            0,
        )]);
        let case_not_affected_fixed_vec = vec![generate_err_msg(
            "CSAFPID-9080701",
            &[ProductStatusGroup::NotAffected, ProductStatusGroup::Fixed],
            0,
        )];
        let case_affected_under_investigation_vec = vec![generate_err_msg(
            "CSAFPID-9080700",
            &[ProductStatusGroup::Affected, ProductStatusGroup::UnderInvestigation],
            0,
        )];
        let case_not_affected_under_investigation_vec = vec![generate_err_msg(
            "CSAFPID-9080701",
            &[ProductStatusGroup::NotAffected, ProductStatusGroup::UnderInvestigation],
            0,
        )];
        let case_affected_fixed_vec = vec![generate_err_msg(
            "CSAFPID-9080702",
            &[ProductStatusGroup::Affected, ProductStatusGroup::Fixed],
            0,
        )];
        let case_affected_unknown = Err(vec![generate_err_msg(
            "CSAFPID-9080700",
            &[ProductStatusGroup::Affected, ProductStatusGroup::Unknown],
            0,
        )]);
        let case_not_affected_unknown = Err(vec![generate_err_msg(
            "CSAFPID-9080700",
            &[ProductStatusGroup::NotAffected, ProductStatusGroup::Unknown],
            0,
        )]);
        let case_fixed_unknown = Err(vec![generate_err_msg(
            "CSAFPID-9080700",
            &[ProductStatusGroup::Fixed, ProductStatusGroup::Unknown],
            0,
        )]);
        let case_under_investigation_unknown = Err(vec![generate_err_msg(
            "CSAFPID-9080700",
            &[ProductStatusGroup::UnderInvestigation, ProductStatusGroup::Unknown],
            0,
        )]);

        // CSAF 2.0 has 10 test cases (01-05, 11-15)
        TESTS_2_0.test_6_1_6.expect(
            case_affected_not_affected.clone(),
            case_affected_not_affected.clone(),
            case_affected_not_affected.clone(),
            Err(case_affected_under_investigation_vec
                .clone()
                .into_iter()
                .chain(case_not_affected_fixed_vec.clone())
                .collect()),
            Err(case_affected_under_investigation_vec
                .clone()
                .into_iter()
                .chain(case_not_affected_under_investigation_vec.clone())
                .chain(case_affected_fixed_vec.clone())
                .collect()),
            Ok(()), // know_affected(0) & recommended(0)
            Ok(()), // first_affected(0) & known_affected(0)
            Ok(()), // known_affected(0) & last_affected(0)
            Ok(()), // first_fixed(1) & fixed(1) + recommended(0) & under_investigation(0)
            Ok(()), // first_affected(0) & known_affected(0) & recommended(0) + first_fixed(2) & fixed(2) + known_not_affected(1) & recommended(1)
        );

        // CSAF 2.1 has 12 test cases (01-06, 11-16)
        TESTS_2_1.test_6_1_6.expect(
            case_affected_not_affected.clone(),
            case_affected_not_affected.clone(),
            case_affected_not_affected,
            Err(case_affected_under_investigation_vec
                .clone()
                .into_iter()
                .chain(case_not_affected_fixed_vec)
                .collect()),
            Err(case_affected_under_investigation_vec
                .into_iter()
                .chain(case_not_affected_under_investigation_vec)
                .chain(case_affected_fixed_vec)
                .collect()),
            case_affected_unknown,
            case_not_affected_unknown,
            case_fixed_unknown,
            case_under_investigation_unknown,
            Ok(()), // know_affected(0) & recommended(0)
            Ok(()), // first_affected(0) & known_affected(0)
            Ok(()), // known_affected(0) & last_affected(0)
            Ok(()), // first_fixed(1) & fixed(1) + recommended(0) & under_investigation(0)
            Ok(()), // first_affected(0) & known_affected(0) & recommended(0) + first_fixed(2) & fixed(2) + known_not_affected(1) & recommended(1)
            Ok(()),
        );
    }
}
