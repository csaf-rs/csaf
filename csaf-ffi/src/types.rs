//! FFI-friendly type definitions mirroring csaf-rs domain types.

// ---------------------------------------------------------------------------
// CSAF Version
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, uniffi::Enum)]
pub enum CsafVersion {
    V20,
    V21,
}

impl From<&csaf::csaf::enums::csaf_version::CsafVersion> for CsafVersion {
    fn from(v: &csaf::csaf::enums::csaf_version::CsafVersion) -> Self {
        match v {
            csaf::csaf::enums::csaf_version::CsafVersion::X20 => Self::V20,
            csaf::csaf::enums::csaf_version::CsafVersion::X21 => Self::V21,
        }
    }
}

// ---------------------------------------------------------------------------
// Document Category
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, uniffi::Enum)]
pub enum DocumentCategory {
    CsafBase,
    CsafInformationalAdvisory,
    CsafSecurityIncidentResponse,
    CsafSecurityAdvisory,
    CsafVex,
    CsafWithdrawn,
    CsafSuperseded,
    CsafDeprecatedSecurityAdvisory,
    CsafBaseOther { value: String },
}

impl From<&csaf::csaf::types::csaf_document_category::CsafDocumentCategory> for DocumentCategory {
    fn from(c: &csaf::csaf::types::csaf_document_category::CsafDocumentCategory) -> Self {
        use csaf::csaf::types::csaf_document_category::CsafDocumentCategory as C;
        match c {
            C::CsafBase => Self::CsafBase,
            C::CsafInformationalAdvisory => Self::CsafInformationalAdvisory,
            C::CsafSecurityIncidentResponse => Self::CsafSecurityIncidentResponse,
            C::CsafSecurityAdvisory => Self::CsafSecurityAdvisory,
            C::CsafVex => Self::CsafVex,
            C::CsafWithdrawn => Self::CsafWithdrawn,
            C::CsafSuperseded => Self::CsafSuperseded,
            C::CsafDeprecatedSecurityAdvisory => Self::CsafDeprecatedSecurityAdvisory,
            C::CsafBaseOther(s) => Self::CsafBaseOther { value: s.clone() },
        }
    }
}

// ---------------------------------------------------------------------------
// Category of Branch
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, uniffi::Enum)]
pub enum CategoryOfTheBranch {
    Architecture,
    HostName,
    Language,
    Legacy,
    PatchLevel,
    Platform,
    ProductFamily,
    ProductName,
    ProductVersion,
    ProductVersionRange,
    ServicePack,
    Specification,
    Vendor,
}

impl From<&csaf::csaf::enums::category_of_the_branch::CategoryOfTheBranch> for CategoryOfTheBranch {
    fn from(c: &csaf::csaf::enums::category_of_the_branch::CategoryOfTheBranch) -> Self {
        use csaf::csaf::enums::category_of_the_branch::CategoryOfTheBranch as C;
        match c {
            C::Architecture => Self::Architecture,
            C::HostName => Self::HostName,
            C::Language => Self::Language,
            C::Legacy => Self::Legacy,
            C::PatchLevel => Self::PatchLevel,
            C::Platform => Self::Platform,
            C::ProductFamily => Self::ProductFamily,
            C::ProductName => Self::ProductName,
            C::ProductVersion => Self::ProductVersion,
            C::ProductVersionRange => Self::ProductVersionRange,
            C::ServicePack => Self::ServicePack,
            C::Specification => Self::Specification,
            C::Vendor => Self::Vendor,
        }
    }
}

// ---------------------------------------------------------------------------
// Product Status Group
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, uniffi::Enum)]
pub enum ProductStatusGroup {
    Affected,
    NotAffected,
    Fixed,
    UnderInvestigation,
    Unknown,
    Recommended,
}

