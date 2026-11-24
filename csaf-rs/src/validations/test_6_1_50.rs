use crate::csaf::enums::version_range::{VersionRange, VersionRangeError};
use crate::csaf_traits::{BranchTrait, CategoryOfTheBranch, CsafTrait, ProductTreeTrait};
use crate::validation::{TestFinding, TestFindingData};
use vers_like_specifier::Vls;
use vers_rs::{VersError, VersionRange as VersVersionRange};

fn non_canonical_product_version_range_error(
    name: String,
    canonical: String,
    instance_path: impl Into<String>,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "The VERS version range given is not in canonical form: `{name}`, canonical would be {canonical}"
        ),
        instance_path: instance_path.into(),
    })
}

fn any_outside_of_vers_all_error(vers: &str, instance_path: impl Into<String>) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "The VERS version range `{vers}` uses the `*` comparator outside of the `vers:all` type, which is not allowed for CSAF"
        ),
        instance_path: instance_path.into(),
    })
}

fn vls_single_version_error(vls: &str, instance_path: impl Into<String>) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "The VLS version range `{vls}` specifies a single version, which is not allowed for VLS in CSAF"
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

fn unsupported_vers_type_warning(vers: &str, vers_type: &str, instance_path: impl Into<String>) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: format!(
            "The version range `{vers}` uses the VERS type `{vers_type}`, which is not supported by this implementation"
        ),
        instance_path: instance_path.into(),
    })
}

fn vers_all_with_constraints_error(vers: &str, instance_path: impl Into<String>) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "The VERS version range `{vers}` is invalid: the VERS type `all` is only valid with the constraint `*` (`vers:all/*`) in CSAF"
        ),
        instance_path: instance_path.into(),
    })
}

