use crate::schema::csaf2_0::schema::Cwe as Cwe20;
use crate::schema::csaf2_1::schema::Cwe as Cwe21;

/// Represents a Common Weakness Enumeration (CWE) in a CSAF document.
/// CWEs are present in CSAF 2.0 and 2.1, with 2.1 adding the `version` field.
/// To accommodate both versions, the `version` field is optional in this struct.
pub struct Cwe {
    /// Holds the ID for the weakness.
    pub id: String,
    /// Holds the full name of the weakness.
    pub name: String,
    /// Holds the version string of the CWE specification this weakness was extracted from.
    /// For CSAF 2.0, this field is `None` as the version is not present in that specification.
    /// For CSAF 2.1, this field is `Some(version)`.
    pub version: Option<String>,
}

impl From<&Cwe21> for Cwe {
    fn from(cwe: &Cwe21) -> Self {
        Self {
            id: cwe.id.to_string(),
            name: cwe.name.to_string(),
            // version is required for CSAF 2.1
            version: Some(cwe.version.to_string()),
        }
    }
}

impl From<&Cwe20> for Cwe {
    fn from(cwe: &Cwe20) -> Self {
        Self {
            id: cwe.id.to_string(),
            name: cwe.name.to_string(),
            // version does not exist in CSAF 2.0
            version: None,
        }
    }
}
