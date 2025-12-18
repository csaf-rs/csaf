use crate::csaf_traits::{CsafTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait};
use crate::validation::ValidationError;
use packageurl::PackageUrl;
use regex::Regex;
use std::str::FromStr;
use std::sync::LazyLock;

static PURL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^pkg:[A-Za-z.\-+][A-Za-z0-9.\-+]*/.+").unwrap());

/// Generates a validation error for a PURL that doesn't comply with CSAF PURL regex.
///
/// Creates a `ValidationError` indicating that a PURL string does not match
/// the required CSAF PURL regex pattern.
///
/// # Arguments
///
/// * `purl_str` - The PURL string that failed regex validation
/// * `path` - The path to the product in the product tree
/// * `index` - The index of the PURL in the purls array
///
/// # Returns
///
/// A `ValidationError` instance for this regex mismatch
fn generate_purl_regex_error(purl_str: &str, path: &str, index: usize) -> ValidationError {
    ValidationError {
        message: format!("PURL doesn't comply with CSAF PURL regex: {}", purl_str),
        instance_path: format!("{}/product_identification_helper/purls/{}", path, index),
    }
}

/// Generates a validation error for an invalid PURL format.
///
/// Creates a `ValidationError` indicating that a PURL string has an invalid format
/// and cannot be parsed according to the PackageUrl specification.
///
/// # Arguments
///
/// * `purl_str` - The PURL string that failed parsing
/// * `error_msg` - The error message from the parser
/// * `path` - The path to the product in the product tree
/// * `index` - The index of the PURL in the purls array
///
/// # Returns
///
/// A `ValidationError` instance for this parsing error
fn generate_purl_format_error(purl_str: &str, error_msg: &str, path: &str, index: usize) -> ValidationError {
    ValidationError {
        message: format!("Invalid PURL format: {}, Error: {}", purl_str, error_msg),
        instance_path: format!("{}/product_identification_helper/purls/{}", path, index),
    }
}

pub fn test_6_1_13_purl(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    if let Some(product_tree) = doc.get_product_tree() {
        product_tree.visit_all_products(&mut |product, path| {
            if let Some(helper) = product.get_product_identification_helper() {
                if let Some(purls) = helper.get_purls() {
                    for (i, purl_str) in purls.iter().enumerate() {
                        // Check against PURL regex
                        if !PURL_REGEX.is_match(purl_str) {
                            errors.get_or_insert_with(Vec::new).push(generate_purl_regex_error(purl_str, &path, i));
                            continue;
                        }
                        // Parse the PURL
                        if let Err(e) = PackageUrl::from_str(purl_str) {
                            errors.get_or_insert_with(Vec::new).push(generate_purl_format_error(purl_str, &e.to_string(), &path, i));
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
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_13() {
        let errors = HashMap::from([
            ("01", vec![generate_purl_format_error("pkg:maven/@1.3.4", "missing name", "/product_tree/full_product_names/0", 0)]),
            ("02", vec![generate_purl_format_error("pkg:oci/com.example/product-A@sha256%3Add134261219b2", "no namespace allowed for type \"oci\"", "/product_tree/full_product_names/0", 0)]),
        ]);
        run_csaf20_tests("13", test_6_1_13_purl, errors.clone());
        run_csaf21_tests("13", test_6_1_13_purl, errors);
    }
}
