use crate::extractor::traits::{CanExtract, Extractor};

/// An extractor that extracts a primitive value at a specified object key, and returns it as an `Option`.
pub struct ExtractPrimitive<Type> {
    result: Option<(String, Type)>,
    mapping: fn(json_pointer: &str, &serde_json::Value) -> Option<(String, Type)>,
}

impl ExtractPrimitive<String> {
    /// Creates a new `ExtractPrimitive` extractor that will extract a string value.
    pub fn new_string() -> ExtractPrimitive<String> {
        ExtractPrimitive {
            result: None,
            mapping: |p, v| v.as_str().map(|s| (p.to_string(), s.to_string())),
        }
    }
}

impl ExtractPrimitive<serde_json::Number> {
    /// Creates a new `ExtractPrimitive` extractor that will extract a number value.
    pub fn new_number() -> ExtractPrimitive<serde_json::Number> {
        ExtractPrimitive {
            result: None,
            mapping: |p, v| v.as_number().map(|s| (p.to_string(), s.clone())),
        }
    }
}
impl ExtractPrimitive<bool> {
    /// Creates a new `ExtractPrimitive` extractor that will extract a boolean value.
    pub fn new_bool() -> ExtractPrimitive<bool> {
        ExtractPrimitive {
            result: None,
            mapping: |p, v| v.as_bool().map(|b| (p.to_string(), b)),
        }
    }
}

impl<T> CanExtract<Option<(String, T)>> for ExtractPrimitive<T> {
    fn extract(&mut self) -> Option<(String, T)> {
        self.result.take()
    }
}

impl<T> Extractor for ExtractPrimitive<T> {
    fn init_primitive(&mut self, json_pointer: &str, primitive: &serde_json::Value) {
        self.result = (self.mapping)(json_pointer, primitive)
    }

    fn keyed_primitive(&mut self, _json_pointer: &str, _name: &str, _primitive: &serde_json::Value) {}

    fn enter_keyed_object(&mut self, _json_pointer: &str, _name: &str) -> bool {
        false
    }

    fn leave_keyed_object(&mut self, _json_pointer: &str, _name: &str) {}
}

/// An extractor that extracts an object structure and returns it as a `serde_json::Value`.
pub struct ExtractJsonValue {
    stack: Vec<serde_json::Value>,
    json_pointer: Option<String>,
}

impl ExtractJsonValue {
    /// Creates a new `ExtractJsonValue` extractor that will extract an object structure and
    /// return it as a `serde_json::Value`.
    pub fn new() -> Self {
        ExtractJsonValue {
            stack: vec![],
            json_pointer: None,
        }
    }
}

impl Default for ExtractJsonValue {
    fn default() -> Self {
        Self::new()
    }
}

impl Extractor for ExtractJsonValue {
    fn init_array(&mut self, json_pointer: &str) {
        self.json_pointer = Some(json_pointer.into());
        self.stack.push(serde_json::Value::Array(Vec::new()))
    }

    fn init_object(&mut self, json_pointer: &str) {
        self.json_pointer = Some(json_pointer.into());
        self.stack.push(serde_json::Value::Object(serde_json::Map::new()))
    }

    fn init_primitive(&mut self, json_pointer: &str, primitive: &serde_json::Value) {
        self.json_pointer = Some(json_pointer.into());
        self.stack.push(primitive.clone())
    }

    fn keyed_primitive(&mut self, _json_pointer: &str, name: &str, primitive: &serde_json::Value) {
        let head = self.stack.pop();
        match head {
            Some(serde_json::Value::Object(mut values)) => {
                values.insert(name.to_string(), primitive.clone());
                self.stack.push(serde_json::Value::Object(values))
            },
            Some(value) => self.stack.push(value),
            None => (),
        };
    }

    fn enter_keyed_object(&mut self, _json_pointer: &str, _name: &str) -> bool {
        self.stack.push(serde_json::Value::Object(serde_json::Map::new()));
        true
    }

