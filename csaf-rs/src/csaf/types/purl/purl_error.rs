use crate::validation::ValidationError;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurlParseError {
    original_purl: String,
    kind: PurlParseErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PurlParseErrorKind {
    InvalidScheme(String),
    InvalidType(String),
    InvalidKey(String),
    MissingName,
    TypeProhibitsNamespace(String),
    InvalidNamespaceComponent(String),
    MissingScheme,
    MissingType,
    InvalidSubpathSegment(String),
    DecodingError,
}

impl PurlParseError {
    /// Private constructor
    fn new(purl_str: &str, kind: PurlParseErrorKind) -> Self {
        PurlParseError {
            original_purl: purl_str.to_owned(),
            kind,
        }
    }

    /// Constructor exposed to CsafPurl, specific to CsafPurl when using [packageurl]
    pub(super) fn from_packageurl_error(purl_str: &str, error: packageurl::Error) -> Self {
        Self::new(purl_str, error.into())
    }

    /// Constructs a `PurlParseError` directly for unit testing
    #[cfg(test)]
    pub fn new_for_test(purl_str: &str, kind: PurlParseErrorKind) -> Self {
        Self::new(purl_str, kind)
    }

    /// Access to kind is limited to unit testing
    #[cfg(test)]
    pub fn kind(&self) -> &PurlParseErrorKind {
        &self.kind
    }

    /// Construct a [ValidationError] from this [PurlParseError].
    pub fn into_validation_error(self, instance_path: String) -> ValidationError {
        ValidationError {
            message: format!("Invalid PURL format: {}, Error: {}", self.original_purl, self.kind),
            instance_path,
        }
    }
}

/// Map all [packageurl::Error]'s to our internal [PurlParseErrorKind].
impl From<packageurl::Error> for PurlParseErrorKind {
    fn from(error: packageurl::Error) -> Self {
        match error {
            packageurl::Error::InvalidScheme(scheme) => Self::InvalidScheme(scheme),
            packageurl::Error::InvalidType(package_type) => Self::InvalidType(package_type),
            packageurl::Error::InvalidKey(key) => Self::InvalidKey(key),
            packageurl::Error::MissingName => Self::MissingName,
            packageurl::Error::TypeProhibitsNamespace(package_type) => Self::TypeProhibitsNamespace(package_type),
            packageurl::Error::InvalidNamespaceComponent(component) => Self::InvalidNamespaceComponent(component),
            packageurl::Error::MissingScheme => Self::MissingScheme,
            packageurl::Error::MissingType => Self::MissingType,
            packageurl::Error::InvalidSubpathSegment(segment) => Self::InvalidSubpathSegment(segment),
            packageurl::Error::DecodingError(_) => Self::DecodingError,
        }
    }
}

impl Display for PurlParseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidScheme(scheme) => write!(f, "invalid scheme: {scheme:?}"),
            Self::InvalidType(package_type) => write!(f, "invalid type: {package_type:?}"),
            Self::InvalidKey(key) => write!(f, "invalid key: {key:?}"),
            Self::MissingName => write!(f, "missing name"),
            Self::TypeProhibitsNamespace(package_type) => {
                write!(f, "no namespace allowed for type {package_type:?}")
            },
            Self::InvalidNamespaceComponent(component) => {
                write!(f, "invalid namespace component: {component:?}")
            },
            Self::MissingScheme => write!(f, "missing scheme"),
            Self::MissingType => write!(f, "missing type"),
            Self::InvalidSubpathSegment(segment) => {
                write!(f, "invalid subpath segment: {segment:?}")
            },
            Self::DecodingError => write!(f, "utf-8 decoding failed"),
        }
    }
}
