use crate::csaf_traits::{CsafTrait, DocumentReferenceTrait, DocumentTrait, TrackingTrait};
use crate::schema::csaf2_1::schema::CategoryOfReference;
use crate::validation::ValidationError;
use regex::Regex;
use std::sync::LazyLock;

/// 6.2.11 Missing Canonical URL
///
/// `/document/references` must contain at least one item with:
/// - category = "self"
/// - url starts with "https://"
/// - url ends with the valid filename according to section 5.1
pub fn test_6_2_11_missing_canonical_url(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    // Get the expected filename from tracking ID
    let expected_filename = generate_filename(doc.get_document().get_tracking().get_id());

    // Check if any reference meets the criteria
    if let Some(references) = doc.get_document().get_references() {
        for reference in references {
            if CategoryOfReference::Self_ == *reference.get_category()
                && reference.get_url().starts_with("https://")
                && reference.get_url().ends_with(&expected_filename)
            {
                return Ok(());
            }
        }
    }

    Err(vec![MISSING_CANONICAL_URL.clone()])
}

/// Generate the valid filename according to section 5.1
fn generate_filename(tracking_id: &str) -> String {
    // Step 1: Convert to lowercase
    let lowercase_id = tracking_id.to_lowercase();

    // Step 2: Replace any character sequence not in [+\-a-z0-9] with single underscore
    static INVALID_CHARS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[^+\-a-z0-9]+").unwrap());
    let cleaned_id = INVALID_CHARS.replace_all(&lowercase_id, "_");

    // Step 3: Append .json
    format!("{cleaned_id}.json")
}

static MISSING_CANONICAL_URL: LazyLock<ValidationError> = LazyLock::new(|| ValidationError {
    message: "Document is missing a canonical URL".to_string(),
    instance_path: "/document/references".to_string(),
});

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_2_11
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_11_missing_canonical_url(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_2_11
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_11_missing_canonical_url(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_11() {
        let err = Err(vec![MISSING_CANONICAL_URL.clone()]);
        let ok = Ok(());

        // CSAF 2.0 has 2 test cases
        TESTS_2_0.test_6_2_11.expect(err.clone(), ok.clone());

        // CSAF 2.1 has 6 test cases
        TESTS_2_1
            .test_6_2_11
            .expect(err.clone(), err.clone(), err, ok.clone(), ok.clone(), ok);
    }

    #[test]
    fn test_generate_filename() {
        // Test examples from the spec
        assert_eq!(
            generate_filename("OASIS_CSAF_TC-CSAF_2.0-2021-6-2-11-01"),
            "oasis_csaf_tc-csaf_2_0-2021-6-2-11-01.json"
        );
        assert_eq!(generate_filename("2022_#01-A"), "2022_01-a.json");

        // Test that multiple consecutive invalid chars become single underscore
        assert_eq!(generate_filename("test###value"), "test_value.json");

        // Test valid characters are preserved
        assert_eq!(generate_filename("Test+123-456"), "test+123-456.json");
    }
}
