use crate::extractor::traits::Extractor;

pub fn visit_json_value(value: &serde_json::Value, visitors: &mut [&mut dyn Extractor]) {
    match value {
        serde_json::Value::Null
        | serde_json::Value::Bool(_)
        | serde_json::Value::Number(_)
        | serde_json::Value::String(_) => visit_json_primitive(value, visitors),
        serde_json::Value::Array(values) => visit_json_array(values, visitors),
        serde_json::Value::Object(map) => visit_json_object(map, visitors),
    }
}

pub fn visit_json_primitive(value: &serde_json::Value, visitors: &mut [&mut dyn Extractor]) {
    for v in visitors.iter_mut() {
        v.init_primitive(&[], value);
    }
}

/// Traverses a JSON object and applies the provided extractors to it.
pub fn visit_json_object(object: &serde_json::Map<String, serde_json::Value>, visitors: &mut [&mut dyn Extractor]) {
    for v in visitors.iter_mut() {
        v.init_object(&[]);
    }
    visit_json_object_at_path(&mut vec![], object, visitors);
}

/// Traverses a JSON array and applies the provided extractors to it.
pub fn visit_json_array(array: &[serde_json::Value], visitors: &mut [&mut dyn Extractor]) {
    for v in visitors.iter_mut() {
        v.init_array(&[]);
    }
    visit_json_array_at_path(&mut vec![], array, visitors);
}

pub fn visit_json_object_at_path(
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
                for v in visitors.iter_mut() {
                    v.keyed_primitive(path, key.as_str(), value)
                }
            },
            serde_json::Value::Array(values) => {
                let mut interesting = false;
                for v in visitors.iter_mut() {
                    interesting |= v.enter_keyed_array(path, key.as_str());
                }
                if interesting {
                    path.push(key.clone());
                    visit_json_array_at_path(path, values, visitors);
                    path.pop();
                }
                for v in visitors.iter_mut() {
                    v.leave_keyed_array(path, key.as_str());
                }
            },
            serde_json::Value::Object(map) => {
                let mut interesting = false;
                for v in visitors.iter_mut() {
                    interesting |= v.enter_keyed_object(path, key.as_str());
                }
                if interesting {
                    path.push(key.clone());
                    visit_json_object_at_path(path, map, visitors);
                    path.pop();
                }
                for v in visitors.iter_mut() {
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
                for v in visitors.iter_mut() {
                    v.indexed_primitive(path, number, value)
                }
            },
            serde_json::Value::Array(values) => {
                let mut interesting = false;
                for v in visitors.iter_mut() {
                    interesting |= v.enter_indexed_array(path, number);
                }
                if interesting {
                    path.push(number.to_string());
                    visit_json_array_at_path(path, values, visitors);
                    path.pop();
                }
                for v in visitors.iter_mut() {
                    v.leave_indexed_array(path, number);
                }
            },
            serde_json::Value::Object(map) => {
                let mut interesting = false;
                for v in visitors.iter_mut() {
                    interesting |= v.enter_indexed_object(path, number);
                }
                if interesting {
                    path.push(number.to_string());
                    visit_json_object_at_path(path, map, visitors);
                    path.pop();
                }
                for v in visitors.iter_mut() {
                    v.leave_indexed_object(path, number);
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::extractor::{
        extract::{ExtractJsonValue, ExtractPrimitive},
        navigate::AtPath,
        traits::CanExtract,
    };

    use super::*;

    #[test]
    fn json_object_at_top_level() {
        let interesting_object = json!({
            "p": null,
            "o": {},
            "a": [null, {}, []]}
        );

        let mut collector = ExtractJsonValue::new();

        visit_json_value(&json!(interesting_object), &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(result, Some(("/".into(), interesting_object)));
    }

    #[test]
    fn json_array_at_top_level() {
        let interesting_object = json!([{
            "p": null,
            "o": {},
            "a": [null, {}, []]}
        ]);

        let mut collector = ExtractJsonValue::new();

        visit_json_value(&json!(interesting_object), &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(result, Some(("/".into(), interesting_object)));
    }

    #[test]
    fn json_primitive_at_top_level() {
        let interesting_object = json!("hello");

        let mut collector = ExtractJsonValue::new();

        visit_json_value(&json!(interesting_object), &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(result, Some(("/".into(), interesting_object)));
    }

    #[test]
    fn two_primitives() {
        let mut x = AtPath::new("x", ExtractPrimitive::new_string());
        let mut y = AtPath::new("y", ExtractPrimitive::new_bool());

        visit_json_value(&json!({"x": "a", "y": true}), &mut [&mut x, &mut y]);

        let result = (x.extract(), y.extract());
        assert_eq!(result, (Some(("/x".into(), "a".into())), Some(("/y".into(), true))));
    }
}
