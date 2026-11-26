use std::collections::HashMap;
use std::sync::LazyLock;

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
    #[allow(clippy::unwrap_used)]
    let licenses: Vec<ScancodeLicense> =
        serde_json::from_str(include_str!("../assets/scancode-licensedb.json")).unwrap();

    let mut entries = HashMap::new();

    // Insert canonical ScanCode identifiers first
    for license in &licenses {
        let info = ScancodeLicenseInfo {
            is_exception: license.is_exception,
            is_deprecated: license.is_deprecated,
        };

        // CSAF 2.1 section 3.2.2.8 requires ScanCode license identifiers to use the "LicenseRef-scancode-" prefix.
        // The lexer strips "LicenseRef-", so the lookup key uses the remaining "scancode-" prefix.
        let scancode_license_key = format!("scancode-{}", license.license_key).to_lowercase();

        entries.insert(scancode_license_key, info);
    }

    // Add LicenseRef aliases without overwriting canonical identifiers
    for license in licenses {
        let info = ScancodeLicenseInfo {
            is_exception: license.is_exception,
            is_deprecated: license.is_deprecated,
        };

        for key in license
            .spdx_license_key
            .into_iter()
            .chain(license.other_spdx_license_keys)
        {
            // TODO #1169: Revisit ScanCode exception identifier handling once
            //  non-SPDX exceptions use the "AdditionRef-" prefix instead of "LicenseRef-".
            if let Some(key) = key.strip_prefix("LicenseRef-") {
                entries.entry(key.to_lowercase()).or_insert(info);
            }
        }
    }

    entries
}
