use crate::csaf_traits::{CsafTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait};
use crate::validation::ValidationError;
use packageurl::PackageUrl;
use std::str::FromStr;

pub fn test_6_1_42_purl_consistency(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    if let Some(product_tree) = doc.get_product_tree() {
        product_tree.visit_all_products(&mut |product, path| {
            if let Some(helper) = product.get_product_identification_helper() {
                if let Some(purls) = helper.get_purls() {
                    if purls.len() <= 1 {
                        return;
                    }

                    let mut base_parts: Option<String> = None;

                    for (i, purl_str) in purls.iter().enumerate() {
                        // Parse the PURL
                        let mut purl = match PackageUrl::from_str(purl_str) {
                            Ok(p) => p,
                            Err(_) => {
                                errors.get_or_insert_with(Vec::new).push(ValidationError {
                                    message: format!("Invalid PURL format: {}", purl_str),
                                    instance_path: format!("{}/product_identification_helper/purls/{}", path, i),
                                });
                                continue;
                            }
                        };

                        // Strip qualifiers
                        let current_parts = purl.clear_qualifiers().to_string();

                        if let Some(ref base) = base_parts {
                            // Must always match
                            if current_parts != *base {
                                errors.get_or_insert_with(Vec::new).push(ValidationError {
                                    message: String::from("PURLs within the same product_identification_helper must only differ in qualifiers"),
                                    instance_path: format!("{}/product_identification_helper/purls/{}", path, i),
                                });
                            }
                        } else {
                            // The first PURL becomes the base for comparison
                            base_parts = Some(current_parts);
                        }
                    }
                }
            }
        });
    }

    errors.map_or(Ok(()), Err)
}

#[cfg(test)]
mod tests {
    use crate::test_helper::run_csaf21_tests;
    use crate::validation::ValidationError;
    use crate::validations::test_6_1_42::test_6_1_42_purl_consistency;
    use std::collections::HashMap;

    static ERROR_MESSAGE: &str = "PURLs within the same product_identification_helper must only differ in qualifiers";

    #[test]
    fn test_test_6_1_42() {
        run_csaf21_tests(
            "42",
            test_6_1_42_purl_consistency, HashMap::from([
                ("01", vec![ValidationError {
                    message: ERROR_MESSAGE.to_string(),
                    instance_path: "/product_tree/full_product_names/0/product_identification_helper/purls/1".to_string(),
                }]),
                ("02", vec![ValidationError {
                    message: ERROR_MESSAGE.to_string(),
                    instance_path: "/product_tree/branches/0/branches/0/branches/0/product/product_identification_helper/purls/2".to_string(),
                }]),
            ])
        );
    }
}