    fn leave_keyed_object(&mut self, _json_pointer: &str, name: &str) {
        let child = self.stack.pop();
        let parent = self.stack.pop();
        match (child, parent) {
            (Some(child), Some(serde_json::Value::Object(mut values))) => {
                values.insert(name.to_string(), child);
                self.stack.push(serde_json::Value::Object(values))
            },
            (_, Some(value)) => self.stack.push(value),
            (_, None) => (),
        };
    }

    fn enter_keyed_array(&mut self, _json_pointer: &str, _name: &str) -> bool {
        self.stack.push(serde_json::Value::Array(vec![]));
        true
    }

    fn leave_keyed_array(&mut self, json_pointer: &str, name: &str) {
        self.leave_keyed_object(json_pointer, name);
    }

    fn indexed_primitive(&mut self, _json_pointer: &str, _index: usize, primitive: &serde_json::Value) {
        let head = self.stack.pop();
        match head {
            Some(serde_json::Value::Array(mut values)) => {
                values.push(primitive.clone());
                self.stack.push(serde_json::Value::Array(values))
            },
            Some(value) => self.stack.push(value),
            None => (),
        };
    }

    fn enter_indexed_object(&mut self, json_pointer: &str, index: usize) -> bool {
        self.enter_keyed_object(json_pointer, index.to_string().as_str())
    }

    fn leave_indexed_object(&mut self, _json_pointer: &str, _index: usize) {
        let child = self.stack.pop();
        let parent = self.stack.pop();
        match (child, parent) {
            (Some(child), Some(serde_json::Value::Array(mut values))) => {
                values.push(child);
                self.stack.push(serde_json::Value::Array(values))
            },
            (_, Some(value)) => self.stack.push(value),
            (_, None) => (),
        };
    }

    fn enter_indexed_array(&mut self, json_pointer: &str, index: usize) -> bool {
        self.enter_keyed_array(json_pointer, index.to_string().as_str())
    }

    fn leave_indexed_array(&mut self, json_pointer: &str, index: usize) {
        self.leave_indexed_object(json_pointer, index);
    }
}

impl CanExtract<Option<(String, serde_json::Value)>> for ExtractJsonValue {
    fn extract(&mut self) -> Option<(String, serde_json::Value)> {
        if self.stack.len() == 1 {
            self.json_pointer.take().zip(self.stack.pop())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::extractor::{
        extract::{ExtractJsonValue, ExtractPrimitive},
        navigate::AtPath,
        visit_json::visit_json_value,
    };

    use super::*;

    #[test]
    fn primitive_at_top_level() {
        let mut collector = ExtractPrimitive::new_string();

        let document = json!("hello");
        visit_json_value(&document, &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(result, Some(("".into(), "hello".into())));
    }

    #[test]
    fn primitive_at_path() {
        let mut collector = AtPath::new("x", ExtractPrimitive::new_string());

        let document = json!({"x": "hello", "y": false});
        visit_json_value(&document, &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(result, Some(("/x".into(), "hello".into())));
    }

    #[test]
    fn number_at_path() {
        let mut collector = AtPath::new("x", ExtractPrimitive::new_number());

        let document = json!({"x": 1e33, "y": false});
        visit_json_value(&document, &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(result, Some(("/x".into(), serde_json::Number::from_f64(1e33).unwrap())));
    }

    #[test]
    fn bool_at_path() {
        let mut collector = AtPath::new("y", ExtractPrimitive::new_bool());

        let document = json!({"x": 1e33, "y": false});
        visit_json_value(&document, &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(result, Some(("/y".into(), false)));
    }

    #[test]
    fn json_at_top_level() {
        let interesting_object = json!({
            "p": null,
            "o": {},
            "a": [null, {}, []]}
        );

        let mut collector = ExtractJsonValue::new();

        visit_json_value(&json!(interesting_object), &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(result, Some(("".into(), interesting_object)));
    }

    #[test]
    fn json_at_path() {
        let interesting_object = json!({
            "p": null,
            "o": {},
            "a": [null, {}, []]}
        );

        let mut collector = AtPath::new("x", ExtractJsonValue::new());

        visit_json_value(&json!({"x": interesting_object, "y": false}), &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(result, Some(("/x".into(), interesting_object)));
    }
}
