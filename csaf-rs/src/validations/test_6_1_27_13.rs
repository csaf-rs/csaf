use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{CsafTrait, DocumentTrait, ProductStatusTrait, VulnerabilityTrait};
use crate::document_category_test_helper::DocumentCategoryTestConfig;
use crate::validation::ValidationError;

fn create_missing_affected_products_error(
    fixed_product: &str,
    fixed_product_index: usize,
    vulnerability_index: usize,
) -> ValidationError {
    ValidationError {
        message: format!(
            "The vulnerability contains the fixed product {fixed_product} which is not a known_affected product"
        ),
        instance_path: format!("/vulnerabilities[{vulnerability_index}]/product_status/fixed/{fixed_product_index}"),
    }
}

/// 6.1.27.13 Fixed Products
///
/// This test only applies to documents with `/document/category` with value `csaf_security_advisory`.
///
/// For each item in /vulnerabilities it MUST be tested that each fixed product also has a version which is affected.
pub fn test_6_1_27_13_fixed_products(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let doc_category = doc.get_document().get_category();

    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(doc.get_document().get_csaf_version(), &doc_category) {
        return Ok(());
    }

    let mut errors = Vec::new();

    let vulnerabilities = doc.get_vulnerabilities();
    for (v_i, vulnerability) in vulnerabilities.iter().enumerate() {
        match vulnerability.get_product_status() {
            None => continue, // ToDo maybe generate warning
            Some(product_status) => {
                match product_status.get_fixed() {
                    None => continue, // we only care if there are fixed products
                    Some(fixed_products) => {
                        let known_affected = product_status.get_known_affected();
                        for (f_i, fixed_product) in fixed_products.enumerate() {
                            if known_affected.is_none() {
                                errors.push(create_missing_affected_products_error(fixed_product, f_i, v_i));
                            }
                        }
                    },
                }
            },
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(())
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig =
    DocumentCategoryTestConfig::new().shared(&[CsafDocumentCategory::CsafSecurityAdvisory]);

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_27_13
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_27_13_fixed_products(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_27_13() {
        let case_exact_product_version = Err(vec![create_missing_affected_products_error("CSAFPID-9080700", 0, 0)]);
        //let case_relations = Err(vec![create_missing_affected_products_error("CSAFPID-9080705", 0, 0)]);

        TESTS_2_1.test_6_1_27_13.expect(
            case_exact_product_version.clone(),
            case_exact_product_version.clone(),
            case_exact_product_version,
            Ok(()), // ToDo this is an invalid case but blocked due to relationship restructuring
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
