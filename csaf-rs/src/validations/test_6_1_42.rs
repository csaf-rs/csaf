use crate::csaf_traits::{CsafTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait};
use crate::validation::ValidationError;
use packageurl::PackageUrl;
use std::str::FromStr;

fn create_invalid_purl_error(purl_str: &str, path: &str, index: usize) -> ValidationError {
    ValidationError {
        message: format!("Invalid PURL format: {}", purl_str),
        instance_path: format!("{}/product_identification_helper/purls/{}", path, index),
    }
}

fn create_purl_consistency_error(path: &str, index: usize) -> ValidationError {
    ValidationError {
        message: String::from("PURLs within the same product_identification_helper must only differ in qualifiers"),
        instance_path: format!("{}/product_identification_helper/purls/{}", path, index),
    }
}

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
                                errors
                                    .get_or_insert_with(Vec::new)
                                    .push(create_invalid_purl_error(purl_str, path, i));
                                continue;
                            },
                        };

                        // Strip qualifiers
                        let current_parts = purl.clear_qualifiers().to_string();

                        if let Some(ref base) = base_parts {
                            // Must always match
                            if current_parts != *base {
                                errors
                                    .get_or_insert_with(Vec::new)
                                    .push(create_purl_consistency_error(path, i));
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
    use super::*;
    use crate::test_helper::run_csaf21_tests;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_42() {
        run_csaf21_tests(
            "42",
            test_6_1_42_purl_consistency,
            HashMap::from([
                (
                    "01",
                    vec![create_purl_consistency_error("/product_tree/full_product_names/0", 1)],
                ),
                (
                    "02",
                    vec![create_purl_consistency_error(
                        "/product_tree/branches/0/branches/0/branches/0/product",
                        2,
                    )],
                ),
            ]),
        );
    }
}
