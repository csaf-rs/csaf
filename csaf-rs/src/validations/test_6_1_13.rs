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
        message: format!("PURL doesn't comply with CSAF PURL regex: {purl_str}"),
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
        message: format!("Invalid PURL format: {purl_str}, Error: {error_msg}"),
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
            if let Some(helper) = product.get_product_identification_helper()
                && let Some(purls) = helper.get_purls()
            {
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
        // CSAF 2.0 has 1 test case (01)
        TESTS_2_0.test_6_1_13.expect(Err(vec![generate_purl_format_error(
            &CsafVersion::X20,
            "pkg:maven/@1.3.4",
            "missing name",
            "/product_tree/full_product_names/0",
            0,
        )]));

        // CSAF 2.1 has 4 test cases (01, 02, 11, 12)
        TESTS_2_1.test_6_1_13.expect(
            Err(vec![generate_purl_format_error(
                &CsafVersion::X21,
                "pkg:maven/@1.3.4",
                "missing name",
                "/product_tree/full_product_names/0",
                0,
            )]),
            Err(vec![generate_purl_format_error(
                &CsafVersion::X21,
                "pkg:oci/com.example/product-A@sha256%3Add134261219b2",
                "no namespace allowed for type \"oci\"",
                "/product_tree/full_product_names/0",
                0,
            )]),
            Ok(()), // case_11
            Ok(()), // case_12
        );
    }
}
