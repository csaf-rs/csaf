use std::marker::PhantomData;

use crate::{
    extractor::traits::{CanExtract, Extractor},
    validation::ValidationError,
};

/// A trait for validators that perform two-step validation by first extracting data
/// from a JSON document using an extractor and then performing validation on the
/// extracted data.
pub trait TwoStepValidator {
    /// Returns a mutable reference to the extractor that will be used to extract data
    /// from the JSON document.
    fn get_extractor(&mut self) -> &mut dyn Extractor;

    /// Performs validation on the extracted data and returns a result indicating whether
    /// the validation passed or failed.
    fn perform_validation(&mut self) -> Result<(), Vec<ValidationError>>;
}

/// A simple implementation of `TwoStepValidator` that uses a closure to perform the validation
/// on the extracted data.
pub struct ClosureValidator<E, O, F> {
    extractor: E,
    validate: F,
    _phantom: PhantomData<fn() -> O>,
}

impl<E, O, F> TwoStepValidator for ClosureValidator<E, O, F>
where
    E: Extractor + CanExtract<O>,
    F: FnMut(O) -> Result<(), Vec<ValidationError>>,
{
    fn get_extractor(&mut self) -> &mut dyn Extractor {
        &mut self.extractor
    }

    fn perform_validation(&mut self) -> Result<(), Vec<ValidationError>> {
        let output = self.extractor.extract();
        (self.validate)(output)
    }
}

/// Creates a new `ClosureValidator` with the provided extractor and validation function.
pub fn make_validator<E, O, F>(extractor: E, validate: F) -> ClosureValidator<E, O, F>
where
    E: Extractor + CanExtract<O>,
    F: FnMut(O) -> Result<(), Vec<ValidationError>>,
{
    ClosureValidator {
        extractor,
        validate,
        _phantom: PhantomData,
    }
}

/// Macro to generate the boilerplate `TestValidator<RawDocument<CommonSecurityAdvisoryFramework>>`
/// impl blocks for validation tests using two step validation.
///
/// # Usage
///
/// For tests that apply to both CSAF 2.0 and 2.1:
/// ```ignore
/// impl_two_step_validator!(ValidatorForTest6_2_13, test_6_2_13_sorting);
/// ```
///
/// For tests that only apply to CSAF 2.0:
/// ```ignore
/// impl_two_step_validator!(csaf2_0, ValidatorForTest, some_validate_fn);
/// ```
///
/// For tests that only apply to CSAF 2.1:
/// ```ignore
/// impl_two_step_validator!(csaf2_1, ValidatorForTest, some_validate_fn);
/// ```
macro_rules! impl_two_step_validator {
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
                let mut validator = $validate_fn();

                if let serde_json::Value::Object(data) = document.get_json() {
                    $crate::extractor::visit_json::visit_json_value(data, &mut [validator.get_extractor()]);
                }

                validator.perform_validation()
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
                let mut validator = $validate_fn();

                if let serde_json::Value::Object(data) = document.get_json() {
                    $crate::extractor::visit_json::visit_json_value(data, &mut [validator.get_extractor()]);
                }

                validator.perform_validation()
            }
        }
    };
}

pub(crate) use impl_two_step_validator;
