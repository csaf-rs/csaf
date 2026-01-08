use crate::csaf_traits::{
    CsafTrait, CsafVersion, DocumentTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait,
};
use crate::validation::ValidationError;
use packageurl::PackageUrl;
use regex::Regex;
use std::str::FromStr;
use std::sync::LazyLock;

static PURL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^pkg:[A-Za-z.\-+][A-Za-z0-9.\-+]*/.+").unwrap());

fn get_purl_instance_path_substring(csaf_version: &CsafVersion) -> &'static str {
    match csaf_version {
        CsafVersion::X20 => "purl",
        CsafVersion::X21 => "purls",
    }
}

fn generate_purl_regex_error(csaf_version: &CsafVersion, purl_str: &str, path: &str, index: usize) -> ValidationError {
    ValidationError {
        message: format!("PURL doesn't comply with CSAF PURL regex: {}", purl_str),
        instance_path: format!(
            "{}/product_identification_helper/{}/{}",
            path,
            get_purl_instance_path_substring(csaf_version),
            index
        ),
    }
}

fn generate_purl_format_error(
    csaf_version: &CsafVersion,
    purl_str: &str,
    error_msg: &str,
    path: &str,
    index: usize,
) -> ValidationError {
    ValidationError {
        message: format!("Invalid PURL format: {}, Error: {}", purl_str, error_msg),
        instance_path: format!(
            "{}/product_identification_helper/{}/{}",
            path,
            get_purl_instance_path_substring(csaf_version),
            index
        ),
    }
}

/// 6.1.13 PURL
/// Checks the validity of PURLs in the document. Validation is done via a Regex specified in the standard and
/// via the packageurl::purl parser.
pub fn test_6_1_13_purl(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    if let Some(product_tree) = doc.get_product_tree() {
        let version = doc.get_document().get_csaf_version();
        product_tree.visit_all_products(&mut |product, path| {
            if let Some(helper) = product.get_product_identification_helper() {
                if let Some(purls) = helper.get_purls() {
                    for (i, purl_str) in purls.iter().enumerate() {
                        // Check against PURL regex
                        if !PURL_REGEX.is_match(purl_str) {
                            errors
                                .get_or_insert_with(Vec::new)
                                .push(generate_purl_regex_error(version, purl_str, path, i));
                            continue;
                        }
                        // Parse the PURL
                        if let Err(e) = PackageUrl::from_str(purl_str) {
                            errors.get_or_insert_with(Vec::new).push(generate_purl_format_error(
                                version,
                                purl_str,
                                &e.to_string(),
                                path,
                                i,
                            ));
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
        let errors_x20 = HashMap::from([
            (
                "01",
                vec![generate_purl_format_error(
                    &CsafVersion::X20,
                    "pkg:maven/@1.3.4",
                    "missing name",
                    "/product_tree/full_product_names/0",
                    0,
                )],
            ),
            (
                "02",
                vec![generate_purl_format_error(
                    &CsafVersion::X20,
                    "pkg:oci/com.example/product-A@sha256%3Add134261219b2",
                    "no namespace allowed for type \"oci\"",
                    "/product_tree/full_product_names/0",
                    0,
                )],
            ),
        ]);
        let errors_x21 = HashMap::from([
            (
                "01",
                vec![generate_purl_format_error(
                    &CsafVersion::X21,
                    "pkg:maven/@1.3.4",
                    "missing name",
                    "/product_tree/full_product_names/0",
                    0,
                )],
            ),
            (
                "02",
                vec![generate_purl_format_error(
                    &CsafVersion::X21,
                    "pkg:oci/com.example/product-A@sha256%3Add134261219b2",
                    "no namespace allowed for type \"oci\"",
                    "/product_tree/full_product_names/0",
                    0,
                )],
            ),
        ]);
        run_csaf20_tests("13", test_6_1_13_purl, errors_x20);
        run_csaf21_tests("13", test_6_1_13_purl, errors_x21);
    }
}
