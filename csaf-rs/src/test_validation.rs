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
