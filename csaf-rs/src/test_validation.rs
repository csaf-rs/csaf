use crate::validation::ValidationError;

/// Trait for test validation logic.
///
/// Implement this trait to provide validation logic for specific test cases.
/// The trait is generic over the document type to allow validators to work
/// with different CSAF versions.
pub trait TestValidator<Doc> {
    /// Validate a CSAF document according to this test's requirements.
    ///
    /// # Arguments
    /// * `doc` - The CSAF document to validate
    ///
    /// # Returns
    /// * `Ok(())` if validation passes
    /// * `Err(Vec<ValidationError>)` if validation fails
    fn validate(&self, doc: &Doc) -> Result<(), Vec<ValidationError>>;
}

pub trait TestValidatorWithRawString {
    /// Validate a CSAF document according to this test's requirements, with access to the raw string content.
    ///
    /// # Arguments
    /// * `raw` - The raw string content of the document
    /// # Returns
    /// * `Ok(())` if validation passes
    /// * `Err(Vec<ValidationError>)` if validation fails
    fn validate(&self, raw: &str) -> Result<(), Vec<ValidationError>>;
}
