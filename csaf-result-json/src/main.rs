use anyhow::{Result, bail};
use clap::{Parser, Subcommand};
use csaf::csaf::loader::detect_version;
use csaf::csaf2_0::loader::load_document as load_document_2_0;
use csaf::csaf2_1::loader::load_document as load_document_2_1;
use csaf::validation::TestResultStatus::{Failure, NotFound, Skipped, Success};
use csaf::validation::{TestResult, Validatable, ValidationError, ValidationResult, validate_by_tests};

#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ResultJson {
    ///Contains the URL of the JSON schema for test result which the document promises to be valid for.
    #[serde(rename = "$schema")]
    pub schema: ::std::string::String,
    ///States whether the file passes all basic tests. This might differ from the result for the specific test.
    pub overall_valid: bool,
    ///Contains the expected result for this specific test. Results for any other tests may be added as secondary results.
    pub primary_result: ResultT,
    ///Contains the current version of this schema
    pub resultschema_version: ::std::string::String,
    ///Contains a list of expected result for other tests. It is not guaranteed to contain expected results for all other tests. Main purpose is to aid in understanding edge cases.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub secondary_results: ::std::option::Option<Vec<ResultT>>,
}

#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ResultT {
    ///Contains a list of errors.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub errors: ::std::option::Option<Vec<ValidationMessageT>>,
    ///Contains the section number of the test in the specification.
    pub id: String,
    ///Contains a list of information.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub infos: ::std::option::Option<Vec<ValidationMessageT>>,
    ///States whether the data passed this specific test.
    pub passed: bool,
    ///Contains a list of warnings.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub warnings: ::std::option::Option<Vec<ValidationMessageT>>,
}

#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ValidationMessageT {
    ///Contains a JSON pointer detailing the path to the instance that raised the issue.
    pub instance_path: ::std::string::String,
    ///Contains the message detailing what the issues is.
    pub message: ::std::string::String,
}

/// A validator for CSAF documents
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Version of CSAF to use
    #[arg(short = 'C', long, default_value = "auto")]
    csaf_version: String,

    /// The validation preset or tests to use
    #[arg(short = 'T', long, default_value = "basic", action = clap::ArgAction::Append)]
    test: Vec<String>,

    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run validation of the given document and create a test result JSON for the given test ID as primary result
    Create {
        /// Primary test ID to generate the test for, further failed tests are reported as secondary results (change with -T)
        #[arg(long)]
        test_id: String,

        /// Path to the CSAF document to validate
        #[arg()]
        path: String,
    },

    /// Run validation of the given document and check validation results against a test result JSON file
    Check {
        /// Path to the test result JSON file to check against
        #[arg(long)]
        result_file: String,

        /// Path to the CSAF document to validate
        #[arg()]
        path: String,
    },
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let mut tests = args.test.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

    match args.cmd {
        Commands::Create { test_id, path } => {
            tests.push(test_id.as_str());

            let version = match args.csaf_version.as_str() {
                "auto" => detect_version(path.as_str())?,
                other => other.to_string(),
            };

            let result = match version.as_str() {
                "2.0" => {
                    let document = load_document_2_0(path.as_str())?;
                    validate_document(document, "2.0", &tests)
                },
                "2.1" => {
                    let document = load_document_2_1(path.as_str())?;
                    validate_document(document, "2.1", &tests)
                },
                _ => bail!(format!("Invalid CSAF version: {version}")),
            };

            let test_result = build_testresult_json(&result, test_id.as_str())?;
            println!("{}", serde_json::to_string_pretty(&test_result)?);
            Ok(())
        },
        Commands::Check { result_file, path } => {
            // Load expected test result file if --check-test-result is set
            let expected = serde_json::from_str::<ResultJson>(
                &std::fs::read_to_string(&result_file)
                    .map_err(|e| anyhow::anyhow!("Failed to read test result file '{result_file}': {e}"))?,
            )
            .map_err(|e| anyhow::anyhow!("Failed to parse test result file '{result_file}': {e}"))?;

            tests.push(expected.primary_result.id.as_str());
            if let Some(secondary) = expected.secondary_results.as_ref() {
                for sec in secondary {
                    tests.push(sec.id.as_str());
                }
            }

            let version = match args.csaf_version.as_str() {
                "auto" => detect_version(path.as_str())?,
                other => other.to_string(),
            };

            let result = match version.as_str() {
                "2.0" => {
                    let document = load_document_2_0(path.as_str())?;
                    validate_document(document, "2.0", &tests)
                },
                "2.1" => {
                    let document = load_document_2_1(path.as_str())?;
                    validate_document(document, "2.1", &tests)
                },
                _ => bail!(format!("Invalid CSAF version: {version}")),
            };

            compare_with_expected(&result, &expected, &expected.primary_result.id);
            Ok(())
        },
    }
}

