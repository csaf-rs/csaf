use crate::extractor::traits::Extractor;

/// Traverses a JSON object and applies the provided extractors to it.
pub fn visit_json_value(object: &serde_json::Map<String, serde_json::Value>, visitors: &mut [&mut dyn Extractor]) {
    visit_json_value_at_path(&mut vec![], object, visitors);
}

/// Traverses a JSON array and applies the provided extractors to it.
pub fn visit_json_array(array: &[serde_json::Value], visitors: &mut [&mut dyn Extractor]) {
    visit_json_array_at_path(&mut vec![], array, visitors);
}

pub fn visit_json_value_at_path(
    path: &mut Vec<String>,
    object: &serde_json::Map<String, serde_json::Value>,
    visitors: &mut [&mut dyn Extractor],
) {
    for (key, value) in object {
        match value {
            serde_json::Value::Null
            | serde_json::Value::Bool(_)
            | serde_json::Value::Number(_)
            | serde_json::Value::String(_) => {
                for v in &mut *visitors {
                    v.keyed_primitive(path, key.as_str(), value)
                }
            },
            serde_json::Value::Array(values) => {
                let mut interesting = false;
                for v in &mut *visitors {
                    interesting |= v.enter_keyed_array(path, key.as_str());
                }
                if interesting {
                    path.push(key.clone());
                    visit_json_array_at_path(path, values, visitors);
                    path.pop();
                }
                for v in &mut *visitors {
                    v.leave_keyed_array(path, key.as_str());
                }
            },
            serde_json::Value::Object(map) => {
                let mut interesting = false;
                for v in &mut *visitors {
                    interesting |= v.enter_keyed_object(path, key.as_str());
                }
                if interesting {
                    path.push(key.clone());
                    visit_json_value_at_path(path, map, visitors);
                    path.pop();
                }
                for v in &mut *visitors {
                    v.leave_keyed_object(path, key.as_str());
                }
            },
        }
    }
}

/// Traverses a JSON array and applies the provided extractors to it.
pub fn visit_json_array_at_path(
    path: &mut Vec<String>,
    array: &[serde_json::Value],
    visitors: &mut [&mut dyn Extractor],
) {
    for (number, value) in array.iter().enumerate() {
        match value {
            serde_json::Value::Null
            | serde_json::Value::Bool(_)
            | serde_json::Value::Number(_)
            | serde_json::Value::String(_) => {
                for v in &mut *visitors {
                    v.indexed_primitive(path, number, value)
                }
            },
            serde_json::Value::Array(values) => {
                let mut interesting = false;
                for v in &mut *visitors {
                    interesting |= v.enter_indexed_array(path, number);
                }
                if interesting {
                    path.push(number.to_string());
                    visit_json_array_at_path(path, values, visitors);
                    path.pop();
                }
                for v in &mut *visitors {
                    v.leave_indexed_array(path, number);
                }
            },
            serde_json::Value::Object(map) => {
                let mut interesting = false;
                for v in &mut *visitors {
                    interesting |= v.enter_indexed_object(path, number);
                }
                if interesting {
                    path.push(number.to_string());
                    visit_json_value_at_path(path, map, visitors);
                    path.pop();
                }
                for v in &mut *visitors {
                    v.leave_indexed_object(path, number);
                }
            },
        }
    }
}
