use crate::csaf::types::purl::csaf_purl::CsafPurl;
use crate::csaf_traits::{CsafTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait};
use crate::validation::ValidationError;

/// 6.1.13 PURL
///
/// Checks the validity of PURLs in the document. There are different regexes for the `purl` / `purls` field in CSAF 2.0 and 2.1.
/// These are enforced during deserialization into the schema types. [CsafPurl] wraps the schema types
/// and parses the PURL string into a `packageurl::PackageUrl` struct, which performs the actual validation according to the PURL specification.
///
/// In this test, we just check if any purls are [CsafPurl::Invalid] and report the errors found.
/// If a purl failed the respective regex, the schema validation failed already, so this test (currently) does not run.
pub fn test_6_1_13_purl(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    if let Some(product_tree) = doc.get_product_tree() {
        product_tree.visit_all_products(&mut |product, path| {
            if let Some(helper) = product.get_product_identification_helper()
                && let Some(purls) = helper.get_purls()
            {
                for (i_p, purl) in purls.into_iter().enumerate() {
                    if let CsafPurl::Invalid(e) = purl {
                        errors
                            .get_or_insert_default()
                            .push(e.into_validation_error(helper.get_purls_json_path(path, i_p)))
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
    use super::*;
    use crate::csaf::types::purl::{PurlParseError, PurlParseErrorKind};
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_13() {
        // Shared expected results (only "purl"/"purls" field name differs between 2.0 and 2.1)
        let case_01_missing_name = |field: &str, idx: &str| -> Result<(), Vec<ValidationError>> {
            Err(vec![
                PurlParseError::new_for_test("pkg:maven/@1.3.4", PurlParseErrorKind::MissingName)
                    .into_validation_error(format!(
                        "/product_tree/full_product_names/0/product_identification_helper/{field}{idx}"
                    )),
            ])
        };

        let case_02_or_s06_type_prohibits_namespace = |field: &str, idx: &str| -> Result<(), Vec<ValidationError>> {
            Err(vec![
                PurlParseError::new_for_test(
                    "pkg:oci/com.example/product-A@sha256%3Add134261219b2",
                    PurlParseErrorKind::TypeProhibitsNamespace("oci".to_string()),
                )
                .into_validation_error(format!(
                    "/product_tree/full_product_names/0/product_identification_helper/{field}{idx}"
                )),
            ])
        };

        // Case 11/S11: valid purl
        // Case 12/S12: valid purl with repo url

        TESTS_2_0.test_6_1_13.expect(
            case_01_missing_name("purl", ""),
            case_02_or_s06_type_prohibits_namespace("purl", ""),
            Ok(()),
            Ok(()),
        );

        TESTS_2_1.test_6_1_13.expect(
            case_01_missing_name("purls", "/0"),
            case_02_or_s06_type_prohibits_namespace("purls", "/0"),
            Ok(()),
            Ok(()),
        );
    }
}
