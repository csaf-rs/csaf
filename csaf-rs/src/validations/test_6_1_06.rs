use crate::csaf_traits::{CsafTrait, ProductStatusGroup, ProductStatusTrait, VulnerabilityTrait};
use crate::validation::ValidationError;
use std::collections::HashMap;

pub fn test_6_1_06_contradicting_product_status(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;
    for (vulnerability_index, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(product_status) = vulnerability.get_product_status() {
            let product_status_map = product_status.get_all_by_product_status();

            // Invert the map: product_id -> list of ProductStatusGroups
            let mut product_to_groups: HashMap<String, Vec<ProductStatusGroup>> = HashMap::new();

            for (group, product_ids) in product_status_map {
                if group == ProductStatusGroup::Recommended {
                    // recommended products must not be checked for contradictions
                    continue;
                }
                for product_id in product_ids {
                    product_to_groups
                        .entry(product_id.to_owned())
                        .or_default()
                        .push(group.clone());
                }
            }

            // Check for products with multiple status groups (contradictions)
            for (product_id, groups) in product_to_groups {
                if groups.len() > 1 {
                    let mut affected_groups = groups;
                    affected_groups.sort();
                    errors.get_or_insert_with(Vec::new).push(generate_err_msg(
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

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_6
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_06_contradicting_product_status(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_6
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_06_contradicting_product_status(doc)
    }
}

fn generate_err_msg(product_id: &str, groups: &[ProductStatusGroup], vulnerability_index: usize) -> ValidationError {
    let group_names: Vec<String> = groups.iter().map(|g| format!("'{}'", g)).collect();
    ValidationError {
        message: format!(
            "Product {} is member of contradicting product status groups: {}",
            product_id,
            group_names.join(", ")
        ),
        instance_path: format!("/vulnerabilities/{}/product_status", vulnerability_index),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_06() {
        let case_01 = Err(vec![generate_err_msg(
            "CSAFPID-9080700",
            &[ProductStatusGroup::Affected, ProductStatusGroup::NotAffected],
            0,
        )]);
        let case_02 = Err(vec![generate_err_msg(
            "CSAFPID-9080700",
            &[ProductStatusGroup::Affected, ProductStatusGroup::NotAffected],
            0,
        )]);
        let case_03 = Err(vec![generate_err_msg(
            "CSAFPID-9080700",
            &[ProductStatusGroup::Affected, ProductStatusGroup::NotAffected],
            0,
        )]);
        let case_04 = Err(vec![
            generate_err_msg(
                "CSAFPID-9080700",
                &[ProductStatusGroup::Affected, ProductStatusGroup::UnderInvestigation],
                0,
            ),
            generate_err_msg(
                "CSAFPID-9080701",
                &[ProductStatusGroup::NotAffected, ProductStatusGroup::Fixed],
                0,
            ),
        ]);
        let case_05 = Err(vec![
            generate_err_msg(
                "CSAFPID-9080700",
                &[ProductStatusGroup::Affected, ProductStatusGroup::UnderInvestigation],
                0,
            ),
            generate_err_msg(
                "CSAFPID-9080701",
                &[ProductStatusGroup::NotAffected, ProductStatusGroup::UnderInvestigation],
                0,
            ),
            generate_err_msg(
                "CSAFPID-9080702",
                &[ProductStatusGroup::Affected, ProductStatusGroup::Fixed],
                0,
            ),
        ]);
        let case_06 = Err(vec![generate_err_msg(
            "CSAFPID-9080700",
            &[ProductStatusGroup::Affected, ProductStatusGroup::Unknown],
            0,
        )]);

        // CSAF 2.0 has 10 test cases (01-05, 11-15)
        TESTS_2_0.test_6_1_6.expect(
            case_01.clone(),
            case_02.clone(),
            case_03.clone(),
            case_04.clone(),
            case_05.clone(),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );

        // CSAF 2.1 has 12 test cases (01-06, 11-16)
        TESTS_2_1.test_6_1_6.expect(
            case_01,
            case_02,
            case_03,
            case_04,
            case_05,
            case_06,
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
