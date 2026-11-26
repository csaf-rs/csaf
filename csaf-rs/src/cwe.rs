use strum::Display;

use crate::csaf::types::csaf_datetime::CsafDateTime;
use jiff::civil::Date;

include!(concat!(env!("OUT_DIR"), "/cwe_generated.rs"));

/// CWE Usage values describing if the usage of the CWE is encouraged
#[derive(Debug, Display, Clone, Copy, PartialEq)]
pub enum CweUsage {
    #[strum(serialize = "Allowed")]
    Allowed,
    #[strum(serialize = "Allowed-with-Review")]
    AllowedWithReview,
    #[strum(serialize = "Discouraged")]
    Discouraged,
    #[strum(serialize = "Prohibited")]
    Prohibited,
    #[strum(serialize = "NotProvided")]
    NotProvided,
}

/// CWE status describing the state of a CWE
#[derive(Debug, Display, Clone, Copy, PartialEq)]
pub enum CweStatus {
    #[strum(serialize = "Draft")]
    Draft,
    #[strum(serialize = "Incomplete")]
    Incomplete,
    #[strum(serialize = "Stable")]
    Stable,
    #[strum(serialize = "Usable")]
    Usable,
    #[strum(serialize = "Deprecated")]
    Deprecated,
}

/// Per-CWE entry data loaded from the CWE CSV assets.
#[derive(Debug)]
pub struct CweEntry {
    pub status: CweStatus,
    pub name: &'static str,
    pub usage: CweUsage,
}

///  A single CWE version's release date and its entries.
#[derive(Debug)]
pub struct CweRelease {
    pub release_date: (i32, u32, u32),
    pub entries: &'static phf::Map<&'static str, CweEntry>,
}

#[allow(clippy::expect_used)]
pub fn get_latest_cwe_version_for_date(date: &CsafDateTime) -> Option<&'static str> {
    let doc_date: Date = match date {
        CsafDateTime::Valid(v) => v.get_as_utc().date(),
        _ => return None,
    };

    let mut latest: Option<(&'static str, Date)> = None;

    for (version, version_data) in CWE_RELEASES.entries() {
        let cwe_release_date = Date::new(
            version_data.release_date.0 as i16,
            version_data.release_date.1 as i8,
            version_data.release_date.2 as i8,
        )
        .unwrap_or(Date::new(1990, 1, 1).expect("Guaranteed to be a valid date"));
        if cwe_release_date <= doc_date && latest.is_none_or(|l| cwe_release_date > l.1) {
            latest = Some((version, cwe_release_date));
        }
    }

    latest.map(|(version, _)| version)
}
