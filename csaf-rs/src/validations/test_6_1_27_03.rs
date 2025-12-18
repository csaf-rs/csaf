use crate::csaf_traits::{CsafTrait, CsafVersion, DocumentCategory, DocumentTrait};
use crate::validation::ValidationError;

/// Creates a ValidationError for documents that should not have vulnerabilities
fn create_vulnerabilities_error() -> ValidationError {
    ValidationError {
        message: "Document with category 'csaf_informational_advisory' must not have a '/vulnerabilities' element"
            .to_string(),
        instance_path: "/vulnerabilities".to_string(),
    }
}

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
        return Err(vec![create_vulnerabilities_error()]);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_27_03() {
        let errors = HashMap::from([
            ("01", vec![create_vulnerabilities_error()]),
            ("02", vec![create_vulnerabilities_error()]),
            ("03", vec![create_vulnerabilities_error()]),
        ]);
        run_csaf20_tests("27-03", test_6_1_27_03_vulnerability, errors.clone());
        run_csaf21_tests("27-03", test_6_1_27_03_vulnerability, errors);
    }
}
