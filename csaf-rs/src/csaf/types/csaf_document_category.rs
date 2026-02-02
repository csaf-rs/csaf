use std::fmt::{Display, Formatter, Result as FmtResult};
use crate::csaf_traits::CsafVersion;
use crate::schema::csaf2_0::schema::DocumentCategory as DocumentCategory20;
use crate::schema::csaf2_1::schema::DocumentCategory as DocumentCategory21;

/// Shared Enum representing document categories
/// Contains well-known categories of CSAF version 2.0 and 2.1 as enum variants
/// All other category strings (which are by definition csaf_base)
/// are represented as DocumentCategory::CsafBaseOther(String)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CsafDocumentCategory {
    // Used for the exact match "csaf_base"
    CsafBase,
    CsafInformationalAdvisory,
    CsafSecurityIncidentResponse,
    CsafSecurityAdvisory,
    CsafVex,
    CsafWithdrawn,
    CsafSuperseded,
    CsafDeprecatedSecurityAdvisory,
    // By definition of the standard, everything that is not an exact match for the categories above,
    // is also csaf_base. This variants represents this case, saving the original string for later
    // validation (see test test 6.1.26).
    CsafBaseOther(String),
}

impl From<&str> for CsafDocumentCategory {
    fn from(category: &str) -> Self {
        match category {
            "csaf_base" => CsafDocumentCategory::CsafBase,
            "csaf_informational_advisory" => CsafDocumentCategory::CsafInformationalAdvisory,
            "csaf_security_incident_response" => CsafDocumentCategory::CsafSecurityIncidentResponse,
            "csaf_security_advisory" => CsafDocumentCategory::CsafSecurityAdvisory,
            "csaf_vex" => CsafDocumentCategory::CsafVex,
            "csaf_deprecated_security_advisory" => CsafDocumentCategory::CsafDeprecatedSecurityAdvisory,
            "csaf_withdrawn" => CsafDocumentCategory::CsafWithdrawn,
            "csaf_superseded" => CsafDocumentCategory::CsafSuperseded,
            default => CsafDocumentCategory::CsafBaseOther(default.to_string()),
        }
    }
}

impl From<&DocumentCategory20> for CsafDocumentCategory {
    fn from(value: &DocumentCategory20) -> Self {
        CsafDocumentCategory::from(value.as_str())
    }
}

impl From<&DocumentCategory21> for CsafDocumentCategory {
    fn from(value: &DocumentCategory21) -> Self {
        CsafDocumentCategory::from(value.as_str())
    }
}

impl CsafDocumentCategory {
    // --------------------------------------------------------------------------
    // Known profiles per CSAF version and some helper functions
    // --------------------------------------------------------------------------
    const CSAF_20_KNOWN_PROFILES: [CsafDocumentCategory; 5] = [
        CsafDocumentCategory::CsafBase,
        CsafDocumentCategory::CsafSecurityIncidentResponse,
        CsafDocumentCategory::CsafInformationalAdvisory,
        CsafDocumentCategory::CsafSecurityAdvisory,
        CsafDocumentCategory::CsafVex,
    ];

    const CSAF_21_KNOWN_PROFILES: [CsafDocumentCategory; 8] = [
        CsafDocumentCategory::CsafBase,
        CsafDocumentCategory::CsafSecurityIncidentResponse,
        CsafDocumentCategory::CsafInformationalAdvisory,
        CsafDocumentCategory::CsafSecurityAdvisory,
        CsafDocumentCategory::CsafVex,
        CsafDocumentCategory::CsafDeprecatedSecurityAdvisory,
        CsafDocumentCategory::CsafWithdrawn,
        CsafDocumentCategory::CsafSuperseded,
    ];

    /// Checks if the category is DocumentCategory::CsafBaseOther
    pub fn is_base_other(&self) -> bool {
        matches!(self, CsafDocumentCategory::CsafBaseOther(_))
    }

