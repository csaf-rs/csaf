use crate::csaf::csaf_traits::{CsafTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait};
use crate::csaf::validation::ValidationError;
use packageurl::PackageUrl;
use regex::Regex;
use std::str::FromStr;
use std::sync::LazyLock;

static PURL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^pkg:[A-Za-z.\-+][A-Za-z0-9.\-+]*/.+").unwrap());
pub fn test_6_1_13_purl(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    if let Some(product_tree) = doc.get_product_tree() {
        product_tree.visit_all_products(&mut |product, path| {
            if let Some(helper) = product.get_product_identification_helper() {
                if let Some(purls) = helper.get_purls() {
                    for (i, purl_str) in purls.iter().enumerate() {
                        // Check against PURL regex
                        if !PURL_REGEX.is_match(purl_str) {
                            errors.get_or_insert_with(Vec::new).push(ValidationError {
                                message: format!("PURL doesn't comply with CSAF PURL regex: {}", purl_str),
                                instance_path: format!("{}/product_identification_helper/purls/{}", path, i),
                            });
                            continue;
                        }
                        // Parse the PURL
                        if let Err(e) = PackageUrl::from_str(purl_str) {
                            errors.get_or_insert_with(Vec::new).push(ValidationError {
                                message: format!("Invalid PURL format: {}, Error: {}", purl_str, e),
                                instance_path: format!("{}/product_identification_helper/purls/{}", path, i),
                            });
                        }
                    }
                }
            }
            Ok(())
        })?;
    }

    errors.map_or(Ok(()), Err)
}

#[cfg(test)]
mod tests {
    use crate::csaf::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::csaf::validation::ValidationError;
    use crate::csaf::validations::test_6_1_13::test_6_1_13_purl;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_13() {
        let errors = HashMap::from([
            ("01", vec![ValidationError {
                message: "Invalid PURL format: pkg:maven/@1.3.4, Error: missing name".to_string(),
                instance_path: "/product_tree/full_product_names/0/product_identification_helper/purls/0".to_string(),
            }]),
            ("02", vec![ValidationError {
                message: "Invalid PURL format: pkg:oci/com.example/product-A@sha256%3Add134261219b2, Error: no namespace allowed for type \"oci\"".to_string(),
                instance_path: "/product_tree/full_product_names/0/product_identification_helper/purls/0".to_string(),
            }]),
        ]);
        run_csaf20_tests("13", test_6_1_13_purl, errors.clone());
        run_csaf21_tests("13", test_6_1_13_purl, errors);
    }
}
