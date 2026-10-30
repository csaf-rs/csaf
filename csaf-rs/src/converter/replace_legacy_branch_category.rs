use crate::converter::helpers::{BranchTraversalWarning, for_each_branch_mut};
use crate::schema::loose;
use serde_json::Value;

use super::{Convert, Diagnostics, Settings, Warning};

pub struct ReplaceLegacyBranchCategory;

impl Convert for ReplaceLegacyBranchCategory {
    fn convert(&self, doc: &mut loose::Document, _settings: &Settings, diagnostics: &mut dyn Diagnostics) {
        let result = for_each_branch_mut(doc, |branch, path| {
            if let Some(Value::String(category)) = branch.get_mut("category")
                && category == "legacy"
            {
                *category = "product_name".to_string();

                diagnostics.warning(format!("{path}/category").into(), Warning::LegacyBranchCategoryReplaced);
            }
        });

        if let Err(error) = result {
            let (path, warning) = match error {
                BranchTraversalWarning::InvalidProductTree => ("/product_tree", Warning::InvalidProductTree),
                BranchTraversalWarning::InvalidBranches => ("/product_tree/branches", Warning::InvalidBranches),
            };

            diagnostics.warning(path.into(), warning);
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use crate::converter::tests::OnlyWarnings;

    use super::*;

    fn assert_conversion(input: Value, expected: Value, expected_warnings: &[(&str, Warning)]) {
        let mut doc = loose::Document::new(input);
        let mut diagnostics = OnlyWarnings::default();

        ReplaceLegacyBranchCategory.convert(&mut doc, &Settings::default(), &mut diagnostics);

        assert_eq!(expected, doc.into_raw());

        let warnings = diagnostics.warnings();
        assert_eq!(expected_warnings.len(), warnings.len());

        for ((path, warning), (expected_path, expected_warning)) in warnings.iter().zip(expected_warnings) {
            assert_eq!(path.as_ref(), *expected_path);
            assert_eq!(warning, expected_warning);
        }
    }

    #[test]
    fn replaces_legacy_category_in_root_branch() {
        let input = json!({
            "product_tree": {
                "branches": [{
                    "category": "legacy",
                    "name": "Old Product",
                    "product": {
                        "name": "Old Product",
                        "product_id": "product-1"
                    }
                }]
            }
        });

        let expected = json!({
            "product_tree": {
                "branches": [{
                    "category": "product_name",
                    "name": "Old Product",
                    "product": {
                        "name": "Old Product",
                        "product_id": "product-1"
                    }
                }]
            }
        });

        assert_conversion(
            input,
            expected,
            &[(
                "/product_tree/branches/0/category",
                Warning::LegacyBranchCategoryReplaced,
            )],
        );
    }

    #[test]
    fn replaces_legacy_category_in_nested_branches() {
        let input = json!({
            "product_tree": {
                "branches": [{
                    "category": "vendor",
                    "name": "Vendor",
                    "branches": [{
                        "category": "legacy",
                        "name": "Old Product",
                        "branches": [{
                            "category": "legacy",
                            "name": "Older Product"
                        }]
                    }]
                }]
            }
        });

        let expected = json!({
            "product_tree": {
                "branches": [{
                    "category": "vendor",
                    "name": "Vendor",
                    "branches": [{
                        "category": "product_name",
                        "name": "Old Product",
                        "branches": [{
                            "category": "product_name",
                            "name": "Older Product"
                        }]
                    }]
                }]
            }
        });

        assert_conversion(
            input,
            expected,
            &[
                (
                    "/product_tree/branches/0/branches/0/category",
                    Warning::LegacyBranchCategoryReplaced,
                ),
                (
                    "/product_tree/branches/0/branches/0/branches/0/category",
                    Warning::LegacyBranchCategoryReplaced,
                ),
            ],
        );
    }

    #[test]
    fn replaces_multiple_legacy_branches() {
        let input = json!({
            "product_tree": {
                "branches": [
                    {
                        "category": "legacy",
                        "name": "Product A"
                    },
                    {
                        "category": "vendor",
                        "name": "Vendor"
                    },
                    {
                        "category": "legacy",
                        "name": "Product B"
                    }
                ]
            }
        });

        let expected = json!({
            "product_tree": {
                "branches": [
                    {
                        "category": "product_name",
                        "name": "Product A"
                    },
                    {
                        "category": "vendor",
                        "name": "Vendor"
                    },
                    {
                        "category": "product_name",
                        "name": "Product B"
                    }
                ]
            }
        });

        assert_conversion(
            input,
            expected,
            &[
                (
                    "/product_tree/branches/0/category",
                    Warning::LegacyBranchCategoryReplaced,
                ),
                (
                    "/product_tree/branches/2/category",
                    Warning::LegacyBranchCategoryReplaced,
                ),
            ],
        );
    }

    #[test]
    fn preserves_branches_without_legacy_category() {
        let input = json!({
            "product_tree": {
                "branches": [{
                    "category": "vendor",
                    "name": "Vendor",
                    "branches": [{
                        "category": "product_name",
                        "name": "Product"
                    }]
                }]
            }
        });

        assert_conversion(input.clone(), input, &[]);
    }

    #[test]
    fn does_not_change_categories_outside_branches() {
        let input = json!({
            "document": {
                "category": "legacy"
            },
            "product_tree": {
                "branches": [{
                    "category": "vendor",
                    "name": "Vendor"
                }],
                "relationships": [{
                    "category": "legacy"
                }]
            }
        });

        assert_conversion(input.clone(), input, &[]);
    }

    #[test]
    fn handles_missing_product_tree() {
        let input = json!({
            "document": {
                "csaf_version": "2.0"
            }
        });

        assert_conversion(input.clone(), input, &[]);
    }

    #[test]
    fn handles_missing_branches() {
        let input = json!({
            "product_tree": {
                "full_product_names": []
            }
        });

        assert_conversion(input.clone(), input, &[]);
    }

    #[test]
    fn warns_when_product_tree_is_not_an_object() {
        let input = json!({
            "product_tree": ["category", "legacy"]
        });

        assert_conversion(input.clone(), input, &[("/product_tree", Warning::InvalidProductTree)]);
    }

    #[test]
    fn warns_when_branches_is_not_an_array() {
        // The `legacy` category must survive untouched: nothing is traversed at all
        let input = json!({
            "product_tree": {
                "branches": {
                    "category": "legacy",
                    "name": "Old Product"
                }
            }
        });

        assert_conversion(
            input.clone(),
            input,
            &[("/product_tree/branches", Warning::InvalidBranches)],
        );
    }

    #[test]
    fn skips_branch_items_that_are_not_objects() {
        let input = json!({
            "product_tree": {
                "branches": [
                    "not an object",
                    {
                        "category": "legacy",
                        "name": "Product"
                    }
                ]
            }
        });

        let expected = json!({
            "product_tree": {
                "branches": [
                    "not an object",
                    {
                        "category": "product_name",
                        "name": "Product"
                    }
                ]
            }
        });

        assert_conversion(
            input,
            expected,
            &[(
                "/product_tree/branches/1/category",
                Warning::LegacyBranchCategoryReplaced,
            )],
        );
    }
}