/// Validate a CSAF document of the specified version with the provided arguments.
///
/// This prints the results of the tests on stdout.
fn validate_document<T>(document: T, version: &str, tests: &[&str]) -> ValidationResult
where
    T: Validatable,
{
    let mut test_ids: Vec<&str> = tests
        .iter()
        .flat_map(|test_or_preset| match T::tests_in_preset(test_or_preset) {
            Some(test_ids) => test_ids,
            None => vec![*test_or_preset],
        })
        .collect();

    test_ids.sort_by(|a, b| {
        a.split('.')
            .map(|p| format!("{p:>2}"))
            .cmp(b.split(".").map(|p| format!("{p:>2}")))
    });
    test_ids.dedup();

    validate_by_tests(&document, version, &test_ids)
}

/// Build a [`TestResultForASingleCsafTestFile`] from a [`ValidationResult`] with the given primary test ID.
fn build_testresult_json(result: &ValidationResult, primary_test_id: &str) -> Result<ResultJson> {
    let primary_test = result
        .test_results
        .iter()
        .find(|r| r.test_id == primary_test_id)
        .ok_or_else(|| anyhow::anyhow!("Test '{primary_test_id}' not found in validation results"))?;

    let primary_result = testresult_create_result(primary_test).ok_or_else(|| {
        anyhow::anyhow!("Cannot create result for test '{primary_test_id}': test was not found/skipped")
    })?;

    let secondary_results: Vec<ResultT> = result
        .test_results
        .iter()
        .filter(|r| r.test_id != primary_test_id)
        .filter_map(testresult_create_result)
        .filter(|r| !r.passed)
        .collect();

    Ok(ResultJson {
        schema: "https://raw.githubusercontent.com/oasis-tcs/csaf/master/csaf_2.1/test/validator/testresult_json_schema.json".to_string(),
        resultschema_version: "2.1".to_string(),
        overall_valid: result.success,
        primary_result,
        secondary_results: if secondary_results.is_empty() {
            None
        } else {
            Some(secondary_results)
        },
    })
}

/// Convert a [`TestResult`] to a [`ResultT`], returning `None` for tests that were not found or skipped,
/// or whose test ID does not match the required pattern.
fn testresult_create_result(test_result: &TestResult) -> Option<ResultT> {
    match &test_result.status {
        Success => Some(ResultT {
            id: test_result.test_id.clone(),
            passed: true,
            errors: None,
            warnings: None,
            infos: None,
        }),
        Failure {
            errors,
            warnings,
            infos,
        } => Some(ResultT {
            id: test_result.test_id.clone(),
            passed: false,
            errors: testresult_create_validationmessages(errors),
            warnings: testresult_create_validationmessages(warnings),
            infos: testresult_create_validationmessages(infos),
        }),
        NotFound | Skipped => None,
    }
}

/// Convert a slice of [`ValidationError`]s to a [`Vec<ValidationMessageT>`], or `None` if empty.
fn testresult_create_validationmessages(errors: &[ValidationError]) -> Option<Vec<ValidationMessageT>> {
    if errors.is_empty() {
        return None;
    }
    let messages: Vec<ValidationMessageT> = errors
        .iter()
        .filter_map(|e| {
            Some(ValidationMessageT {
                instance_path: e.instance_path.clone(),
                message: e.message.parse().ok()?,
            })
        })
        .collect();
    if messages.is_empty() { None } else { Some(messages) }
}

/// Get the error, warning, and info slices for a specific test ID from a [`ValidationResult`].
fn get_test_messages<'a>(
    result: &'a ValidationResult,
    test_id: &str,
) -> (&'a [ValidationError], &'a [ValidationError], &'a [ValidationError]) {
    result
        .test_results
        .iter()
        .find(|r| r.test_id == test_id)
        .and_then(|r| match &r.status {
            Failure {
                errors,
                warnings,
                infos,
            } => Some((errors.as_slice(), warnings.as_slice(), infos.as_slice())),
            _ => None,
        })
        .unwrap_or((&[], &[], &[]))
}

