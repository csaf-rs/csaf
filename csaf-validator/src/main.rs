use anstream::println;
use anyhow::{Result, bail};
use clap::{CommandFactory, Parser};
use csaf::csaf::loader::detect_version;
use csaf::csaf2_0::loader::load_document as load_document_2_0;
use csaf::csaf2_1::loader::load_document as load_document_2_1;
use csaf::schema::csaf2_1::testresult_schema::{
    JsonSchema, NumberOfTheTest, ResultT, TestResultForASingleCsafTestFile, ValidationMessageT, ValidationMessagesT,
};
use csaf::validation::ValidationError;
use csaf::validation::{
    TestResult,
    TestResultStatus::{Failure, NotFound, Skipped, Success},
    Validatable, ValidationResult, validate_by_tests,
};
use std::ops::Deref;

/// A validator for CSAF documents
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the CSAF document(s) to validate
    #[arg(action = clap::ArgAction::Append)]
    path: Vec<String>,

    /// Version of CSAF to use
    #[arg(short = 'C', long, default_value = "auto")]
    csaf_version: String,

    /// The validation preset or tests to use
    #[arg(short = 'T', long, default_value = "basic", action = clap::ArgAction::Append)]
    test: Vec<String>,

    /// Enables verbose output, showing additional information about the validation process and results
    #[arg(short = 'v', long)]
    verbose: bool,

    /// Create a test result JSON for the given test ID as primary result, with other test failed results as secondary results
    #[arg(long, value_name = "TEST_ID")]
    create_test_result: Option<String>,

    /// Check validation results against a test result JSON file (mutually exclusive with --create-test-result)
    #[arg(long, value_name = "RESULT_FILE", conflicts_with = "create_test_result")]
    check_test_result: Option<String>,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    if args.path.is_empty() {
        Args::command().print_help()?;
        return Ok(());
    }

    if args.create_test_result.is_some() && args.path.len() != 1 {
        bail!("--create-test-result requires exactly one input file");
    }

    if args.check_test_result.is_some() && args.path.len() != 1 {
        bail!("--check-test-result requires exactly one input file");
    }

    if args
        .path
        .iter()
        .map(|file| validate_file(file.deref(), &args))
        .filter(|result| match result {
            Ok(result) => !result.success,
            Err(err) => {
                eprintln!("{err}");
                true
            },
        })
        .count()
        > 0
        && args.create_test_result.is_none()
    {
        bail!("One or more files failed validation");
    }
    Ok(())
}

/// Try to validate a file as a CSAF document based on the specified version.
fn validate_file(path: &str, args: &Args) -> Result<ValidationResult> {
    // Load expected test result file if --check-test-result is set
    let expected: Option<TestResultForASingleCsafTestFile> = if let Some(result_path) = &args.check_test_result {
        let content = std::fs::read_to_string(result_path)
            .map_err(|e| anyhow::anyhow!("Failed to read test result file '{result_path}': {e}"))?;
        Some(
            serde_json::from_str(&content)
                .map_err(|e| anyhow::anyhow!("Failed to parse test result file '{result_path}': {e}"))?,
        )
    } else {
        None
    };

    // Determine primary test ID and secondary test IDs from flags or loaded expected result
    let primary_test_id = args
        .create_test_result
        .as_ref()
        .or(expected.as_ref().map(|exp| exp.primary_result.id.deref()));

    // Collect all secondary test IDs from the expected result file so they are also executed
    let secondary_test_ids: Vec<String> = expected
        .as_ref()
        .and_then(|exp| exp.secondary_results.as_ref())
        .map(|results| results.iter().map(|r| r.id.deref().clone()).collect())
        .unwrap_or_default();

    if args.create_test_result.is_none() {
        let file_color = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Cyan.into()));
        println!("Validating file: {file_color}{path}{file_color:#}");
    }
    let version = match args.csaf_version.as_str() {
        "auto" => detect_version(path)?,
        other => other.to_string(),
    };

    let tests: Vec<_> = args
        .test
        .iter()
        .map(|s| s.as_str())
        .chain(primary_test_id.iter().map(|s| s.as_str()))
        .chain(secondary_test_ids.iter().map(|s| s.as_str()))
        .collect();

    let mut result = match version.as_str() {
        "2.0" => {
            let document = load_document_2_0(path)?;
            validate_document(document, "2.0", args, &tests)
        },
        "2.1" => {
            let document = load_document_2_1(path)?;
            validate_document(document, "2.1", args, &tests)
        },
        _ => bail!(format!("Invalid CSAF version: {version}")),
    };
    if let Some(primary_id) = &args.create_test_result {
        let test_result = build_testresult_json(&result, primary_id)?;
        println!("{}", serde_json::to_string_pretty(&test_result)?);
    }
    if let Some(exp) = &expected {
        let primary_id = exp.primary_result.id.deref().as_str();
        let comparison_ok = compare_with_expected(&result, exp, primary_id);
        result.success = comparison_ok;
    }
    Ok(result)
}

