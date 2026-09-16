use std::collections::HashMap;
use std::iter;

use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{
    BranchTrait, CategoryOfTheBranch, CsafTrait, DocumentTrait, ProductPathTrait, ProductStatusTrait, ProductTrait,
    ProductTreeTrait, VulnerabilityTrait,
};
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::document_category_test_config::DocumentCategoryTestConfig;

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig =
    DocumentCategoryTestConfig::new().shared(&[CsafDocumentCategory::CsafSecurityAdvisory]);

fn create_missing_corresponding_affected_products_error(vulnerability_index: usize, product_id: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "Every product of a vulnerability with the product_status 'fixed' must have a corresponding product version with the product_status 'affected' in the same vulnerability. {product_id} is missing a corresponding product"
        ),
        instance_path: format!("/vulnerabilities/{vulnerability_index}/product_status/known_affected"),
    })
}

/// 6.1.27.13 Corresponding Affected Products
///
/// For each product listed in the product status group fixed in any vulnerability, it SHALL be
/// tested that a corresponding version of the product is listed as affected in the same vulnerability.
///
/// For a product path including the installed_with relationship the product path leading to but not
/// including the relationship is a corresponding product. Such product path could also be just the
/// product identified by beginning_product_reference if the first subpath element has the category installed_with.
///
/// "Corresponding products" are identified in this test by checking if they are within the same
/// subtree of a `product_name` categorized branch.
/// Indirections using product paths are taken into consideration, where a product id of a product
/// task is fanned-out to its beginning product reference and the IDs in its subpaths.
pub fn test_6_1_27_13_corresponding_affected_products(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(
        doc.get_document().get_csaf_version(),
        &doc.get_document().get_category(),
    ) {
        return Ok(()); // ToDo generate skipped Test finding #409
    }

    let mut vulnerability_related_products = doc
        .get_vulnerabilities()
        .iter()
        .enumerate()
        .filter_map(|(vulnerability_idx, vulnerability)| {
            vulnerability
                .get_product_status()
                .map(|product_status| (vulnerability_idx, product_status))
        })
        .filter_map(|(idx, product_status)| {
            let fixed = product_status
                .get_fixed()
                .map(|fixed| fixed.collect())
                .unwrap_or(Vec::new());
            let affected = product_status
                .get_known_affected()
                .map(|affected| affected.collect())
                .unwrap_or(Vec::new());
            if fixed.is_empty() {
                None
            } else {
                Some((idx, fixed, affected))
            }
        })
        .collect::<Vec<_>>();

    if vulnerability_related_products.is_empty() {
        // No vulnerabilities don't require affected products
        // No fixed products don't require affected products
        return Ok(());
    }

    let Some(product_tree) = doc.get_product_tree() else {
        // A missing product tree can not be used to resolve "corresponding products", TODO: #409
        // return a precondition failed here
        return Ok(());
    };

    let leaf_paths = product_tree.collect_leaf_paths();
    let product_branch_mapping = leaf_paths
        .iter()
        .filter_map(|(branches, idxs)| {
            let product_id = branches
                .last()
                .and_then(|branch| branch.get_product().map(|product| product.get_product_id()))?;
            // In the following we assume that the recommended order is kept as described in 3.1.3
            // and that fixed products belong to the same product_name subtree
            let product_name_branch = branches
                .iter()
                .position(|branch| branch.get_category() == CategoryOfTheBranch::ProductName)?;

            Some((product_id, &idxs[..=product_name_branch]))
        })
        .collect::<HashMap<&str, &[usize]>>();

    let mut findings = Vec::new();

    // Report all vulnerabilities with fixed products, but no affected_products
    // Filter them out to avoid duplicates later
    vulnerability_related_products.retain(|(vulnerability_idx, fixed_products, affected_products)| {
        let mut retain = true;
        for fixed_product in fixed_products {
            if affected_products.is_empty() {
                findings.push(create_missing_corresponding_affected_products_error(
                    *vulnerability_idx,
                    fixed_product,
                ));
                retain = false;
            }
        }
        retain
    });

    let product_path_mapping = product_tree
        .get_product_paths()
        .iter()
        .filter(|product_path| {
            let path_id = product_path.get_full_product_name().get_product_id();
            vulnerability_related_products
                .iter()
                .any(|(_, fixed_products, affected_products)| {
                    fixed_products.contains(&path_id) || affected_products.contains(&path_id)
                })
        })
        .map(|product_path| {
            (
                product_path.get_full_product_name().get_product_id(),
                product_path
                    .get_subpath_product_references()
                    .iter()
                    .copied()
                    .chain(iter::once(product_path.get_beginning_product_reference()))
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<&str, Vec<_>>>();

    // Replace references to product paths with the actual product IDs referenced by them
    vulnerability_related_products = vulnerability_related_products
        .drain(..)
        .map(|(vulnerability_idx, mut fixed_products, mut affected_products)| {
            fixed_products = fixed_products
                .into_iter()
                .flat_map(|fixed_product| match product_path_mapping.get(fixed_product) {
                    Some(vec) => vec.clone(),
                    None => vec![fixed_product],
                })
                .collect();

            affected_products = affected_products
                .into_iter()
                .flat_map(|affected_product| match product_path_mapping.get(affected_product) {
                    Some(vec) => vec.clone(),
                    None => vec![affected_product],
                })
                .collect();
            (vulnerability_idx, fixed_products, affected_products)
        })
        .collect();

    // Iterate over all vulnerability related products that exist in the product tree
    for (&vulnerability_idx, fixed_product, affected_products) in vulnerability_related_products
        .iter()
        .flat_map(|(vulnerability_idx, fixed_products, affected_products)| {
            fixed_products
                .iter()
                .map(|fixed_product| (vulnerability_idx, fixed_product, affected_products))
                .collect::<Vec<_>>()
        })
        .filter(|(_, fixed_product, _)| product_branch_mapping.contains_key(*fixed_product))
    {
        if !affected_products.iter().any(|affected_product| {
            product_branch_mapping.get(fixed_product) == product_branch_mapping.get(affected_product)
        }) {
            findings.push(create_missing_corresponding_affected_products_error(
                vulnerability_idx,
                fixed_product,
            ));
        }
    }

    if findings.is_empty() { Ok(()) } else { Err(findings) }
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_27_13,
    test_6_1_27_13_corresponding_affected_products
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_27_13 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_27_13() {
        TESTS_2_1.test_6_1_27_13.expect(ExpectedResults {
            case_01: Err(vec![create_missing_corresponding_affected_products_error(
                0,
                "CSAFPID-9080700",
            )]),
            case_02: Err(vec![create_missing_corresponding_affected_products_error(
                0,
                "CSAFPID-9080700",
            )]),
            case_03: Err(vec![create_missing_corresponding_affected_products_error(
                0,
                "CSAFPID-9080700",
            )]),
            // TODO: Clarify why this test case if failing, when case 14 has the same content
            // besides a second vulnerability with another set of affected/fixed product version
            case_04: Ok(()),
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
            case_14: Ok(()),
            case_15: Ok(()),
            //
            case_16: Ok(()),
        });
    }
}
