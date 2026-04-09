use crate::csaf::consts::chars::{is_hyphen_dash_char, is_invisible_char, is_underscore_char};
use crate::csaf::enums::csaf_version::CsafVersion;
use crate::schema::csaf2_0::schema::DocumentCategory as DocumentCategory20;
use crate::schema::csaf2_1::schema::DocumentCategory as DocumentCategory21;
use std::fmt::{Display, Formatter, Result as FmtResult};

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
    // is also csaf_base. This variant represents this case, saving the original string for later
    // validation (see test 6.1.26).
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
        matches!(
            self,
            CsafDocumentCategory::CsafBase | CsafDocumentCategory::CsafBaseOther(_)
        )
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

    /// Helper function to remove whitespace, underscores and (various unicode) dashes / hyphens from a string
    ///
    /// There is a known issue in CSAF 2.0 around these ignored chars, i.e. the standard only states
    /// "dash, whitespace, and underscore characters" to be relevant characters.
    /// In CSAF 2.1, this was clarified to include "[...] minus, white space, and underscore [...] and
    /// "[...] Dash and hyphen characters (independent of their graphical variants) [...]". This is a best-effort
    /// implementation to cover as many of these characters as possible (and might need to be updated).
    ///
    /// We additionally cover some zero-width / invisible characters which would also break validation.
    fn get_with_ignored_chars_removed(s: &str) -> String {
        s.chars()
            .filter(|c| !(c.is_whitespace() || is_invisible_char(c) || is_hyphen_dash_char(c) || is_underscore_char(c)))
            .collect()
    }

    /// Helper function to check if a string starts with `csaf_`. See [Self::starts_with_csaf_underscore]
    /// for more details.
    #[inline]
    fn string_starts_with_csaf_underscore(s: &str) -> bool {
        // Lowercase and Split the string at "csaf"
        match s.to_lowercase().split_once("csaf") {
            None => {
                // There is no "csaf" in the string
                false
            },
            Some((prefix, postfix)) => {
                // Check if everything before "csaf" is only whitespace or hyphen / underscore variants
                if !Self::get_with_ignored_chars_removed(prefix).is_empty() {
                    return false;
                }
                postfix.chars().next().is_some_and(|c| is_underscore_char(&c))
            },
        }
    }

    /// Checks if the category string starts with `csaf_` (case-insensitive), where the `_` can be
    /// any of the known underscore variant characters from [is_underscore_char].
    /// Also checks that everything before `csaf_` consists only of whitespace, underscores and hyphens variants.
    ///
    /// Examples:
    /// `csaf_base` -> true
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

    /// Helper function to check if a string starts with `csaf_deprecated_`. See [Self::starts_with_csaf_deprecated]
    /// for more details.
    #[inline]
    fn string_starts_with_csaf_deprecated_underscore(s: &str) -> bool {
        let lower = s.to_lowercase();
        // Split at "csaf"
        match lower.split_once("csaf") {
            None => false,
            Some((prefix, after_csaf)) => {
                // Everything before "csaf" must be only ignored chars
                if !Self::get_with_ignored_chars_removed(prefix).is_empty() {
                    return false;
                }
                // First char after "csaf" must be an underscore variant
                let mut chars = after_csaf.chars();
                if !chars.next().is_some_and(|c| is_underscore_char(&c)) {
                    return false;
                }
                let after_first_underscore = chars.as_str();
                // Must continue with "deprecated" followed by another underscore variant
                after_first_underscore
                    .strip_prefix("deprecated")
                    .is_some_and(|s| s.chars().next().is_some_and(|c| is_underscore_char(&c)))
            },
        }
    }

    /// Checks if the category string starts with `csaf_deprecated_` (case-insensitive), where the `_` can be
    /// any of the known underscore variant characters from [is_underscore_char].
    /// Also checks that everything before `csaf_deprecated_` consists only of whitespace, underscores and hyphens variants.
    ///
    /// Examples:
    /// `csaf_deprecated_security_advisory` -> true
    /// ` csaf_deprecated_security_advisory` -> true
    /// `CSAF_DEPRECATED_foo` -> true
    /// `csaf＿deprecated＿foo` -> true
    /// `csaf_base` -> false
    /// `csaf_vex` -> false
    pub fn starts_with_csaf_deprecated(&self) -> bool {
        // The only known variant starting with csaf_deprecated_ is CsafDeprecatedSecurityAdvisory
        if matches!(self, CsafDocumentCategory::CsafDeprecatedSecurityAdvisory) {
            return true;
        }

        // For CsafBaseOther, check the actual string with unicode-aware logic
        if let CsafDocumentCategory::CsafBaseOther(s) = self {
            return Self::string_starts_with_csaf_deprecated_underscore(s);
        }

        // All other known variants don't start with csaf_deprecated_
        false
    }

    /// Helper function to normalize a category string
    #[inline]
    fn string_normalize(s: &str) -> String {
        // lowercase
        let mut normalized = s.to_lowercase();
        // remove ignored chars
        normalized = Self::get_with_ignored_chars_removed(&normalized);
        // remove leading "csaf"
        normalized.strip_prefix("csaf").unwrap_or(&normalized).to_string()
    }

    /// Normalizes the document category string by removing leading "csaf" and any whitespace, hyphen or underscore.
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
    use rstest::rstest;

    #[rstest]
    // basic example
    #[case("csaf_base", true)]
    // casing
    #[case("csaf_basE", true)]
    #[case("CSAF_base", true)]
    // leading (multiple) whitespace, hyphen, underscore variants
    #[case(" csaf_base", true)]
    #[case("_csaf_base", true)]
    #[case("-csaf_base", true)]
    // this is U+FF3F Fullwidth Low Line
    #[case("＿csaf_base", true)]
    #[case("__csaf_base", true)]
    #[case(" _ csaf_base", true)]
    // underscore variant in "middle" underscore
    #[case("csaf＿base", true)]
    // not starting with csaf
    #[case("saf_base", false)]
    fn string_starts_with_csaf_underscore(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(
            CsafDocumentCategory::string_starts_with_csaf_underscore(input),
            expected,
            "input: {input:?}"
        );
    }

    #[rstest]
    // basic example
    #[case("csaf_deprecated_security_advisory", true)]
    // known other categories
    #[case("csaf_base", false)]
    #[case("csaf_vex", false)]
    #[case("csaf_security_advisory", false)]
    #[case("csaf_informational_advisory", false)]
    #[case("csaf_security_incident_response", false)]
    #[case("csaf_withdrawn", false)]
    #[case("csaf_superseded", false)]
    // casing
    #[case("CSAF_DEPRECATED_SOMETHING", true)]
    #[case("Csaf_Deprecated_Something", true)]
    // with underscore variants
    #[case("csaf\u{FF3F}deprecated\u{FF3F}foo", true)]
    #[case("csaf_deprecated\u{FF3F}bar", true)]
    #[case("csaf\u{FF3F}deprecated_bar", true)]
    // with leading underscore, hyphen, whitespace
    #[case(" csaf_deprecated_foo", true)]
    #[case("_csaf_deprecated_foo", true)]
    #[case("-csaf_deprecated_foo", true)]
    // no underscore before / after deprecated
    #[case("csaf_deprecated", false)]
    #[case("csafdeprecated_foo", false)]
    // no csaf prefix
    #[case("deprecated_something", false)]
    #[case("some_other_category", false)]
    fn string_starts_with_csaf_deprecated_underscore(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(
            CsafDocumentCategory::string_starts_with_csaf_deprecated_underscore(input),
            expected,
            "input: {input:?}"
        );
    }

    #[rstest]
    // Known profiles
    #[case("csaf_base", "base")]
    #[case("csaf_informational_advisory", "informationaladvisory")]
    #[case("csaf_security_incident_response", "securityincidentresponse")]
    #[case("csaf_security_advisory", "securityadvisory")]
    #[case("csaf_vex", "vex")]
    #[case("csaf_withdrawn", "withdrawn")]
    #[case("csaf_superseded", "superseded")]
    #[case("csaf_deprecated_security_advisory", "deprecatedsecurityadvisory")]
    // Casing variants
    #[case("csaf-basE", "base")]
    #[case("Csaf_base", "base")]
    // Hyphen / dash / underscore variants
    #[case("csaf‐base", "base")]
    #[case("csaf＿base", "base")]
    // Whitespace variants
    #[case("csaf base", "base")]
    #[case("csaf\tbase", "base")]
    // Multiple ignored chars
    #[case("csaf__base--", "base")]
    // Leading ignored chars
    #[case(" csaf_base", "base")]
    #[case("_csaf_base", "base")]
    #[case("-csaf_base", "base")]
    #[case("__csaf_base", "base")]
    // Without csaf prefix
    #[case("saf_base", "safbase")]
    #[case("_saf_base", "safbase")]
    #[case("Some_Other-Category", "someothercategory")]
    // Empty after csaf prefix
    #[case("Csaf_", "")]
    fn string_normalize(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(
            CsafDocumentCategory::string_normalize(input),
            expected,
            "input: {input:?}"
        );
    }
}
