use crate::schema::csaf2_0::schema::AlgorithmOfTheCryptographicHash as AlgorithmOfTheCryptographicHash20;
use crate::schema::csaf2_1::schema::AlgorithmOfTheCryptographicHash as AlgorithmOfTheCryptographicHash21;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Enum representing supported hash algorithms
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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

    /// Checks if the algorithm is a known/supported algorithm.
    pub fn is_mentioned_in_spec(&self) -> bool {
        match self {
            CsafHashAlgorithm::Other(_) => !matches!(self.normalize(), CsafHashAlgorithm::Other(_)),
            _ => true,
        }
    }

    /// In the future, there will need to be a distinction between algorithms "supported" by the impl
    /// and the algorithms "mentioned in the spec", which need to all be supported.
    /// I.e. supported should be the superset of "mentioned in spec".
    ///
    /// For now, we just return "is_mentioned_in_spec".
    pub fn is_supported_algorithm(&self) -> bool {
        self.is_mentioned_in_spec()
    }

    /// Normalize a hash algorithm for matching purposes.
    ///
    /// TODO: Update this once https://github.com/oasis-tcs/csaf/issues/1264 has been answered.
    /// For now, we are only lowercasing the potentially cased HashAlgorithm::Other variants from
    /// CSAF 2.0. Further normalization could include trimming, removing hyphens, etc.
    pub fn normalize(&self) -> CsafHashAlgorithm {
        Self::lowercase_algorithm(self)
    }

    /// Converts the algorithm to lowercase, if lowercasing makes a difference.
    /// Returns `Some(lowercased)` if lowercasing makes a difference, `None` if not.
    ///
    /// All variants other than [CsafHashAlgorithm::Other] are lowercase by default.
    /// They always return `None`.
    ///
    /// This will also only have an effect on CSAF 2.0 documents, as in CSAF 2.1 docs,
    /// the hash algorithm values are lowercased by definition / regex.
    ///
    /// Currently unused, this might be needed for the converter.
    #[allow(dead_code)]
    fn checked_lowercase_algorithm(original: &CsafHashAlgorithm) -> Option<CsafHashAlgorithm> {
        match original {
            CsafHashAlgorithm::Other(original_str) => {
                let lowercased = CsafHashAlgorithm::from_str(original_str.to_lowercase().as_str());
                (original != &lowercased).then_some(lowercased)
            },
            _ => None,
        }
    }

    /// Converts the algorithm to lowercase, irrespective of whether lowercasing makes a difference.
    ///
    /// All variants other than [CsafHashAlgorithm::Other] are lowercase by default, here a clone of
    /// `original` is returned.
    ///
    /// Otherwise, it returns the lowercased algorithm.
    ///
    /// This will also only have an effect on CSAF 2.0 documents, as in CSAF 2.1 docs,
    /// the hash algorithm values are lowercased by definition / regex.
    ///
    /// This might be needed for the converter.
    fn lowercase_algorithm(original: &CsafHashAlgorithm) -> CsafHashAlgorithm {
        match original {
            CsafHashAlgorithm::Other(s) => CsafHashAlgorithm::from_str(s.to_lowercase().as_str()),
            _ => original.clone(),
        }
    }
}

impl CsafHashAlgorithm {
    /// Converts a string representation to a `CsafHashAlgorithm`.
    ///
    /// This is intentionally not a `From<&str>` impl to prevent external callers
    /// from using it. Use the `From<&AlgorithmOfTheCryptographicHash*>` impls instead.
    pub(crate) fn from_str(algo: &str) -> Self {
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
        CsafHashAlgorithm::from_str(algo.as_str())
    }
}

