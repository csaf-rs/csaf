use std::collections::HashMap;
use std::str::FromStr;
use crate::csaf::csaf2_1::schema::CategoryOfTheRemediation;
use crate::csaf::getter_traits::{CsafTrait, RemediationTrait, VulnerabilityTrait};

pub enum ValidationError {}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ValidationPreset {
    Basic,
    Extended,
    Full,
}

impl FromStr for ValidationPreset {
    type Err = ();

    fn from_str(input: &str) -> Result<ValidationPreset, Self::Err> {
        match input {
            "basic" => Ok(ValidationPreset::Basic),
            "extended" => Ok(ValidationPreset::Extended),
            "full" => Ok(ValidationPreset::Full),
            _ => Err(()),
        }
    }
}

pub trait Validate {
    /// Validates this object according to a validation preset
    fn validate_preset(&'static self, preset: ValidationPreset);

    /// Validates this object according to a specific test ID.
    fn validate_by_test(&self, version: &str);
}

pub type Test<VersionedDocument> =
    fn(&VersionedDocument) -> Result<(), String>;

/// Represents something which is validatable according to the CSAF standard.
/// This trait MUST be implemented by the struct that represents a CSAF document
/// in the respective version.
///
/// It can then be used to validate documents with either [validate_by_preset] or [validate_by_test].
pub trait Validatable<VersionedDocument> {
    /// Returns a hashmap containing the test ID per preset
    fn presets(&self) -> HashMap<ValidationPreset, Vec<&str>>;

    /// Returns a hashmap containing the test function per test ID
    fn tests(&self) -> HashMap<&str, Test<VersionedDocument>>;

    fn doc(&self) -> &VersionedDocument;
}

/// Executes all tests of the specified [preset] against the [target]
/// (which is of type [VersionedDocument], e.g. a CSAF 2.0 document).
pub fn validate_by_preset<VersionedDocument>(
    target: &impl Validatable<VersionedDocument>,
    preset: ValidationPreset,
) {
    println!("Validating document with {:?} preset... \n", preset);

    // Loop through tests
    if let Some(tests) = target.presets().get(&preset) {
        for test_id in tests {
            println!("Executing Test {}... ", test_id);
            validate_by_test(target, test_id);

            println!()
        }
    } else {
        println!("No tests found for preset")
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

pub fn test_6_01_35_contradicting_remediations(
    target: &impl CsafTrait,
) -> Result<(), String> {
    for v in target.get_vulnerabilities().iter() {
        let mut product_categories: HashMap<String, CategoryOfTheRemediation> = HashMap::new();
        for r in v.get_remediations().iter() {
            if let Some(product_ids) = r.get_product_ids() {
                let category = r.get_category();
                for p in product_ids.iter() {
                    if let Some(existing_category) = product_categories.get(p) {
                        if existing_category != &category {
                            return Err(format!(
                                "Product {} has contradicting remediations: {} and {}",
                                p, existing_category, category
                            ));
                        }
                    }
                    product_categories.insert(p.clone(), category.clone());
                }
            }
        }
    }
    Ok(())
}

