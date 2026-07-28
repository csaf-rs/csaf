#[cfg(feature = "converter")]
pub mod converter;
#[cfg(feature = "validation")]
pub mod csaf;
#[cfg(feature = "validation")]
pub mod csaf2_0;
#[cfg(feature = "validation")]
pub mod csaf2_1;
#[cfg(feature = "validation")]
pub mod csaf_traits;
#[cfg(feature = "validation")]
pub(crate) mod cvss;
#[cfg(feature = "validation")]
pub mod helpers;
pub mod json;
pub(crate) mod macros;
pub mod schema;
#[cfg(all(test, feature = "validation"))]
pub mod test_result_comparison;
#[cfg(test)]
pub mod test_structure;
#[cfg(feature = "validation")]
pub mod test_validation;
#[cfg(feature = "validation")]
pub mod validation;
#[cfg(feature = "validation")]
pub mod validation_result;
#[cfg(feature = "validation")]
pub mod validations;

/// The CVSS metric types returned by `ContentTrait`'s typed accessors
/// (`get_cvss_v2_typed`, `get_cvss_v3_typed`, `get_cvss_v4_typed`).
#[cfg(feature = "validation")]
pub use cvss_rs;
/// The SSVC selection types returned by `ContentTrait::get_ssvc_v2`.
#[cfg(feature = "validation")]
pub use ssvc;