/// Validate a CSAF document of the specified version with the provided arguments.
///
/// This prints the results of the tests on stdout.
fn validate_document<T>(document: T, version: &str, args: &Args, tests: &[&str]) -> ValidationResult
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

    let result = validate_by_tests(&document, version, &test_ids);

    if args.create_test_result.is_none() && args.check_test_result.is_none() {
        print_validation_result(&result, args.verbose);
    }
    result
}

/// Print a validation result to stdout (for CLI use)
pub fn print_validation_result(result: &ValidationResult, verbose: bool) {
    if verbose {
        println!("CSAF Version: {}", result.version);
    }

    // Print individual test results
    for test_result in &result.test_results {
        if verbose {
            print_individual_test_result(test_result);
        }
        print_individual_test_failures(test_result);
    }

    // Print summary
    match (result.num_errors, result.num_warnings, result.num_infos) {
        (0, 0, 0) => println!("✅  Validation passed! No errors found."),
        (0, 0, infos) => println!("💡  Validation passed with {infos} info(s)."),
        (0, warnings, infos) => println!("⚠️  Validation passed with {warnings} warning(s) and {infos} info(s)."),
        (errors, warnings, infos) => {
            println!("❌  Validation failed with {errors} error(s), {warnings} warning(s) and {infos} info(s).")
        },
    }

    if result.num_not_found > 0 {
        let bold = anstyle::Style::new().underline();
        println!(
            "{bold}Note:{bold:#} {} test(s) were not found during validation.",
            result.num_not_found
        );
    }
    println!();
}

/// Print individual test result to stdout.
fn print_individual_test_result(test_result: &TestResult) {
    // Common prefix for all test statuses
    let prefix = format!("Executing Test {:10} ... ", test_result.test_id);
    print!("{prefix}");

    match &test_result.status {
        Success => {
            // Yay, success!
            println!("✅  Success");
        },
        Failure {
            errors,
            warnings,
            infos,
        } => {
            if !errors.is_empty() {
                println!("❌ {} error(s) found", errors.len());
            } else if !warnings.is_empty() {
                println!("⚠️  {} warning(s) found", warnings.len());
            } else {
                println!("💡  {} info(s) found", infos.len());
            };
        },
        NotFound => {
            // Test not found
            println!("❓  Test not found");
        },
        Skipped => {
            // Test skipped
            println!("⏭️  Test skipped");
        },
    }
}

/// Print individual information about test failures, warnings, and infos.
fn print_individual_test_failures(test_result: &TestResult) {
    if let Failure {
        errors,
        warnings,
        infos,
    } = &test_result.status
    {
        let test_id = &test_result.test_id;
        let path_color = anstyle::Style::new().dimmed();
        for ValidationError { message, instance_path } in errors {
            let color = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Red.into()));
            println!(" {path_color}{instance_path}{path_color:#}: {color}{message} [Error {test_id}]{color:#}");
        }
        for ValidationError { message, instance_path } in warnings {
            let color = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Yellow.into()));
            println!(" {path_color}{instance_path}{path_color:#}: {color}{message} [Warning {test_id}]{color:#}");
        }
        for ValidationError { message, instance_path } in infos {
            let color = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Blue.into()));
            println!(" {path_color}{instance_path}{path_color:#}: {color}{message} [Info {test_id}]{color:#}");
        }
    }
}

