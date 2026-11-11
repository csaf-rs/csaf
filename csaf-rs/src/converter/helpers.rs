use std::collections::HashSet;

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

pub(super) fn for_each_product_identification_helper_with_path(
    doc: &mut loose::Document,
    mut callback: impl FnMut(&mut Map<String, Value>, String),
) {
    let product_tree = match doc.v20_view_mut().product_tree_mut() {
        Some(product_tree) => product_tree,
        None => return,
    };
    let Ok(product_tree) = product_tree.caution_as_object_mut() else {
        return;
    };

    for (key, value) in product_tree.iter_mut() {
        visit_product_tree_value_with_path(value, format!("/product_tree/{key}"), &mut callback);
    }
}

/// Retains only the first occurrence of each value in the array stored under `field`,
/// comparing by [`Value`] equality and preserving order. Does nothing if the field is
/// missing or not an array.
/// serde_json's `Hash` impls agree with `Eq` (floats normalize -0.0; maps hash key-sorted under
/// `preserve_order`), so set-based deduplication matches equality semantics.
pub(super) fn retain_unique_field_values(helper: &mut Map<String, Value>, field: &str) {
    let Some(Value::Array(values)) = helper.get_mut(field) else {
        return;
    };

    let mut seen = HashSet::new();
    values.retain(|value| seen.insert(value.clone()));
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

fn visit_product_tree_value_with_path(
    value: &mut Value,
    path: String,
    callback: &mut impl FnMut(&mut Map<String, Value>, String),
) {
    match value {
        Value::Object(object) => {
            if let Some(Value::Object(helper)) = object.get_mut("product_identification_helper") {
                callback(helper, format!("{path}/product_identification_helper"));
            }
            if object
                .get("product_identification_helper")
                .is_some_and(|helper| helper.as_object().is_some_and(Map::is_empty))
            {
                object.remove("product_identification_helper");
            }
            for (key, child) in object.iter_mut() {
                visit_product_tree_value_with_path(
                    child,
                    format!("{path}/{}", key.replace('~', "~0").replace('/', "~1")),
                    callback,
                );
            }
        },
        Value::Array(array) => {
            for (index, child) in array.iter_mut().enumerate() {
                visit_product_tree_value_with_path(child, format!("{path}/{index}"), callback);
            }
        },
        _ => {},
    }
}
