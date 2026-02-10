use crate::csaf_traits::{CsafTrait, Cwe, VulnerabilityTrait};
use crate::helpers::CWE_ENTRIES;
use crate::validation::ValidationError;

fn generate_incorrect_cwe_name_error(
    cwe: &str,
    given_name: &str,
    correct_name: &str,
    version: &str,
    path: &str,
) -> ValidationError {
    ValidationError {
        message: format!(
            "CWE '{cwe}' exists, but is '{correct_name}' instead of '{given_name}' in version '{version}'."
        ),
        instance_path: path.to_string(),
    }
}

fn generate_incorrect_cwe_error(cwe: &str, version: &str, path: &str) -> ValidationError {
    ValidationError {
        message: format!("CWE '{cwe}' does not exist in version '{version}'."),
        instance_path: path.to_string(),
    }
}

fn generate_incorrect_cwe_version_error(version: &str, path: &str) -> ValidationError {
    ValidationError {
        message: format!("Unknown CWE version '{version}'."),
        instance_path: path.to_string(),
    }
}

fn check_cwe(cwe: &Cwe, version: &str, path: &str, errors: &mut Vec<ValidationError>) {
    if !CWE_ENTRIES.contains_key(version) {
        errors.push(generate_incorrect_cwe_version_error(version, path));
    } else if let Some(cwe_name) = CWE_ENTRIES[version].get(&cwe.id) {
        if *cwe_name != cwe.name {
            errors.push(generate_incorrect_cwe_name_error(
                &cwe.id, &cwe.name, cwe_name, version, path,
            ));
        }
    } else {
        errors.push(generate_incorrect_cwe_error(&cwe.id, version, path));
    }
}

pub fn test_6_1_11_cwe(doc: &impl CsafTrait, use_2_1: bool) -> Result<(), Vec<ValidationError>> {
    let vulnerabilities = doc.get_vulnerabilities();
    let mut errors = Vec::new();

    // Map occurrence paths indexes to CVE identifiers
    for (i_r, vulnerability) in vulnerabilities.iter().enumerate() {
        let cwe = vulnerability.get_cwe();
        if let Some(cwe) = cwe {
            for (i_cwe, cwe_item) in cwe.iter().enumerate() {
                match use_2_1 {
                    true => check_cwe(
                        cwe_item,
                        cwe_item.version.as_deref().unwrap_or("<empty>"),
                        format!("/vulnerabilities/{i_r}/cwes/{i_cwe}").as_str(),
                        &mut errors,
                    ),
                    false => check_cwe(
                        cwe_item,
                        "latest",
                        format!("/vulnerabilities/{i_r}/cwe").as_str(),
                        &mut errors,
                    ),
                }
            }
        }
    }

    match errors.len() {
        0 => Ok(()),
        _ => Err(errors),
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_11
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_11_cwe(doc, false)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_11
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_11_cwe(doc, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_11() {
        TESTS_2_0.test_6_1_11.expect(Err(vec![generate_incorrect_cwe_name_error(
            "CWE-79",
            "Improper Input Validation",
            "Improper Neutralization of Input During Web Page Generation ('Cross-site Scripting')",
            "latest",
            "/vulnerabilities/0/cwe",
        )]));
        TESTS_2_1.test_6_1_11.expect(
            Err(vec![generate_incorrect_cwe_name_error(
                "CWE-79",
                "Improper Input Validation",
                "Improper Neutralization of Input During Web Page Generation ('Cross-site Scripting')",
                "4.13",
                "/vulnerabilities/0/cwes/0",
            )]),
            Err(vec![generate_incorrect_cwe_error(
                "CWE-1419",
                "4.12",
                "/vulnerabilities/0/cwes/0",
            )]),
            Err(vec![generate_incorrect_cwe_name_error(
                "CWE-1324",
                "Sensitive Information Accessible by Physical Probing of JTAG Interface",
                "DEPRECATED: Sensitive Information Accessible by Physical Probing of JTAG Interface",
                "4.10",
                "/vulnerabilities/0/cwes/0",
            )]),
            Err(vec![generate_incorrect_cwe_name_error(
                "CWE-1192",
                "Improper Identifier for IP Block used in System-On-Chip (SOC)",
                "System-on-Chip (SoC) Using Components without Unique, Immutable Identifiers",
                "4.13",
                "/vulnerabilities/0/cwes/0",
            )]),
            Err(vec![generate_incorrect_cwe_error(
                "CWE-19",
                "2.11",
                "/vulnerabilities/0/cwes/0",
            )]),
            Err(vec![generate_incorrect_cwe_error(
                "CWE-1008",
                "4.13",
                "/vulnerabilities/1/cwes/1",
            )]),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
