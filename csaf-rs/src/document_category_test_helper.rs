use crate::csaf_traits::{CsafVersion, DocumentCategory};

/// Configuration for tests that need to check document categories per CSAF version.
///
/// This struct allows defining which document categories a test applies to,
/// separately for CSAF 2.0 and CSAF 2.1 versions. Categories in `shared_categories`
/// apply to both versions.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DocumentCategoryTestConfig {
    /// Categories that apply to both CSAF 2.0 and 2.1 (optional)
    pub shared_categories: Option<&'static [DocumentCategory]>,
    /// Additional categories specific to CSAF 2.0 (optional)
    pub csaf20_categories: Option<&'static [DocumentCategory]>,
    /// Additional categories specific to CSAF 2.1 (optional)
    pub csaf21_categories: Option<&'static [DocumentCategory]>,
}

impl DocumentCategoryTestConfig {
    /// Creates a new empty DocumentCategoryTestConfig.
    pub const fn new() -> Self {
        Self {
            shared_categories: None,
            csaf20_categories: None,
            csaf21_categories: None,
        }
    }

    /// Sets categories that apply to both CSAF 2.0 and 2.1.
    pub const fn shared(mut self, categories: &'static [DocumentCategory]) -> Self {
        self.shared_categories = Some(categories);
        self
    }

    /// Sets additional categories specific to CSAF 2.0.
    pub const fn csaf20(mut self, categories: &'static [DocumentCategory]) -> Self {
        self.csaf20_categories = Some(categories);
        self
    }

    /// Sets additional categories specific to CSAF 2.1.
    pub const fn csaf21(mut self, categories: &'static [DocumentCategory]) -> Self {
        self.csaf21_categories = Some(categories);
        self
    }

    /// Checks if a profile test should run based on the CSAF version and document category.
    ///
    /// Returns `true` if the test should run (i.e., the test is relevant for the given document category
    /// for the given CSAF version).
    ///
    /// The check includes both shared categories and version-specific categories.
    pub fn matches_category_with_csaf_version(
        &self,
        csaf_version: &CsafVersion,
        document_category: &DocumentCategory,
    ) -> bool {
        // First check shared categories
        if let Some(shared) = self.shared_categories
            && shared.contains(document_category)
        {
            return true;
        }

        // Then check version-specific categories
        match csaf_version {
            CsafVersion::X20 => self
                .csaf20_categories
                .map(|cats| cats.contains(document_category))
                .unwrap_or_else(|| {
                    if self.shared_categories.is_none() {
                        panic!("Test applicability was checked for CSAF 2.0 on a config that does not contain CSAF 2.0-specific categories or shared categories. (This looks like a dev error)")
                    }
                    false
                }),
            CsafVersion::X21 => self
                .csaf21_categories
                .map(|cats| cats.contains(document_category))
                .unwrap_or_else(|| {
                    if self.shared_categories.is_none() {
                        panic!("Test applicability was checked for CSAF 2.1 on a config that does not contain CSAF 2.1-specific categories or shared categories. (This looks like a dev error.)")
                    }
                    false
                }),
        }
    }

    /// Checks if a profile test should run based on the document category only,
    /// irrespective of the CSAF version.
    pub fn matches_category(&self, document_category: &DocumentCategory) -> bool {
        if let Some(shared) = self.shared_categories {
            return shared.contains(document_category);
        }
        panic!(
            "Test applicability without a specified CSAF doc version was checked on a config that does not specify version-independent categories. (This looks like a dev error.)"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_with_csaf20_csaf_21_specific_categories() {
        const TEST_CONFIG: DocumentCategoryTestConfig = DocumentCategoryTestConfig::new()
            .shared(&[DocumentCategory::CsafSecurityAdvisory])
            .csaf20(&[DocumentCategory::CsafVex])
            .csaf21(&[DocumentCategory::CsafWithdrawn]);

        // Shared applies to both
        assert!(
            TEST_CONFIG.matches_category_with_csaf_version(&CsafVersion::X20, &DocumentCategory::CsafSecurityAdvisory)
        );
        assert!(
            TEST_CONFIG.matches_category_with_csaf_version(&CsafVersion::X21, &DocumentCategory::CsafSecurityAdvisory)
        );

        // CSAF 2.0-specific applies only to 2.0
        assert!(TEST_CONFIG.matches_category_with_csaf_version(&CsafVersion::X20, &DocumentCategory::CsafVex));
        assert!(!TEST_CONFIG.matches_category_with_csaf_version(&CsafVersion::X21, &DocumentCategory::CsafVex));

        // CSAF 2.1-specific applies only to 2.1
        assert!(!TEST_CONFIG.matches_category_with_csaf_version(&CsafVersion::X20, &DocumentCategory::CsafWithdrawn));
        assert!(TEST_CONFIG.matches_category_with_csaf_version(&CsafVersion::X21, &DocumentCategory::CsafWithdrawn));

        // Other categories do not apply
        assert!(
            !TEST_CONFIG
                .matches_category_with_csaf_version(&CsafVersion::X20, &DocumentCategory::CsafInformationalAdvisory)
        );
        assert!(
            !TEST_CONFIG
                .matches_category_with_csaf_version(&CsafVersion::X21, &DocumentCategory::CsafInformationalAdvisory)
        );
    }

    #[test]
    fn test_config_with_only_shared_categories() {
        const TEST_CONFIG: DocumentCategoryTestConfig = DocumentCategoryTestConfig::new().shared(&[
            DocumentCategory::CsafSecurityAdvisory,
            DocumentCategory::CsafInformationalAdvisory,
        ]);

        // Shared categories apply
        assert!(TEST_CONFIG.matches_category(&DocumentCategory::CsafSecurityAdvisory));
        assert!(TEST_CONFIG.matches_category(&DocumentCategory::CsafInformationalAdvisory));

        // Other categories do not apply
        assert!(!TEST_CONFIG.matches_category(&DocumentCategory::CsafVex));
        assert!(!TEST_CONFIG.matches_category(&DocumentCategory::CsafWithdrawn));
    }

    #[test]
    fn test_config_without_shared_categories_panic_on_is_ignored_for() {
        const TEST_CONFIG: DocumentCategoryTestConfig = DocumentCategoryTestConfig::new()
            .csaf20(&[DocumentCategory::CsafVex])
            .csaf21(&[DocumentCategory::CsafWithdrawn]);

        let result = std::panic::catch_unwind(|| {
            TEST_CONFIG.matches_category(&DocumentCategory::CsafSecurityAdvisory);
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_config_without_shared_or_csaf_20_categories_panics_on_is_ignored_for_on_csaf_version() {
        const TEST_CONFIG: DocumentCategoryTestConfig =
            DocumentCategoryTestConfig::new().csaf21(&[DocumentCategory::CsafWithdrawn]);

        let result = std::panic::catch_unwind(|| {
            TEST_CONFIG.matches_category_with_csaf_version(&CsafVersion::X20, &DocumentCategory::CsafVex);
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_config_without_shared_or_csaf_21_categories_panics_on_is_ignored_for_on_csaf_version() {
        const TEST_CONFIG: DocumentCategoryTestConfig =
            DocumentCategoryTestConfig::new().csaf20(&[DocumentCategory::CsafVex]);

        let result = std::panic::catch_unwind(|| {
            TEST_CONFIG.matches_category_with_csaf_version(&CsafVersion::X21, &DocumentCategory::CsafWithdrawn);
        });
        assert!(result.is_err());
    }
}
