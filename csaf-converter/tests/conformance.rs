use std::fs;
use std::path::PathBuf;

use csaf_converter::{ConversionOptions, DiagnosticsSpy};
use jsonpath_rust::JsonPath as _;
use serde::Deserialize;

macro_rules! run {
    () => {{
        struct S;
        run_by_name(std::any::type_name::<S>())
    }};
}

// TODO use proper test names but keep the index in the beginning of the name
// NOTE tests can be moved into modules to group them logically
#[ignore]
#[test]
fn _0_removal_of_additional_property() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _1() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _2() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _3() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _4() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _5() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _6() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _7() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _8() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _9() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _10() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _11() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _12() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _13() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _14() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _15() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _16() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _17() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _18() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _19() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _20() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _21() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _22() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _23() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _24() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _25() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _26() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _27() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _28() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _29() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _30() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _31() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _32() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _33() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _34() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _35() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _36() -> anyhow::Result<()> {
    run!()
}

#[ignore]
#[test]
fn _37() -> anyhow::Result<()> {
    run!()
}

fn run_by_name(tyname: &str) -> anyhow::Result<()> {
    // example `tyname`: "csaf_converter::tests::conformance::_123_test_name::S"
    let mut parts = tyname.rsplit("::");
    parts.next(); // e.g. "S"
    let testname = parts.next().unwrap(); // e.g. "_123_test_name"
    let mut parts = testname.split("_");
    parts.next(); // ""
    let index = parts.next().unwrap().parse::<usize>()?; // e.g. "123"
    run_by_index(index)
}

fn run_by_index(index: usize) -> anyhow::Result<()> {
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
    let mut diagnostics = DiagnosticsSpy::default();
    let output = csaf_converter::convert(&input, &ConversionOptions::default(), &mut diagnostics)?;

    for assert in asserts {
        match assert {
            Assert::ErrorMsg { substring_matches } => {
                let errors = diagnostics
                    .errors()
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
                    .warnings()
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
