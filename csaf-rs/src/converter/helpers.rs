use serde_json::{Map, Value};

use crate::schema::loose;

/// Runs `callback` for every product identification helper in the product tree.
/// Does deviate from v20 / v21 view semantics.
/// TODO: For tasks that do not require a switch from v20 / v21 semantics, we should
/// consider a more type-safe approach.
pub(super) fn for_each_product_identification_helper_via_as_object_mut(
    doc: &mut loose::Document,
    mut callback: impl FnMut(&mut Map<String, Value>),
) {
    let product_tree = match doc.v20_view_mut().product_tree_mut() {
        Some(product_tree) => product_tree,
        None => return,
    };

    let Ok(product_tree) = product_tree.caution_as_object_mut() else {
        return;
    };

    for value in product_tree.values_mut() {
        visit_product_tree_value(value, &mut callback);
    }
}

fn visit_product_tree_value(value: &mut Value, callback: &mut impl FnMut(&mut Map<String, Value>)) {
    match value {
        Value::Object(object) => {
            if let Some(Value::Object(helper)) = object.get_mut("product_identification_helper") {
                callback(helper);
            }

            for child in object.values_mut() {
                visit_product_tree_value(child, callback);
            }
        },
        Value::Array(array) => {
            for child in array {
                visit_product_tree_value(child, callback);
            }
        },
        _ => {},
    }
}

#[derive(Debug)]
pub(super) enum BranchTraversalWarning {
    InvalidProductTree,
    InvalidBranches,
}

/// Runs `callback` for every branch below `/product_tree/branches`, depth first,
/// passing the branch object and its JSON pointer.
///
/// Returns `Ok(())` both when branches were traversed and when there is legitimately
/// nothing to do: `product_tree` and `branches` are optional in CSAF 2.0, so their
/// absence is a valid document.
///
/// `Err` is returned only when one of those nodes exists but deviates from the CSAF 2.0 schema.
pub(super) fn for_each_branch_mut(
    doc: &mut loose::Document,
    mut callback: impl FnMut(&mut Map<String, Value>, &str),
) -> Result<(), BranchTraversalWarning> {
    let Some(product_tree) = doc.v20_view_mut().product_tree_mut() else {
        return Ok(());
    };

    let product_tree = product_tree
        .caution_as_object_mut()
        .map_err(|_| BranchTraversalWarning::InvalidProductTree)?;

    let Some(branches) = product_tree.get_mut("branches") else {
        return Ok(());
    };

    let Value::Array(branches) = branches else {
        return Err(BranchTraversalWarning::InvalidBranches);
    };

    visit_branches_mut(branches, "/product_tree/branches", &mut callback);

    Ok(())
}

fn visit_branches_mut(branches: &mut [Value], path: &str, callback: &mut impl FnMut(&mut Map<String, Value>, &str)) {
    for (index, branch) in branches.iter_mut().enumerate() {
        let Value::Object(branch) = branch else {
            continue;
        };

        let branch_path = format!("{path}/{index}");

        callback(branch, &branch_path);

        if let Some(Value::Array(children)) = branch.get_mut("branches") {
            visit_branches_mut(children, &format!("{branch_path}/branches"), callback);
        }
    }
}
