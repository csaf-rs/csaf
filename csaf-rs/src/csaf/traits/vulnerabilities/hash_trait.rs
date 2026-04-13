use crate::csaf::types::csaf_hash_algo::CsafHashAlgorithm;
use crate::csaf_traits::FileHashTrait;
use crate::schema::csaf2_0::schema::{CryptographicHashes as CryptographicHashes20, FileHash as FileHash20};
use crate::schema::csaf2_1::schema::{CryptographicHashes as CryptographicHashes21, FileHash as FileHash21};
use std::ops::Deref;

/// Trait representing a collection of file_hashes for a file as part of a product identification helper
pub trait HashTrait {
    type FileHashType: FileHashTrait;

    /// Returns the filename
    fn get_filename(&self) -> &String;

    /// returns the file hashes
    fn get_file_hashes(&self) -> &Vec<Self::FileHashType>;

    /// Returns true if only hashes with the specified algorithm are present.
    /// The hash algorithms in the document are normalized.
    /// The `algorithm` parameter is **NOT** normalized, as this is so far being run for known
    /// algorithms, which are normalized by definition.
    ///
    /// TODO: This might change based on https://github.com/oasis-tcs/csaf/issues/1264
    fn contains_only_hash_algorithm(&self, algorithm: CsafHashAlgorithm) -> bool {
        let file_hashes = self.get_file_hashes();
        if file_hashes.is_empty() {
            return false;
        }
        file_hashes.iter().all(|h| h.get_algorithm().normalize() == algorithm)
    }
}

impl HashTrait for CryptographicHashes20 {
    type FileHashType = FileHash20;

    fn get_filename(&self) -> &String {
        self.filename.deref()
    }

    fn get_file_hashes(&self) -> &Vec<Self::FileHashType> {
        self.file_hashes.as_ref()
    }
}

impl HashTrait for CryptographicHashes21 {
    type FileHashType = FileHash21;

    fn get_filename(&self) -> &String {
        self.filename.deref()
    }

    fn get_file_hashes(&self) -> &Vec<Self::FileHashType> {
        self.file_hashes.as_ref()
    }
}

#[cfg(test)]
mod tests_contains_only_hash_algorithm {
    use super::*;
    use CsafHashAlgorithm::*;
    use rstest::rstest;

    /// Mock implementing `FileHashTrait`
    struct MockFileHash(CsafHashAlgorithm);

    impl FileHashTrait for MockFileHash {
        fn get_hash(&self) -> &String {
            unimplemented!()
        }
        fn get_algorithm(&self) -> CsafHashAlgorithm {
            self.0.clone()
        }
    }

    /// Mock implementing `HashTrait`
    struct MockCryptographicHashes {
        file_hashes: Vec<MockFileHash>,
    }

    impl MockCryptographicHashes {
        fn new(algos: &[CsafHashAlgorithm]) -> Self {
            Self {
                file_hashes: algos.iter().map(|a| MockFileHash(a.clone())).collect(),
            }
        }
    }

    impl HashTrait for MockCryptographicHashes {
        type FileHashType = MockFileHash;

        fn get_filename(&self) -> &String {
            unimplemented!()
        }

        fn get_file_hashes(&self) -> &Vec<MockFileHash> {
            &self.file_hashes
        }
    }

    #[rstest]
    // single element
    #[case(&[Md5], Md5)]
    #[case(&[Sha1], Sha1)]
    // multiple identical
    #[case(&[Md5, Md5], Md5)]
    #[case(&[Sha1, Sha1, Sha1], Sha1)]
    fn returns_true(#[case] algos: &[CsafHashAlgorithm], #[case] expected: CsafHashAlgorithm) {
        let mock = MockCryptographicHashes::new(algos);
        assert!(mock.contains_only_hash_algorithm(expected));
    }

    #[rstest]
    // empty
    #[case(&[], Md5)]
    // single element
    #[case(&[Md5], Sha1)]
    // mixed algorithms
    #[case(&[Md5, Sha256], Md5)]
    #[case(&[Md5, Sha1, Sha256], Sha1)]
    // algorithm not used at all
    #[case(&[Md5, Sha256], Sha1)]
    fn returns_false(#[case] algos: &[CsafHashAlgorithm], #[case] expected: CsafHashAlgorithm) {
        let mock = MockCryptographicHashes::new(algos);
        assert!(!mock.contains_only_hash_algorithm(expected));
    }
}
