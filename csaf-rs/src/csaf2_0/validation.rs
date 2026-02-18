use crate::csaf::raw::{RawDocument, RawValidatable};
use crate::csaf2_0::testcases::*;
use crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework;
use crate::test_validation::TestValidator;
use crate::validation::{TestResult, TestResultStatus, Validatable, ValidationPreset};

enum Severity {
    Error,
    Warning,
    Info,
}

fn to_test_result(
    test_id: &str,
    severity: Severity,
    result: Option<Result<(), Vec<crate::validation::ValidationError>>>,
) -> TestResult {
    TestResult {
        test_id: test_id.to_string(),
        status: match result {
            None => TestResultStatus::NotFound,
            Some(Ok(())) => TestResultStatus::Success,
            Some(Err(data)) => match severity {
                Severity::Error => TestResultStatus::Failure {
                    errors: data,
                    warnings: vec![],
                    infos: vec![],
                },
                Severity::Warning => TestResultStatus::Failure {
                    errors: vec![],
                    warnings: data,
                    infos: vec![],
                },
                Severity::Info => TestResultStatus::Failure {
                    errors: vec![],
                    warnings: vec![],
                    infos: data,
                },
            },
        },
    }
}

impl Validatable for CommonSecurityAdvisoryFramework {
    fn tests_in_preset(preset: &ValidationPreset) -> Vec<&str> {
        match preset {
            ValidationPreset::Basic => mandatory_tests(),
            ValidationPreset::Extended => [mandatory_tests(), recommended_tests()].concat(),
            ValidationPreset::Full => [mandatory_tests(), recommended_tests(), informative_tests()].concat(),
        }
    }

