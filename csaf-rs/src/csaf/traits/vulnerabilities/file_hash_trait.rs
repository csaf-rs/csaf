use crate::csaf::traits::util::impl_str_field_getter;
use crate::csaf::types::csaf_hash_algo::CsafHashAlgorithm;
use crate::schema::csaf2_0::schema::FileHash as FileHash20;
use crate::schema::csaf2_1::schema::FileHash as FileHash21;

/// Trait representing a file_hash, identified by the used hash algorithm and the hash
pub trait FileHashTrait {
    /// Returns the hash
    fn get_hash(&self) -> &str;

    /// Returns the hashing algorithm as HashAlgorithm enum
    fn get_algorithm(&self) -> CsafHashAlgorithm;
}

impl FileHashTrait for FileHash20 {
    fn get_algorithm(&self) -> CsafHashAlgorithm {
        CsafHashAlgorithm::from(&self.algorithm)
    }

    impl_str_field_getter!(get_hash, value);
}

impl FileHashTrait for FileHash21 {
    impl_str_field_getter!(get_hash, value);

    fn get_algorithm(&self) -> CsafHashAlgorithm {
        CsafHashAlgorithm::from(&self.algorithm)
    }
}
