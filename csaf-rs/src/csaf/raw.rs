use std::cell::OnceCell;

use serde::de::DeserializeOwned;

use crate::validation::{TestResult, TestResultStatus, Validatable, ValidationError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawDocument<T> {
    raw: serde_json::Value,
    parsed: OnceCell<Result<T, String>>,
}

impl<T> RawDocument<T> {
    pub fn get_json(&self) -> &serde_json::Value {
        &self.raw
    }
}

impl<T> RawDocument<T>
where
    T: serde::de::DeserializeOwned,
{
    pub fn new(raw: serde_json::Value) -> Self {
        Self {
            raw,
            parsed: OnceCell::new(),
        }
    }
}

impl<T> HasParsed for RawDocument<T>
where
    T: serde::de::DeserializeOwned,
{
    type Parsed = T;

    fn get_parsed(&self) -> &Result<Self::Parsed, String> {
        self.parsed
            .get_or_init(|| serde_json::from_value::<T>(self.raw.clone()).map_err(|e| e.to_string()))
    }
}

pub trait HasParsed {
    type Parsed;
    fn get_parsed(&self) -> &Result<Self::Parsed, String>;
}

pub trait RawValidatable {
    fn run_raw_test(&self, test_id: &str) -> TestResult {
        TestResult {
            test_id: test_id.to_string(),
            status: crate::validation::TestResultStatus::NotFound,
        }
    }
}

impl<T> Validatable for T
where
    T: HasParsed + RawValidatable,
    T::Parsed: Validatable,
{
    /// Returns the test IDs belonging to a preset
    fn tests_in_preset(preset: &str) -> Option<Vec<&'static str>> {
        T::Parsed::tests_in_preset(preset)
    }

    /// Runs a test by test ID
    fn run_test(&self, test_id: &str) -> TestResult {
        let raw_result = self::RawValidatable::run_raw_test(self, test_id);
        if TestResultStatus::NotFound != raw_result.status {
            return raw_result;
        }

        match self.get_parsed() {
            Ok(parsed) => parsed.run_test(test_id),
            Err(_) => TestResult {
                test_id: test_id.to_string(),
                status: crate::validation::TestResultStatus::Skipped,
            },
        }
    }
}

impl<T, CSAF> crate::test_validation::TestValidator<RawDocument<CSAF>> for T
where
    T: crate::test_validation::TestValidator<CSAF>,
    CSAF: DeserializeOwned,
{
    fn validate(&self, doc: &RawDocument<CSAF>) -> Result<(), Vec<ValidationError>> {
        let parsed = doc.get_parsed();
        match parsed {
            Ok(parsed_doc) => self.validate(parsed_doc),
            Err(_) => Ok(()),
        }
    }
}