/// Convert a slice of [`ValidationError`]s to a [`ValidationMessagesT`], or `None` if empty.
fn testresult_create_validationmessages(errors: &[ValidationError]) -> Option<ValidationMessagesT> {
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
    if messages.is_empty() {
        None
    } else {
        Some(ValidationMessagesT(messages))
    }
}

/// Convert a [`TestResult`] to a [`ResultT`], returning `None` for tests that were not found or skipped,
/// or whose test ID does not match the required pattern.
fn testresult_create_result(test_result: &TestResult) -> Option<ResultT> {
    let id: NumberOfTheTest = test_result.test_id.parse().ok()?;
    match &test_result.status {
        Success => Some(ResultT {
            id,
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
            id,
            passed: false,
            errors: testresult_create_validationmessages(errors),
            warnings: testresult_create_validationmessages(warnings),
            infos: testresult_create_validationmessages(infos),
        }),
        NotFound | Skipped => None,
    }
}

/// Build a [`TestResultForASingleCsafTestFile`] from a [`ValidationResult`] with the given primary test ID.
fn build_testresult_json(result: &ValidationResult, primary_test_id: &str) -> Result<TestResultForASingleCsafTestFile> {
    let primary_test = result
        .test_results
        .iter()
        .find(|r| r.test_id == primary_test_id)
        .ok_or_else(|| anyhow::anyhow!("Test '{primary_test_id}' not found in validation results"))?;

    let primary_result = testresult_create_result(primary_test).ok_or_else(|| {
        anyhow::anyhow!(
            "Cannot create result for test '{primary_test_id}': test ID does not match the required pattern, or test was not found/skipped"
        )
    })?;

    let secondary_results: Vec<ResultT> = result
        .test_results
        .iter()
        .filter(|r| r.test_id != primary_test_id)
        .filter_map(testresult_create_result)
        .filter(|r| !r.passed)
        .collect();

    Ok(TestResultForASingleCsafTestFile {
        schema: JsonSchema::HttpsRawGithubusercontentComOasisTcsCsafMasterCsaf21TestValidatorTestresultJsonSchemaJson,
        overall_valid: result.success,
        primary_result,
        resultschema_version: "2.1".to_string(),
        secondary_results: if secondary_results.is_empty() {
            None
        } else {
            Some(secondary_results)
        },
    })
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

/// Compare a [`ValidationResult`] against an expected [`TestResultForASingleCsafTestFile`].
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
fn compare_with_expected(
    result: &ValidationResult,
    expected: &TestResultForASingleCsafTestFile,
    primary_test_id: &str,
) -> bool {
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
        expected
            .primary_result
            .errors
            .as_ref()
            .map(|v| v.as_slice())
            .unwrap_or(empty),
        "error",
        &mut has_errors,
    );
    check_primary_messages(
        val_warnings,
        expected
            .primary_result
            .warnings
            .as_ref()
            .map(|v| v.as_slice())
            .unwrap_or(empty),
        "warning",
        &mut has_errors,
    );
    check_primary_messages(
        val_infos,
        expected
            .primary_result
            .infos
            .as_ref()
            .map(|v| v.as_slice())
            .unwrap_or(empty),
        "info",
        &mut has_errors,
    );

    // --- Secondary results (one-way: expected entries must be present in validation) ---
    for secondary in expected.secondary_results.iter().flatten() {
        let test_id: &str = secondary.id.deref().as_str();
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

        check_secondary_messages(
            val_errors,
            secondary.errors.as_ref().map(|v| v.as_slice()).unwrap_or(empty),
            "error",
        );
        check_secondary_messages(
            val_warnings,
            secondary.warnings.as_ref().map(|v| v.as_slice()).unwrap_or(empty),
            "warning",
        );
        check_secondary_messages(
            val_infos,
            secondary.infos.as_ref().map(|v| v.as_slice()).unwrap_or(empty),
            "info",
        );
    }

    if has_errors {
        println!("{error_color}❌ Test result comparison failed!{error_color:#}");
    } else {
        println!("✅ Test result comparison passed!");
    }

    !has_errors
}
