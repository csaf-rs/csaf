use serde_json::Value;

use crate::schema::loose;

use super::{Convert, Diagnostics, Settings, Warning};

pub struct ReplaceLegacyBranchCategory;

impl Convert for ReplaceLegacyBranchCategory {
    fn convert(&self, doc: &mut loose::Document, _settings: &Settings, diagnostics: &mut dyn Diagnostics) {
        let Some(product_tree) = doc.v20_view_mut().product_tree_mut() else {
            return;
        };

        let Ok(product_tree) = product_tree.caution_as_object_mut() else {
            return;
        };

        let Some(Value::Array(branches)) = product_tree.get_mut("branches") else {
            return;
        };

        replace_legacy_in_branches(branches, "/product_tree/branches", diagnostics);
    }
}

fn replace_legacy_in_branches(branches: &mut [Value], path: &str, diagnostics: &mut dyn Diagnostics) {
    for (index, branch) in branches.iter_mut().enumerate() {
        let Value::Object(branch) = branch else {
            continue;
        };

        let branch_path = format!("{path}/{index}");

        if let Some(Value::String(category)) = branch.get_mut("category")
            && category == "legacy"
        {
            *category = "product_name".to_string();

            diagnostics.warning(
                format!("{branch_path}/category").into(),
                Warning::LegacyBranchCategoryReplaced,
            );
        }

        if let Some(Value::Array(children)) = branch.get_mut("branches") {
            replace_legacy_in_branches(children, &format!("{branch_path}/branches"), diagnostics);
        }
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
