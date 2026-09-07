use crate::csaf::types::purl::csaf_purl::CsafPurl;
use crate::csaf_traits::{CsafTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait};
use crate::validation::{IntoTestFindingError, TestFinding};

/// 6.1.13 PURL
///
/// Checks the validity of PURLs in the document. There are different regexes for the `purl` / `purls` field in CSAF 2.0 and 2.1.
/// These are enforced during deserialization into the schema types. [CsafPurl] wraps the schema types
/// and parses the PURL string into a `packageurl::PackageUrl` struct, which performs the actual validation according to the PURL specification.
///
/// In this test, we just check if any purls are [CsafPurl::Invalid] and report the errors found.
/// If a purl failed the respective regex, the schema validation failed already, so this test (currently) does not run.
pub fn test_6_1_13_purl(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let mut errors: Option<Vec<TestFinding>> = None;

    if let Some(product_tree) = doc.get_product_tree() {
        product_tree.visit_all_products(&mut |product, path| {
            if let Some(helper) = product.get_product_identification_helper()
                && let Some(purls) = helper.get_purls()
            {
                for (i_p, purl) in purls.into_iter().enumerate() {
                    if let CsafPurl::Invalid(e) = purl {
                        errors
                            .get_or_insert_default()
                            .push(e.into_test_finding_error(&helper.get_purls_json_path(path, i_p)))
                    }
                }
            }
        });
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(ValidatorForTest6_1_13, test_6_1_13_purl);

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use super::*;
    use crate::csaf::types::purl::{PurlParseError, PurlParseErrorKind};
    use crate::csaf2_0::testcases::ExpectedResults_6_1_13 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_13 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use crate::validation::IntoTestFindingError;
    use crate::validations::test_6_1_13::tests::PurlPath::{Purl2_0, Purl2_1};

    enum PurlPath {
        Purl2_0,
        Purl2_1(u32),
    }
    impl Display for PurlPath {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                self::Purl2_0 => write!(f, "purl"),
                self::Purl2_1(idx) => write!(f, "purls/{idx}"),
            }
        }
    }

    #[test]
    fn test_test_6_1_13() {
        // Shared expected results (only "purl"/"purls" field name differs between 2.0 and 2.1)
        let case_01_missing_name = |purl_path: PurlPath| -> Result<(), Vec<TestFinding>> {
            Err(vec![
                PurlParseError::new_for_test("pkg:maven/@1.3.4", PurlParseErrorKind::MissingName)
                    .into_test_finding_error(&format!(
                        "/product_tree/full_product_names/0/product_identification_helper/{purl_path}"
                    )),
            ])
        };

        let case_02_or_s06_type_prohibits_namespace = |purl_path: PurlPath| -> Result<(), Vec<TestFinding>> {
            Err(vec![
                PurlParseError::new_for_test(
                    "pkg:oci/com.example/product-A@sha256%3Add134261219b2",
                    PurlParseErrorKind::TypeProhibitsNamespace("oci".to_string()),
                )
                .into_test_finding_error(&format!(
                    "/product_tree/full_product_names/0/product_identification_helper/{purl_path}"
                )),
            ])
        };

        // Case 11/S11: valid purl
        // Case 12/S12: valid purl with repo url

        TESTS_2_0.test_6_1_13.expect(ExpectedResults_2_0 {
            case_01: case_01_missing_name(Purl2_0),
            // cases s01-s05 are broken by schema check and would result in an OK here, refactor after lienent parsing
            case_s06: case_02_or_s06_type_prohibits_namespace(Purl2_0),
            case_s11: Ok(()),
            case_s12: Ok(()),
        });

        TESTS_2_1.test_6_1_13.expect(ExpectedResults_2_1 {
            case_01: case_01_missing_name(Purl2_1(0)),
            case_02: case_02_or_s06_type_prohibits_namespace(Purl2_1(0)),
            case_11: Ok(()),
            case_12: Ok(()),
        });
    }
}
