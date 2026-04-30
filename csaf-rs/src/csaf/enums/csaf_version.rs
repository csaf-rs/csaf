/// Enum representing CSAF versions
///
/// Contrary to other enums that are based on enums in the generated schemas, we are re-defining
/// this enum in the trait. Each schema only contains an enum with "their" version, and merging them
/// would be more complex than defining them here and mapping to them in each implementation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum CsafVersion {
    #[serde(rename = "2.0")]
    X20,
    #[serde(rename = "2.1")]
    X21,
}

impl CsafVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            CsafVersion::X20 => "2.0",
            CsafVersion::X21 => "2.1",
        }
    }
}

impl AsRef<str> for CsafVersion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CsafVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CsafVersion::X20 => write!(f, "2.0"),
            CsafVersion::X21 => write!(f, "2.1"),
        }
    }
}

impl TryFrom<String> for CsafVersion {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "2.0" => Ok(CsafVersion::X20),
            "2.1" => Ok(CsafVersion::X21),
            _ => Err(format!(
                "Unsupported CSAF version: {value}. Supported versions are 2.0 and 2.1."
            )),
        }
    }
}
