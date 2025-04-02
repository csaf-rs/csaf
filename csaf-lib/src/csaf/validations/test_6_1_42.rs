use crate::csaf::getter_traits::{BranchTrait, CsafTrait, FullProductNameTrait, ProductIdentificationHelperTrait, ProductTreeTrait, RelationshipTrait};
use crate::csaf::validation::ValidationError;
use purl::GenericPurl;

pub fn test_6_1_42_purl_consistency(
    doc: &impl CsafTrait,
) -> Result<(), ValidationError> {
    // Skip if product_tree is None
    if let Some(product_tree) = doc.get_product_tree() {
        // Check full_product_names
        for (i, fpn) in product_tree.get_full_product_names().iter().enumerate() {
            if let Some(helper) = fpn.get_product_identification_helper() {
                if let Some(purls) = helper.get_purls() {
                    check_purls_consistency(
                        purls,
                        &format!("/product_tree/full_product_names/{}/product_identification_helper/purls", i)
                    )?;
                }
            }
        }

        // Check branches recursively
        if let Some(branches) = product_tree.get_branches() {
            check_branches_recursive(branches, "/product_tree/branches")?;
        }

        // Check relationships
        for (index, rel) in product_tree.get_relationships().iter().enumerate() {
            let fpn = rel.get_full_product_name();
            if let Some(helper) = fpn.get_product_identification_helper() {
                if let Some(purls) = helper.get_purls() {
                    check_purls_consistency(
                        purls,
                        &format!("/product_tree/relationships/{}/full_product_name/product_identification_helper/purls", index)
                    )?;
                }
            }
        }
    }

    Ok(())
}

// Helper function to check purl consistency in branches recursively
fn check_branches_recursive(
    branches: &[impl BranchTrait],
    path_base: &str,
) -> Result<(), ValidationError> {
    for (index, branch) in branches.iter().enumerate() {
        let current_path = format!("{}/{}", path_base, index);

        // Check product in branch if exists
        if let Some(product) = branch.get_product() {
            if let Some(helper) = product.get_product_identification_helper() {
                if let Some(purls) = helper.get_purls() {
                    check_purls_consistency(
                        purls,
                        &format!("{}/product/product_identification_helper/purls", current_path)
                    )?;
                }
            }
        }

        // Check sub-branches recursively
        if let Some(sub_branches) = branch.get_branches() {
            check_branches_recursive(
                sub_branches,
                &format!("{}/branches", current_path)
            )?;
        }
    }

    Ok(())
}

fn check_purls_consistency(purls: &[String], json_path: &str) -> Result<(), ValidationError> {
    if purls.len() <= 1 {
        return Ok(());
    }

    let mut base_parts: Option<String> = None;

    for (i, purl_str) in purls.iter().enumerate() {
        // Parse the PURL
        let purl = match purl_str.parse::<GenericPurl<String>>() {
            Ok(p) => p,
            Err(_) => {
                return Err(ValidationError {
                    message: format!("Invalid PURL format: {}", purl_str),
                    instance_path: format!("{}/{}", json_path, i),
                });
            }
        };

        // Strip qualifiers
        let current_parts = match purl.into_builder().without_qualifiers().build() {
            Ok(purl) => purl.to_string(),
            Err(_) => {
                return Err(ValidationError {
                    message: format!("Error whilst stripping qualifiers from PURL: {}", purl_str),
                    instance_path: format!("{}/{}", json_path, i),
                });
            },
        };

        if let Some(ref base) = base_parts {
            // Must always match
            if current_parts != *base {
                return Err(ValidationError {
                    message: String::from("PURLs within the same product_identification_helper must only differ in qualifiers"),
                    instance_path: format!("{}/{}", json_path, i),
                });
            }
        } else {
            // First PURL becomes the base for comparison
            base_parts = Some(current_parts);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::csaf::test_helper::run_csaf21_tests;
    use crate::csaf::validation::ValidationError;
    use crate::csaf::validations::test_6_1_42::test_6_1_42_purl_consistency;

    static ERROR_MESSAGE: &str = "PURLs within the same product_identification_helper must only differ in qualifiers";

    #[test]
    fn test_test_6_1_42() {
        run_csaf21_tests(
            "42",
            test_6_1_42_purl_consistency, HashMap::from([
                ("01", &ValidationError {
                    message: ERROR_MESSAGE.to_string(),
                    instance_path: "/product_tree/full_product_names/0/product_identification_helper/purls/1".to_string(),
                }),
                ("02", &ValidationError {
                    message: ERROR_MESSAGE.to_string(),
                    instance_path: "/product_tree/branches/0/branches/0/branches/0/product/product_identification_helper/purls/2".to_string(),
                }),
            ])
        );
    }
}
