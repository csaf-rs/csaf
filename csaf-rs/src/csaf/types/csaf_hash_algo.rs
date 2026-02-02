use std::fmt::{Display, Formatter, Result as FmtResult};
use crate::schema::csaf2_0::schema::AlgorithmOfTheCryptographicHash as AlgorithmOfTheCryptographicHash20;
use crate::schema::csaf2_1::schema::AlgorithmOfTheCryptographicHash as AlgorithmOfTheCryptographicHash21;

/// Enum representing supported hash algorithms
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CsafHashAlgorithm {
    Blake2b512,
    Blake2s256,
    Md4,
    Md5,
    Md5Sha1,
    Mdc2,
    Ripemd,
    Ripemd160,
    Rmd160,
    Sha1,
    Sha224,
    Sha256,
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    Sha384,
    Sha512,
    Sha512_224,
    Sha512_256,
    Shake128,
    Shake256,
    Sm3,
    Ssl3Md5,
    Ssl3Sha1,
    Whirlpool,
    Other(String),
}

impl CsafHashAlgorithm {

    /// Checks if the original algorithm string is lowercase
    pub fn is_lowercase(&self) -> bool {
        if let CsafHashAlgorithm::Other(algo) = self {
            // these could be not lowercase, so we need to check
            algo.chars().all(|c| !c.is_alphabetic() || c.is_lowercase())
        } else {
            // Known algorithms are always lowercase (as the openssl cli output is lowercase
            true
        }
    }

    /// Checks if the algorithm is a known algorithm
    pub fn is_known_algorithm(&self) -> bool {
        !matches!(self, CsafHashAlgorithm::Other(_))
    }

    /// Converts the algorithm to lowercase if it is an Other variant
    /// This might be helpful for the converter
    pub fn convert_to_lowercase(&self) -> CsafHashAlgorithm {
        match self {
            CsafHashAlgorithm::Other(algo) => CsafHashAlgorithm::from(algo.to_lowercase().as_str()),
            _ => self.clone(),
        }
    }
}

impl From<&str> for CsafHashAlgorithm {
    fn from(algo: &str) -> Self {
        match algo {
            "blake2b512" => CsafHashAlgorithm::Blake2b512,
            "blake2s256" => CsafHashAlgorithm::Blake2s256,
            "md4" => CsafHashAlgorithm::Md4,
            "md5" => CsafHashAlgorithm::Md5,
            "md5-sha1" => CsafHashAlgorithm::Md5Sha1,
            "mdc2" => CsafHashAlgorithm::Mdc2,
            "ripemd" => CsafHashAlgorithm::Ripemd,
            "ripemd160" => CsafHashAlgorithm::Ripemd160,
            "rmd160" => CsafHashAlgorithm::Rmd160,
            "sha1" => CsafHashAlgorithm::Sha1,
            "sha224" => CsafHashAlgorithm::Sha224,
            "sha256" => CsafHashAlgorithm::Sha256,
            "sha3-224" => CsafHashAlgorithm::Sha3_224,
            "sha3-256" => CsafHashAlgorithm::Sha3_256,
            "sha3-384" => CsafHashAlgorithm::Sha3_384,
            "sha3-512" => CsafHashAlgorithm::Sha3_512,
            "sha384" => CsafHashAlgorithm::Sha384,
            "sha512" => CsafHashAlgorithm::Sha512,
            "sha512-224" => CsafHashAlgorithm::Sha512_224,
            "sha512-256" => CsafHashAlgorithm::Sha512_256,
            "shake128" => CsafHashAlgorithm::Shake128,
            "shake256" => CsafHashAlgorithm::Shake256,
            "sm3" => CsafHashAlgorithm::Sm3,
            "ssl3-md5" => CsafHashAlgorithm::Ssl3Md5,
            "ssl3-sha1" => CsafHashAlgorithm::Ssl3Sha1,
            "whirlpool" => CsafHashAlgorithm::Whirlpool,
            other => CsafHashAlgorithm::Other(other.to_string()),
        }
    }
}

impl From<&AlgorithmOfTheCryptographicHash20> for CsafHashAlgorithm {
    fn from(algo: &AlgorithmOfTheCryptographicHash20) -> Self {
        CsafHashAlgorithm::from(algo.as_str())
    }
}

impl From<&AlgorithmOfTheCryptographicHash21> for CsafHashAlgorithm {
    fn from(algo: &AlgorithmOfTheCryptographicHash21) -> Self {
        CsafHashAlgorithm::from(algo.as_str())
    }
}

