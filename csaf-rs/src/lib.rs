#[cfg(feature = "converter")]
pub mod converter;
pub mod csaf;
pub mod csaf2_0;
pub mod csaf2_1;
pub mod csaf_traits;
pub(crate) mod cvss;
pub(crate) mod cwe_files;
pub mod helpers;
pub mod json;
pub(crate) mod macros;
pub mod schema;
#[cfg(test)]
pub mod test_result_comparison;
#[cfg(test)]
pub mod test_structure;
pub mod test_validation;
pub mod validation;
pub mod validation_result;
pub mod validations;

/// The CVSS metric types returned by `ContentTrait`'s typed accessors
/// (`get_cvss_v2_typed`, `get_cvss_v3_typed`, `get_cvss_v4_typed`).
pub use cvss_rs;
/// The SSVC selection types returned by `ContentTrait::get_ssvc_v2`.
pub use ssvc;
