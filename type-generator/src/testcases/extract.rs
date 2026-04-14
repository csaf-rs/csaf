use std::collections::HashSet;

use super::{RawTest, RawTestCase};
use crate::testcases::TestGroup::{Informative, Mandatory, OptionalRecommended};

/// Parses `testcases.json` (and a supplemental `testcases.json` file) and extracts
/// [RawTest] entries. This function does not do code generation.
///
/// For every test entry the function collects failure and valid documents from
/// both JSON files and provides them as [RawTestCase].
pub(crate) fn extract_test_entries_from_json(
    base_dir: &str,
    supplemental_base_dir: &str,
    testcases: &serde_json::Value,
    supplemental_testcases: &serde_json::Value,
) -> Vec<RawTest> {
    // extract test collections
    let tests = testcases["tests"]
        .as_array()
        .expect("testcases.json should have a 'tests' array");
    let supplemental_tests = supplemental_testcases["tests"]
        .as_array()
        .expect("supplemental testcases.json should have a 'tests' array");

    let mut entries = Vec::new();

    for test in tests {
        let id = test["id"].as_str().expect("test should have 'id' string").to_string();
        let mut seen = HashSet::new();
        let group = match test["group"].as_str().expect("test should have 'group' string") {
            "mandatory" => Mandatory,
            "optional" | "recommended" => OptionalRecommended,
            "informative" => Informative,
            unknown => {
                panic!(
                    "test '{id}': unknown group '{unknown}', expected 'mandatory', 'optional', 'recommended' or 'informative'"
                )
            },
        };

        let mut docs: Vec<RawTestCase> = Vec::new();

        let mut collect = |test: &serde_json::Value, dir: &str| {
            if let Some(failures) = test["failures"].as_array() {
                extract_test_doc(failures, dir, &mut docs);
            }
            if let Some(valid_cases) = test.get("valid").and_then(|v| v.as_array()) {
                extract_test_doc(valid_cases, dir, &mut docs);
            }
        };

        // Collect from upstream test cases
        collect(test, base_dir);

        // Check for additional test cases in the supplemental testcases.json
        for additional_test in supplemental_tests {
            let additional_test_id = additional_test["id"]
                .as_str()
                .expect("additional test should have 'id' string");
            if additional_test_id == id {
                collect(additional_test, supplemental_base_dir);
            }
        }

        // Check for duplicate case numbers which would cause compile errors
        // in the generated code output
        if let Some(dup) = docs.iter().map(|d| &d.case_num).find(|c| !seen.insert((*c).clone())) {
            panic!("duplicate case number '{dup}' in test '{id}'");
        }

        entries.push(RawTest { id, group, docs });
    }

    entries
}

/// Extract the test case info from the raw JSON.
///
/// Panics if the test case json does not contain `name` or if the test case number can't
/// be extracted from `name`.
fn extract_test_doc(tests: &[serde_json::Value], base_dir: &str, generated_docs: &mut Vec<RawTestCase>) {
    for test in tests {
        let name = test["name"].as_str().expect("test should have 'name' string");
        let case_num = extract_test_case_number(name).expect("filename should contain a test case number");
        generated_docs.push(RawTestCase {
            case_num,
            name: name.to_string(),
            base_dir: base_dir.to_string(),
        });
    }
}

/// Extract test case number from filename like "oasis_csaf_tc-csaf_2_0-2021-6-1-08-01.json" -> "01"
///
/// Returns `Some(case_number)` if the filename contains a test case number, or `None` if it doesn't.
fn extract_test_case_number(filename: &str) -> Option<String> {
    // Remove path prefix and .json suffix
    let name = filename.split('/').next_back().unwrap_or(filename);
    let name = name.strip_suffix(".json").unwrap_or(name);

    // Get the last component after the last dash
    if !name.contains('-') {
        return None;
    }
    name.split('-').next_back().map(|s| s.to_string())
}

#[cfg(test)]
mod tests_extract_test_case_number {
    use super::*;

    #[test]
    fn test_extract_test_case_number_typical() {
        assert_eq!(
            extract_test_case_number("oasis_csaf_tc-csaf_2_0-2021-6-1-08-01.json"),
            Some("01".to_string())
        );
    }

    #[test]
    fn test_extract_test_case_number_with_path() {
        assert_eq!(
            extract_test_case_number("some/path/to/oasis_csaf_tc-csaf_2_0-2021-6-1-08-01.json"),
            Some("01".to_string())
        );
    }

    #[test]
    fn test_extract_test_case_number_no_case_number() {
        assert_eq!(extract_test_case_number("some/path/to/no_case_number.json"), None,);
    }
}

#[cfg(test)]
mod tests_extract_test_doc {
    use super::*;
    use serde_json::json;

    #[test]
    #[should_panic(expected = "filename should contain a test case number")]
    fn test_extract_test_doc_panics_on_missing_case_number() {
        let tests = vec![json!({"name": "no_case_number.json"})];
        let mut docs = Vec::new();
        extract_test_doc(&tests, "demo", &mut docs);
    }

    #[test]
    #[should_panic(expected = "test should have 'name' string")]
    fn test_extract_test_doc_panics_on_missing_name() {
        let tests = vec![serde_json::json!({"not_name": "value"})];
        let mut docs = Vec::new();
        extract_test_doc(&tests, "demo", &mut docs);
    }

    #[test]
    fn test_extract_test_doc_populates_docs() {
        let tests = vec![
            json!({"name": "some/path/to/oasis_csaf_tc-csaf_2_0-2021-6-1-08-01.json"}),
            json!({"name": "some/path/to/oasis_csaf_tc-csaf_2_0-2021-6-1-08-02.json"}),
        ];
        let mut docs = Vec::new();
        extract_test_doc(&tests, "demo", &mut docs);
        assert_eq!(docs.len(), 2);
        assert_eq!(docs[0].name, "some/path/to/oasis_csaf_tc-csaf_2_0-2021-6-1-08-01.json");
        assert_eq!(docs[0].base_dir, "demo");
        assert_eq!(docs[1].name, "some/path/to/oasis_csaf_tc-csaf_2_0-2021-6-1-08-02.json");
        assert_eq!(docs[1].base_dir, "demo");
    }
}

#[cfg(test)]
mod tests_extract_test_entries {
    use super::*;
    use serde_json::json;

    #[test]
    #[should_panic(expected = "duplicate case number '01' in test '6.1.1'")]
    fn test_panics_on_duplicate_case_num() {
        let testcases = json!({
            "tests": [{
                "id": "6.1.1",
                "group": "mandatory",
                "failures": [
                    {"name": "mandatory/test-case-01.json"},
                    {"name": "mandatory/test-case-other-01.json"}
                ]
            }]
        });
        let supplemental = json!({ "tests": [] });
        extract_test_entries_from_json("demo", "supp_demo", &testcases, &supplemental);
    }
}
