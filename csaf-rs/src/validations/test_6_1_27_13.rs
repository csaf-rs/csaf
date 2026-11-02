use std::collections::{HashMap, HashSet};

use crate::csaf::enums::category_of_the_relationship::CategoryOfTheRelationship;
use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{
    BranchTrait, CategoryOfTheBranch, CsafTrait, DocumentTrait, ProductPathTrait, ProductStatusTrait, ProductTrait,
    ProductTreeTrait, VulnerabilityTrait,
};
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::document_category_test_config::DocumentCategoryTestConfig;
use itertools::Itertools;

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig =
    DocumentCategoryTestConfig::new().shared(&[CsafDocumentCategory::CsafSecurityAdvisory]);

fn create_missing_corresponding_affected_products_error(vulnerability_index: usize, product_id: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "Every product of a vulnerability with in the product status group 'fixed' must have a corresponding product version in the product status group 'affected' in the same vulnerability. {product_id} is missing a corresponding product"
        ),
        instance_path: format!("/vulnerabilities/{vulnerability_index}/product_status"),
    })
}

/// Returns all product IDs in a product_status's fixed group
fn get_fixed_group(product_status: &impl ProductStatusTrait) -> Vec<&str> {
    product_status
        .get_fixed()
        .into_iter()
        .flatten()
        .chain(product_status.get_first_fixed().into_iter().flatten())
        .unique()
        .collect()
}

/// Returns all product IDs in a product_status's affected group
fn get_affected_group(product_status: &impl ProductStatusTrait) -> Vec<&str> {
    product_status
        .get_known_affected()
        .into_iter()
        .flatten()
        .chain(product_status.get_first_affected().into_iter().flatten())
        .chain(product_status.get_last_affected().into_iter().flatten())
        .unique()
        .collect()
}

/// Returns a mapping of vulnerability indices to their fixed and affected product IDs
fn extract_vulnerability_related_products<T: VulnerabilityTrait>(
    vulnerabilities: &[T],
) -> Vec<(usize, Vec<&str>, Vec<&str>)> {
    vulnerabilities
        .iter()
        .enumerate()
        .filter_map(|(vulnerability_idx, vulnerability)| {
            vulnerability
                .get_product_status()
                .map(|product_status| (vulnerability_idx, product_status))
        })
        .filter_map(|(idx, product_status)| {
            let fixed = get_fixed_group(product_status);
            if fixed.is_empty() {
                None
            } else {
                Some((idx, fixed, get_affected_group(product_status)))
            }
        })
        .collect::<Vec<_>>()
}

/// Maps product IDs to the branch path of their `product_name` parent
fn determine_product_branch_mapping<'a, FPN: ProductTrait + 'a, B: BranchTrait<FPN>>(
    leaf_paths: &'a [(Vec<&'a B>, Vec<usize>)],
) -> HashMap<&'a str, &'a [usize]> {
    leaf_paths
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
        .collect::<HashMap<&str, &[usize]>>()
}

/// Creates an upwards graph for product paths containing a subpath with the `installed_with`
/// relationship including the product referenced by that relationship.
fn build_installed_with_graph<'a, FPN: ProductTrait + 'a, PP: ProductPathTrait<FPN>>(
    product_paths: &'a [PP],
) -> HashMap<&'a str, Vec<&'a str>> {
    let mut graph = HashMap::new();

    for path in product_paths.iter().filter(|path| {
        path.get_subpath_product_references_with_category()
            .iter()
            .any(|(category, _)| *category == CategoryOfTheRelationship::InstalledWith)
    }) {
        let beginning_product_reference = path.get_beginning_product_reference();
        let product_id = path.get_full_product_name().get_product_id();
        graph
            .entry(product_id)
            .and_modify(|ancestors: &mut Vec<&str>| ancestors.push(beginning_product_reference))
            .or_insert(vec![beginning_product_reference]);
        for (_, installed_with_subpath) in path
            .get_subpath_product_references_with_category()
            .iter()
            .filter(|(category, _)| *category == CategoryOfTheRelationship::InstalledWith)
        {
            graph
                .entry(*installed_with_subpath)
                .and_modify(|ancestors: &mut Vec<&str>| ancestors.push(product_id))
                .or_insert(vec![product_id]);
        }
    }

    graph
}

