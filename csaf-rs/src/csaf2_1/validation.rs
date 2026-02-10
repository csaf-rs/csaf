use crate::csaf::raw::{RawDocument, RawValidatable};
use crate::csaf2_1::testcases::*;
use crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework;
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
                "6.1.11" => None, // Some(ValidatorForTest6_1_11.validate(self)),
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
                "6.1.27.12" => None, // Some(ValidatorForTest6_1_27_12.validate(self)),
                "6.1.27.13" => None, // Some(ValidatorForTest6_1_27_13.validate(self)),
                "6.1.27.14" => None, // Some(ValidatorForTest6_1_27_14.validate(self)),
                "6.1.27.15" => None, // Some(ValidatorForTest6_1_27_15.validate(self)),
                "6.1.27.16" => None, // Some(ValidatorForTest6_1_27_16.validate(self)),
                "6.1.27.17" => None, // Some(ValidatorForTest6_1_27_17.validate(self)),
                "6.1.27.18" => None, // Some(ValidatorForTest6_1_27_18.validate(self)),
                "6.1.27.19" => None, // Some(ValidatorForTest6_1_27_19.validate(self)),
                "6.1.28" => Some(ValidatorForTest6_1_28.validate(self)),
                "6.1.29" => Some(ValidatorForTest6_1_29.validate(self)),
                "6.1.30" => Some(ValidatorForTest6_1_30.validate(self)),
                "6.1.31" => Some(ValidatorForTest6_1_31.validate(self)),
                "6.1.32" => Some(ValidatorForTest6_1_32.validate(self)),
                "6.1.33" => Some(ValidatorForTest6_1_33.validate(self)),
                "6.1.34" => Some(ValidatorForTest6_1_34.validate(self)),
                "6.1.35" => Some(ValidatorForTest6_1_35.validate(self)),
                "6.1.36" => Some(ValidatorForTest6_1_36.validate(self)),
                "6.1.37" => Some(ValidatorForTest6_1_37.validate(self)),
                "6.1.38" => Some(ValidatorForTest6_1_38.validate(self)),
                "6.1.39" => Some(ValidatorForTest6_1_39.validate(self)),
                "6.1.40" => Some(ValidatorForTest6_1_40.validate(self)),
                "6.1.41" => Some(ValidatorForTest6_1_41.validate(self)),
                "6.1.42" => Some(ValidatorForTest6_1_42.validate(self)),
                "6.1.43" => Some(ValidatorForTest6_1_43.validate(self)),
                "6.1.44" => Some(ValidatorForTest6_1_44.validate(self)),
                "6.1.45" => Some(ValidatorForTest6_1_45.validate(self)),
                "6.1.46" => Some(ValidatorForTest6_1_46.validate(self)),
                "6.1.47" => Some(ValidatorForTest6_1_47.validate(self)),
                "6.1.48" => Some(ValidatorForTest6_1_48.validate(self)),
                "6.1.49" => Some(ValidatorForTest6_1_49.validate(self)),
                "6.1.50" => None, // Some(ValidatorForTest6_1_50.validate(self)),
                "6.1.51" => None, // Some(ValidatorForTest6_1_51.validate(self)),
                "6.1.52" => None, // Some(ValidatorForTest6_1_52.validate(self)),
                "6.1.53" => None, // Some(ValidatorForTest6_1_53.validate(self)),
                "6.1.54" => None, // Some(ValidatorForTest6_1_54.validate(self)),
                "6.1.55" => None, // Some(ValidatorForTest6_1_55.validate(self)),
                "6.1.56" => None, // Some(ValidatorForTest6_1_56.validate(self)),
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
                "6.2.19" => None,   // Some(ValidatorForTest6_2_19.validate(self)),
                "6.2.20" => None,   // Some(ValidatorForTest6_2_20.validate(self)),
                "6.2.21" => None,   // Some(ValidatorForTest6_2_21.validate(self)),
                "6.2.22" => None,   // Some(ValidatorForTest6_2_22.validate(self)),
                "6.2.23" => None,   // Some(ValidatorForTest6_2_23.validate(self)),
                "6.2.24" => None,   // Some(ValidatorForTest6_2_24.validate(self)),
                "6.2.25" => None,   // Some(ValidatorForTest6_2_25.validate(self)),
                "6.2.26" => None,   // Some(ValidatorForTest6_2_26.validate(self)),
                "6.2.27" => None,   // Some(ValidatorForTest6_2_27.validate(self)),
                "6.2.28" => None,   // Some(ValidatorForTest6_2_28.validate(self)),
                "6.2.29" => None,   // Some(ValidatorForTest6_2_29.validate(self)),
                "6.2.30" => None,   // Some(ValidatorForTest6_2_30.validate(self)),
                "6.2.31" => None,   // Some(ValidatorForTest6_2_31.validate(self)),
                "6.2.32" => None,   // Some(ValidatorForTest6_2_32.validate(self)),
                "6.2.33" => None,   // Some(ValidatorForTest6_2_33.validate(self)),
                "6.2.34" => None,   // Some(ValidatorForTest6_2_34.validate(self)),
                "6.2.35" => None,   // Some(ValidatorForTest6_2_35.validate(self)),
                "6.2.36" => None,   // Some(ValidatorForTest6_2_36.validate(self)),
                "6.2.37" => None,   // Some(ValidatorForTest6_2_37.validate(self)),
                "6.2.38" => None,   // Some(ValidatorForTest6_2_38.validate(self)),
                "6.2.39.1" => None, // Some(ValidatorForTest6_2_39_1.validate(self)),
                "6.2.39.2" => None, // Some(ValidatorForTest6_2_39_2.validate(self)),
                "6.2.39.3" => None, // Some(ValidatorForTest6_2_39_3.validate(self)),
                "6.2.39.4" => None, // Some(ValidatorForTest6_2_39_4.validate(self)),
                "6.2.40" => None,   // Some(ValidatorForTest6_2_40.validate(self)),
                "6.2.41" => None,   // Some(ValidatorForTest6_2_41.validate(self)),
                "6.2.42" => None,   // Some(ValidatorForTest6_2_42.validate(self)),
                "6.2.43" => None,   // Some(ValidatorForTest6_2_43.validate(self)),
                "6.2.44" => None,   // Some(ValidatorForTest6_2_44.validate(self)),
                "6.2.45" => None,   // Some(ValidatorForTest6_2_45.validate(self)),
                "6.2.46" => None,   // Some(ValidatorForTest6_2_46.validate(self)),
                "6.2.47" => None,   // Some(ValidatorForTest6_2_47.validate(self)),
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
                "6.3.1" => None,  // Some(ValidatorForTests6_3_1.validate(self)),
                "6.3.2" => None,  // Some(ValidatorForTests6_3_2.validate(self)),
                "6.3.3" => None,  // Some(ValidatorForTests6_3_3.validate(self)),
                "6.3.4" => None,  // Some(ValidatorForTests6_3_4.validate(self)),
                "6.3.5" => None,  // Some(ValidatorForTests6_3_5.validate(self)),
                "6.3.6" => None,  // Some(ValidatorForTests6_3_6.validate(self)),
                "6.3.7" => None,  // Some(ValidatorForTests6_3_7.validate(self)),
                "6.3.8" => None,  // Some(ValidatorForTests6_3_8.validate(self)),
                "6.3.9" => None,  // Some(ValidatorForTests6_3_9.validate(self)),
                "6.3.10" => None, // Some(ValidatorForTests6_3_10.validate(self)),
                "6.3.11" => None, // Some(ValidatorForTests6_3_11.validate(self)),
                "6.3.12" => None, // Some(ValidatorForTests6_3_12.validate(self)),
                "6.3.13" => None, // Some(ValidatorForTests6_3_13.validate(self)),
                "6.3.14" => None, // Some(ValidatorForTests6_3_14.validate(self)),
                "6.3.15" => None, // Some(ValidatorForTests6_3_15.validate(self)),
                "6.3.16" => None, // Some(ValidatorForTests6_3_16.validate(self)),
                "6.3.17" => None, // Some(ValidatorForTests6_3_17.validate(self)),
                "6.3.18" => None, // Some(ValidatorForTests6_3_18.validate(self)),
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
                _ => None,
            },
        )
    }
}
