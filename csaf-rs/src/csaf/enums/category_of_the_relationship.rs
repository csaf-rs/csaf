use strum::{AsRefStr, Display};

/// Enum representing the category of a subpath in a product path
///
/// A shared type is needed for traits, but CSAF version 2.0 and version 2.1 share the same
/// categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, AsRefStr)]
pub enum CategoryOfTheRelationship {
    #[strum(serialize = "default_component_of")]
    DefaultComponentOf,
    #[strum(serialize = "external_component_of")]
    ExternalComponentOf,
    #[strum(serialize = "installed_on")]
    InstalledOn,
    #[strum(serialize = "installed_with")]
    InstalledWith,
    #[strum(serialize = "optional_component_of")]
    OptionalComponentOf,
}
