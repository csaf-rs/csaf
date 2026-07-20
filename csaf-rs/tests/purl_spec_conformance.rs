//! Holds the 6.1.13 PURL pipeline to the official purl-spec test suite (the copies under
//! `assets/purl-spec`, synced from the pinned `purl-spec` submodule by
//! `scripts/update/update_assets.sh`): every `parse`/`roundtrip` case runs through the same two layers the
//! validator applies — the `PackageUrlRepresentation` schema pattern, then the `packageurl`
//! parse behind [`CsafPurl`] — and the verdict must match the suite's expectation.
//!
//! Current divergences are tracked in the `KNOWN_GAPS_*` lists below, one entry per case that
//! misjudges today. The lists are self-pruning: a fix that makes an entry pass again fails the
//! test until the stale entry is removed, and any new divergence fails it immediately.

use csaf::csaf::types::purl::csaf_purl::CsafPurl;
use csaf::schema::csaf2_0::schema::PackageUrlRepresentation as Representation20;
use csaf::schema::csaf2_1::schema::PackageUrlRepresentation as Representation21;
use serde::Deserialize;
use std::str::FromStr;

/// Suite cases the CSAF 2.0 pipeline misjudges today, as `file-name expected input-purl` lines.
/// `valid` means the suite calls the purl valid and the pipeline rejects it (a conformant
/// document fails validation); `invalid` means the reverse (a nonconformant purl passes).
const KNOWN_GAPS_2_0: &[&str] = &[
    "chrome-extension-test.json invalid pkg:chrome-extension/44444algnefjeiefhmpklpfiohadpglk",
    "chrome-extension-test.json invalid pkg:chrome-extension/dlpngalgnefjeiefhmpklpfiohadpglk@1.2.3-beta",
    "chrome-extension-test.json invalid pkg:chrome-extension/dlpngalgnefjeiefhmpklpfiohadpglk@1.2.3.4.5",
    "chrome-extension-test.json invalid pkg:chrome-extension/dogs",
    "cpan-test.json invalid pkg:cpan/GDT/URI::PackageURL",
    "cpan-test.json invalid pkg:cpan/LWP::UserAgent@6.7.6",
    "julia-test.json invalid pkg:julia/Dates",
    "maven-test.json valid pkg:///maven/org.apache.commons/io",
    "maven-test.json valid pkg://maven/org.apache.commons/io",
    "maven-test.json valid pkg:/maven/org.apache.commons/io",
    "npm-test.json valid pkg:npm/@babel/core#/googleapis/api/annotations/",
    "otp-test.json invalid pkg:otp/namespace/hex@2.1.1",
    "swift-test.json invalid pkg:swift/Alamofire@5.4.3",
    "swift-test.json invalid pkg:swift/github.com/Alamofire/@5.4.3",
    "vcpkg-test.json invalid pkg:vcpkg/boost/asio@1.84.0",
    "vscode-extension-test.json invalid pkg:vscode-extension/java@1.46.2025091308",
];

/// The CSAF 2.1 pipeline's divergences: the 2.0 set plus the purls the stricter 2.1 schema
/// pattern rejects — all of them mixed-case types, which the purl specification accepts (the
/// type is case-insensitive and canonicalizes to lowercase).
const KNOWN_GAPS_2_1: &[&str] = &[
    "chrome-extension-test.json invalid pkg:chrome-extension/44444algnefjeiefhmpklpfiohadpglk",
    "chrome-extension-test.json invalid pkg:chrome-extension/dlpngalgnefjeiefhmpklpfiohadpglk@1.2.3-beta",
    "chrome-extension-test.json invalid pkg:chrome-extension/dlpngalgnefjeiefhmpklpfiohadpglk@1.2.3.4.5",
    "chrome-extension-test.json invalid pkg:chrome-extension/dogs",
    "cpan-test.json invalid pkg:cpan/GDT/URI::PackageURL",
    "cpan-test.json invalid pkg:cpan/LWP::UserAgent@6.7.6",
    "golang-test.json valid pkg:GOLANG/google.golang.org/genproto#/googleapis/api/annotations/",
    "golang-test.json valid pkg:GOLANG/google.golang.org/genproto@abcdedf#/googleapis/api/annotations/",
    "julia-test.json invalid pkg:julia/Dates",
    "maven-test.json valid pkg:///maven/org.apache.commons/io",
    "maven-test.json valid pkg://maven/org.apache.commons/io",
    "maven-test.json valid pkg:/maven/org.apache.commons/io",
    "maven-test.json valid pkg:Maven/net.sf.jacob-project/jacob@1.14.3?classifier=x86&type=dll",
    "maven-test.json valid pkg:Maven/org.apache.xmlgraphics/batik-anim@1.9.1?classifier=sources&repositorY_url=https://repo.spring.io/release",
    "maven-test.json valid pkg:Maven/org.apache.xmlgraphics/batik-anim@1.9.1?type=pom&repositorY_url=repo.spring.io/release",
    "npm-test.json valid pkg:npm/@babel/core#/googleapis/api/annotations/",
    "nuget-test.json valid pkg:Nuget/EnterpriseLibrary.Common@6.0.1304",
    "otp-test.json invalid pkg:otp/namespace/hex@2.1.1",
    "pypi-test.json valid pkg:PYPI/Django_package@1.11.1.dev1",
    "rpm-test.json valid pkg:Rpm/fedora/curl@7.50.3-1.fc25?Arch=i386&Distro=fedora-25",
    "swift-test.json invalid pkg:swift/Alamofire@5.4.3",
    "swift-test.json invalid pkg:swift/github.com/Alamofire/@5.4.3",
    "vcpkg-test.json invalid pkg:vcpkg/boost/asio@1.84.0",
    "vscode-extension-test.json invalid pkg:vscode-extension/java@1.46.2025091308",
];

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
fn assert_suite(accepts: impl Fn(&str) -> bool, known_gaps: &[&str], version: &str) {
    let mut divergences = Vec::new();
    for (file, expected_valid, purl) in suite_cases() {
        if accepts(&purl) != expected_valid {
            let verdict = if expected_valid { "valid" } else { "invalid" };
            divergences.push(format!("{file} {verdict} {purl}"));
        }
    }
    divergences.sort();
    let mut expected: Vec<&str> = known_gaps.to_vec();
    expected.sort_unstable();
    for divergence in &divergences {
        assert!(
            expected.contains(&divergence.as_str()),
            "CSAF {version}: new divergence from the purl suite (add deliberately or fix):\n  {divergence}"
        );
    }
    for gap in &expected {
        assert!(
            divergences.iter().any(|d| d == gap),
            "CSAF {version}: stale KNOWN_GAPS entry — the pipeline now matches the suite here, remove it:\n  {gap}"
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
        KNOWN_GAPS_2_0,
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
        KNOWN_GAPS_2_1,
        "2.1",
    );
}
