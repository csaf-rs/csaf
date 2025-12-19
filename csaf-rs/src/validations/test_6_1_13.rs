use crate::csaf_traits::{CsafTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait};
use crate::validation::ValidationError;
use packageurl::PackageUrl;
use regex::Regex;
use std::str::FromStr;
use std::sync::LazyLock;

static PURL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^pkg:[A-Za-z.\-+][A-Za-z0-9.\-+]*/.+").unwrap());

fn generate_purl_regex_error(purl_str: &str, path: &str, index: usize) -> ValidationError {
    ValidationError {
        message: format!("PURL doesn't comply with CSAF PURL regex: {}", purl_str),
        instance_path: format!("{}/product_identification_helper/purls/{}", path, index),
    }
}

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
                            errors
                                .get_or_insert_with(Vec::new)
                                .push(generate_purl_regex_error(purl_str, path, i));
                            continue;
                        }
                        // Parse the PURL
                        if let Err(e) = PackageUrl::from_str(purl_str) {
                            errors.get_or_insert_with(Vec::new).push(generate_purl_format_error(
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

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_13
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_13_purl(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_13
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_13_purl(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_13() {
        let case_01 = Err(vec![generate_purl_format_error(
            "pkg:maven/@1.3.4",
            "missing name",
            "/product_tree/full_product_names/0",
            0,
        )]);

        let case_02 = Err(vec![generate_purl_format_error(
            "pkg:oci/com.example/product-A@sha256%3Add134261219b2",
            "no namespace allowed for type \"oci\"",
            "/product_tree/full_product_names/0",
            0,
        )]);

        // CSAF 2.0 has 1 test case (01)
        TESTS_2_0.test_6_1_13.expect(case_01.clone());

        // CSAF 2.1 has 4 test cases (01, 02, 11, 12)
        TESTS_2_1.test_6_1_13.expect(
            case_01,
            case_02,
            Ok(()), // case_11
            Ok(()), // case_12
        );
    }
}
