use crate::schema::csaf2_0::schema::ModelNumber as ModelNumber20;
use crate::schema::csaf2_0::schema::SerialNumber as SerialNumber20;
use crate::schema::csaf2_0::schema::StockKeepingUnit as StockKeepingUnit20;
use crate::schema::csaf2_1::schema::ModelNumber as ModelNumber21;
use crate::schema::csaf2_1::schema::SerialNumber as SerialNumber21;
use crate::schema::csaf2_1::schema::StockKeepingUnit as StockKeepingUnit21;
use std::fmt::Display;

/// A helper struct to encapsulate the logic for counting unescaped '*' characters in product identification fields.
/// This will be extended once we get to the CSAF 2.0 -> 2.1 converter
pub(crate) struct ProductIdentificationHelperNumber(String);

impl ProductIdentificationHelperNumber {
    pub(crate) fn count_unescaped_stars_in(s: &str) -> u32 {
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
        Self::count_unescaped_stars_in(&self.0)
    }


    /// Escapes model, serial, or stock keeping unit characters for CSAF 2.1.
    ///
    /// Backslashes are always doubled because they introduce escape sequences in
    /// CSAF 2.1. Question marks and asterisks are escaped only when the caller
    /// configured them to be interpreted as literal characters; otherwise they
    /// retain their wildcard semantics.
    pub(crate) fn escape_for_csaf21(input: &str, question_mark_is_literal: bool, asterisk_is_literal: bool) -> String {
        let mut value = String::with_capacity(input.len());
        for character in input.chars() {
            match character {
                '\\' => value.push_str("\\\\"),
                '?' if question_mark_is_literal => value.push_str("\\?"),
                '*' if asterisk_is_literal => value.push_str("\\*"),
                _ => value.push(character),
            }
        }
        value
    }

    /// Returns whether `value` ends with an unescaped asterisk.
    ///
    /// A trailing asterisk preceded by an odd number of backslashes is
    /// escaped and therefore does not indicate an open-ended value.
    pub(crate) fn ends_in_unescaped_star(value: &str) -> bool {
        value.ends_with('*')
            && value
                .chars()
                .rev()
                .skip(1)
                .take_while(|character| *character == '\\')
                .count()
                % 2
                == 0
    }
}

pub struct CsafStockKeepingUnit(ProductIdentificationHelperNumber);

impl CsafStockKeepingUnit {
    pub fn count_unescaped_stars(&self) -> u32 {
        self.0.count_unescaped_stars()
    }
}

impl Display for CsafStockKeepingUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.0.fmt(f)
    }
}

impl From<&StockKeepingUnit20> for CsafStockKeepingUnit {
    fn from(value: &StockKeepingUnit20) -> Self {
        Self(ProductIdentificationHelperNumber(value.to_string()))
    }
}

impl From<&StockKeepingUnit21> for CsafStockKeepingUnit {
    fn from(value: &StockKeepingUnit21) -> Self {
        Self(ProductIdentificationHelperNumber(value.to_string()))
    }
}

impl From<&str> for CsafStockKeepingUnit {
    fn from(value: &str) -> Self {
        Self(ProductIdentificationHelperNumber(value.to_string()))
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
        Self(ProductIdentificationHelperNumber(value.to_string()))
    }
}

impl From<&ModelNumber21> for CsafModelNumber {
    fn from(value: &ModelNumber21) -> Self {
        Self(ProductIdentificationHelperNumber(value.to_string()))
    }
}

impl From<&str> for CsafModelNumber {
    fn from(value: &str) -> Self {
        Self(ProductIdentificationHelperNumber(value.to_string()))
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
        Self(ProductIdentificationHelperNumber(value.to_string()))
    }
}

impl From<&SerialNumber21> for CsafSerialNumber {
    fn from(value: &SerialNumber21) -> Self {
        Self(ProductIdentificationHelperNumber(value.to_string()))
    }
}

impl From<&str> for CsafSerialNumber {
    fn from(value: &str) -> Self {
        Self(ProductIdentificationHelperNumber(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::ProductIdentificationHelperNumber as PIHNumber;
    use rstest::rstest;

    #[rstest]
    #[case("abcdef", 0)]
    #[case("*", 1)]
    #[case("abc*def", 1)]
    #[case("abc*def*ghi", 2)]
    #[case("abc\\*def", 0)]
    #[case("abc\\\\*def", 1)]
    #[case("abc\\\\\\*def", 0)]
    #[case("abc\\\\\\\\*def", 1)]
    #[case("abc\\\\\\*\\\\\\*def", 0)]
    #[case("abc\\\\*\\\\*def", 2)]
    #[case("\\*\\*\\*", 0)]
    #[case("abc\\*def*ghi", 1)]
    #[case("abc\\\\*def\\*ghi", 1)]
    fn test_count_unescaped_stars(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(PIHNumber::count_unescaped_stars_in(input), expected);
    }
}