impl From<&AlgorithmOfTheCryptographicHash21> for CsafHashAlgorithm {
    fn from(algo: &AlgorithmOfTheCryptographicHash21) -> Self {
        CsafHashAlgorithm::from_str(algo.as_str())
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
    use rstest::rstest;

    #[test]
    fn test_from_str() {
        // Test from string for known algorithms
        assert_eq!(CsafHashAlgorithm::from_str("sha256"), CsafHashAlgorithm::Sha256);

        // Test from string for unknown algorithm
        assert_eq!(
            CsafHashAlgorithm::from_str("custom-algo"),
            CsafHashAlgorithm::Other("custom-algo".to_string())
        );
    }

    #[rstest]
    // known algorithms
    #[case(CsafHashAlgorithm::Sha256, true)]
    // Other with lowercase (and non-alphabetic chars)
    #[case(CsafHashAlgorithm::Other("customhash".to_string()), true)]
    #[case(CsafHashAlgorithm::Other("custom123".to_string()), true)]
    #[case(CsafHashAlgorithm::Other("custom-hash".to_string()), true)]
    // Other with only non-alphabetic chars
    #[case(CsafHashAlgorithm::Other("12345".to_string()), true)]
    #[case(CsafHashAlgorithm::Other("123-456".to_string()), true)]
    // Other with uppercase (and non-alphabetic chars)
    #[case(CsafHashAlgorithm::Other("Whirlpool".to_string()), false)]
    #[case(CsafHashAlgorithm::Other("Md5".to_string()), false)]
    #[case(CsafHashAlgorithm::Other("Sha3-512".to_string()), false)]
    fn test_is_lowercase(#[case] algo: CsafHashAlgorithm, #[case] expected: bool) {
        assert_eq!(algo.is_lowercase(), expected);
    }

    #[rstest]
    // Known algorithm
    #[case(CsafHashAlgorithm::Sha256, true)]
    // Other variant that is not a known variant with different casing
    #[case(CsafHashAlgorithm::Other("custom".to_string()), false)]
    #[case(CsafHashAlgorithm::Other("Custom".to_string()), false)]
    // Other variant that is a known variant with different casing
    #[case(CsafHashAlgorithm::Other("WHIRLPOOL".to_string()), true)]
    #[case(CsafHashAlgorithm::Other("MD5".to_string()), true)]
    #[case(CsafHashAlgorithm::Other("SHA3-256".to_string()), true)]
    fn test_is_known_algorithm(#[case] algo: CsafHashAlgorithm, #[case] expected: bool) {
        assert_eq!(algo.is_mentioned_in_spec(), expected);
    }

    #[rstest]
    // Known algorithms should remain unchanged
    #[case(CsafHashAlgorithm::Sha256, CsafHashAlgorithm::Sha256)]
    // Other variant already lowercase should remain unchanged
    #[case(CsafHashAlgorithm::Other("customhash".to_string()), CsafHashAlgorithm::Other("customhash".to_string()))]
    // Other variant with uppercase should be lowercased
    #[case(CsafHashAlgorithm::Other("CustomHash".to_string()), CsafHashAlgorithm::Other("customhash".to_string()))]
    // Other variant whose lowercase matches a known algorithm should become the known algorithm
    #[case(CsafHashAlgorithm::Other("SHA1".to_string()), CsafHashAlgorithm::Sha1)]
    fn test_convert_to_lowercase(#[case] input: CsafHashAlgorithm, #[case] expected: CsafHashAlgorithm) {
        assert_eq!(CsafHashAlgorithm::lowercase_algorithm(&input), expected);
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

    #[rstest]
    // Known variants
    #[case(CsafHashAlgorithm::Md5, CsafHashAlgorithm::Md5, true)]
    #[case(CsafHashAlgorithm::Md5, CsafHashAlgorithm::Sha1, false)]
    // Other variants
    #[case(CsafHashAlgorithm::Other("custom".to_string()), CsafHashAlgorithm::Other("custom".to_string()), true)]
    #[case(CsafHashAlgorithm::Other("HashFoo".to_string()), CsafHashAlgorithm::Other("HashBar".to_string()), false)]
    #[case(CsafHashAlgorithm::Other("custom".to_string()), CsafHashAlgorithm::Sha256, false)]
    #[case(CsafHashAlgorithm::Sha256, CsafHashAlgorithm::Other("custom".to_string()), false)]
    // Other variants with different casing
    #[case(CsafHashAlgorithm::Other("MD5".to_string()), CsafHashAlgorithm::Other("md5".to_string()), true)]
    #[case(CsafHashAlgorithm::Other("SHA3-256".to_string()), CsafHashAlgorithm::Other("sha3-256".to_string()), true)]
    // Other variant with different casing of a known variant
    #[case(CsafHashAlgorithm::Other("SHA256".to_string()), CsafHashAlgorithm::Sha256, true)]
    #[case(CsafHashAlgorithm::Sha256, CsafHashAlgorithm::Other("SHA256".to_string()), true)]
    #[case(CsafHashAlgorithm::Other("SSL3-MD5".to_string()), CsafHashAlgorithm::Ssl3Md5, true)]
    #[case(CsafHashAlgorithm::Ssl3Md5, CsafHashAlgorithm::Other("SSL3-MD5".to_string()), true)]
    fn test_normalize(#[case] a: CsafHashAlgorithm, #[case] b: CsafHashAlgorithm, #[case] expected: bool) {
        assert_eq!(a.normalize() == b.normalize(), expected);
    }
}