impl Display for CsafHashAlgorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                CsafHashAlgorithm::Blake2b512 => "blake2b512",
                CsafHashAlgorithm::Blake2s256 => "blake2s256",
                CsafHashAlgorithm::Md4 => "md4",
                CsafHashAlgorithm::Md5 => "md5",
                CsafHashAlgorithm::Md5Sha1 => "md5-sha1",
                CsafHashAlgorithm::Mdc2 => "mdc2",
                CsafHashAlgorithm::Ripemd => "ripemd",
                CsafHashAlgorithm::Ripemd160 => "ripemd160",
                CsafHashAlgorithm::Rmd160 => "rmd160",
                CsafHashAlgorithm::Sha1 => "sha1",
                CsafHashAlgorithm::Sha224 => "sha224",
                CsafHashAlgorithm::Sha256 => "sha256",
                CsafHashAlgorithm::Sha3_224 => "sha3-224",
                CsafHashAlgorithm::Sha3_256 => "sha3-256",
                CsafHashAlgorithm::Sha3_384 => "sha3-384",
                CsafHashAlgorithm::Sha3_512 => "sha3-512",
                CsafHashAlgorithm::Sha384 => "sha384",
                CsafHashAlgorithm::Sha512 => "sha512",
                CsafHashAlgorithm::Sha512_224 => "sha512-224",
                CsafHashAlgorithm::Sha512_256 => "sha512-256",
                CsafHashAlgorithm::Shake128 => "shake128",
                CsafHashAlgorithm::Shake256 => "shake256",
                CsafHashAlgorithm::Sm3 => "sm3",
                CsafHashAlgorithm::Ssl3Md5 => "ssl3-md5",
                CsafHashAlgorithm::Ssl3Sha1 => "ssl3-sha1",
                CsafHashAlgorithm::Whirlpool => "whirlpool",
                CsafHashAlgorithm::Other(other) => other.as_str(),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        // Test from string for known algorithms
        assert_eq!(CsafHashAlgorithm::from("sha256"), CsafHashAlgorithm::Sha256);

        // Test from string for unknown algorithm
        assert_eq!(CsafHashAlgorithm::from("custom-algo"), CsafHashAlgorithm::Other("custom-algo".to_string()));
    }

    #[test]
    fn test_is_lowercase_known_algorithms() {
        // All known algorithms should return true
        assert!(CsafHashAlgorithm::Sha256.is_lowercase());
    }

    #[test]
    fn test_is_lowercase_other_lowercase() {
        // Other variant with lowercase string should return true
        assert!(CsafHashAlgorithm::Other("customhash".to_string()).is_lowercase());
        assert!(CsafHashAlgorithm::Other("custom123".to_string()).is_lowercase());
        assert!(CsafHashAlgorithm::Other("custom-hash".to_string()).is_lowercase());
        // Other variant with no alphabetic characters should return true
        assert!(CsafHashAlgorithm::Other("12345".to_string()).is_lowercase());
        assert!(CsafHashAlgorithm::Other("123-456".to_string()).is_lowercase());
    }

    #[test]
    fn test_is_lowercase_other_uppercase() {
        // Other variant with uppercase characters should return false
        assert!(!CsafHashAlgorithm::Other("CustomHash".to_string()).is_lowercase());
        assert!(!CsafHashAlgorithm::Other("Custom123".to_string()).is_lowercase());
        assert!(!CsafHashAlgorithm::Other("Custom-hash".to_string()).is_lowercase());
    }

    #[test]
    fn test_is_known_algorithm() {
        // Known algorithms should return true
        assert!(CsafHashAlgorithm::Sha256.is_known_algorithm());

        // Other variant should return false
        assert!(!CsafHashAlgorithm::Other("custom".to_string()).is_known_algorithm());
    }

    #[test]
    fn test_convert_to_lowercase() {
        // Known algorithms should remain unchanged
        let sha256 = CsafHashAlgorithm::Sha256;
        assert_eq!(sha256.convert_to_lowercase(), CsafHashAlgorithm::Sha256);

        // Other variant with uppercase should be lowercased
        let upper = CsafHashAlgorithm::Other("CustomHash".to_string());
        let expected = CsafHashAlgorithm::Other("customhash".to_string());
        assert_eq!(upper.convert_to_lowercase(), expected);

        // Other variant already lowercase should remain unchanged
        let lower = CsafHashAlgorithm::Other("customhash".to_string());
        let expected_lower = CsafHashAlgorithm::Other("customhash".to_string());
        assert_eq!(lower.convert_to_lowercase(), expected_lower);

        // Other variant with whose lowercase is a known algorithm should be the known algorithm
        let lower = CsafHashAlgorithm::Other("SHA1".to_string());
        let expected_lower = CsafHashAlgorithm::Sha1;
        assert_eq!(lower.convert_to_lowercase(), expected_lower);
    }

    #[test]
    fn test_display() {
        // Test Display implementation
        assert_eq!(CsafHashAlgorithm::Sha256.to_string(), "sha256");
        assert_eq!(CsafHashAlgorithm::Other("custom".to_string()).to_string(), "custom");
    }

    #[test]
    fn test_from_algorithm_of_the_cryptographic_hash_20() {
        // Test conversion from csaf 20 schema
        let algo_20 = AlgorithmOfTheCryptographicHash20::try_from("sha256").unwrap();
        assert_eq!(CsafHashAlgorithm::from(&algo_20), CsafHashAlgorithm::Sha256);
    }

    #[test]
    fn test_from_algorithm_of_the_cryptographic_hash_21() {
        // Test conversion from csaf 21 schema
        let algo_21 = AlgorithmOfTheCryptographicHash21::try_from("sha256").unwrap();
        assert_eq!(CsafHashAlgorithm::from(&algo_21), CsafHashAlgorithm::Sha256);
    }
}
