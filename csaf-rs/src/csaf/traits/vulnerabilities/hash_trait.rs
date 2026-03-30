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

    /// Returns true if only hashes with the specified algorithm are present
    fn contains_only_hash_algorithm(&self, algorithm: CsafHashAlgorithm) -> bool {
        for hash in self.get_file_hashes() {
            if hash.get_algorithm() != algorithm {
                return false;
            }
        }
        true
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
