use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::validation::{TestFinding, TestFindingData};
use std::sync::LazyLock;

/// 6.2.11 Missing Canonical URL
///
/// It SHALL be tested that the CSAF document has a canonical URL.
/// 
/// [`filter_canonical_url_candidates`](crate::csaf_traits::filter_canonical_url_candidates)
/// is used to determine whether a canonical URL is present in the document's references.
///
/// The warning message and instance path distinguish between there being no references at all and
/// there being references but none of them fulfilling the canonical URL requirements.
pub fn test_6_2_11_missing_canonical_url(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let document = doc.get_document();
    if document.get_references().is_none() {
        return Err(vec![MISSING_CANONICAL_URL_NO_REFS.clone()]);
    }
    if !document.has_canonical_url() {
        return Err(vec![MISSING_CANONICAL_URL_NO_MATCH.clone()]);
    }
    Ok(())
}

static MISSING_CANONICAL_URL_NO_MATCH: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Warning(TestFindingData {
        message: "Document is missing a canonical URL, as no reference fulfills the requirements ".to_string(),
        instance_path: "/document/references".to_string(),
    })
});

static MISSING_CANONICAL_URL_NO_REFS: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Warning(TestFindingData {
        message: "Document is missing a canonical URL, as there are no references".to_string(),
        instance_path: "/document".to_string(),
    })
});

crate::test_validation::impl_validator!(ValidatorForTest6_2_11, test_6_2_11_missing_canonical_url);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_2_11 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_11 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_11() {
        let no_canonical_url = Err(vec![MISSING_CANONICAL_URL_NO_MATCH.clone()]);
        let no_refs = Err(vec![MISSING_CANONICAL_URL_NO_REFS.clone()]);

        TESTS_2_0.test_6_2_11.expect(ExpectedResults_2_0 {
            case_01: no_canonical_url.clone(),
            case_11: Ok(()),
        });

        // Failing test cases:
        // 01  - URL does not end with the canonical filename
        // 02  - invalid character in filename is not replaced
        // 03  - filename is not converted to lowercase
        // 04  - URL has no path separator before the canonical filename
        // 05  - URL has an empty hostname
        // s01 - no references
        // s02 - external reference category instead of self
        // s03 - HTTP URL instead of HTTPS
        // s04 - no single reference satisfies all conditions
        // s05 - missing reference category
        // s06 - URLs with query parameter and fragment after the canonical filename

        // Valid test cases:
        // 11  - canonical URL
        // 12  - invalid character in URL path but outside the filename
        // 13  - canonical URL in a different path
        // s11 - one matching reference among non-matching references
        // s12 - multiple invalid character sequences in the tracking ID
        // s13 - multiple matching references
        TESTS_2_1.test_6_2_11.expect(ExpectedResults_2_1 {
            case_01: no_canonical_url.clone(),
            case_02: no_canonical_url.clone(),
            case_03: no_canonical_url.clone(),
            case_04: no_canonical_url.clone(),
            case_05: no_canonical_url.clone(),
            case_s01: no_refs.clone(),
            case_s02: no_canonical_url.clone(),
            case_s03: no_canonical_url.clone(),
            case_s04: no_canonical_url.clone(),
            case_s05: no_canonical_url.clone(),
            case_s06: no_canonical_url,
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
            case_s11: Ok(()),
            case_s12: Ok(()),
            case_s13: Ok(()),
        });
    }
}
