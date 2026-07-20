//! Holds the 6.1.13 PURL pipeline to the official purl-spec test suite (the copies under
//! `assets/purl-spec`, synced from the pinned `purl-spec` submodule by
//! `scripts/update/update_assets.sh`): every `parse`/`roundtrip` case runs through the same two layers the
//! validator applies — the `PackageUrlRepresentation` schema pattern, then the `packageurl`
//! parse behind [`CsafPurl`] — and the verdict must match the suite's expectation.
//!
//! Current divergences are pinned in `tests/purl_spec_known_gaps.json`, one entry per case
//! that misjudges today. The lists are self-pruning: a fix that makes an entry pass again
//! fails the test until the stale entry is removed, and any new divergence fails it
//! immediately.

use csaf::csaf::types::purl::csaf_purl::CsafPurl;
use csaf::schema::csaf2_0::schema::PackageUrlRepresentation as Representation20;
use csaf::schema::csaf2_1::schema::PackageUrlRepresentation as Representation21;
use serde::Deserialize;
use std::str::FromStr;

/// Suite cases the pipeline misjudges today, as `file-name expected input-purl` lines keyed
/// by CSAF version. `valid` means the suite calls the purl valid and the pipeline rejects it
/// (a conformant document fails validation); `invalid` means the reverse (a nonconformant
/// purl passes).
fn known_gaps(version: &str) -> Vec<String> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/purl_spec_known_gaps.json");
    let text = std::fs::read_to_string(path).expect("known-gaps file reads");
    let mut gaps: std::collections::BTreeMap<String, Vec<String>> =
        serde_json::from_str(&text).expect("known-gaps file parses");
    gaps.remove(version)
        .unwrap_or_else(|| panic!("no known-gaps list for CSAF {version}"))
}

#[derive(Deserialize)]
struct SuiteFile {
    #[serde(default)]
    tests: Vec<Case>,
}

#[derive(Deserialize)]
struct Case {
    test_type: String,
    #[serde(default)]
    expected_failure: bool,
    input: serde_json::Value,
}

/// Every `(file, expected verdict, purl)` of the suite's parse and roundtrip cases, deduplicated
/// (a purl often appears as both a parse and a roundtrip case).
fn suite_cases() -> Vec<(String, bool, String)> {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/purl-spec");
    let mut cases = Vec::new();
    for dir in ["spec", "types"] {
        let dir_path = format!("{root}/{dir}");
        let mut paths: Vec<_> = std::fs::read_dir(&dir_path)
            .unwrap_or_else(|e| panic!("{dir_path}: {e} — run scripts/update/update_assets.sh"))
            .map(|entry| entry.expect("directory entry").path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
            .collect();
        paths.sort();
        for path in paths {
            let text = std::fs::read_to_string(&path).expect("suite file reads");
            let suite: SuiteFile = serde_json::from_str(&text).expect("suite file parses");
            let file = path.file_name().expect("file name").to_string_lossy().into_owned();
            for case in suite.tests {
                if case.test_type != "parse" && case.test_type != "roundtrip" {
                    continue;
                }
                let Some(purl) = case.input.as_str() else {
                    continue;
                };
                let entry = (file.clone(), !case.expected_failure, purl.to_owned());
                if !cases.contains(&entry) {
                    cases.push(entry);
                }
            }
        }
    }
    assert!(cases.len() > 150, "the suite loaded suspiciously few cases");
    cases
}

/// Runs the whole suite through one representation's pipeline and holds the divergences to the
/// known-gaps list, in both directions.
fn assert_suite(accepts: impl Fn(&str) -> bool, version: &str) {
    let mut divergences = Vec::new();
    for (file, expected_valid, purl) in suite_cases() {
        if accepts(&purl) != expected_valid {
            let verdict = if expected_valid { "valid" } else { "invalid" };
            divergences.push(format!("{file} {verdict} {purl}"));
        }
    }
    divergences.sort();
    let mut expected = known_gaps(version);
    expected.sort_unstable();
    for divergence in &divergences {
        assert!(
            expected.contains(divergence),
            "CSAF {version}: new divergence from the purl suite (add deliberately or fix):\n  {divergence}"
        );
    }
    for gap in &expected {
        assert!(
            divergences.iter().any(|d| d == gap),
            "CSAF {version}: stale known-gaps entry — the pipeline now matches the suite here, remove it:\n  {gap}"
        );
    }
}

#[test]
fn the_2_0_pipeline_matches_the_official_suite_modulo_known_gaps() {
    assert_suite(
        |purl| match Representation20::from_str(purl) {
            Err(_) => false,
            Ok(representation) => matches!(CsafPurl::from(&representation), CsafPurl::Valid(_)),
        },
        "2.0",
    );
}

#[test]
fn the_2_1_pipeline_matches_the_official_suite_modulo_known_gaps() {
    assert_suite(
        |purl| match Representation21::from_str(purl) {
            Err(_) => false,
            Ok(representation) => matches!(CsafPurl::from(&representation), CsafPurl::Valid(_)),
        },
        "2.1",
    );
}