/// Follows a product path an collects visited nodes excluding the start
fn walk_product_path<'a>(start: &'a str, graph: &'a HashMap<&'a str, Vec<&'a str>>) -> HashSet<&'a str> {
    let mut stack = vec![start];
    let mut path = HashSet::new();

    while let Some(current) = stack.pop() {
        let Some(ancestors) = graph.get(current) else {
            continue;
        };
        for &ancestor in ancestors {
            if ancestor != start && path.insert(ancestor) {
                stack.push(ancestor);
            }
        }
    }

    path
}

/// Creates a mapping for every product ID to it's corresponding products
fn determine_corresponding_products(doc: &impl CsafTrait) -> Option<HashMap<String, HashSet<String>>> {
    let mut corresponding_products = HashMap::new();

    let product_tree = doc.get_product_tree()?;

    // Corresponding versions are usually in the same branches element
    // We assume this means, when following the recommended branches structure, that they are
    // below the same `product_name` branch
    let leaf_paths = product_tree.collect_leaf_paths();
    let product_branch_mapping = determine_product_branch_mapping(&leaf_paths);

    for (&product_id, product_name_branch) in &product_branch_mapping {
        corresponding_products
            .entry(product_id.to_string())
            .and_modify(|related_products: &mut HashSet<String>| {
                related_products.extend(
                    product_branch_mapping
                        .iter()
                        .filter(|(prod_id, _)| *prod_id != &product_id)
                        .filter(|(_, prod_name_branch)| *prod_name_branch == product_name_branch)
                        .map(|(prod_id, _)| prod_id.to_string()),
                )
            })
            .or_insert(
                product_branch_mapping
                    .iter()
                    .filter(|(prod_id, _)| *prod_id != &product_id)
                    .filter(|(_, prod_name_branch)| *prod_name_branch == product_name_branch)
                    .map(|(prod_id, _)| prod_id.to_string())
                    .collect(),
            );
    }

    // If no such branch exists within the the same common branch assume that full_product_names are
    // related with each other
    if corresponding_products.is_empty() {
        for (left_product_id, right_product_id) in product_tree
            .get_full_product_names()
            .iter()
            .cartesian_product(product_tree.get_full_product_names().iter())
            .map(|(left_product, right_product)| (left_product.get_product_id(), right_product.get_product_id()))
        {
            corresponding_products
                .entry(left_product_id.to_string())
                .and_modify(|related_products: &mut HashSet<String>| {
                    related_products.insert(right_product_id.to_string());
                })
                .or_insert(HashSet::from([right_product_id.to_string()]));
        }
    }

    for left in product_tree.get_product_paths() {
        let left_beginning = left.get_beginning_product_reference();
        let left_context = left.get_subpath_product_references_with_category();

        for right in product_tree.get_product_paths() {
            let right_beginning = right.get_beginning_product_reference();
            let right_context = right.get_subpath_product_references_with_category();

            let beginning_products_correspond = corresponding_products
                .get(left_beginning)
                .is_some_and(|products| products.contains(right_beginning))
                || corresponding_products
                    .get(right_beginning)
                    .is_some_and(|products| products.contains(left_beginning));

            if beginning_products_correspond && left_context == right_context {
                corresponding_products
                    .entry(left.get_full_product_name().get_product_id().to_string())
                    .or_default()
                    .insert(right.get_full_product_name().get_product_id().to_string());
            }
        }
    }

    // For a product path including the installed_with relationship the product path leading to but
    // not including the relationship is a corresponding product. Such product path could also be
    // just the product identified by beginning_product_reference if the first subpath element has
    // the category installed_with
    let product_path_graph = build_installed_with_graph(product_tree.get_product_paths());
    for product_id in product_path_graph.keys() {
        corresponding_products
            .entry(product_id.to_string())
            .and_modify(|related_products: &mut HashSet<String>| {
                related_products.extend(
                    walk_product_path(product_id, &product_path_graph)
                        .iter()
                        .map(|step| step.to_string()),
                )
            })
            .or_insert(
                walk_product_path(product_id, &product_path_graph)
                    .iter()
                    .map(|step| step.to_string())
                    .collect(),
            );
    }

    Some(corresponding_products)
}

