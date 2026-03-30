/// Enum representing CSAF versions
///
/// Contrary to other enums that are based on enums in the generated schemas, we are re-defining
/// this enum in the trait. Each schema only contains an enum with "their" version, and merging them
/// would be more complex than defining them here and mapping to them in each implementation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CsafVersion {
    X20,
    X21,
}
