use crate::csaf_traits::{CsafTrait, CsafVersion, DocumentTrait};

/// Checks if a CSAF document is version 2.0
///
/// Works with any type implementing CsafTrait
pub fn is_csaf_2_0<Doc: CsafTrait>(doc: &Doc) -> bool {
    doc.get_document().get_csaf_version() == &CsafVersion::X20
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf::raw::HasParsed;
    use crate::csaf2_0::loader::load_document as load_document_2_0;
    use crate::csaf2_1::loader::load_document as load_document_2_1;

    #[test]
    fn test_is_csaf_2_0_returns_true_for_v20() {
        let doc = load_document_2_0(
            "../csaf/csaf_2.0/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_0-2021-6-1-01-11.json",
        )
        .expect("Failed to load CSAF 2.0 test document");
        let parsed = doc.get_parsed().as_ref().expect("Failed to parse CSAF 2.0 document");
        assert!(is_csaf_2_0(parsed));
    }

    #[test]
    fn test_is_csaf_2_0_returns_false_for_v21() {
        let doc = load_document_2_1(
            "../csaf/csaf_2.1/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_1-2024-6-1-06-11.json",
        )
        .expect("Failed to load CSAF 2.1 test document");
        let parsed = doc.get_parsed().as_ref().expect("Failed to parse CSAF 2.1 document");
        assert!(!is_csaf_2_0(parsed));
    }
}
