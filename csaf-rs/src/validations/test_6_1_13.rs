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

fn generate_purl_format_error_message(
    csaf_version: &CsafVersion,
    purl_str: &str,
    error: PurlParseError,
    path: &str,
    index: usize,
) -> ValidationError {
    ValidationError {
        message: format!("Invalid PURL format: {purl_str}, Error: {error}"),
        instance_path: format!(
            "{}/product_identification_helper/{}/{}",
            path,
            get_purl_instance_path_substring(csaf_version),
            index
        ),
    }
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum PurlParseError {
    #[error("invalid scheme: {0:?}")]
    InvalidScheme(String),
    #[error("missing scheme")]
    MissingScheme,
    #[error("invalid type: {0:?}")]
    InvalidType(String),
    #[error("missing type")]
    MissingType,
    #[error("invalid key: {0:?}")]
    InvalidKey(String),
    #[error("missing name")]
    MissingName,
    #[error("no namespace allowed for type {0:?}")]
    TypeProhibitsNamespace(String),
    #[error("invalid namespace component: {0:?}")]
    InvalidNamespaceComponent(String),
    #[error("invalid subpath segment: {0:?}")]
    InvalidSubpathSegment(String),
    #[error("utf-8 decoding failed")]
    DecodingError,
    #[error("CSAF error")]
    CsafError,
}

impl From<packageurl::Error> for PurlParseError {
    fn from(err: packageurl::Error) -> Self {
        match err {
            packageurl::Error::InvalidScheme(scheme) => PurlParseError::InvalidScheme(scheme),
            packageurl::Error::InvalidType(typ) => PurlParseError::InvalidType(typ),
            packageurl::Error::InvalidKey(key) => PurlParseError::InvalidKey(key),
            packageurl::Error::MissingName => PurlParseError::MissingName,
            packageurl::Error::TypeProhibitsNamespace(typ) => PurlParseError::TypeProhibitsNamespace(typ),
            packageurl::Error::InvalidNamespaceComponent(component) => {
                PurlParseError::InvalidNamespaceComponent(component)
            },
            packageurl::Error::MissingScheme => PurlParseError::MissingScheme,
            packageurl::Error::MissingType => PurlParseError::MissingType,
            packageurl::Error::InvalidSubpathSegment(segment) => PurlParseError::InvalidSubpathSegment(segment),
            packageurl::Error::DecodingError(_) => PurlParseError::DecodingError,
        }
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
                    // Check against PURL spec, because it has to be valid
                    if let Err(e) = PackageUrl::from_str(purl_str) {
                        errors
                            .get_or_insert_with(Vec::new)
                            .push(generate_purl_format_error_message(version, purl_str, e.into(), path, i));
                        continue;
                    }

                    // Check against regex from standard, because it is more strict in some ways (e.g. it prohibits double // after scheme)
                    if !PURL_REGEX.is_match(purl_str) {
                        errors
                            .get_or_insert_with(Vec::new)
                            .push(generate_purl_format_error_message(
                                version,
                                purl_str,
                                PurlParseError::CsafError,
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
        TESTS_2_0.test_6_1_13.expect(
            Err(vec![generate_purl_format_error_message(
                &CsafVersion::X20,
                "pkg:maven/@1.3.4",
                PurlParseError::MissingName,
                "/product_tree/full_product_names/0",
                0,
            )]),
            Err(vec![
                generate_purl_format_error_message(
                    // missing scheme
                    &CsafVersion::X20,
                    "somepackage",
                    PurlParseError::MissingScheme,
                    "/product_tree/full_product_names/0",
                    0,
                ),
                generate_purl_format_error_message(
                    // missing scheme because colon can't be encoded
                    &CsafVersion::X20,
                    "pkg%3Amaven%2Flogging",
                    PurlParseError::MissingScheme,
                    "/product_tree/full_product_names/1",
                    0,
                ),
            ]),
            Err(vec![generate_purl_format_error_message(
                // invalid scheme
                &CsafVersion::X20,
                "http://maven/logging@1.3.4",
                PurlParseError::InvalidScheme("http".to_string()),
                "/product_tree/full_product_names/0",
                0,
            )]),
            Err(vec![generate_purl_format_error_message(
                // prohibited double / after scheme
                &CsafVersion::X20,
                "pkg://maven/logging@1.3.4",
                PurlParseError::CsafError,
                "/product_tree/full_product_names/0",
                0,
            )]),
            Err(vec![generate_purl_format_error_message(
                // missing type
                &CsafVersion::X20,
                "pkg:/somepackage",
                PurlParseError::MissingType,
                "/product_tree/full_product_names/0",
                0,
            )]),
            Err(vec![generate_purl_format_error_message(
                // missing type
                &CsafVersion::X20,
                "pkg:ma%3Fen/somepackage",
                PurlParseError::InvalidType("ma%3Fen".to_string()),
                "/product_tree/full_product_names/0",
                0,
            )]),
        );

        // CSAF 2.1 has 4 test cases (01, 02, 11, 12)
        TESTS_2_1.test_6_1_13.expect(
            Err(vec![generate_purl_format_error_message(
                &CsafVersion::X21,
                "pkg:maven/@1.3.4",
                PurlParseError::MissingName,
                "/product_tree/full_product_names/0",
                0,
            )]),
            Err(vec![generate_purl_format_error_message(
                &CsafVersion::X21,
                "pkg:oci/com.example/product-A@sha256%3Add134261219b2",
                PurlParseError::TypeProhibitsNamespace("oci".to_string()),
                "/product_tree/full_product_names/0",
                0,
            )]),
            Err(vec![
                generate_purl_format_error_message(
                    // missing scheme
                    &CsafVersion::X21,
                    "somepackage",
                    PurlParseError::MissingScheme,
                    "/product_tree/full_product_names/0",
                    0,
                ),
                generate_purl_format_error_message(
                    // missing scheme because colon can't be encoded
                    &CsafVersion::X21,
                    "pkg%3Amaven%2Flogging",
                    PurlParseError::MissingScheme,
                    "/product_tree/full_product_names/1",
                    0,
                ),
            ]),
            Err(vec![generate_purl_format_error_message(
                // invalid scheme
                &CsafVersion::X21,
                "http://maven/logging@1.3.4",
                PurlParseError::InvalidScheme("http".to_string()),
                "/product_tree/full_product_names/0",
                0,
            )]),
            Err(vec![generate_purl_format_error_message(
                // prohibited double / after scheme
                &CsafVersion::X21,
                "pkg://maven/logging@1.3.4",
                PurlParseError::CsafError,
                "/product_tree/full_product_names/0",
                0,
            )]),
            Err(vec![generate_purl_format_error_message(
                // missing type
                &CsafVersion::X21,
                "pkg:/somepackage",
                PurlParseError::MissingType,
                "/product_tree/full_product_names/0",
                0,
            )]),
            Err(vec![generate_purl_format_error_message(
                // missing type
                &CsafVersion::X21,
                "pkg:ma%3Fen/somepackage",
                PurlParseError::InvalidType("ma%3Fen".to_string()),
                "/product_tree/full_product_names/0",
                0,
            )]),
            Ok(()), // case_11
            Ok(()), // case_12
        );
    }
}
