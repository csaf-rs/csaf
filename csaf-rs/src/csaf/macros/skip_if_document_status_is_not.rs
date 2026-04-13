/// Macro to skip a validation test if the document status is NOT one of the provided variants.
///
/// If the document's status does not match any of the provided variants,
/// the macro returns `Ok(())` early (i.e. the test is skipped).
///
/// TODO #409 this will be wasSkipped later, now unified in one place :)
///
/// # Usage
///
/// ```ignore
/// let status = tracking.get_status();
/// skip_if_document_status_is_not!(status, Final, Interim);
/// ```
///
/// Expands to the equivalent of:
/// ```ignore
/// if !matches!(status, DocumentStatus::Final | DocumentStatus::Interim) {
///     return Ok(());
/// }
/// ```
macro_rules! skip_if_document_status_is_not {
    ($status:expr, $($variant:ident),+ $(,)?) => {
        if !matches!(
            $status,
            $(crate::schema::csaf2_1::schema::DocumentStatus::$variant)|+
        ) {
            return Ok(());
        }
    };
}

pub(crate) use skip_if_document_status_is_not;

#[cfg(test)]
mod tests {
    use crate::schema::csaf2_1::schema::DocumentStatus;

    fn run_skip_if_is_not_interim_or_final(status: DocumentStatus) -> Result<(), ()> {
        skip_if_document_status_is_not!(status, Interim, Final);
        Err(()) // err is just used as a marker for "did not skip"
    }

    fn run_skip_if_is_not_draft(status: DocumentStatus) -> Result<(), ()> {
        skip_if_document_status_is_not!(status, Draft);
        Err(()) // err is just used as a marker for "did not skip"
    }

    #[test]
    fn test_doc_status_final() {
        assert_eq!(run_skip_if_is_not_interim_or_final(DocumentStatus::Final), Err(()));
        assert_eq!(run_skip_if_is_not_draft(DocumentStatus::Final), Ok(()));
    }

    #[test]
    fn test_doc_status_interim() {
        assert_eq!(run_skip_if_is_not_interim_or_final(DocumentStatus::Interim), Err(()));
        assert_eq!(run_skip_if_is_not_draft(DocumentStatus::Interim), Ok(()));
    }

    #[test]
    fn test_doc_status_draft() {
        assert_eq!(run_skip_if_is_not_interim_or_final(DocumentStatus::Draft), Ok(()));
        assert_eq!(run_skip_if_is_not_draft(DocumentStatus::Draft), Err(()));
    }
}
