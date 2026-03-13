use std::fmt::{Display, Formatter, Result as FmtResult};

/// Newtype wrapper for integer versioning (e.g., "1", "42") around `u64`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntVerVersion(u64);

impl IntVerVersion {
    /// Returns the version number
    pub fn get(&self) -> u64 {
        self.0
    }
}

impl Display for IntVerVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for IntVerVersion {
    fn from(value: u64) -> Self {
        IntVerVersion(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intver_version() {
        let version = IntVerVersion::from(42);
        assert_eq!(version.get(), 42);
        assert_eq!(version.to_string(), "42");
    }
}
