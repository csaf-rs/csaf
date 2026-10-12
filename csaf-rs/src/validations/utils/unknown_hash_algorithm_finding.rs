use crate::csaf::types::csaf_hash_algo::CsafHashAlgorithm;
use crate::validation::TestFindingData;

/// Shared function to create a finding about a hash algorithm not being supported by the implementation.
/// The finding message differentiates between algorithms that are mentioned in the specification and those that are not.
pub(crate) fn create_unknown_hash_algorithm_finding_data(
    algorithm: &CsafHashAlgorithm,
    path: &str,
    hash_index: usize,
    file_hash_index: usize,
) -> TestFindingData {
    // TODO Right now, is_supported == is_mentioned_in_spec, and this only gets called when
    // TODO !is_supported, so this can't be reached.
    let message = if algorithm.is_mentioned_in_spec() {
        format!("Hash algorithm '{algorithm}' is listed in the specification but not supported by this implementation")
    } else {
        format!(
            "Hash algorithm '{algorithm}' is not listed in section 3.1.4.3.2 of the CSAF 2.1 specification \
             and not supported by this implementation"
        )
    };

    TestFindingData {
        message,
        instance_path: format!(
            "{path}/product_identification_helper/hashes/{hash_index}/file_hashes/{file_hash_index}/algorithm"
        ),
    }
}
