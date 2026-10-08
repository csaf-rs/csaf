use crate::csaf::enums::version_range::VersionRange;
use crate::csaf_traits::{BranchTrait, CategoryOfTheBranch, CsafTrait, ProductTreeTrait};
use crate::validation::{TestFinding, TestFindingData};

fn product_version_range_only_one_version_error(instance_path: impl Into<String>) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: "The version range given identifies just a single version.".to_string(),
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
            let version_range = VersionRange::new(branch.get_name());

            // error if the version_range does not identify more than one version
            if version_range.is_single_version() == Some(true) {
                errors
                    .get_or_insert_default()
                    .push(product_version_range_only_one_version_error(path));
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

    #[test]
    fn test_6_1_59() {
        TESTS_2_1.test_6_1_59.expect(ExpectedResults {
            // Case 01: invalid vers (`vers:intdot/4.2.0`)
            case_01: Err(vec![product_version_range_only_one_version_error(
                "/product_tree/branches/0/branches/0/branches/0",
            )]),
            // Case 02: invalid vls (`4.2.0`)
            case_02: Err(vec![product_version_range_only_one_version_error(
                "/product_tree/branches/0/branches/0/branches/0",
            )]),
            // Case 11: No product_version_range, only product_version
            case_11: Ok(()),
            // Case 12: valid vers (`vers:intdot/<4.2.0`)
            case_12: Ok(()),
            // Case 13: valid vls (`<4.2.0`)
            case_13: Ok(()),
        });
    }
}
