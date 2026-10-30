use std::path::Path;

use anyhow::bail;
use clap::{CommandFactory, Parser};
use csaf_rs::validation::{
    TestResult,
    TestResultStatus::{Failure, NotFound, Skipped, Success},
    ValidationError, ValidationResult,
};
/// A validator for CSAF documents
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ValidatorArgs {
    /// Path(s) to CSAF document(s) to validate, specify multiple files as: `PATH`... (e.g. `doc1.json doc2.json`)
    #[arg(action = clap::ArgAction::Append)]
    pub path: Vec<String>,

    /// Version of CSAF to use
    #[arg(short = 'C', long, default_value = "auto")]
    pub csaf_version: String,

    /// Set of tests to run, choose either one or more test IDs by repeating the option
    /// (e.g. -T 6.2.12 -T 6.3.11), or a preset test suite [default: "basic"]:
    ///     - basic: Validate mandatory tests and schema
    ///     - extended: Validate mandatory tests, recommended tests and JSON schema
    ///     - full: Validate all tests and JSON schema
    ///     - schema: Check if the input file(s) match the JSON schema
    /// In addition for CSAF version 2.1 the following preset test suites are available:
    ///     - mandatory: Validate all mandatory tests
    ///     - recommended: Validate all recommended tests
    ///     - informative: Validate all informative tests
    ///     - external-request-free: Validate full test suite without 6.3.6 and 6.3.7
    ///     - consistent-revision-history: Validate 6.1.14, 6.1.18, 6.1.19, 6.1.21, 6.1.22, 6.1.37, 6.2.4 - 6.2.6
    ///       6.2.21 and 6.2.33
    ///     - consistent-date-times: Validate 6.1.37, 6.1.45, 6.1.49, 6.1.51 - 6.1.53
    ///     - ssvc: Validate 6.1.46 - 6.1.49, 6.2.3, 6.2.34 - 6.2.37, 6.3.13 - 6.3.15
    #[arg(short = 'T', long, verbatim_doc_comment, default_value = "basic", hide_default_value = true, action = clap::ArgAction::Append)]
    pub test: Vec<String>,

    /// Provide information about the validator invocation as well as individual test results
    #[arg(short = 'v', long)]
    pub verbose: bool,
}

pub fn run_validator(args: ValidatorArgs) -> Result<(), anyhow::Error> {
    if args.path.is_empty() {
        ValidatorArgs::command().print_help()?;
        return Ok(());
    }

    if args
        .path
        .iter()
        .map(|file| {
            let path = Path::new(&file);
            let file_color = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Cyan.into()));
            println!("Validating file: {file_color}{}{file_color:#}", path.display());
            super::validate_file(path, args.csaf_version.as_str(), &args.test)
        })
        .filter(|result| match result {
            Ok(result) => {
                print_validation_result(result, args.verbose);
                !result.success
            },
            Err(err) => {
                println!("{err}\n");
                true
            },
        })
        .count()
        > 0
    {
        bail!("One or more files failed validation");
    }
    Ok(())
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
