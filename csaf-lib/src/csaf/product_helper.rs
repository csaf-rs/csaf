use std::collections::HashSet;
use crate::csaf::schema::{Branch, CommonSecurityAdvisoryFramework, ProductIdT};

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

        // /vulnerabilities[]/scores[]/products[]
        for threat in vuln.scores.iter() {
            ids.extend(threat.products.iter());
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
        ids.extend(x.iter().flat_map(|x| gather_product_definitions_from_branch(x)))
    }

    ids
}

pub fn gather_product_definitions(doc: &CommonSecurityAdvisoryFramework) -> Vec<&ProductIdT> {
    let mut ids = Vec::<&ProductIdT>::new();

    if let Some(x) = doc.product_tree.as_ref() {
        // /product_tree/branches[](/branches[])*/product/product_id
        if let Some(branch) = x.branches.as_ref() {
            ids.extend(branch.iter().flat_map(|x| gather_product_definitions_from_branch(x)));
        }

        // /product_tree/full_product_names[]/product_id
        ids.extend(x.full_product_names.iter().map(|x|&x.product_id));

        // /product_tree/relationships[]/relates_to_product_reference
        ids.extend(x.relationships.iter().map(|x| &x.product_reference));
    }

    ids
}