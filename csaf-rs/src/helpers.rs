use chrono::NaiveDate;
use rust_embed::RustEmbed;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

#[derive(RustEmbed)]
#[folder = "assets/cwe/"]
#[include = "*.csv"]
struct CweCsvFiles;

pub type CweReleaseDateAndData = (NaiveDate, HashMap<String, String>);
pub static CWE_ENTRIES: LazyLock<HashMap<String, CweReleaseDateAndData>> = LazyLock::new(|| {
    let mut entries = HashMap::new();

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
            let mut versioned_data: HashMap<String, String> = HashMap::new();
            let content =
                std::str::from_utf8(&file.data).expect("Files in assets/cwe should be valid UTF-8 encoded text files.");
            for line in content.lines() {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() >= 2 {
                    let id = format!("CWE-{}", parts[0].trim());
                    let name = parts[1].trim().to_string();
                    versioned_data.insert(id, name);
                }
            }
            entries.insert(version.to_string(), (release_date, versioned_data));
        }
    }

    entries
});

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

pub const CSAF_2_0_SCHEMA_URL: &str = "https://docs.oasis-open.org/csaf/csaf/v2.0/csaf_json_schema.json";
pub static CSAF_2_0_SCHEMA: LazyLock<Value> = LazyLock::new(|| {
    let schema_str = include_str!("../assets/csaf_2.0_json_schema.json");
    serde_json::from_str(schema_str).unwrap()
});

pub const CSAF_2_1_SCHEMA_URL: &str = "https://docs.oasis-open.org/csaf/csaf/v2.1/schema/csaf.json";
pub static CSAF_2_1_SCHEMA: LazyLock<Value> = LazyLock::new(|| {
    let schema_str = include_str!("../assets/csaf_2.1_json_schema.json");
    serde_json::from_str(schema_str).unwrap()
});

pub const CVSS_V2_SCHEMA_URL: &str = "https://www.first.org/cvss/cvss-v2.0.json";
pub static CVSS_V2_SCHEMA: LazyLock<Value> = LazyLock::new(|| {
    let schema_str = include_str!("../assets/cvss-v2.0.json");
    serde_json::from_str(schema_str).unwrap()
});

pub const CVSS_V3_0_SCHEMA_URL: &str = "https://www.first.org/cvss/cvss-v3.0.json";
pub static CVSS_V3_0_SCHEMA: LazyLock<Value> = LazyLock::new(|| {
    let schema_str = include_str!("../assets/cvss-v3.0.json");
    serde_json::from_str(schema_str).unwrap()
});

pub const CVSS_V3_1_SCHEMA_URL: &str = "https://www.first.org/cvss/cvss-v3.1.json";
pub static CVSS_V3_1_SCHEMA: LazyLock<Value> = LazyLock::new(|| {
    let schema_str = include_str!("../assets/cvss-v3.1.json");
    serde_json::from_str(schema_str).unwrap()
});

pub const CVSS_V4_0_2_SCHEMA_URL: &str = "https://www.first.org/cvss/cvss-v4.0.2.json";
pub static CVSS_V4_0_2_SCHEMA: LazyLock<Value> = LazyLock::new(|| {
    let schema_str = include_str!("../assets/cvss-v4.0.2.json");
    serde_json::from_str(schema_str).unwrap()
});

pub const SSVC_2_SCHEMA_URL: &str = "https://certcc.github.io/SSVC/data/schema/v2/SelectionList_2_0_0.schema.json";
pub static SSVC_2_SCHEMA: LazyLock<Value> = LazyLock::new(|| {
    let schema_str = ssvc::assets::SELECTION_LIST_SCHEMA;
    serde_json::from_str(schema_str).unwrap()
});

pub const EXTENSION_METASCHEMA_URL: &str =
    "https://docs.oasis-open.org/csaf/csaf/v2.1/schema/extension-metaschema.json";
pub static EXTENSION_METASCHEMA: LazyLock<Value> = LazyLock::new(|| {
    let schema_str = include_str!("../assets/extension-metaschema.json");
    serde_json::from_str(schema_str).unwrap()
});

pub const EXTENSION_SCHEMA_URL: &str = "https://docs.oasis-open.org/csaf/csaf/v2.1/schema/extension-content.json";
pub static EXTENSION_SCHEMA: LazyLock<Value> = LazyLock::new(|| {
    let schema_str = include_str!("../assets/extension-content.json");
    serde_json::from_str(schema_str).unwrap()
});
