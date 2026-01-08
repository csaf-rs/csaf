use std::sync::LazyLock;

use crate::csaf_traits::{CsafTrait, CsafVersion, DocumentCategory, DocumentTrait};
use crate::validation::ValidationError;

static VULNERABILITIES_ERROR: LazyLock<ValidationError> = LazyLock::new(|| ValidationError {
    message: "Document with category 'csaf_informational_advisory' must not have a '/vulnerabilities' element"
        .to_string(),
    instance_path: "/vulnerabilities".to_string(),
});

/// 6.1.27.3 Vulnerabilities
///
/// This test only applies to documents with `/document/category` with value `csaf_informational_advisory` for
/// `/document/csaf_version` `2.0` and additionally to documents with `/document/category` with
/// value `csaf_withdrawn` and `csaf_superseded` for `/document/csaf_version` `2.1`.
///
/// Documents with this category must not have a `/vulnerabilities` element.
pub fn test_6_1_27_03_vulnerability(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    // check if document is relevant document category in csaf 2.0
    if *doc.get_document().get_csaf_version() == CsafVersion::X20
        && doc.get_document().get_category() != DocumentCategory::CsafInformationalAdvisory
    {
        return Ok(());
    }

    // check if document is relevant document category in csaf 2.1
    if *doc.get_document().get_csaf_version() == CsafVersion::X21 {
        let doc_category = doc.get_document().get_category();
        if doc_category != DocumentCategory::CsafInformationalAdvisory
            && doc_category != DocumentCategory::CsafWithdrawn
            && doc_category != DocumentCategory::CsafSuperseded
        {
            return Ok(());
        }
    }

    // return error if there are elements in /vulnerabilities
    if !doc.get_vulnerabilities().is_empty() {
        return Err(vec![VULNERABILITIES_ERROR.clone()]);
    }

    Ok(())
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_27_3
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_27_03_vulnerability(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_27_3
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_27_03_vulnerability(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_27_03() {
        let err = Err(vec![VULNERABILITIES_ERROR.clone()]);

        TESTS_2_0.test_6_1_27_3.expect(err.clone());
        TESTS_2_1
            .test_6_1_27_3
            .expect(err.clone(), err.clone(), err.clone(), Ok(()), Ok(()), Ok(()));
    }
}