    fn run_test(&self, test_id: &str) -> TestResult {
        let mandatory_result = to_test_result(
            test_id,
            Severity::Error,
            match test_id {
                // mandatory tests
                "6.1.1" => Some(ValidatorForTest6_1_1.validate(self)),
                "6.1.2" => Some(ValidatorForTest6_1_2.validate(self)),
                "6.1.3" => Some(ValidatorForTest6_1_3.validate(self)),
                "6.1.4" => Some(ValidatorForTest6_1_4.validate(self)),
                "6.1.5" => Some(ValidatorForTest6_1_5.validate(self)),
                "6.1.6" => Some(ValidatorForTest6_1_6.validate(self)),
                "6.1.7" => Some(ValidatorForTest6_1_7.validate(self)),
                "6.1.8" => Some(ValidatorForTest6_1_8.validate(self)),
                "6.1.9" => None,  // Some(ValidatorForTest6_1_9.validate(self)),
                "6.1.10" => None, // Some(ValidatorForTest6_1_10.validate(self)),
                "6.1.11" => Some(ValidatorForTest6_1_11.validate(self)),
                "6.1.12" => Some(ValidatorForTest6_1_12.validate(self)),
                "6.1.13" => Some(ValidatorForTest6_1_13.validate(self)),
                "6.1.14" => Some(ValidatorForTest6_1_14.validate(self)),
                "6.1.15" => Some(ValidatorForTest6_1_15.validate(self)),
                "6.1.16" => Some(ValidatorForTest6_1_16.validate(self)),
                "6.1.17" => Some(ValidatorForTest6_1_17.validate(self)),
                "6.1.18" => Some(ValidatorForTest6_1_18.validate(self)),
                "6.1.19" => Some(ValidatorForTest6_1_19.validate(self)),
                "6.1.20" => Some(ValidatorForTest6_1_20.validate(self)),
                "6.1.21" => Some(ValidatorForTest6_1_21.validate(self)),
                "6.1.22" => Some(ValidatorForTest6_1_22.validate(self)),
                "6.1.23" => Some(ValidatorForTest6_1_23.validate(self)),
                "6.1.24" => Some(ValidatorForTest6_1_24.validate(self)),
                "6.1.25" => Some(ValidatorForTest6_1_25.validate(self)),
                "6.1.26" => None, // Some(ValidatorForTest6_1_26.validate(self)),
                "6.1.27.1" => Some(ValidatorForTest6_1_27_1.validate(self)),
                "6.1.27.2" => Some(ValidatorForTest6_1_27_2.validate(self)),
                "6.1.27.3" => Some(ValidatorForTest6_1_27_3.validate(self)),
                "6.1.27.4" => Some(ValidatorForTest6_1_27_4.validate(self)),
                "6.1.27.5" => Some(ValidatorForTest6_1_27_5.validate(self)),
                "6.1.27.6" => Some(ValidatorForTest6_1_27_6.validate(self)),
                "6.1.27.7" => Some(ValidatorForTest6_1_27_7.validate(self)),
                "6.1.27.8" => Some(ValidatorForTest6_1_27_8.validate(self)),
                "6.1.27.9" => Some(ValidatorForTest6_1_27_9.validate(self)),
                "6.1.27.10" => Some(ValidatorForTest6_1_27_10.validate(self)),
                "6.1.27.11" => Some(ValidatorForTest6_1_27_11.validate(self)),
                "6.1.28" => Some(ValidatorForTest6_1_28.validate(self)),
                "6.1.29" => Some(ValidatorForTest6_1_29.validate(self)),
                "6.1.30" => Some(ValidatorForTest6_1_30.validate(self)),
                "6.1.31" => Some(ValidatorForTest6_1_31.validate(self)),
                "6.1.32" => Some(ValidatorForTest6_1_32.validate(self)),
                "6.1.33" => Some(ValidatorForTest6_1_33.validate(self)),
                _ => None,
            },
        );
        if TestResultStatus::NotFound != mandatory_result.status {
            return mandatory_result;
        }

        let recommended_result = to_test_result(
            test_id,
            Severity::Warning,
            match test_id {
                // recommended tests
                "6.2.1" => Some(ValidatorForTest6_2_1.validate(self)),
                "6.2.2" => Some(ValidatorForTest6_2_2.validate(self)),
                "6.2.3" => Some(ValidatorForTest6_2_3.validate(self)),
                "6.2.4" => Some(ValidatorForTest6_2_4.validate(self)),
                "6.2.5" => Some(ValidatorForTest6_2_5.validate(self)),
                "6.2.6" => Some(ValidatorForTest6_2_6.validate(self)),
                "6.2.7" => Some(ValidatorForTest6_2_7.validate(self)),
                "6.2.8" => None, // Some(ValidatorForTest6_2_8.validate(self)),
                "6.2.9" => None, // Some(ValidatorForTest6_2_9.validate(self)),
                "6.2.10" => Some(ValidatorForTest6_2_10.validate(self)),
                "6.2.11" => Some(ValidatorForTest6_2_11.validate(self)),
                "6.2.12" => Some(ValidatorForTest6_2_12.validate(self)),
                "6.2.13" => {
                    // see below in RawValidatable
                    return TestResult {
                        test_id: test_id.to_string(),
                        status: TestResultStatus::Skipped,
                    };
                },
                "6.2.14" => None, // Some(ValidatorForTest6_2_14.validate(self)),
                "6.2.15" => Some(ValidatorForTest6_2_15.validate(self)),
                "6.2.16" => Some(ValidatorForTest6_2_16.validate(self)),
                "6.2.17" => Some(ValidatorForTest6_2_17.validate(self)),
                "6.2.18" => Some(ValidatorForTest6_2_18.validate(self)),
                "6.2.19" => None, // Some(ValidatorForTest6_2_19.validate(self)),
                "6.2.20" => {
                    // see below in RawValidatable
                    return TestResult {
                        test_id: test_id.to_string(),
                        status: TestResultStatus::Skipped,
                    };
                },
                _ => None,
            },
        );
        if TestResultStatus::NotFound != recommended_result.status {
            return recommended_result;
        }

        to_test_result(
            test_id,
            Severity::Info,
            match test_id {
                // informative tests
                "6.3.1" => Some(ValidatorForTest6_3_1.validate(self)),
                "6.3.2" => None, // Some(ValidatorForTest6_3_2.validate(self)),
                "6.3.3" => Some(ValidatorForTest6_3_3.validate(self)),
                "6.3.4" => Some(ValidatorForTest6_3_4.validate(self)),
                "6.3.5" => Some(ValidatorForTest6_3_5.validate(self)),
                "6.3.6" => None, // Some(ValidatorForTest6_3_6.validate(self)),
                "6.3.7" => None, // Some(ValidatorForTest6_3_7.validate(self)),
                "6.3.8" => None, // Some(ValidatorForTest6_3_8.validate(self)),
                "6.3.9" => None, // Some(ValidatorForTest6_3_9.validate(self)),
                "6.3.10" => Some(ValidatorForTest6_3_10.validate(self)),
                "6.3.11" => Some(ValidatorForTest6_3_11.validate(self)),
                // invalid tests
                _ => None,
            },
        )
    }
}

impl RawValidatable for RawDocument<CommonSecurityAdvisoryFramework> {
    fn run_raw_test(&self, test_id: &str) -> TestResult {
        to_test_result(
            test_id,
            Severity::Warning,
            match test_id {
                "6.2.13" => Some(ValidatorForTest6_2_13.validate(self)),
                "6.2.20" => Some(ValidatorForTest6_2_20.validate(self)),
                _ => None,
            },
        )
    }
}
