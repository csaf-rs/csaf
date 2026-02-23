use crate::schema::csaf2_0::schema::ModelNumber as ModelNumber20;
use crate::schema::csaf2_0::schema::SerialNumber as SerialNumber20;
use crate::schema::csaf2_1::schema::ModelNumber as ModelNumber21;
use crate::schema::csaf2_1::schema::SerialNumber as SerialNumber21;
use std::fmt::Display;

/// A helper struct to encapsulate the logic for counting unescaped '*' characters in product identification fields.
/// This will be extended once we get to the CSAF 2.0 -> 2.1 converter
struct ProductIdentificationHelperNumber(String);

impl ProductIdentificationHelperNumber {
    // Inlined for easier unit testability
    #[inline]
    fn count_unescaped_stars_impl(s: &str) -> u32 {
        let mut escaped = false;
        let mut count = 0u32;
        for c in s.chars() {
            match c {
                '\\' => escaped = !escaped,
                '*' if !escaped => count += 1,
                _ => escaped = false,
            }
        }
        count
    }

    /// Counts the number of unescaped '*' characters in a given string.
    /// An asterisk is considered "unescaped" if it is not preceded by a backslash ('\\').
    /// Consecutive backslashes alternate between escaping or not escaping characters.
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice to be analyzed.
    ///
    /// # Returns
    ///
    /// Returns the number of unescaped '*' characters found in the string.
    pub fn count_unescaped_stars(&self) -> u32 {
        Self::count_unescaped_stars_impl(&self.0)
    }
}

/// Wrapper for the CSAF 2.0 / 2.1 model number field, providing a method to count unescaped '*' characters.
pub struct CsafModelNumber(ProductIdentificationHelperNumber);

impl CsafModelNumber {
    pub fn count_unescaped_stars(&self) -> u32 {
        self.0.count_unescaped_stars()
    }
}

impl Display for CsafModelNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.0.fmt(f)
    }
}

impl From<&ModelNumber20> for CsafModelNumber {
    fn from(value: &ModelNumber20) -> Self {
        CsafModelNumber(ProductIdentificationHelperNumber(value.to_string()))
    }
}

impl From<&ModelNumber21> for CsafModelNumber {
    fn from(value: &ModelNumber21) -> Self {
        CsafModelNumber(ProductIdentificationHelperNumber(value.to_string()))
    }
}

impl From<&str> for CsafModelNumber {
    fn from(value: &str) -> Self {
        CsafModelNumber(ProductIdentificationHelperNumber(value.to_string()))
    }
}

/// Wrapper for the CSAF 2.0 / 2.1 serial number field, providing a method to count unescaped '*' characters.
pub struct CsafSerialNumber(ProductIdentificationHelperNumber);

impl Display for CsafSerialNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.0.fmt(f)
    }
}

impl CsafSerialNumber {
    pub fn count_unescaped_stars(&self) -> u32 {
        self.0.count_unescaped_stars()
    }
}

impl From<&SerialNumber20> for CsafSerialNumber {
    fn from(value: &SerialNumber20) -> Self {
        CsafSerialNumber(ProductIdentificationHelperNumber(value.to_string()))
    }
}

impl From<&SerialNumber21> for CsafSerialNumber {
    fn from(value: &SerialNumber21) -> Self {
        CsafSerialNumber(ProductIdentificationHelperNumber(value.to_string()))
    }
}

impl From<&str> for CsafSerialNumber {
    fn from(value: &str) -> Self {
        CsafSerialNumber(ProductIdentificationHelperNumber(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::ProductIdentificationHelperNumber as PIHNumber;

    #[test]
    fn test_count_unescaped_stars() {
        assert_eq!(PIHNumber::count_unescaped_stars_impl("abcdef"), 0);
        assert_eq!(PIHNumber::count_unescaped_stars_impl("*"), 1);
        assert_eq!(PIHNumber::count_unescaped_stars_impl("abc*def"), 1);
        assert_eq!(PIHNumber::count_unescaped_stars_impl("abc*def*ghi"), 2);
        assert_eq!(PIHNumber::count_unescaped_stars_impl("abc\\*def"), 0);
        assert_eq!(PIHNumber::count_unescaped_stars_impl("abc\\\\*def"), 1);
        assert_eq!(PIHNumber::count_unescaped_stars_impl("abc\\\\\\*def"), 0);
        assert_eq!(PIHNumber::count_unescaped_stars_impl("abc\\\\\\\\*def"), 1);
        assert_eq!(PIHNumber::count_unescaped_stars_impl("abc\\\\\\*\\\\\\*def"), 0);
        assert_eq!(PIHNumber::count_unescaped_stars_impl("abc\\\\*\\\\*def"), 2);
        assert_eq!(PIHNumber::count_unescaped_stars_impl("\\*\\*\\*"), 0);
        assert_eq!(PIHNumber::count_unescaped_stars_impl("abc\\*def*ghi"), 1);
        assert_eq!(PIHNumber::count_unescaped_stars_impl("abc\\\\*def\\*ghi"), 1);
    }
}
