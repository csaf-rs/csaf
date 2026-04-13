use crate::csaf::consts::chars::is_invisible_char;
use crate::csaf_traits::{BranchTrait, CategoryOfTheBranch, CsafTrait, ProductTreeTrait};
use crate::validation::ValidationError;

fn create_misuse_at_vendor_name_error(vendor_name: &str, path: &str) -> ValidationError {
    let message: String  = match vendor_name == "Open Source" {
        true => "For vendor branches, the branch name 'Open Source' is banned due to misuse. Please use a more appropriate vendor name for the open source project.".to_string(),
        false => format!("For vendor branches, the branch name 'Open Source' (and similar, provided: '{vendor_name}') is banned due to misuse. Please use a more appropriate vendor name for the open source project."),
    };
    ValidationError {
        message,
        instance_path: format!("{path}/name"),
    }
}

/// Checks if a name is `Open Source` (case-insensitive, white space insensitive).
///
/// We additionally remove zero-width/invisible chars that would otherwise break the matching ([is_invisible_char])
#[inline]
fn is_open_source(name: &str) -> bool {
    let normalized: String = name
        .chars()
        .filter(|c| !c.is_whitespace() && !is_invisible_char(c))
        .collect::<String>()
        .to_lowercase();
    normalized == "opensource"
}
/// 6.2.48 Misuse at Vendor Name
///
/// For each item in branches with category `vendor` it MUST be tested that the name is not
/// `Open Source` (case-insensitive, white space insensitive).
pub fn test_6_2_48_misuse_at_vendor_name(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let Some(product_tree) = doc.get_product_tree() else {
        return Ok(()); // TODO #409
    };

    let mut errors: Option<Vec<ValidationError>> = None;

    product_tree.visit_all_branches(&mut |branch, path| {
        if branch.get_category() == &CategoryOfTheBranch::Vendor && is_open_source(branch.get_name()) {
            errors
                .get_or_insert_default()
                .push(create_misuse_at_vendor_name_error(branch.get_name(), path));
        }
    });

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_2_48, test_6_2_48_misuse_at_vendor_name);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use rstest::rstest;

    #[test]
    fn test_test_6_2_48() {
        let case_01_open_source_isduba = Err(vec![create_misuse_at_vendor_name_error(
            "Open Source",
            "/product_tree/branches/0",
        )]);
        let case_02_open_source_curl = Err(vec![create_misuse_at_vendor_name_error(
            "Open Source",
            "/product_tree/branches/0",
        )]);
        let case_03_open_source_case_whitespace = Err(vec![create_misuse_at_vendor_name_error(
            "opensource",
            "/product_tree/branches/0",
        )]);

        // Case 11-13: Same as 01-03, but with appropriate values for the vendor branch name (ISDuBA Dev, curl, ...)
        // Case S11: product family branch with name "Open Source"

        TESTS_2_1.test_6_2_48.expect(
            case_01_open_source_isduba,
            case_02_open_source_curl,
            case_03_open_source_case_whitespace,
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }

    #[rstest]
    #[case("Open Source")]
    #[case("open source")]
    #[case("OPEN SOURCE")]
    #[case("OpEn SoUrCe")]
    #[case("opensource")]
    #[case("OPENSOURCE")]
    #[case("Open   Source")]
    #[case("  Open Source  ")]
    #[case("Open\tSource")]
    #[case("Open\nSource")]
    #[case("Open\u{00A0}Source")]
    #[case("Open\u{200B}Source")]
    fn test_is_open_source_true(#[case] name: &str) {
        assert!(
            is_open_source(name),
            "Expected '{name}' to be detected as 'Open Source'"
        );
    }

    #[rstest]
    #[case("ISDuBA Dev")]
    #[case("Open Source Ltd.")]
    #[case("My Open Source")]
    #[case("Open")]
    #[case("Source")]
    #[case(" ")]
    fn test_is_open_source_false(#[case] name: &str) {
        assert!(
            !is_open_source(name),
            "Expected '{name}' to NOT be detected as 'Open Source'"
        );
    }
}
