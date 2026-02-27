use crate::csaf_traits::{CsafTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait};
use crate::validation::ValidationError;
use packageurl::PackageUrl;
use std::str::FromStr;

fn create_invalid_purl_error(purl_str: &str, path: &str, index: usize) -> ValidationError {
    ValidationError {
        message: format!("Invalid PURL format: {purl_str}"),
        instance_path: format!("{path}/product_identification_helper/purls/{index}"),
    }
}

fn create_purl_consistency_error(path: &str, index: usize) -> ValidationError {
    ValidationError {
        message: String::from("PURLs within the same product_identification_helper must only differ in qualifiers"),
        instance_path: format!("{path}/product_identification_helper/purls/{index}"),
    }
}

/// 6.1.42 PURL Consistency
/// Checks the consistency of PURLs within the same product_identification_helper. PURLs must only differ in qualifiers.
pub fn test_6_1_42_purl_consistency(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    if let Some(product_tree) = doc.get_product_tree() {
        product_tree.visit_all_products(&mut |product, path| {
            if let Some(helper) = product.get_product_identification_helper()
                && let Some(purls) = helper.get_purls()
            {
                // break early if there are 0 or 1 PURLs, as consistency is not an issue
                if purls.len() <= 1 {
                    return;
                }

                let mut base: Option<String> = None;

                for (i, purl_str) in purls.iter().enumerate() {
                    // Parse the PURL
                    let mut purl = match PackageUrl::from_str(purl_str) {
                        Ok(p) => p,
                        Err(_) => {
                            // ToDo create percondition failed warning
                            continue;
                        },
                    };

                    // Strip qualifiers
                    let current_value = purl.clear_qualifiers().to_string();

                    if let Some(ref base_value) = base {
                        // Must always match
                        if current_value != *base_value {
                            errors
                                .get_or_insert_with(Vec::new)
                                .push(create_purl_consistency_error(path, i));
                        }
                    } else {
                        // The first PURL becomes the base for comparison
                        base = Some(current_value);
                    }
                }
            }
        });
    }

    errors.map_or(Ok(()), Err)
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_42
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_42_purl_consistency(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_42() {
        TESTS_2_1.test_6_1_42.expect(
            Err(vec![create_purl_consistency_error(
                "/product_tree/full_product_names/0",
                1,
            )]),
            Err(vec![create_purl_consistency_error(
                "/product_tree/branches/0/branches/0/branches/0/product",
                2,
            )]),
            Ok(()),
            Ok(()),
        );
    }
}