/// Compare a [`ValidationResult`] against an expected [`ResultJson`].
///
/// Prints discovered issues and returns `true` if no comparison errors were found.
///
/// Primary result comparison (by JSON pointer / `instance_path`):
/// - Error present in validation but missing from expected -> printed as an error
/// - Error present in expected but missing from validation -> printed as an error
/// - Both sides have the same path but different messages -> printed as a warning
///
/// Secondary result comparison (one-way, never causes failure):
/// - Each expected error/warning/info that is absent from the validation -> printed as a warning
/// - Both sides have the same path but different messages -> printed as an info
fn compare_with_expected(result: &ValidationResult, expected: &ResultJson, primary_test_id: &str) -> bool {
    let mut has_errors = false;

    let error_color = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Red.into()));
    let warning_color = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Yellow.into()));
    let info_color = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Blue.into()));

    // --- Primary result ---
    let (val_errors, val_warnings, val_infos) = get_test_messages(result, primary_test_id);

    let check_primary_messages = |val_msgs: &[ValidationError],
                                  exp_msgs: &[ValidationMessageT],
                                  kind: &str,
                                  has_errors: &mut bool| {
        // Unexpected messages in validation
        for msg in val_msgs {
            match exp_msgs.iter().find(|e| e.instance_path == msg.instance_path) {
                None => {
                    println!(
                        "{error_color}❌ [Primary/{kind}] Unexpected {kind} at '{}': {}{error_color:#}",
                        msg.instance_path, msg.message
                    );
                    *has_errors = true;
                },
                Some(exp) if exp.message.as_str() != msg.message.as_str() => {
                    println!(
                        "{warning_color}⚠️  [Primary/{kind}] Message mismatch at '{}': expected '{}', got '{}'{warning_color:#}",
                        msg.instance_path,
                        exp.message.as_str(),
                        msg.message
                    );
                },
                _ => {},
            }
        }
        // Missing messages (in expected but absent from validation)
        for exp in exp_msgs {
            if val_msgs.iter().all(|m| m.instance_path != exp.instance_path) {
                println!(
                    "{error_color}❌ [Primary/{kind}] Expected {kind} not found at '{}': {}{error_color:#}",
                    exp.instance_path,
                    exp.message.as_str()
                );
                *has_errors = true;
            }
        }
    };

    let empty: &[ValidationMessageT] = &[];
    check_primary_messages(
        val_errors,
        expected.primary_result.errors.as_deref().unwrap_or(empty),
        "error",
        &mut has_errors,
    );
    check_primary_messages(
        val_warnings,
        expected.primary_result.warnings.as_deref().unwrap_or(empty),
        "warning",
        &mut has_errors,
    );
    check_primary_messages(
        val_infos,
        expected.primary_result.infos.as_deref().unwrap_or(empty),
        "info",
        &mut has_errors,
    );

    // --- Secondary results (one-way: expected entries must be present in validation) ---
    for secondary in expected.secondary_results.iter().flatten() {
        let test_id: &str = secondary.id.as_str();
        let (val_errors, val_warnings, val_infos) = get_test_messages(result, test_id);

        let check_secondary_messages = |val_msgs: &[ValidationError], exp_msgs: &[ValidationMessageT], kind: &str| {
            for exp in exp_msgs {
                match val_msgs.iter().find(|m| m.instance_path == exp.instance_path) {
                    None => {
                        println!(
                            "{warning_color}⚠️  [Secondary {test_id}/{kind}] Expected {kind} not found at '{}': {}{warning_color:#}",
                            exp.instance_path,
                            exp.message.as_str()
                        );
                    },
                    Some(found) if found.message.as_str() != exp.message.as_str() => {
                        println!(
                            "{info_color}💡 [Secondary {test_id}/{kind}] Message mismatch at '{}': expected '{}', got '{}'{info_color:#}",
                            exp.instance_path,
                            exp.message.as_str(),
                            found.message
                        );
                    },
                    _ => {},
                }
            }
        };

        check_secondary_messages(val_errors, secondary.errors.as_deref().unwrap_or(empty), "error");
        check_secondary_messages(val_warnings, secondary.warnings.as_deref().unwrap_or(empty), "warning");
        check_secondary_messages(val_infos, secondary.infos.as_deref().unwrap_or(empty), "info");
    }

    if has_errors {
        println!("{error_color}❌ Test result comparison failed!{error_color:#}");
    } else {
        println!("✅ Test result comparison passed!");
    }

    !has_errors
}
