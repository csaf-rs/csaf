use std::fmt::Display;

use chrono::NaiveDate;

use crate::csaf::types::csaf_datetime::CsafDateTime;

include!(concat!(env!("OUT_DIR"), "/cwe_generated.rs"));

/// CWE Usage values describing if the usage of the CWE is encouraged
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CweUsage {
    Allowed,
    AllowedWithReview,
    Discouraged,
    Prohibited,
    NotProvided,
}

impl Display for CweUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CweUsage::Allowed => "Allowed",
                CweUsage::AllowedWithReview => "Allowed-with-Review",
                CweUsage::Discouraged => "Discouraged",
                CweUsage::Prohibited => "Prohibited",
                CweUsage::NotProvided => "Not Provided",
            }
        )
    }
}

/// CWE status describing the state of a CWE
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CweStatus {
    Draft,
    Incomplete,
    Stable,
    Usable,
    Deprecated,
}

impl Display for CweStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CweStatus::Draft => "Draft",
                CweStatus::Incomplete => "Incomplete",
                CweStatus::Stable => "Stable",
                CweStatus::Usable => "Usable",
                CweStatus::Deprecated => "Deprecated",
            }
        )
    }
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
    let doc_date: NaiveDate = match date {
        CsafDateTime::Valid(v) => v.get_as_utc().date_naive(),
        _ => return None,
    };

    let mut latest: Option<(&'static str, NaiveDate)> = None;

    for (version, version_data) in CWE_RELEASES.entries() {
        let cwe_release_date = NaiveDate::from_ymd_opt(
            version_data.release_date.0,
            version_data.release_date.1,
            version_data.release_date.2,
        )
        .unwrap_or(NaiveDate::from_ymd_opt(1990, 1, 1).expect("Guaranteed to be a naive date"));
        if cwe_release_date <= doc_date && latest.is_none_or(|l| cwe_release_date > l.1) {
            latest = Some((version, cwe_release_date));
        }
    }

    latest.map(|(version, _)| version)
}
