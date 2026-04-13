use crate::csaf::types::csaf_hash_algo::CsafHashAlgorithm;
use crate::schema::csaf2_0::schema::FileHash as FileHash20;
use crate::schema::csaf2_1::schema::FileHash as FileHash21;
use std::ops::Deref;

/// Trait representing a file_hash, identified by the used hash algorithm and the hash
pub trait FileHashTrait {
    /// Returns the hash
    fn get_hash(&self) -> &String;

    /// Returns the hashing algorithm as HashAlgorithm enum
    fn get_algorithm(&self) -> CsafHashAlgorithm;
}

impl FileHashTrait for FileHash20 {
    fn get_algorithm(&self) -> CsafHashAlgorithm {
        CsafHashAlgorithm::from(&self.algorithm)
    }

    fn get_hash(&self) -> &String {
        self.value.deref()
    }
}

impl FileHashTrait for FileHash21 {
    fn get_hash(&self) -> &String {
        self.value.deref()
    }

    fn get_algorithm(&self) -> CsafHashAlgorithm {
        CsafHashAlgorithm::from(&self.algorithm)
    }
}
