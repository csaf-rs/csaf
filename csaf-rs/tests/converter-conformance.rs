use std::borrow::Cow;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

use csaf_rs::converter::{self, Diagnostics, Error, Settings, Warning};
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
fn _1_replace_leap_second_and_warn() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _2_identify_invalid_leap_second() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _3_replace_legacy_product_category() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _4_reject_unresolvable_stacked_product_tree() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _5_resolve_stacked_product_tree() -> TestResult<()> {
    run!()
}

#[test]
fn _6_warn_on_other_publisher_category() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _7_resolve_product_version_range() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _8_resolve_product_version_before_range() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _9_resolve_upper_open_ended_range() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _10_resolve_multi_constraint_upper_range() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _11_resolve_vers_single_version_range() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _12_resolve_vls_single_version_range() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _13_remove_invalid_cpe() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _14_canonicalize_hash_algorithm() -> TestResult<()> {
    run!()
}

#[test]
fn _15_lowercase_hash_value() -> TestResult<()> {
    run!()
}

#[test]
fn _16_move_single_purl_to_purls_array() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _17_add_model_number_wildcard() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _18_add_serial_number_wildcard() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _19_add_sku_wildcard() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _20_remove_invalid_model_number_wildcards() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _21_escape_model_number_backslashes() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _22_remove_invalid_serial_number_wildcards() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _23_escape_serial_number_backslashes() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _24_remove_invalid_sku_wildcards() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _25_escape_sku_backslashes() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _26_default_missing_tlp_to_clear() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _27_remove_tracking_id_from_title() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _28_convert_vendor_fix_for_not_affected_product() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _29_classify_superseded_document() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _30_classify_withdrawn_document() -> TestResult<()> {
    run!()
}

#[test]
fn _31_update_document_version() -> TestResult<()> {
    run!()
}

#[test]
fn _32_update_white_tlp_to_clear() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _33_convert_license_expression() -> TestResult<()> {
    run!()
}

#[test]
fn _34_release_date_converted_to_disclosure_date() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _35_convert_none_available_to_fix_planned() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _36_canonicalize_known_hash_algorithm_spelling() -> TestResult<()> {
    run!()
}

#[test]
fn _37_schema_is_set_to_the_csaf_2_1_schema_uri() -> TestResult<()> {
    run!()
}

#[test]
fn _38_preserve_tlp_green() -> TestResult<()> {
    run!()
}

#[test]
fn _39_preserve_tlp_amber() -> TestResult<()> {
    run!()
}

#[test]
fn _40_preserve_tlp_red() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _41_remove_none_available_when_vendor_fix_known() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _42_repair_stacked_product_version_tree() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _43_add_missing_fixed_product() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _44_convert_missing_known_affected() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _45_assign_product_specific_document_notes() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _46_leave_unassigned_document_note() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _47_convert_product_relationships() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _48_convert_cwe_with_matching_name() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _49_select_cwe_version_matching_name() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _50_preserve_cvss_v4() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _51_preserve_invalid_cvss_v4_as_note() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _52_assign_vulnerability_note_to_product() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _53_leave_unassigned_vulnerability_note() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _54_convert_valid_ssvc_vector() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _55_preserve_invalid_ssvc_vector_as_note() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _56_preserve_ssvc_context_as_note() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _57_split_mixed_remediation_products() -> TestResult<()> {
    run!()
}

#[test]
fn _58_preserve_vendor_fix_for_affected_product() -> TestResult<()> {
    run!()
}

#[test]
fn _59_preserve_none_available_remediation() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _60_normalize_superseded_category() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _61_normalize_withdrawn_category() -> TestResult<()> {
    run!()
}

#[test]
fn _62_preserve_registered_vulnerability_id() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _63_convert_registered_vulnerability_id() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _64_preserve_unconvertible_registered_id() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _65_convert_legacy_vulnerability_id() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _66_preserve_unconvertible_legacy_id() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _67_preserve_unregistered_vulnerability_id() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _68_move_multiple_license_expressions_to_notes() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _69_move_publisher_contact_details() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _70_preserve_publisher_openpgp_key() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _71_preserve_publisher_email() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _72_preserve_first_publisher_email() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _73_move_invalid_publisher_key_to_contact_details() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _74_reject_product_version_range_followed_by_product_version() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _75_reject_invalid_upper_open_ended_range() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _76_preserve_file_hashes() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _77_preserve_multiple_file_hash_algorithms() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _78_preserve_sbom_urls() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _79_preserve_skus() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _80_preserve_generic_uris() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _81_convert_withdrawn_category_from_title() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _82_convert_withdrawn_category() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _83_convert_superseded_category_from_title() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _84_convert_superseded_category() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _85_split_vendor_fix_remediation() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _86_convert_none_available_remediation() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _87_resolve_stacked_product_tree_branches() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _88_resolve_multiple_product_tree_paths() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _89_resolve_product_version_branches() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _90_convert_multiple_license_expressions() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _91_convert_product_relationships_to_paths() -> TestResult<()> {
    run!()
}

#[ignore]
#[test]
fn _92_convert_multiple_registered_vulnerability_ids() -> TestResult<()> {
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
    if let Some(remark) = remark {
        eprintln!("remark: {remark}");
    }

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
                let matches = output.query(query)?;

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
    remark: Option<String>,
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
    fn error(&mut self, _path: Cow<'static, str>, error: Error) {
        self.errors.push(error)
    }

    fn warning(&mut self, _path: Cow<'static, str>, warning: Warning) {
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
