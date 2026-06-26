use crate::extractor::traits::Extractor;

/// Traverses a JSON object and applies the provided extractors to it.
pub fn visit_json_value(object: &serde_json::Map<String, serde_json::Value>, visitors: &mut [&mut dyn Extractor]) {
    for (key, value) in object {
        match value {
            serde_json::Value::Null
            | serde_json::Value::Bool(_)
            | serde_json::Value::Number(_)
            | serde_json::Value::String(_) => {
                for v in &mut *visitors {
                    v.keyed_primitive(key.as_str(), value)
                }
            },
            serde_json::Value::Array(values) => {
                let mut interesting = false;
                for v in &mut *visitors {
                    interesting |= v.enter_keyed_array(key.as_str());
                }
                if interesting {
                    visit_json_array(values, visitors);
                }
                for v in &mut *visitors {
                    v.leave_keyed_array(key.as_str());
                }
            },
            serde_json::Value::Object(map) => {
                let mut interesting = false;
                for v in &mut *visitors {
                    interesting |= v.enter_keyed_object(key.as_str());
                }
                if interesting {
                    visit_json_value(map, visitors);
                }
                for v in &mut *visitors {
                    v.leave_keyed_object(key.as_str());
                }
            },
        }
    }
}

/// Traverses a JSON array and applies the provided extractors to it.
pub fn visit_json_array(array: &[serde_json::Value], visitors: &mut [&mut dyn Extractor]) {
    for (number, value) in array.iter().enumerate() {
        match value {
            serde_json::Value::Null
            | serde_json::Value::Bool(_)
            | serde_json::Value::Number(_)
            | serde_json::Value::String(_) => {
                for v in &mut *visitors {
                    v.indexed_primitive(number, value)
                }
            },
            serde_json::Value::Array(values) => {
                let mut interesting = false;
                for v in &mut *visitors {
                    interesting |= v.enter_indexed_array(number);
                }
                if interesting {
                    visit_json_array(values, visitors);
                }
                for v in &mut *visitors {
                    v.leave_indexed_array(number);
                }
            },
            serde_json::Value::Object(map) => {
                let mut interesting = false;
                for v in &mut *visitors {
                    interesting |= v.enter_indexed_object(number);
                }
                if interesting {
                    visit_json_value(map, visitors);
                }
                for v in &mut *visitors {
                    v.leave_indexed_object(number);
                }
            },
        }
    }
}