/// Checks a VERS version range whose VERS type is not supported by `vers-rs`.
///
/// - `vers:all` is not a VERS type in the sense of the VERS specification, but CSAF defines `vers:all/*` as the
///   special string to convey "all versions". It is therefore fully checkable here, and no warning is issued.
/// - For all other types, the canonical form and the comparison semantics can't be checked. Therefore a warning
///   including the VERS type is emitted, and the constraints are checked on a best-effort basis, i.e. whether they are
///   a syntactically valid list of `<comparator><version>` pairs (see [`Vls`]), and whether the `*` comparator is used
///   outside of `vers:all`.
fn check_unsupported_vers_type(name: &str, instance_path: &str, findings: &mut Option<Vec<TestFinding>>) {
    let Some((vers_type, constraints)) = name.strip_prefix("vers:").and_then(|rest| rest.split_once('/')) else {
        // not splittable into type and constraints, nothing more can be checked
        return;
    };

    if vers_type == "all" {
        if constraints != "*" {
            findings
                .get_or_insert_default()
                .push(vers_all_with_constraints_error(name, instance_path));
        }
        return;
    }

    findings
        .get_or_insert_default()
        .push(unsupported_vers_type_warning(name, vers_type, instance_path));

    if constraints == "*" {
        findings
            .get_or_insert_default()
            .push(any_outside_of_vers_all_error(name, instance_path));
    } else if let Err(vls_error) = constraints.parse::<Vls>() {
        findings
            .get_or_insert_default()
            .push(invalid_product_version_range_error(
                name.to_string(),
                vls_error.to_string(),
                instance_path,
            ));
    }
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
                // TODO: Opened an issue with vers to add this as a util function, will be removed later
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
            VersionRange::Vls(vls) => {
                if vls.is_single_version() {
                    findings
                        .get_or_insert_default()
                        .push(vls_single_version_error(name, path));
                }
            },
            VersionRange::Invalid(VersionRangeError::InvalidVers(invalid_vers_error)) => {
                if matches!(
                    invalid_vers_error,
                    VersError::UnsupportedVersioningScheme(_) | VersError::UnknownVersioningScheme(_)
                ) {
                    check_unsupported_vers_type(name, path, &mut findings);
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
        // TODO: Supplementary test case for single version VLS
        TESTS_2_1.test_6_1_50.expect(ExpectedResults {
            // TODO: This error message could be more percise, the problem are the whitespaces outside the constraint
            case_01: Err(vec![invalid_product_version_range_error(
                "all versions < 4.2.0".into(),
                "Invalid constraint(s): Invalid character(s) in version string: ' ', '<'".to_string(),
                "/product_tree/branches/0/branches/0/branches/0",
            )]),
            // TODO: This error message could be more precise, whitespace is the offending char, not a control char
            case_02: Err(vec![invalid_product_version_range_error(
                "vers:npm/ >= 2.2.0| <2.3.0".into(),
                "Invalid characters in version range specifier: Control characters (tabs, newlines, carriage returns) are not permitted".to_string(),
                "/product_tree/branches/0/branches/0/branches/0",
            )]),
            // vers:all only allows the `*` constraint
            case_03: Err(vec![vers_all_with_constraints_error(
                "vers:all/<V3.1.2.2",
                "/product_tree/branches/0/branches/0/branches/0",
            )]),
            case_04: Err(vec![invalid_product_version_range_error(
                "all versions".into(),
                "Invalid constraint(s): Invalid character(s) in version string: ' '".to_string(),
                "/product_tree/branches/0/branches/1/branches/0",
            )]),
            case_05: Err(vec![
                invalid_product_version_range_error(
                    "> abc and <= def".into(),
                    "Invalid constraint(s): Invalid character(s) in version string: ' ', '<', '='"
                        .to_string(),
                    "/product_tree/branches/0/branches/0/branches/0",
                ),
                invalid_product_version_range_error(
                    "all versions < v0.8.15".into(),
                    "Invalid constraint(s): Invalid character(s) in version string: ' ', '<'"
                        .to_string(),
                    "/product_tree/branches/0/branches/1/branches/0",
                ),
            ]),
            // unsupported VERS types are checked on a best-effort basis: the broken comparator / empty
            // constraints are reported as errors, the unsupported type additionally as a warning
            case_06: Err(vec![
                invalid_product_version_range_error(
                    "vers:someweirdunknownversiontype/<4.2.0|>3.91.1|<>3.2.0|<1.2".to_string(),
                    "Invalid constraint(s): Invalid character(s) in version string: '>'".to_string(),
                    "/product_tree/branches/0/branches/0/branches/0",
                ),
                unsupported_vers_type_warning(
                    "vers:someweirdunknownversiontype/<4.2.0|>3.91.1|<>3.2.0|<1.2",
                    "someweirdunknownversiontype",
                    "/product_tree/branches/0/branches/0/branches/0",
                ),
                invalid_product_version_range_error(
                    "vers:somecoolunknownversiontype/||>Jo_3,16|<=2.Ko_5,20".to_string(),
                    "Invalid constraint(s): Empty constraint, Empty constraint".to_string(),
                    "/product_tree/branches/0/branches/0/branches/1",
                ),
                unsupported_vers_type_warning(
                    "vers:somecoolunknownversiontype/||>Jo_3,16|<=2.Ko_5,20",
                    "somecoolunknownversiontype",
                    "/product_tree/branches/0/branches/0/branches/1",
                ),
            ]),
            // Supplementary case: `*` outside of `vers:all` for an unsupported type, and `vers:all` with constraints
            case_s01: Err(vec![
                unsupported_vers_type_warning(
                    "vers:someweirdunknownversiontype/*",
                    "someweirdunknownversiontype",
                    "/product_tree/branches/0/branches/0/branches/0",
                ),
                any_outside_of_vers_all_error(
                    "vers:someweirdunknownversiontype/*",
                    "/product_tree/branches/0/branches/0/branches/0",
                ),
                vers_all_with_constraints_error(
                    "vers:all/>=1.0.0|<2.0.0",
                    "/product_tree/branches/0/branches/0/branches/1",
                ),
            ]),
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
            // `vers:all/*` is the special string for "all versions" and valid
            case_14: Ok(()),
            case_15: Ok(()),
            // Supplementary case: a single version is a valid constraint for VERS (unlike for vls), also for unsupported types
            case_s11: Err(vec![
                unsupported_vers_type_warning(
                    "vers:someweirdunknownversiontype/1.2.3",
                    "someweirdunknownversiontype",
                    "/product_tree/branches/0/branches/0/branches/0",
                ),
                unsupported_vers_type_warning(
                    "vers:somecoolunknownversiontype/>=1.0|<2.0",
                    "somecoolunknownversiontype",
                    "/product_tree/branches/0/branches/0/branches/1",
                ),
            ]),
            case_16: Err(vec![
                unsupported_vers_type_warning(
                    "vers:someweirdunknownversiontype/<4.2.0|>3.91.1|!=3.2.0|<1.2",
                    "someweirdunknownversiontype",
                    "/product_tree/branches/0/branches/0/branches/0",
                ),
                unsupported_vers_type_warning(
                    "vers:somecoolunknownversiontype/>Jo_3,16|<=2.Ko_5,20",
                    "somecoolunknownversiontype",
                    "/product_tree/branches/0/branches/0/branches/1",
                ),
            ]),
        });
    }
}
