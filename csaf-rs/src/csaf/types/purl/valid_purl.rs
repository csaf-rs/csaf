use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidPurl {
    original_purl: String,
    normalized_purl: String,
    base_without_qualifiers: String,
}

impl ValidPurl {
    pub(super) fn new(original_purl: String, normalized_purl: String, base_without_qualifiers: String) -> Self {
        Self {
            original_purl,
            normalized_purl,
            base_without_qualifiers,
        }
    }

    #[cfg(test)]
    pub fn new_for_test(original_purl: String, normalized_purl: String, base_without_qualifiers: String) -> Self {
        Self::new(original_purl, normalized_purl, base_without_qualifiers)
    }

    pub fn original_purl(&self) -> &str {
        &self.original_purl
    }

    pub fn normalized_purl(&self) -> &str {
        &self.normalized_purl
    }

    pub fn base_without_qualifiers(&self) -> &str {
        &self.base_without_qualifiers
    }
}

impl Display for ValidPurl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (normalized: {})", self.original_purl, self.normalized_purl)
    }
}