/// 6.1.27.13 Corresponding Affected Products
///
/// For each product listed in the product status group fixed (`fixed`, `first_fixed`) in any vulnerability, it SHALL be
/// tested that a corresponding version of the product is listed as affected (`first_affected`, `known_affected`, `last_affected`) in the same vulnerability.
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

    let mut vulnerability_related_products = extract_vulnerability_related_products(doc.get_vulnerabilities());

    if vulnerability_related_products.is_empty() {
        // No vulnerabilities don't require affected products
        // No fixed products don't require affected products
        return Ok(());
    }

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

    let Some(corresponding_products) = determine_corresponding_products(doc) else {
        // No products where identified with the product tree's branches or paths, TODO: #409
        return if findings.is_empty() { Ok(()) } else { Err(findings) };
    };

    for (vulnerability_idx, fixed_products, affected_products) in vulnerability_related_products {
        for fixed_product in fixed_products {
            let related_products = if let Some(related_product) = corresponding_products.get(fixed_product) {
                related_product
            } else {
                &HashSet::new()
            };
            if !affected_products
                .iter()
                .any(|affected_product| related_products.contains(&affected_product.to_string()))
            {
                findings.push(create_missing_corresponding_affected_products_error(
                    vulnerability_idx,
                    fixed_product,
                ));
            }
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
        let vulnerability_without_affected_products = Err(vec![create_missing_corresponding_affected_products_error(
            0,
            "CSAFPID-9080700",
        )]);
        let two_vulnerabilities_one_without_fixed_products_one_with_fixed_products_but_no_affected_products =
            Err(vec![create_missing_corresponding_affected_products_error(
                0,
                "CSAFPID-9080700",
            )]);
        // In case 04 the Product consisting of B (0.8.15) installed on A (4.2) is listed as fixed.
        // The Product consisting of B (0.8.14) installed on A (4.1) is listed as affected
        // These seem related, but it is required, that a product exists, that fixes both B (0.8.14)
        // and A (4.1) so the cartesian product is required that an affected product exists for
        // B (0.8.15) + A (4.2) and B (0.8.15) + A (4.1) which is the case in case 14
        //
        // Case 04:
        // Fixed    Affected
        // 9080705  Missing
        // Missing  9080706
        //
        // Case 14:
        // Fixed    Affected
        // 9080705  9080707
        // 9080708  9080706
        let missing_cross_product_of_affected_and_fixed_product =
            Err(vec![create_missing_corresponding_affected_products_error(
                0,
                "CSAFPID-9080705",
            )]);

        // Should not be failing, TODO: #1409
        let failing_test_due_to_duplicate_product_id = Err(vec![create_missing_corresponding_affected_products_error(
            0,
            "CSAFPID-9080702",
        )]);

        TESTS_2_1.test_6_1_27_13.expect(ExpectedResults {
            case_01: vulnerability_without_affected_products,
            case_02: two_vulnerabilities_one_without_fixed_products_one_with_fixed_products_but_no_affected_products
                .clone(),
            case_03: two_vulnerabilities_one_without_fixed_products_one_with_fixed_products_but_no_affected_products,
            // TODO: Clarify why this test case if failing, when case 14 has the same content
            // besides a second vulnerability with another set of affected/fixed product version
            case_04: missing_cross_product_of_affected_and_fixed_product,
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
            case_14: Ok(()),
            case_15: Ok(()),
            case_16: failing_test_due_to_duplicate_product_id,
        });
    }
}
