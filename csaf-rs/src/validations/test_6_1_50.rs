use crate::csaf::enums::version_range::{VersionRange, VersionRangeError};
use crate::csaf_traits::{BranchTrait, CategoryOfTheBranch, CsafTrait, ProductTreeTrait};
use crate::validation::{TestFinding, TestFindingData};
use vers_rs::{VersError, VersionRange as VersVersionRange};

fn non_canonical_product_version_range_error(
    name: String,
    canonical: String,
    instance_path: impl Into<String>,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "The VERS version range given is not in canonical form: `{name}`, canonical would be vers={canonical}"
        ),
        instance_path: instance_path.into(),
    })
}

fn any_outside_of_vers_all_error(vers: &str, instance_path: impl Into<String>) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "The VERS version range `{vers}` uses the `*` comparator outside of the `vers:all` type, which is not allowed"
        ),
        instance_path: instance_path.into(),
    })
}

fn invalid_product_version_range_error(vers: String, message: String, instance_path: impl Into<String>) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("The version range `{vers}` is invalid: {message}"),
        instance_path: instance_path.into(),
    })
}

fn unsupported_vers_type_warning(vers: String, instance_path: impl Into<String>) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: format!("The version range `{vers}` uses a VERS type not supported by this implementation"),
        instance_path: instance_path.into(),
    })
}

/// 6.1.50 Product Version Range Rules
///
/// For each element of type `$['$defs'].branches_t` with `category` of `product_version_range`, it SHALL be tested
/// that the value of `name` complies with the rules given in section 3.1.3.3.2.
/// VERS types not supported by the implementation SHALL result in a warning which SHALL include the VERS type name used.
/// Nevertheless, all other rules SHALL be checked to the extent possible.
pub fn test_6_1_50_product_version_range_rules(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // skip if there is no product tree
    let Some(product_tree) = doc.get_product_tree() else {
        return Ok(());
    };

    let mut findings: Option<Vec<TestFinding>> = None;

    // for every branch
    product_tree.visit_all_branches(&mut |branch, path| {
        // only for product version range
        if branch.get_category() != CategoryOfTheBranch::ProductVersionRange {
            return;
        }

        // get branch name
        let name = branch.get_name();

        let version_range = VersionRange::parse(name);
        match version_range {
            VersionRange::DynamicVers(vers) => {
                if vers.versioning_scheme() != "all"
                    && vers.versioning_scheme() != "none"
                    && vers.constraints().len() == 1
                    && let Some(first_constraint) = vers.constraints().first()
                    && matches!(first_constraint.comparator, vers_rs::Comparator::Any)
                {
                    findings
                        .get_or_insert_default()
                        .push(any_outside_of_vers_all_error(name, path));
                }

                // check if the version range was in canonical form already
                if vers.to_string() != name {
                    findings
                        .get_or_insert_default()
                        .push(non_canonical_product_version_range_error(
                            name.to_string(),
                            vers.to_string(),
                            path,
                        ));
                }
            },
            VersionRange::Vls(_) => {
                // it was parsed as vls, there are no non-canonical forms for vls, so this is always fine
            },
            VersionRange::Invalid(VersionRangeError::InvalidVers(invalid_vers_error)) => {
                if matches!(
                    invalid_vers_error,
                    VersError::UnsupportedVersioningScheme(_) | VersError::UnknownVersioningScheme(_)
                ) {
                    findings
                        .get_or_insert_default()
                        .push(unsupported_vers_type_warning(name.to_string(), path));
                } else {
                    findings
                        .get_or_insert_default()
                        .push(invalid_product_version_range_error(
                            name.to_string(),
                            invalid_vers_error.to_string(),
                            path,
                        ));
                }
            },
            VersionRange::Invalid(VersionRangeError::InvalidVls(invalid_vls_error)) => {
                findings
                    .get_or_insert_default()
                    .push(invalid_product_version_range_error(
                        name.to_string(),
                        invalid_vls_error.to_string(),
                        path,
                    ));
            },
        }
    });

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_50, test_6_1_50_product_version_range_rules);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_50 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_6_1_50() {
        // TODO: Supplementary test case that exercises the "only vers:all / none may do the * comparator" rule.
        TESTS_2_1.test_6_1_50.expect(ExpectedResults {
            // TODO: This error message could be more percise, the problem are the whitespaces outside the constraint
            case_01: Err(vec![invalid_product_version_range_error(
                "all versions < 4.2.0".into(),
                "Invalid character(s) in VLS: ' '".to_string(),
                "/product_tree/branches/0/branches/0/branches/0",
            )]),
            // TODO: This error message could be more percise, whitespace is the offending char, not a control char
            case_02: Err(vec![invalid_product_version_range_error(
                "vers:npm/ >= 2.2.0| <2.3.0".into(),
                "Invalid characters in version range specifier: Control characters (tabs, newlines, carriage returns) are not permitted".to_string(),
                "/product_tree/branches/0/branches/0/branches/0",
            )]),
            // TODO: vers all needs to be supported
            case_03: Err(vec![unsupported_vers_type_warning(
                "vers:all/<V3.1.2.2".into(),
                "/product_tree/branches/0/branches/0/branches/0",
            )]),
            case_04: Err(vec![
                invalid_product_version_range_error(
                    "<= 2024.04".into(),
                    "Invalid character(s) in VLS: ' '".to_string(),
                    "/product_tree/branches/0/branches/0/branches/0",
                ),
                invalid_product_version_range_error(
                    "all versions".into(),
                    "Invalid character(s) in VLS: ' '".to_string(),
                    "/product_tree/branches/0/branches/1/branches/0",
                ),
            ]),
            case_05: Err(vec![
                invalid_product_version_range_error(
                    "> abc and <= def".into(),
                    "Invalid character(s) in VLS: ' '".to_string(),
                    "/product_tree/branches/0/branches/0/branches/0",
                ),
                invalid_product_version_range_error(
                    "all versions < v0.8.15".into(),
                    "Invalid character(s) in VLS: ' '".to_string(),
                    "/product_tree/branches/0/branches/1/branches/0",
                ),
            ]),
            // TODO: We should report the broken comparator / empty constraint instead of the unsupported schema
            case_06: Err(vec![
                unsupported_vers_type_warning(
                    "vers:someweirdunknownversiontype/<4.2.0|>3.91.1|<>3.2.0|<1.2".to_string(),
                    "/product_tree/branches/0/branches/0/branches/0",
                ),
                unsupported_vers_type_warning(
                    "vers:somecoolunknownversiontype/||>Jo_3,16|<=2.Ko_5,20".to_string(),
                    "/product_tree/branches/0/branches/0/branches/1",
                ),
            ]),
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
            // TODO: Typo in test file
            case_14: Err(vec![invalid_product_version_range_error(
                "vers:/all".into(),
                "Missing versioning scheme".to_string(),
                "/product_tree/branches/0/branches/1/branches/0",
            )]),
            case_15: Ok(()),
            case_16: Err(vec![
                unsupported_vers_type_warning(
                    "vers:someweirdunknownversiontype/<4.2.0|>3.91.1|!=3.2.0|<1.2".into(),
                    "/product_tree/branches/0/branches/0/branches/0",
                ),
                unsupported_vers_type_warning(
                    "vers:somecoolunknownversiontype/>Jo_3,16|<=2.Ko_5,20".into(),
                    "/product_tree/branches/0/branches/0/branches/1",
                ),
            ]),
        });
    }
}
