use std::collections::HashMap;
use std::str::FromStr;

pub enum ValidationError {}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
    fn validate_profile(&'static self, profile: ValidationProfile);

    /// Validates this object according to a specific test ID.
    fn validate_by_test(&self, version: &str);
}

pub type Test<VersionedDocument> =
    fn(&VersionedDocument) -> Result<(), String>;

/// Represents something which is validatable according to the CSAF standard.
/// This trait MUST be implemented by the struct that represents a CSAF document
/// in the respective version.
///
/// It can then be used to validate documents with either [validate_by_profile] or [validate_by_test].
pub trait Validatable<VersionedDocument> {
    /// Returns a hashmap containing the test ID per profile
    fn profiles(&self) -> HashMap<ValidationProfile, Vec<&str>>;

    /// Returns a hashmap containing the test function per test ID
    fn tests(&self) -> HashMap<&str, Test<VersionedDocument>>;

    fn doc(&self) -> &VersionedDocument;
}

/// Executes all tests of the specified [profile] against the [target]
/// (which is of type [VersionedDocument], e.g. a CSAF 2.0 document).
pub fn validate_by_profile<VersionedDocument>(
    target: &impl Validatable<VersionedDocument>,
    profile: ValidationProfile,
) {
    println!("Validating document with {:?} profile... \n", profile);

    // Loop through tests
    if let Some(tests) = target.profiles().get(&profile) {
        for test_id in tests {
            println!("Executing Test {}... ", test_id);
            validate_by_test(target, test_id);

            println!()
        }
    } else {
        println!("No tests found for profile")
    }
}

pub fn validate_by_test<VersionedDocument>(
    target: &impl Validatable<VersionedDocument>,
    test_id: &str,
) {
    if let Some(test_fn) = target.tests().get(test_id) {
        let _ = match test_fn(target.doc()) {
            Ok(()) => println!("> Test Success"),
            Err(e) => println!("> Error: {}", e),
        };
    } else {
        println!("Test with ID {} is missing implementation", test_id);
    }
}