impl From<&csaf::csaf::enums::product_status_group::ProductStatusGroup> for ProductStatusGroup {
    fn from(g: &csaf::csaf::enums::product_status_group::ProductStatusGroup) -> Self {
        use csaf::csaf::enums::product_status_group::ProductStatusGroup as G;
        match g {
            G::Affected => Self::Affected,
            G::NotAffected => Self::NotAffected,
            G::Fixed => Self::Fixed,
            G::UnderInvestigation => Self::UnderInvestigation,
            G::Unknown => Self::Unknown,
            G::Recommended => Self::Recommended,
        }
    }
}

// ---------------------------------------------------------------------------
// CWE
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, uniffi::Record)]
pub struct Cwe {
    pub id: String,
    pub name: String,
    pub version: Option<String>,
}

impl From<&csaf::csaf::types::csaf_cwe::Cwe> for Cwe {
    fn from(c: &csaf::csaf::types::csaf_cwe::Cwe) -> Self {
        Self {
            id: c.id.clone(),
            name: c.name.clone(),
            version: c.version.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// DateTime
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, uniffi::Enum)]
pub enum CsafDateTime {
    Valid { raw_string: String, utc_string: String },
    Invalid { raw_string: String },
}

impl From<&csaf::csaf::types::csaf_datetime::CsafDateTime> for CsafDateTime {
    fn from(dt: &csaf::csaf::types::csaf_datetime::CsafDateTime) -> Self {
        match dt {
            csaf::csaf::types::csaf_datetime::CsafDateTime::Valid(v) => Self::Valid {
                raw_string: v.get_raw_string().to_string(),
                utc_string: v.get_as_utc().to_rfc3339(),
            },
            csaf::csaf::types::csaf_datetime::CsafDateTime::Invalid(e) => Self::Invalid {
                raw_string: e.get_raw_string().to_string(),
            },
        }
    }
}

// ---------------------------------------------------------------------------
// Language
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, uniffi::Enum)]
pub enum CsafLanguage {
    Valid { value: String },
    Invalid { value: String },
}

impl From<&csaf::csaf::types::language::CsafLanguage> for CsafLanguage {
    fn from(l: &csaf::csaf::types::language::CsafLanguage) -> Self {
        use csaf::csaf::types::language::CsafLanguage as L;
        match l {
            L::Valid(v) => Self::Valid { value: v.to_string() },
            L::Invalid(s, _) => Self::Invalid { value: s.clone() },
        }
    }
}

// ---------------------------------------------------------------------------
// Vulnerability Metric type
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, uniffi::Enum)]
pub enum VulnerabilityMetric {
    SsvcV1,
    CvssV2 { version: String },
    CvssV3 { version: String },
    CvssV4 { version: String },
    Epss,
    QualitativeSeverityRating,
}

impl From<&csaf::csaf::types::csaf_vuln_metric::CsafVulnerabilityMetric> for VulnerabilityMetric {
    fn from(m: &csaf::csaf::types::csaf_vuln_metric::CsafVulnerabilityMetric) -> Self {
        use csaf::csaf::types::csaf_vuln_metric::CsafVulnerabilityMetric as M;
        match m {
            M::SsvcV1 => Self::SsvcV1,
            M::CvssV2(v) => Self::CvssV2 { version: v.clone() },
            M::CvssV3(v) => Self::CvssV3 { version: v.clone() },
            M::CvssV4(v) => Self::CvssV4 { version: v.clone() },
            M::Epss => Self::Epss,
            M::QualitativeSeverityRating => Self::QualitativeSeverityRating, // Not currently supported in the API, but could be added in the future if needed.
        }
    }
}

// ---------------------------------------------------------------------------
// Product reference tuple (used throughout the API)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, uniffi::Record)]
pub struct ProductReference {
    pub product_id: String,
    pub json_path: String,
}

impl From<(String, String)> for ProductReference {
    fn from((product_id, json_path): (String, String)) -> Self {
        Self { product_id, json_path }
    }
}

// ---------------------------------------------------------------------------
// Note
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, uniffi::Record)]
pub struct Note {
    pub category: String,
    pub title: Option<String>,
}

// ---------------------------------------------------------------------------
// Vulnerability ID
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, uniffi::Record)]
pub struct VulnerabilityId {
    pub system_name: String,
    pub text: String,
}
