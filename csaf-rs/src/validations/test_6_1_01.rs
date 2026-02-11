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
            errors
                .get_or_insert_with(Vec::new)
                .push(generate_err_msg(ref_id, ref_path));
        }
    }
    errors.map_or(Ok(()), Err)
}

fn generate_err_msg(ref_id: &str, ref_path: &str) -> ValidationError {
    ValidationError {
        message: format!("Missing definition of product_id: {ref_id}"),
        instance_path: ref_path.to_string(),
    }
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
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_6_1_01() {
        TESTS_2_0.test_6_1_1.expect(
            Err(vec![
                generate_err_msg("CSAFPID-9080700", "/product_tree/product_groups/0/product_ids/0"),
                generate_err_msg("CSAFPID-9080701", "/product_tree/product_groups/0/product_ids/1"),
            ]),
            Err(vec![
                generate_err_msg("CSAFPID-9080701", "/vulnerabilities/0/flags/0/product_ids/1"),
                generate_err_msg("CSAFPID-9080702", "/vulnerabilities/1/flags/0/product_ids/0"),
            ]),
            Err(vec![
                generate_err_msg("CSAFPID-9080701", "/product_tree/relationships/0/product_reference"),
                generate_err_msg(
                    "CSAFPID-9080702",
                    "/product_tree/relationships/0/relates_to_product_reference",
                ),
                generate_err_msg("CSAFPID-9080703", "/vulnerabilities/0/product_status/first_affected/0"),
                generate_err_msg("CSAFPID-9080703", "/vulnerabilities/0/product_status/first_fixed/0"),
                generate_err_msg("CSAFPID-9080703", "/vulnerabilities/0/product_status/fixed/0"),
                generate_err_msg("CSAFPID-9080703", "/vulnerabilities/0/product_status/known_affected/0"),
                generate_err_msg(
                    "CSAFPID-9080703",
                    "/vulnerabilities/0/product_status/known_not_affected/0",
                ),
                generate_err_msg("CSAFPID-9080703", "/vulnerabilities/0/product_status/last_affected/0"),
                generate_err_msg("CSAFPID-9080703", "/vulnerabilities/0/product_status/recommended/0"),
                generate_err_msg(
                    "CSAFPID-9080703",
                    "/vulnerabilities/0/product_status/under_investigation/0",
                ),
                generate_err_msg("CSAFPID-9080704", "/vulnerabilities/0/remediations/0/product_ids/0"),
                generate_err_msg("CSAFPID-9080705", "/vulnerabilities/0/scores/0/products/0"),
                generate_err_msg("CSAFPID-9080706", "/vulnerabilities/0/threats/0/product_ids/0"),
            ]),
            Ok(()),
            Ok(()),
        );

        TESTS_2_1.test_6_1_1.expect(
            Err(vec![
                generate_err_msg("CSAFPID-9080700", "/product_tree/product_groups/0/product_ids/0"),
                generate_err_msg("CSAFPID-9080701", "/product_tree/product_groups/0/product_ids/1"),
            ]),
            Err(vec![
                generate_err_msg("CSAFPID-9080701", "/document/notes/0/product_ids/0"),
                generate_err_msg("CSAFPID-9080703", "/product_tree/relationships/0/product_reference"),
                generate_err_msg(
                    "CSAFPID-9080704",
                    "/product_tree/relationships/0/relates_to_product_reference",
                ),
                generate_err_msg("CSAFPID-9080705", "/vulnerabilities/0/involvements/0/product_ids/0"),
                generate_err_msg("CSAFPID-9080706", "/vulnerabilities/0/flags/0/product_ids/0"),
                generate_err_msg("CSAFPID-9080707", "/vulnerabilities/0/metrics/0/products/0"),
                generate_err_msg("CSAFPID-9080708", "/vulnerabilities/0/notes/0/product_ids/0"),
                generate_err_msg("CSAFPID-9080709", "/vulnerabilities/0/product_status/first_affected/0"),
                generate_err_msg("CSAFPID-9080709", "/vulnerabilities/0/product_status/first_fixed/0"),
                generate_err_msg("CSAFPID-9080709", "/vulnerabilities/0/product_status/fixed/0"),
                generate_err_msg("CSAFPID-9080709", "/vulnerabilities/0/product_status/known_affected/0"),
                generate_err_msg(
                    "CSAFPID-9080709",
                    "/vulnerabilities/0/product_status/known_not_affected/0",
                ),
                generate_err_msg("CSAFPID-9080709", "/vulnerabilities/0/product_status/last_affected/0"),
                generate_err_msg("CSAFPID-9080709", "/vulnerabilities/0/product_status/recommended/0"),
                generate_err_msg(
                    "CSAFPID-9080709",
                    "/vulnerabilities/0/product_status/under_investigation/0",
                ),
                generate_err_msg("CSAFPID-9080710", "/vulnerabilities/0/remediations/0/product_ids/0"),
                generate_err_msg("CSAFPID-9080711", "/vulnerabilities/0/threats/0/product_ids/0"),
            ]),
        );
    }
}