    /// Checks if the category is DocumentCategory::CsafBase or DocumentCategory::CsafBaseOther
    pub fn is_base(&self) -> bool {
        matches!(self, CsafDocumentCategory::CsafBase | CsafDocumentCategory::CsafBaseOther(_))
    }

    /// Checks if the document category is a known profile for the given CSAF version
    pub fn is_known_profile(&self, version: &CsafVersion) -> bool {
        match version {
            CsafVersion::X20 => Self::CSAF_20_KNOWN_PROFILES.contains(self),
            CsafVersion::X21 => Self::CSAF_21_KNOWN_PROFILES.contains(self),
        }
    }

    /// Returns a `, ` concatenated string of known profiles for the given CSAF version
    pub fn known_profile_concat(version: &CsafVersion) -> String {
        let profiles: &[CsafDocumentCategory] = match version {
            CsafVersion::X20 => &Self::CSAF_20_KNOWN_PROFILES,
            CsafVersion::X21 => &Self::CSAF_21_KNOWN_PROFILES,
        };
        profiles
            .iter()
            .map(|profile| profile.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }

    /// Returns a vector of tuples containing normalized known profile strings and their original enum values
    pub fn known_profiles_normalized(version: &CsafVersion) -> Vec<(String, CsafDocumentCategory)> {
        let profiles: &[CsafDocumentCategory] = match version {
            CsafVersion::X20 => &Self::CSAF_20_KNOWN_PROFILES,
            CsafVersion::X21 => &Self::CSAF_21_KNOWN_PROFILES,
        };
        profiles
            .iter()
            .map(|profile| (profile.normalize(), profile.clone()))
            .collect()
    }

    // --------------------------------------------------------------------------
    // Helper functions for normalization and checking of "csaf_" prefix (used in 6.1.26)
    // --------------------------------------------------------------------------

    /// Helper function to remove whitespace, underscores and (various unicode) dashes / hyphens from a string
    fn get_with_forbidden_chars_removed(s: &str) -> String {
        const FORBIDDEN_CHARS: &[char] = &[
            '\u{002D}', // hyphen-minus U+002D
            '\u{005F}', // underscore U+005F
            '\u{02D7}', // modifier letter minus sign U+02D7
            '\u{058A}', // armenian hyphen U+058A
            '\u{1400}', // canadian syllabics carrier khaki U+1400
            '\u{1806}', // mongolian 't'odo soft hyphen U+1806
            '\u{2010}', // hyphen U+2010
            '\u{2011}', // non-breaking hyphen U+2011
            '\u{2012}', // figure dash U+2012
            '\u{2013}', // en dash U+2013
            '\u{2014}', // em dash U+2014
            '\u{2015}', // horizontal bar U+2015
            '\u{2043}', // hyphen bullet U+2043
            '\u{207B}', // superscript minus U+207B
            '\u{208B}', // subscript minus U+208B
            '\u{2212}', // minus sign U+2212
            '\u{2500}', // box drawings light horizontal U+2500
            '\u{2501}', // box drawings heavy horizontal U+2501
            '\u{2796}', // heavy minus sign U+2796
            '\u{29FF}', // right-pointing curved angle bracket U+29FF
            '\u{2E3A}', // two-em dash U+2E3A
            '\u{2E3B}', // three-em dash U+2E3B
            '\u{FE58}', // small em dash U+FE58
            '\u{FE63}', // small hyphen-minus U+FE63
            '\u{FF0D}', // fullwidth hyphen-minus U+FF0D
        ];
        s.chars()
            .filter(|c| !c.is_whitespace() && !FORBIDDEN_CHARS.contains(c))
            .collect()
    }

    /// Helper function to check if a string starts with `csaf_` (case-insensitive)
    #[inline]
    fn string_starts_with_csaf_underscore(s: &str) -> bool {
        // Lowercase and Split the string at "csaf_"
        if let Some(prefix) = s.to_lowercase().split("csaf_").next() {
            // the category contains "csaf_"
            // return true if everything before "csaf_" is whitespace, underscore or hyphen
            Self::get_with_forbidden_chars_removed(prefix).is_empty()
        } else {
            // the category does not contain "csaf_"
            false
        }
    }

    /// Checks if the category string starts with `csaf_` (case-insensitive)
    /// also checks if everything before `csaf_` whitespace, underscores and hyphens
    ///
    /// Examples:
    /// `csaf_base´ -> true
    /// `csaf_basE` -> true
    /// ` csaf_base` -> true
    /// `_csaf_base` -> true
    /// `-csaf_base` -> true
    /// `CSAF_base` -> true
    /// `saf_base` -> false
    pub fn starts_with_csaf_underscore(&self) -> bool {
        // check if this is DocumentCategory::BaseOther
        // if it is not, the string does start with "csaf_" by convention
        if !self.is_base_other() {
            return true;
        }

        Self::string_starts_with_csaf_underscore(&self.to_string())
    }


    /// Helper function to normalize a category string
    #[inline]
    fn string_normalize(s: &str) -> String {
        // lowercase
        let mut normalized = s.to_lowercase();
        // remove forbidden chars
        normalized = Self::get_with_forbidden_chars_removed(&normalized);
        // remove leading "csaf"
        normalized.strip_prefix("csaf").unwrap_or(&normalized).to_string()
    }

    /// Normalizes the document category string by removing leading "csaf" and any whitespace, hyphen or underscore
    ///
    /// Examples:
    /// `csaf_base` -> `base`
    /// `csaf-basE` -> `base`
    /// ` csaf_base` -> `base`
    /// `_csaf_base` -> `base`
    /// `-csaf_base` -> `base`
    /// `Csaf_base` -> `base`
    /// `saf_base` -> `safbase`
    /// `_saf_base` -> `safbase`
    /// `Some_Other-Category` -> `someothercategory`
    pub fn normalize(&self) -> String {
        Self::string_normalize(&self.to_string())
    }
}

impl Display for CsafDocumentCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            CsafDocumentCategory::CsafBase => write!(f, "csaf_base"),
            CsafDocumentCategory::CsafInformationalAdvisory => write!(f, "csaf_informational_advisory"),
            CsafDocumentCategory::CsafSecurityIncidentResponse => write!(f, "csaf_security_incident_response"),
            CsafDocumentCategory::CsafSecurityAdvisory => write!(f, "csaf_security_advisory"),
            CsafDocumentCategory::CsafVex => write!(f, "csaf_vex"),
            CsafDocumentCategory::CsafDeprecatedSecurityAdvisory => write!(f, "csaf_deprecated_security_advisory"),
            CsafDocumentCategory::CsafWithdrawn => write!(f, "csaf_withdrawn"),
            CsafDocumentCategory::CsafSuperseded => write!(f, "csaf_superseded"),
            CsafDocumentCategory::CsafBaseOther(other) => write!(f, "{other}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod string_starts_with_csaf_underscore_tests {
        use super::*;

        #[test]
        fn test_exact_csaf_underscore_prefix_returns_true() {
            // `csaf_basE` -> true
            assert!(CsafDocumentCategory::string_starts_with_csaf_underscore("csaf_basE"));

        }

        #[test]
        fn test_uppercase_csaf_returns_true() {
            // `CSAF_base` -> true
            assert!(CsafDocumentCategory::string_starts_with_csaf_underscore("CSAF_base"));
        }

        #[test]
        fn test_leading_chars_before_csaf_underscore_returns_true() {
            // ` csaf_base` -> true
            assert!(CsafDocumentCategory::string_starts_with_csaf_underscore(" csaf_base"));
            // `_csaf_base` -> true
            assert!(CsafDocumentCategory::string_starts_with_csaf_underscore("_csaf_base"));
            // `-csaf_base` -> true
            assert!(CsafDocumentCategory::string_starts_with_csaf_underscore("-csaf_base"));
            // `__csaf_base` -> true
            assert!(CsafDocumentCategory::string_starts_with_csaf_underscore("__csaf_base"));
            // ` _ csaf_base` -> true
            assert!(CsafDocumentCategory::string_starts_with_csaf_underscore(" _ csaf_base"));
        }

        #[test]
        fn test_no_csaf_underscore_returns_false() {
            // `saf_base` -> false
            assert!(!CsafDocumentCategory::string_starts_with_csaf_underscore("saf_base"));
        }
    }

