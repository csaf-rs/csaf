use crate::csaf_traits::{CsafTrait, VulnerabilityTrait};
use crate::validation::ValidationError;
use std::collections::HashMap;

fn generate_duplicate_cve_error(cve: &str, path: usize) -> ValidationError {
    ValidationError {
        message: format!("Duplicate usage of same CVE identifier '{}'", cve),
        instance_path: format!("/vulnerabilities/{}/cve", path),
    }
}

/// Test 6.1.23: Multiple Use of Same CVE
///
/// Vulnerability items must not contain the same string in the `/vulnerabilities[]/cve` field.
pub fn test_6_1_23_multiple_use_of_same_cve(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let vulnerabilities = doc.get_vulnerabilities();

    // Map occurrence paths indexes to CVE identifiers
    let mut cve_paths: HashMap<String, Vec<usize>> = HashMap::new();
    for (i_r, vulnerability) in vulnerabilities.iter().enumerate() {
        let cve = vulnerability.get_cve();
        if let Some(cve) = cve {
            let path = cve_paths.entry(cve.clone()).or_default();
            path.push(i_r);
        }
    }

    // Generate errors for CVE identifiers with multiple occurrence paths indexes
    let mut errors = Vec::new();
    for (cve, paths) in &cve_paths {
        if paths.len() > 1 {
            errors.extend(paths.iter().map(|path| generate_duplicate_cve_error(cve, *path)));
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};

    #[test]
    fn test_test_6_1_23() {
        let errors = std::collections::HashMap::from([(
            "01",
            vec![
                generate_duplicate_cve_error("CVE-2017-0145", 0),
                generate_duplicate_cve_error("CVE-2017-0145", 1),
            ],
        )]);
        run_csaf20_tests("23", test_6_1_23_multiple_use_of_same_cve, errors.clone());
        run_csaf21_tests("23", test_6_1_23_multiple_use_of_same_cve, errors);
    }
}
