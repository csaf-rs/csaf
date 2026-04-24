//! FFI-friendly type definitions mirroring csaf-rs domain types.

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