    mod normalize_tests {
        use super::*;

        #[test]
        fn test_known_profiles_normalize_correctly() {
            assert_eq!(CsafDocumentCategory::string_normalize("csaf_base"), "base");
            assert_eq!(CsafDocumentCategory::string_normalize("csaf_informational_advisory"), "informationaladvisory");
            assert_eq!(CsafDocumentCategory::string_normalize("csaf_security_incident_response"), "securityincidentresponse");
            assert_eq!(CsafDocumentCategory::string_normalize("csaf_security_advisory"), "securityadvisory");
            assert_eq!(CsafDocumentCategory::string_normalize("csaf_vex"), "vex");
            assert_eq!(CsafDocumentCategory::string_normalize("csaf_withdrawn"), "withdrawn");
            assert_eq!(CsafDocumentCategory::string_normalize("csaf_superseded"), "superseded");
            assert_eq!(CsafDocumentCategory::string_normalize("csaf_deprecated_security_advisory"), "deprecatedsecurityadvisory");
        }

        #[test]
        fn test_csaf_base_normalizes_to_base() {
            // casing
            assert_eq!(CsafDocumentCategory::string_normalize("csaf-basE"), "base");
            assert_eq!(CsafDocumentCategory::string_normalize("Csaf_base"), "base");

            // hyphen
            assert_eq!(CsafDocumentCategory::string_normalize("csaf‐base"), "base");
            // em dash
            assert_eq!(CsafDocumentCategory::string_normalize("csaf—base"), "base");
            // en dash
            assert_eq!(CsafDocumentCategory::string_normalize("csaf–base"), "base");
            // figure dash
            assert_eq!(CsafDocumentCategory::string_normalize("csaf‒base"), "base");
            // horizontal bar
            assert_eq!(CsafDocumentCategory::string_normalize("csaf―base"), "base");
            // non-breaking hyphen
            assert_eq!(CsafDocumentCategory::string_normalize("csaf‑base"), "base");
            // hyphen minus
            assert_eq!(CsafDocumentCategory::string_normalize("csaf-base"), "base");
            // minus
            assert_eq!(CsafDocumentCategory::string_normalize("csaf−base"), "base");

            // white spaces
            assert_eq!(CsafDocumentCategory::string_normalize("csaf base"), "base");
            assert_eq!(CsafDocumentCategory::string_normalize("csaf\tbase"), "base");

            // multiple
            assert_eq!(CsafDocumentCategory::string_normalize("csaf__base--"), "base");

        }

        #[test]
        fn test_leading_chars_before_csaf_normalizes_correctly() {
            // ` csaf_base`, `_csaf_base`, `-csaf_base` -> `base`
            assert_eq!(CsafDocumentCategory::string_normalize(" csaf_base"), "base");
            assert_eq!(CsafDocumentCategory::string_normalize("_csaf_base"), "base");
            assert_eq!(CsafDocumentCategory::string_normalize("-csaf_base"), "base");
        }

        #[test]
        fn test_without_csaf_prefix() {
            // `saf_base` -> `safbase`
            assert_eq!(CsafDocumentCategory::string_normalize("saf_base"), "safbase");
            // `_saf_base` -> `safbase`
            assert_eq!(CsafDocumentCategory::string_normalize("_saf_base"), "safbase");

        }

        #[test]
        fn test_other_category_normalizes_correctly() {
            // `Some_Other-Category` -> `someothercategory`
            assert_eq!(CsafDocumentCategory::string_normalize("Some_Other-Category"), "someothercategory");
        }

        #[test]
        fn test_empty_after_csaf_prefix() {
            // `Csaf_` -> ``
            assert_eq!(CsafDocumentCategory::string_normalize("Csaf_"), "");
        }

    }
}
