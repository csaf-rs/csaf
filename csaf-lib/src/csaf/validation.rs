use std::str::FromStr;

pub enum ValidationError {}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ValidationProfile {
    Basic,
    Extended,
    Full,
}

impl FromStr for ValidationProfile {
    type Err = ();

    fn from_str(input: &str) -> Result<ValidationProfile, Self::Err> {
        match input {
            "basic" => Ok(ValidationProfile::Basic),
            "extended" => Ok(ValidationProfile::Extended),
            "full" => Ok(ValidationProfile::Full),
            _ => Err(()),
        }
    }
}

pub trait Validate {
    /// Validates this object according to a validation profile
    fn validate_profile(&self, profile: ValidationProfile);

    /// Validates this object according to a specific test ID.
    fn validate_by_test(&self, version: &str);
}
