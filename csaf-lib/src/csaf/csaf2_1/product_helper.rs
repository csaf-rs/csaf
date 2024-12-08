use super::schema::{Branch, BranchesT, CommonSecurityAdvisoryFramework, ProductIdT, ProductTree};
use std::collections::HashSet;

pub fn gather_product_references(doc: &CommonSecurityAdvisoryFramework) -> HashSet<&ProductIdT> {
    let mut ids = HashSet::<&ProductIdT>::new();

    if let Some(x) = doc.product_tree.as_ref() {
        //  /product_tree/product_groups[]/product_ids[]
        ids.extend(x.product_groups.iter().flat_map(|x| &x.product_ids));

        // /product_tree/relationships[]/product_reference
        ids.extend(x.relationships.iter().map(|x| &x.product_reference));

        //   /product_tree/relationships[]/relates_to_product_reference
        ids.extend(x.relationships.iter().map(|x| &x.product_reference));
    }

    for vuln in doc.vulnerabilities.iter() {
        // /vulnerabilities[]/product_status/first_affected[]
        // /vulnerabilities[]/product_status/first_fixed[]
        // /vulnerabilities[]/product_status/fixed[]
        // /vulnerabilities[]/product_status/known_affected[]
        // /vulnerabilities[]/product_status/known_not_affected[]
        // /vulnerabilities[]/product_status/last_affected[]
        // /vulnerabilities[]/product_status/recommended[]
        // /vulnerabilities[]/product_status/under_investigation[]
        if let Some(status) = vuln.product_status.as_ref() {
            if let Some(x) = status.first_affected.as_ref() {
                ids.extend(x.iter());
            }
            if let Some(x) = status.first_fixed.as_ref() {
                ids.extend(x.iter());
            }
            if let Some(x) = status.fixed.as_ref() {
                ids.extend(x.iter());
            }
            if let Some(x) = status.known_affected.as_ref() {
                ids.extend(x.iter());
            }
            if let Some(x) = status.last_affected.as_ref() {
                ids.extend(x.iter());
            }
            if let Some(x) = status.recommended.as_ref() {
                ids.extend(x.iter());
            }
            if let Some(x) = status.under_investigation.as_ref() {
                ids.extend(x.iter());
            }
        }

        // /vulnerabilities[]/remediations[]/product_ids[]
        for rem in vuln.remediations.iter() {
            if let Some(x) = rem.product_ids.as_ref() {
                ids.extend(x.iter());
            }
        }

        // /vulnerabilities[]/metrics[]/products[]
        if let Some(metrics) = vuln.metrics.as_ref() {
            for metric in metrics {
                ids.extend(metric.products.iter())
            }
        }

        // /vulnerabilities[]/threats[]/product_ids[]
        for threat in vuln.threats.iter() {
            if let Some(x) = threat.product_ids.as_ref() {
                ids.extend(x.iter());
            }
        }
    }

    ids
}

pub fn gather_product_definitions_from_branch(branch: &Branch) -> Vec<&ProductIdT> {
    let mut ids = Vec::<&ProductIdT>::new();

    // Gather from /product/product_id
    if let Some(product) = branch.product.as_ref() {
        ids.push(&product.product_id);
    }

    // Go into the branch
    if let Some(x) = branch.branches.as_ref() {
        ids.extend(
            x.iter()
                .flat_map(|x| gather_product_definitions_from_branch(x)),
        )
    }

    ids
}

pub fn count_branch_depth(branch: &Branch) -> u32 {
    let mut i = 0;

    // Go into the branch, calculate the depth and take the maximum value
    if let Some(x) = branch.branches.as_ref() {
        let values = x.iter().map(|x| count_branch_depth(x)).collect::<Vec<_>>();

        // Add the maximum value to the branch depth
        i += values.iter().cloned().fold(0, u32::max)
    }

    i
}

pub fn count_branch_depth_tree(tree: &ProductTree) -> u32 {
    let mut i = 0;

    // Go into the branch, calculate the depth and take the maximum value
    if let Some(x) = tree.branches.as_ref() {
        let values = x.iter().map(|x| count_branch_depth(x)).collect::<Vec<_>>();

        // Add the maximum value to the branch depth
        i += values.iter().cloned().fold(0, u32::max)
    }

    i
}

pub fn gather_product_definitions(doc: &CommonSecurityAdvisoryFramework) -> Vec<&ProductIdT> {
    let mut ids = Vec::<&ProductIdT>::new();

    if let Some(x) = doc.product_tree.as_ref() {
        // /product_tree/branches[](/branches[])*/product/product_id
        if let Some(branch) = x.branches.as_ref() {
            ids.extend(
                branch
                    .iter()
                    .flat_map(|x| gather_product_definitions_from_branch(x)),
            );
        }

        // /product_tree/full_product_names[]/product_id
        ids.extend(x.full_product_names.iter().map(|x| &x.product_id));

        // /product_tree/relationships[]/full_product_name/product_id
        ids.extend(
            x.relationships
                .iter()
                .map(|x| &x.full_product_name.product_id),
        );
    }

    ids
}
