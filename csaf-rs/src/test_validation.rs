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

/// Macro to generate the boilerplate `TestValidator<CommonSecurityAdvisoryFramework>` impl blocks
/// for validation tests.
///
/// # Usage
///
/// For tests that apply to both CSAF 2.0 and 2.1:
/// ```ignore
/// impl_validator!(ValidatorForTest6_1_1, validate_missing_product_id);
/// ```
///
/// For tests that only apply to CSAF 2.0:
/// ```ignore
/// impl_validator!(csaf2_0, ValidatorForTest6_2_10, test_6_2_10_missing_tlp_label);
/// ```
///
/// For tests that only apply to CSAF 2.1:
/// ```ignore
/// impl_validator!(csaf2_1, ValidatorForTest6_1_41, test_6_1_41_missing_sharing_group_name);
/// ```
macro_rules! impl_validator {
    ($validator:ident, $validate_fn:path) => {
        $crate::test_validation::impl_validator!(csaf2_0, $validator, $validate_fn);
        $crate::test_validation::impl_validator!(csaf2_1, $validator, $validate_fn);
    };
    (csaf2_0, $validator:ident, $validate_fn:path) => {
        impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
            for crate::csaf2_0::testcases::$validator
        {
            fn validate(
                &self,
                doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
            ) -> Result<(), Vec<crate::validation::ValidationError>> {
                $validate_fn(doc)
            }
        }
    };
    (csaf2_1, $validator:ident, $validate_fn:path) => {
        impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
            for crate::csaf2_1::testcases::$validator
        {
            fn validate(
                &self,
                doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
            ) -> Result<(), Vec<crate::validation::ValidationError>> {
                $validate_fn(doc)
            }
        }
    };
}

pub(crate) use impl_validator;

/// Macro to generate the boilerplate `TestValidator<RawDocument<CommonSecurityAdvisoryFramework>>`
/// impl blocks for validation tests that operate on raw JSON.
///
/// # Usage
///
/// For tests that apply to both CSAF 2.0 and 2.1:
/// ```ignore
/// impl_raw_json_validator!(ValidatorForTest6_2_13, test_6_2_13_sorting);
/// ```
///
/// For tests that only apply to CSAF 2.0:
/// ```ignore
/// impl_raw_json_validator!(csaf2_0, ValidatorForTest, some_validate_fn);
/// ```
///
/// For tests that only apply to CSAF 2.1:
/// ```ignore
/// impl_raw_json_validator!(csaf2_1, ValidatorForTest, some_validate_fn);
/// ```
macro_rules! impl_raw_json_validator {
    ($validator:ident, $validate_fn:path) => {
        $crate::test_validation::impl_raw_json_validator!(csaf2_0, $validator, $validate_fn);
        $crate::test_validation::impl_raw_json_validator!(csaf2_1, $validator, $validate_fn);
    };
    (csaf2_0, $validator:ident, $validate_fn:path) => {
        impl
            crate::test_validation::TestValidator<
                crate::csaf::raw::RawDocument<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>,
            > for crate::csaf2_0::testcases::$validator
        {
            fn validate(
                &self,
                document: &crate::csaf::raw::RawDocument<
                    crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
                >,
            ) -> Result<(), Vec<crate::validation::ValidationError>> {
                $validate_fn(document.get_json())
            }
        }
    };
    (csaf2_1, $validator:ident, $validate_fn:path) => {
        impl
            crate::test_validation::TestValidator<
                crate::csaf::raw::RawDocument<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>,
            > for crate::csaf2_1::testcases::$validator
        {
            fn validate(
                &self,
                document: &crate::csaf::raw::RawDocument<
                    crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
                >,
            ) -> Result<(), Vec<crate::validation::ValidationError>> {
                $validate_fn(document.get_json())
            }
        }
    };
}

pub(crate) use impl_raw_json_validator;
