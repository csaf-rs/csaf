use crate::csaf::enums::version_range::VersionRange;
use crate::csaf_traits::{BranchTrait, CategoryOfTheBranch, CsafTrait, ProductTreeTrait};
use crate::validation::{TestFinding, TestFindingData};

fn product_version_range_only_one_version_error(version_range: &str, instance_path: impl Into<String>) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("The product version range `{version_range}` given identifies just a single version."),
        instance_path: instance_path.into(),
    })
}

fn product_version_range_invalid_single_version_undetermined_info(
    version_range: &str,
    instance_path: impl Into<String>,
) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: format!(
            "The product version range `{version_range}` is invalid. We can't determine if it identifies a single version or not.",
        ),
        instance_path: instance_path.into(),
    })
}

/// Single Version as Product Version Range
///
/// For each product_version_range, it SHALL be tested that it does not identify only a single version.
pub fn test_6_1_59_product_version_range_identifies_single_version(
    doc: &impl CsafTrait,
) -> Result<(), Vec<TestFinding>> {
    let Some(product_tree) = doc.get_product_tree() else {
        return Ok(()); // this will be a Passed::NoData later (#409)
    };

    let mut errors: Option<Vec<TestFinding>> = None;

    product_tree.visit_all_branches(&mut |branch, path| {
        if branch.get_category() == CategoryOfTheBranch::ProductVersionRange {
            let branch_name = branch.get_name();
            let version_range = VersionRange::parse(branch_name);

            match version_range.is_single_version() {
                // not a single version
                Some(false) => {},
                // a single version
                Some(true) => {
                    errors
                        .get_or_insert_default()
                        .push(product_version_range_only_one_version_error(branch_name, path));
                },
                // we don't know
                None => {
                    errors.get_or_insert_default().push(
                        product_version_range_invalid_single_version_undetermined_info(branch_name, path),
                    );
                },
            }
        }
    });

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_59,
    test_6_1_59_product_version_range_identifies_single_version
);

#[cfg(test)]
mod tests {
    use super::product_version_range_only_one_version_error;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_59 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use crate::validation::{TestFinding, TestFindingData};
    use crate::validations::test_6_1_59::product_version_range_invalid_single_version_undetermined_info;

    #[test]
    fn product_version_range_only_one_version_error_generates_error_finding() {
        assert_eq!(
            product_version_range_only_one_version_error("4.2.0", "/product_tree/branches/0"),
            TestFinding::Error(TestFindingData {
                message: "The product version range `4.2.0` given identifies just a single version.".to_string(),
                instance_path: "/product_tree/branches/0".to_string(),
            })
        );
    }

    #[test]
    fn product_version_range_invalid_single_version_undetermined_info_generates_info_finding() {
        assert_eq!(
            product_version_range_invalid_single_version_undetermined_info(
                "invalid version",
                "/product_tree/branches/0",
            ),
            TestFinding::Information(TestFindingData {
                message: "The product version range `invalid version` is invalid. We can't determine if it identifies a single version or not."
                    .to_string(),
                instance_path: "/product_tree/branches/0".to_string(),
            })
        );
    }

    #[test]
    fn test_6_1_59() {
        TESTS_2_1.test_6_1_59.expect(ExpectedResults {
            // Case 01: invalid vers (`vers:intdot/4.2.0`)
            case_01: Err(vec![product_version_range_only_one_version_error(
                "vers:intdot/4.2.0",
                "/product_tree/branches/0/branches/0/branches/0",
            )]),
            // Case 02: invalid vls (`4.2.0`)
            case_02: Err(vec![product_version_range_only_one_version_error(
                "4.2.0",
                "/product_tree/branches/0/branches/0/branches/0",
            )]),
            // Case 11: No product_version_range, only product_version
            case_11: Ok(()),
            // Case 12: valid vers (`vers:intdot/<4.2.0`)
            case_12: Ok(()),
            // Case 13: valid vls (`<4.2.0`)
            case_13: Ok(()),
            case_s01: Err(vec![product_version_range_invalid_single_version_undetermined_info(
                "invalid version",
                "/product_tree/branches/0/branches/0/branches/0",
            )]),
        });
    }
}
