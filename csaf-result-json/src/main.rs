mod compare;
mod convert;
mod result;

use anyhow::{Result, bail};
use clap::{Parser, Subcommand};
use compare::compare_result_jsons;
use convert::build_testresult_json;
use csaf::csaf::loader::detect_version;
use csaf::csaf2_0::loader::load_document as load_document_2_0;
use csaf::csaf2_1::loader::load_document as load_document_2_1;
use csaf::validation::{Validatable, ValidationResult, validate_by_tests};
use result::ResultJson;

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
        #[arg()]
        test_id: String,

        /// Path to the CSAF document to validate
        #[arg()]
        csaf_document: String,
    },

    /// Run validation of the given document and check validation results against a test result JSON file
    Check {
        /// Path to the test result JSON file to check against
        #[arg()]
        result_file: String,

        /// Path to the CSAF document to validate
        #[arg()]
        csaf_document: String,
    },

    /// Compare two test result JSON files against each other
    Compare {
        /// Path to the actual test result JSON file (e.g. created via the `create` subcommand)
        #[arg()]
        actual_file: String,

        /// Path to the expected test result JSON file
        #[arg()]
        expected_file: String,
    },
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let mut tests = args.test.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

    match args.cmd {
        Commands::Create {
            test_id,
            csaf_document: path,
        } => {
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
        Commands::Check {
            result_file,
            csaf_document: path,
        } => {
            // Load expected test result file
            let expected = serde_json::from_str::<ResultJson>(
                &std::fs::read_to_string(&result_file)
                    .map_err(|e| anyhow::anyhow!("Failed to read test result file '{result_file}': {e}"))?,
            )
            .map_err(|e| anyhow::anyhow!("Failed to parse test result file '{result_file}': {e}"))?;
            if expected.schema != result::RESULT_JSON_SCHEMA {
                bail!(
                    "Test result file '{result_file}' has unexpected $schema: '{}'",
                    expected.schema
                );
            }

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

            let actual = build_testresult_json(&result, expected.primary_result.id.as_str())?;
            compare_result_jsons(&actual, &expected);
            Ok(())
        },
        Commands::Compare {
            actual_file,
            expected_file,
        } => {
            let actual = serde_json::from_str::<ResultJson>(
                &std::fs::read_to_string(&actual_file)
                    .map_err(|e| anyhow::anyhow!("Failed to read actual result file '{actual_file}': {e}"))?,
            )
            .map_err(|e| anyhow::anyhow!("Failed to parse actual result file '{actual_file}': {e}"))?;
            if actual.schema != result::RESULT_JSON_SCHEMA {
                bail!(
                    "Actual result file '{actual_file}' has unexpected $schema: '{}'",
                    actual.schema
                );
            }

            let expected = serde_json::from_str::<ResultJson>(
                &std::fs::read_to_string(&expected_file)
                    .map_err(|e| anyhow::anyhow!("Failed to read expected result file '{expected_file}': {e}"))?,
            )
            .map_err(|e| anyhow::anyhow!("Failed to parse expected result file '{expected_file}': {e}"))?;
            if expected.schema != result::RESULT_JSON_SCHEMA {
                bail!(
                    "Expected result file '{expected_file}' has unexpected $schema: '{}'",
                    expected.schema
                );
            }

            if actual.primary_result.id != expected.primary_result.id {
                bail!(
                    "Primary test ID mismatch between actual ('{}') and expected ('{}') result files",
                    actual.primary_result.id,
                    expected.primary_result.id
                );
            }

            compare_result_jsons(&actual, &expected);
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
