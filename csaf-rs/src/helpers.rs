use crate::csaf::types::csaf_datetime::CsafDateTime;
use chrono::NaiveDate;
use rust_embed::RustEmbed;
use std::collections::HashMap;
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

#[derive(Clone, Copy)]
pub struct ScancodeLicenseInfo {
    pub is_exception: bool,
    pub is_deprecated: bool,
}

pub static SCANCODE_LICENSEDB: LazyLock<HashMap<String, ScancodeLicenseInfo>> =
    LazyLock::new(scancode_licensedb_entries);

fn scancode_licensedb_entries() -> HashMap<String, ScancodeLicenseInfo> {
    let licenses: Vec<ScancodeLicense> =
        serde_json::from_str(include_str!("../assets/scancode-licensedb.json")).unwrap();

    licenses
        .into_iter()
        .flat_map(|license| {
            let info = ScancodeLicenseInfo {
                is_exception: license.is_exception,
                is_deprecated: license.is_deprecated,
            };

            std::iter::once(&license.spdx_license_key)
                .filter_map(|key| key.as_ref())
                .chain(license.other_spdx_license_keys.iter())
                .filter_map(|key| {
                    key.strip_prefix("LicenseRef-").map(|key| (key.to_lowercase(), info)) // TODO #1169: Revisit ScanCode exception identifier handling once non-SPDX exceptions use the 'AdditionRef-' prefix instead of 'LicenseRef-'
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

// TODO: Remove internal CPE implementation once #1158 is resolvable
fn split_only_unescaped_separators(s: &str, separator: char, escape_char: char) -> Vec<&str> {
    let mut splits = vec![];

    let mut escap_char_count: usize = 0;
    let mut last_pos = 0;
    for (pos, c) in s.char_indices() {
        match c {
            c if c == escape_char => escap_char_count += 1,
            c if c == separator && escap_char_count.is_multiple_of(2) => {
                splits.push(&s[last_pos..pos]);
                last_pos = pos + 1;
                escap_char_count = 0;
            },
            _ => escap_char_count = 0,
        }
    }
    splits.push(&s[last_pos..]);
    splits
}

/// A simple CPE 2.3 formatted string binding implementation
// TODO: Replace once https://github.com/KenDJohnson/cpe-rs/pull/16 is merged (see #1158)
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Fsb<'a> {
    pub(crate) part: &'a str,
    pub(crate) vendor: &'a str,
    pub(crate) product: &'a str,
    pub(crate) version: &'a str,
    pub(crate) update: &'a str,
    pub(crate) edition: &'a str,
    pub(crate) language: &'a str,
    pub(crate) sw_edition: &'a str,
    pub(crate) target_sw: &'a str,
    pub(crate) target_hw: &'a str,
    pub(crate) other: &'a str,
}

impl<'a> Fsb<'a> {
    /// Parse a CPE formatted string. If the string is not parsable None is returned,
    /// this is a simplification but sufficient for our purposes.
    ///
    /// This function will decode special characters to their semantic meaning.
    pub(crate) fn parse(fsb: &'a str) -> Option<Self> {
        let stripped_fsb = fsb.strip_prefix("cpe:2.3:")?;

        // It is necessary to prevent splitting on escaped separators
        let components = split_only_unescaped_separators(stripped_fsb, ':', '\\');
        let mut components = components.iter();

        macro_rules! parse_field {
            ($attribute:ident) => {
                let &$attribute = components.next()?;
            };
        }

        parse_field!(part);
        parse_field!(vendor);
        parse_field!(product);
        parse_field!(version);
        parse_field!(update);
        parse_field!(edition);
        parse_field!(language);
        parse_field!(sw_edition);
        parse_field!(target_sw);
        parse_field!(target_hw);
        parse_field!(other);

        // Fail on additional fiels
        if components.next().is_some() {
            return None;
        }

        Some(Self {
            part,
            vendor,
            product,
            version,
            update,
            edition,
            language,
            sw_edition,
            target_sw,
            target_hw,
            other,
        })
    }
}
