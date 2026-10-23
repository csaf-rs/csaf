use crate::converter::helpers::for_each_branch_mut;
use crate::schema::loose;
use serde_json::Value;

use super::{Convert, Diagnostics, Settings, Warning};

pub struct ReplaceLegacyBranchCategory;

impl Convert for ReplaceLegacyBranchCategory {
    fn convert(&self, doc: &mut loose::Document, _settings: &Settings, diagnostics: &mut dyn Diagnostics) {
        for_each_branch_mut(doc, |branch, path| {
            if let Some(Value::String(category)) = branch.get_mut("category")
                && category == "legacy"
            {
                *category = "product_name".to_string();

                diagnostics.warning(format!("{path}/category").into(), Warning::LegacyBranchCategoryReplaced);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use crate::converter::tests::OnlyWarnings;

    use super::*;

    fn assert_conversion(input: Value, expected: Value, expected_paths: &[&str]) {
        let mut doc = loose::Document::new(input);
        let mut diagnostics = OnlyWarnings::default();

        ReplaceLegacyBranchCategory.convert(&mut doc, &Settings::default(), &mut diagnostics);

        assert_eq!(expected, doc.into_raw());

        let warnings = diagnostics.warnings();
        assert_eq!(expected_paths.len(), warnings.len());

        for ((path, warning), expected_path) in warnings.iter().zip(expected_paths) {
            assert_eq!(path.as_ref(), *expected_path);
            assert_eq!(warning, &Warning::LegacyBranchCategoryReplaced);
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

        assert_conversion(input, expected, &["/product_tree/branches/0/category"]);
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
                "/product_tree/branches/0/branches/0/category",
                "/product_tree/branches/0/branches/0/branches/0/category",
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
            &["/product_tree/branches/0/category", "/product_tree/branches/2/category"],
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
}
