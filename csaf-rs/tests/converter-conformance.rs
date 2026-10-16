use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

use csaf::converter::{self, Diagnostics, Error, Settings, Warning};
use jsonpath_rust::JsonPath as _;
use serde::Deserialize;

macro_rules! run {
    () => {{
        struct S;
        run_by_name(std::any::type_name::<S>())
    }};
}

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

// TODO use proper test names but keep the index in the beginning of the name
// NOTE tests can be moved into modules to group them logically
#[ignore]
#[test]
fn _0_removal_of_additional_property() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _1() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _2() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _3() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _4() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _5() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _6() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _7() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _8() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _9() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _10() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _11() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _12() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _13() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _14() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _15() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _16() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _17() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _18() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _19() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _20() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _21() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _22() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _23() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _24() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _25() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _26() -> TestResult<()> {
    run!()
}

#[test]
fn _27_document_version_is_updated_to_2_dot_1() -> TestResult<()> {
    run!()
}

#[test]
fn _28_tlp_updated_from_white_to_clear() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _29() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _30() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _31() -> TestResult<()> {
    run!()
}

#[test]
fn _32_canonicalize_known_hash_algorithm_spelling() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _33() -> TestResult<()> {
    run!()
}

#[test]
fn _34_tlp_green_label_is_preserved() -> TestResult<()> {
    run!()
}

#[test]
fn _35_tlp_amber_label_is_preserved() -> TestResult<()> {
    run!()
}

#[test]
fn _36_tlp_red_label_is_preserved() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _37() -> TestResult<()> {
    run!()
}

fn run_by_name(tyname: &str) -> TestResult<()> {
    // example `tyname`: "csaf_converter::tests::conformance::_123_test_name::S"
    let mut parts = tyname.rsplit("::");
    parts.next(); // e.g. "S"
    let testname = parts.next().unwrap(); // e.g. "_123_test_name"
    let mut parts = testname.split("_");
    parts.next(); // ""
    let index = parts.next().unwrap().parse::<usize>()?; // e.g. "123"
    run_by_index(index)
}

fn run_by_index(index: usize) -> TestResult<()> {
    let testsuite_root = repo_root().join("csaf-2.0-to-csaf-2.1");
    let contents = fs::read_to_string(testsuite_root.join("converter-testcases-20-21.json"))?;
    // TODO the value returned by `from_str` could be cached in static variable
    let Schema { converter_tests } = serde_json::from_str(&contents)?;

    let ConverterTest {
        input: filename,
        remark,
        asserts,
    } = &converter_tests[index];
    eprintln!("remark: {remark}");

    let input = fs::read_to_string(testsuite_root.join("input").join(filename))?;
    let value: serde_json::Value = serde_json::from_str(&input)?;
    let mut diagnostics = DiagnosticsSpy::default();
    let output = converter::convert(value, &Settings::default(), &mut diagnostics).into_raw();

    for assert in asserts {
        match assert {
            Assert::ErrorMsg { substring_matches } => {
                let errors = diagnostics
                    .errors
                    .iter()
                    .map(|error| error.to_string())
                    .collect::<Vec<_>>();

                for substring in substring_matches {
                    // XXX maybe only one error msg must contain all substrings?
                    assert!(errors.iter().any(|msg| msg.contains(substring)));
                }
            },

            Assert::WarningMsg { substring_matches } => {
                let warnings = diagnostics
                    .warnings
                    .iter()
                    .map(|warning| warning.to_string())
                    .collect::<Vec<_>>();

                for substring in substring_matches {
                    // XXX maybe only one warning msg must contain all substrings?
                    assert!(warnings.iter().any(|msg| msg.contains(substring)));
                }
            },

            Assert::JsonPath { query, expected_result } => {
                let matches = output.query(&query)?;

                for (expected, got) in expected_result.iter().zip(&matches) {
                    assert_eq!(expected, *got);
                }
                assert_eq!(expected_result.len(), matches.len());
            },

            Assert::Success { value: success } => {
                assert_eq!(*success, !diagnostics.contains_errors());
            },
        }
    }

    Ok(())
}

fn repo_root() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path
}

#[derive(Deserialize)]
struct Schema {
    converter_tests: Vec<ConverterTest>,
}

#[derive(Deserialize)]
struct ConverterTest {
    input: String,
    remark: String,
    asserts: Vec<Assert>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum Assert {
    #[serde(rename = "errormsg")]
    ErrorMsg { substring_matches: Vec<String> },
    #[serde(rename = "warningmsg")]
    WarningMsg { substring_matches: Vec<String> },
    #[serde(rename = "jsonpath")]
    JsonPath {
        query: String,
        expected_result: Vec<serde_json::Value>,
    },
    #[serde(rename = "success")]
    Success { value: bool },
}

#[derive(Default)]
struct DiagnosticsSpy {
    errors: Vec<Error>,
    warnings: Vec<Warning>,
}

impl DiagnosticsSpy {
    fn contains_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

impl Diagnostics for DiagnosticsSpy {
    fn error(&mut self, error: Error) {
        self.errors.push(error)
    }

    fn warning(&mut self, warning: Warning) {
        self.warnings.push(warning)
    }
}

// As more features are implemented in the converter, more conformance tests will start passing.
// This unit test ensures that conformance tests that just started passing are not left `#[ignored]`
// effectively turning this test suite into a "ratchet mechanism"
#[test]
fn did_not_forget_to_unignore_a_test() -> TestResult<()> {
    let test_runner = env::current_exe()?;
    let output = Command::new(test_runner).arg("--ignored").output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0 passed"));
    Ok(())
}
