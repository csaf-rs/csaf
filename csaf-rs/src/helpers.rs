use crate::csaf::types::csaf_datetime::CsafDateTime;
use chrono::NaiveDate;
use rust_embed::RustEmbed;
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

#[derive(RustEmbed)]
#[folder = "assets/cwe/"]
#[include = "*.csv"]
struct CweCsvFiles;

/// Per-CWE entry data loaded from the CWE CSV assets.
pub struct CweData {
    pub status: String,
    pub name: String,
    /// Vulnerability-mapping usage from MappingNotes/Usage (CWE schema 7.0+).
    /// `None` for CWE versions that predate the Usage field
    pub usage: Option<String>,
}

/// A single CWE version's release date and its entries.
pub struct CweVersionData {
    pub release_date: NaiveDate,
    pub entries: HashMap<String, CweData>,
}

/// Maps CWE version (e.g. "4.20") to its release date and its entries
pub type CweVersionLookup = HashMap<String, CweVersionData>;

pub static CWE_ENTRIES: LazyLock<CweVersionLookup> = LazyLock::new(|| {
    let mut versions = HashMap::new();

    for filename in CweCsvFiles::iter() {
        if let Some(file) = CweCsvFiles::get(&filename) {
            let version_and_date = &filename
                .strip_prefix("cwe_")
                .expect("Filenames in assets/cwe should start with 'cwe_'.")
                .strip_suffix(".csv")
                .expect("Filenames in assets/cwe should end with '.csv'.");
            let version_parts = version_and_date.split("_").collect::<Vec<&str>>();
            let version = version_parts[0];
            let release_date = match version_parts[1] {
                "" => NaiveDate::from_ymd_opt(1970, 1, 1).expect("Fallback date should be valid."),
                date_str => NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                    .expect("Date part of filenames in assets/cwe should be in 'YYYY-MM-DD' format."),
            };
            let mut entries: HashMap<String, CweData> = HashMap::new();
            let content =
                std::str::from_utf8(&file.data).expect("Files in assets/cwe should be valid UTF-8 encoded text files.");
            for line in content.lines() {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() >= 3 {
                    let id = format!("CWE-{}", parts[0].trim());
                    let status = parts[1].trim().to_string();
                    let name = parts[2].trim().to_string();
                    // Column 3 is a flag: "1" means the usage string in column 4 is present.
                    let usage = if parts.get(3).map(|f| f.trim()) == Some("1") {
                        parts.get(4).map(|u| u.trim().to_string())
                    } else {
                        None
                    };
                    entries.insert(id, CweData { status, name, usage });
                }
            }
            versions.insert(version.to_string(), CweVersionData { release_date, entries });
        }
    }

    versions
});

pub fn get_latest_cwe_version_for_date(date: &CsafDateTime) -> Option<&'static String> {
    // Convert to a date (UTC) and compare against the release dates stored in the CWE assets.
    let doc_date: NaiveDate = match date {
        CsafDateTime::Valid(v) => v.get_as_utc().date_naive(),
        _ => return None,
    };

    let mut latest: Option<(&'static String, &NaiveDate)> = None;

    for (version, version_data) in CWE_ENTRIES.iter() {
        if version_data.release_date <= doc_date && latest.as_ref().is_none_or(|l| version_data.release_date > *l.1) {
            latest = Some((version, &version_data.release_date));
        }
    }

    latest.map(|(version, _)| version)
}

#[derive(::serde::Deserialize)]
pub struct ScancodeLicense {
    pub license_key: String,
    pub category: String,
    pub spdx_license_key: Option<String>,
    pub other_spdx_license_keys: Vec<String>,
    pub is_exception: bool,
    pub is_deprecated: bool,
    pub json: String,
    pub yaml: String,
    pub html: String,
    pub license: String,
}

pub static SCANCODE_LICENSEDB_LICENSES: LazyLock<HashSet<String>> = LazyLock::new(|| {
    let licenses: Vec<ScancodeLicense> =
        serde_json::from_str(include_str!("../assets/scancode-licensedb.json")).unwrap();
    licenses
        .into_iter()
        .flat_map(|license| {
            std::iter::once(&license.spdx_license_key)
                .filter_map(|key| key.as_ref())
                .chain(license.other_spdx_license_keys.iter())
                .filter_map(|key| key.strip_prefix("LicenseRef-").map(|k| k.to_string()))
                .collect::<Vec<String>>()
        })
        .collect()
});

/// Defangs a URL by replacing dangerous characters to prevent accidental execution.
/// Replaces:
/// - `https://` with `hXXps[://]`
/// - `http://` with `hXXp[://]`
/// - `.` with `[.]`
pub fn defang_url(url: &str) -> String {
    url.replace("https://", "hXXps[://]")
        .replace("http://", "hXXp[://]")
        .replace(".", "[.]")
}

/// Helper function to get the HTTP status code of a URL
/// Returns the status code as u16, or 0 if the request fails (no connection, network error, etc.)
/// Only available when the `external-connections` feature is enabled.
#[cfg(feature = "external-connections")]
pub fn get_status_code(url: &str) -> u16 {
    match ureq::head(url).call() {
        Ok(response) => response.status().as_u16(),
        Err(_) => 0,
    }
}
