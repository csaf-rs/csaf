use crate::csaf_traits::CsafTrait;
use crate::validation::ValidationError;

/// 6.3.7 Use of Self Referencing URLs Failing to Resolve
///
/// When the `external-connections` feature is not enabled, this validation is skipped and returns Ok(()).
pub fn test_6_3_7_use_of_self_referencing_urls_failing_to_resolve(
    _doc: &impl CsafTrait,
) -> Result<(), Vec<ValidationError>> {
    // TODO: #407 this would be another use-case?
    Ok(())
}

crate::test_validation::impl_validator!(
    ValidatorForTest6_3_7,
    test_6_3_7_use_of_self_referencing_urls_failing_to_resolve
);
