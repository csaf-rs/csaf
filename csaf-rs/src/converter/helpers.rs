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
