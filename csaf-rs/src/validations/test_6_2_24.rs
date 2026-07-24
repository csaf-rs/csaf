use chrono::NaiveDate;

use crate::csaf::types::csaf_datetime::CsafDateTime;
use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait, VulnerabilityTrait};
use crate::helpers::CWE_ENTRIES;
use crate::validation::ValidationError;
use semver::Version;

fn create_non_latest_cwe_error(cwe: &str, version: &str, latest: &str, i_r: usize, i_cwe: usize) -> ValidationError {
    // Parsing both strings as semantic versions. CWE assets use two-part
    // versions like "4.13". `semver::Version::parse` expects three parts in the version
    // (major.minor.patch), we normalize the version by appending 0 until we have three segments
    //  and preserve prerelease/build metadata.
    fn normalize_to_semver_str(s: &str) -> String {
        // split off prerelease/build metadata
        let mut rest = "";
        let mut core = s;
        if let Some(idx) = s.find('-') {
            core = &s[..idx];
            rest = &s[idx..];
        } else if let Some(idx) = s.find('+') {
            core = &s[..idx];
            rest = &s[idx..];
        }

        let mut parts: Vec<&str> = core.split('.').collect();
        while parts.len() < 3 {
            parts.push("0");
        }
        format!("{}{}", parts.join("."), rest)
    }

    let norm_version = normalize_to_semver_str(version);
    let norm_latest = normalize_to_semver_str(latest);
    let parsed_version = Version::parse(norm_version.as_str());
    let parsed_latest = Version::parse(norm_latest.as_str());

    let error_message = match (parsed_version, parsed_latest) {
        (Ok(v), Ok(l)) => {
            if v < l {
                format!("Weakness '{cwe}' uses non-latest CWE version {version} (latest: {latest}).")
            } else if v > l {
                format!("Weakness '{cwe}' uses a future CWE version {version} (latest: {latest}).")
            } else {
                format!("Weakness '{cwe}' uses latest CWE version {version} but validation is incorrect.")
            }
        },
        // If parsing fails for either side, fall back to string comparison to avoid panics.
        _ => {
            if version < latest {
                format!("Weakness '{cwe}' uses non-latest CWE version {version} (latest: {latest}).")
            } else if version > latest {
                format!("Weakness '{cwe}' uses a future CWE version {version} (latest: {latest}).")
            } else {
                format!("Weakness '{cwe}' uses latest CWE version {version} but validation is incorrect.")
            }
        },
    };

    ValidationError {
        message: error_message,
        instance_path: format!("/vulnerabilities/{i_r}/cwes/{i_cwe}/version"),
    }
}

fn get_latest_cwe_version_for_date(date: &CsafDateTime) -> Option<&'static String> {
    // Convert to a date (UTC) and compare against the release dates stored in the CWE assets.
    let doc_date: NaiveDate = match date {
        CsafDateTime::Valid(v) => v.get_as_utc().date_naive(),
        _ => return None,
    };

    let mut latest: Option<(&'static String, &NaiveDate)> = None;

    for (version, (release_date, _)) in CWE_ENTRIES.iter() {
        if *release_date <= doc_date && latest.as_ref().is_none_or(|l| *release_date > *l.1) {
            latest = Some((version, release_date));
        }
    }

    latest.map(|(version, _)| version)
}

/// 6.2.24 Usage of Non-Latest CWE Version
///
/// For each item in the CWE array it MUST be tested that the latest CWE version
/// available at the time of the last revision was used. The test SHALL fail if
/// a later CWE version was available (i.e. the CWE item does not reference the
/// most recent CWE version as of the document's current_release_date).
pub fn test_6_2_24_usage_of_non_latest_cwe_version(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let vulnerabilities = doc.get_vulnerabilities();
    let mut errors: Vec<ValidationError> = Vec::new();

    let tracking = doc.get_document().get_tracking();
    let current_release_date = tracking.get_current_release_date();

    // Determine the latest CWE version available at document current_release_date.
    let latest_version = get_latest_cwe_version_for_date(&current_release_date);

    // If we cannot determine the latest CWE version for the document,
    // we generate an error as the cwe list was not initialized correctly.
    if let Some(latest) = latest_version {
        for (i_r, vulnerability) in vulnerabilities.iter().enumerate() {
            if let Some(cwes) = vulnerability.get_cwes() {
                for (i_cwe, cwe_item) in cwes.iter().enumerate() {
                    // Extract the CWE version that is mandatory for CSAF 2.1 but
                    // optional for CSAF 2.0., this test is only relevant for the former
                    let Some(version) = cwe_item.version.as_deref() else {
                        continue;
                    };

                    if version != latest {
                        errors.push(create_non_latest_cwe_error(&cwe_item.id, version, latest, i_r, i_cwe));
                    }
                }
            }
        }
    } else {
        errors.push(ValidationError {
            message: "CWE version information is not available for the current release date.".to_string(),
            instance_path: "/document/tracking/current_release_date".to_string(),
        });
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_24,
    test_6_2_24_usage_of_non_latest_cwe_version
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_24() {
        let case_01_cwe_version_before_latest = Err(vec![create_non_latest_cwe_error("CWE-256", "4.12", "4.13", 0, 0)]);

        let case_02_cwe_version_after_latest = Err(vec![create_non_latest_cwe_error("CWE-143", "4.15", "4.13", 0, 0)]);

        let case_03_cwe_version_mismatch = Err(vec![
            create_non_latest_cwe_error("CWE-262", "1.8.1", "4.13", 0, 0),
            create_non_latest_cwe_error("CWE-287", "1.0", "4.13", 0, 2),
        ]);

        let case_04_cwe_version_mismatch_multi_vulnerabilities = Err(vec![
            create_non_latest_cwe_error("CWE-158", "1.3", "4.13", 0, 0),
            create_non_latest_cwe_error("CWE-138", "2.1", "4.13", 0, 1),
            create_non_latest_cwe_error("CWE-318", "4.14", "4.13", 1, 0),
            create_non_latest_cwe_error("CWE-61", "4.15", "4.13", 2, 0),
        ]);

        TESTS_2_1.test_6_2_24.expect(
            case_01_cwe_version_before_latest,
            case_02_cwe_version_after_latest,
            case_03_cwe_version_mismatch,
            case_04_cwe_version_mismatch_multi_vulnerabilities,
            Ok(()), // Case 11: 1 vuln, 1 correct cwe version, correction of case 01 with version 4.12 -> 4.13
            Ok(()), // Case 12: 1 vuln, 1 correct cwe version, correction of case 02 where version 4.15 was newer than latest 4.13
            Ok(()), // Case 13: 1 vuln, 3 correct cwe versions, correction of case 03 with versions 1.8.1 -> 4.13, 1.0 -> 4.13
            Ok(()), // Case 14: 3 vulns, 4 correct cwe versions, correction of case 04 with versions 1.3 -> 4.13, 2.1 -> 4.13, 4.14 -> 4.13, 4.15 -> 4.13
        );
    }
}
