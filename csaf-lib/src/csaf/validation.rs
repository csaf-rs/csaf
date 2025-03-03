use crate::csaf::csaf2_1::schema::CategoryOfTheRemediation;
use crate::csaf::getter_traits::{CsafTrait, RemediationTrait, VulnerabilityTrait};
use std::collections::HashMap;
use std::str::FromStr;

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

static MUT_EX_MEASURES: &[CategoryOfTheRemediation] = &[
    CategoryOfTheRemediation::NoneAvailable,
    CategoryOfTheRemediation::Workaround,
    CategoryOfTheRemediation::Mitigation,
];

static MUT_EX_FIX_STATES: &[CategoryOfTheRemediation] = &[
    CategoryOfTheRemediation::NoneAvailable,
    CategoryOfTheRemediation::NoFixPlanned,
    CategoryOfTheRemediation::FixPlanned,
    CategoryOfTheRemediation::OptionalPatch,
    CategoryOfTheRemediation::VendorFix,
];

pub fn test_6_01_35_contradicting_remediations(
    target: &impl CsafTrait,
) -> Result<(), String> {
    for v in target.get_vulnerabilities().iter() {
        // Data struct to store observed remediation categories per product IT
        let mut product_categories: HashMap<String, Vec<CategoryOfTheRemediation>> = HashMap::new();
        for r in v.get_remediations().iter() {
            // Only handle Remediations having product IDs associated
            if let Some(product_ids) = r.get_all_product_ids(target) {
                // Category of current remediation
                let cat = r.get_category();
                // Iterate over product IDs
                for p in product_ids {
                    // Check if product ID has categories associated
                    if let Some(exist_cat_set) = product_categories.get(&p) {
                        // Check if any seen category conflicts with the current one
                        if exist_cat_set.iter().any(|e_cat| {
                            MUT_EX_MEASURES.contains(e_cat) && MUT_EX_MEASURES.contains(&cat)
                            || MUT_EX_FIX_STATES.contains(e_cat) && MUT_EX_FIX_STATES.contains(&cat)
                        }) {
                            return Err(format!(
                                "Product {} has contradicting remediations: {} and {}",
                                p, exist_cat_set.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", "), cat
                            ));
                        }
                        let mut new_cat_vec = exist_cat_set.clone();
                        new_cat_vec.push(cat.clone());
                        product_categories.insert(p, new_cat_vec);
                    } else {
                        product_categories.insert(p, Vec::from([cat.clone()]));
                    }
                }
            }
        }
    }
    Ok(())
}

