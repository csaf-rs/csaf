use crate::csaf::getter_traits::{BranchTrait, CsafTrait, FullProductNameTrait, MetricTrait, ProductGroupTrait, ProductStatusTrait, ProductTreeTrait, RelationshipTrait, RemediationTrait, ThreatTrait, VulnerabilityTrait};
use std::collections::HashSet;

pub fn gather_product_references(doc: &impl CsafTrait) -> HashSet<String> {
    let mut ids = HashSet::<String>::new();

    if let Some(x) = doc.get_product_tree().as_ref() {
        // /product_tree/product_groups[]/product_ids[]
        ids.extend(x.get_product_groups().iter().flat_map(|x| x.get_product_ids()).map(|x| x.to_owned()));

        // /product_tree/relationships[]/product_reference
        ids.extend(x.get_relationships().iter().map(|x| x.get_product_reference().to_owned()));

        // /product_tree/relationships[]/relates_to_product_reference
        ids.extend(x.get_relationships().iter().map(|x| x.get_relates_to_product_reference().to_owned()));
    }

    for vuln in doc.get_vulnerabilities().iter() {
        // /vulnerabilities[]/product_status/first_affected[]
        // /vulnerabilities[]/product_status/first_fixed[]
        // /vulnerabilities[]/product_status/fixed[]
        // /vulnerabilities[]/product_status/known_affected[]
        // /vulnerabilities[]/product_status/known_not_affected[]
        // /vulnerabilities[]/product_status/last_affected[]
        // /vulnerabilities[]/product_status/recommended[]
        // /vulnerabilities[]/product_status/under_investigation[]
        if let Some(status) = vuln.get_product_status().as_ref() {
            if let Some(x) = status.get_first_affected().as_ref() {
                ids.extend(x.iter().map(|x| (*x).clone()));
            }
            if let Some(x) = status.get_first_fixed().as_ref() {
                ids.extend(x.iter().map(|x| (*x).clone()));
            }
            if let Some(x) = status.get_fixed().as_ref() {
                ids.extend(x.iter().map(|x| (*x).clone()));
            }
            if let Some(x) = status.get_known_affected().as_ref() {
                ids.extend(x.iter().map(|x| (*x).clone()));
            }
            if let Some(x) = status.get_last_affected().as_ref() {
                ids.extend(x.iter().map(|x| (*x).clone()));
            }
            if let Some(x) = status.get_recommended().as_ref() {
                ids.extend(x.iter().map(|x| (*x).clone()));
            }
            if let Some(x) = status.get_under_investigation().as_ref() {
                ids.extend(x.iter().map(|x| (*x).clone()));
            }
        }

        // /vulnerabilities[]/remediations[]/product_ids[]
        for rem in vuln.get_remediations().iter() {
            if let Some(x) = rem.get_product_ids().as_ref() {
                ids.extend(x.iter().map(|x| (*x).clone()));
            }
        }

        // /vulnerabilities[]/metrics[]/products[]
        if let Some(metrics) = vuln.get_metrics().as_ref() {
            metrics.iter().for_each(|metric| {
                ids.extend(metric.get_products().iter().map(|x| (*x).clone()))
            });
        }

        // /vulnerabilities[]/threats[]/product_ids[]
        for threat in vuln.get_threats().iter() {
            if let Some(x) = threat.get_product_ids().as_ref() {
                ids.extend(x.iter().map(|x| (*x).clone()));
            }
        }
    }

    ids
}

pub fn gather_product_definitions_from_branch(branch: &impl BranchTrait) -> Vec<String> {
    let mut ids = Vec::<String>::new();

    // Gather from /product/product_id
    if let Some(product) = branch.get_product() {
        ids.push(product.get_product_id().to_owned());
    }

    // Go into the branch
    if let Some(x) = branch.get_branches().as_ref() {
        ids.extend(
            x.iter()
                .flat_map(|x| gather_product_definitions_from_branch(x)),
        )
    }

    ids
}

pub fn check_branch_depth(branch: &impl BranchTrait, max_depth: u32, depth: u32) -> bool {
    // Recurse into sub-branches.
    if let Some(x) = branch.get_branches().as_ref() {
        if depth == max_depth {
            // Since we are inspecting the children, they will have a depth of max_depth + 1.
            return false
        }
        if !x.iter().all(|x| check_branch_depth(x, max_depth, depth + 1)) {
            // Check recursively if any sub-branch exceeds the recursion limit.
            return false
        }
    }
    true
}

pub fn check_branch_depth_tree(tree: &impl ProductTreeTrait, max_depth: u32) -> bool {
    // All children of the root branch have depth 1, perform recursive depth check on them.
    if let Some(x) = tree.get_branches().as_ref() {
        x.iter().all(|x| check_branch_depth(x, max_depth, 1))
    } else {
        true
    }
}

pub fn gather_product_definitions(doc: &impl CsafTrait) -> Vec<String> {
    let mut ids = Vec::<String>::new();

    if let Some(tree) = doc.get_product_tree().as_ref() {
        // /product_tree/branches[](/branches[])*/product/product_id
        if let Some(branch) = tree.get_branches().as_ref() {
            for sub_branch in branch.iter() {
                ids.extend(
                    gather_product_definitions_from_branch(sub_branch).iter()
                        .map(|x| x.to_owned())
                );
            }
        }

        // /product_tree/full_product_names[]/product_id
        ids.extend(tree.get_full_product_names().iter().map(|x| x.get_product_id().to_owned()));

        // /product_tree/relationships[]/full_product_name/product_id
        ids.extend(
            tree.get_relationships()
                .iter()
                .map(|x| x.get_full_product_name().get_product_id().to_owned()),
        );
    }

    ids
}
