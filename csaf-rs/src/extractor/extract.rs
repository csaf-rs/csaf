use crate::extractor::traits::{CanExtract, Extractor};

/// An extractor that extracts a primitive value at a specified key, and returns it as an `Option`.
pub struct ExtractPrimitive<Type> {
    name: String,
    depth: usize,
    result: Option<(String, Type)>,
    mapping: fn(path: &str, &serde_json::Value) -> Option<(String, Type)>,
}

impl ExtractPrimitive<String> {
    /// Creates a new `ExtractPrimitive` extractor that will extract a string value at the
    /// specified key.
    pub fn new_string(name: &str) -> ExtractPrimitive<String> {
        ExtractPrimitive {
            name: name.to_string(),
            depth: 0,
            result: None,
            mapping: |p, v| v.as_str().map(|s| (p.to_string(), s.to_string())),
        }
    }
}
impl ExtractPrimitive<serde_json::Number> {
    /// Creates a new `ExtractPrimitive` extractor that will extract a number value at the
    /// specified key.
    pub fn new_number(name: &str) -> ExtractPrimitive<serde_json::Number> {
        ExtractPrimitive {
            name: name.to_string(),
            depth: 0,
            result: None,
            mapping: |p, v| v.as_number().map(|s| (p.to_string(), s.clone())),
        }
    }
}
impl ExtractPrimitive<bool> {
    /// Creates a new `ExtractPrimitive` extractor that will extract a boolean value at the
    /// specified key.
    pub fn new_bool(name: &str) -> ExtractPrimitive<bool> {
        ExtractPrimitive {
            name: name.to_string(),
            depth: 0,
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
    fn keyed_primitive(&mut self, path: &[String], name: &str, primitive: &serde_json::Value) {
        if self.depth == 0 && name == self.name {
            let full_path = [&["".to_string()], path, &[name.to_string()]].concat();
            self.result = (self.mapping)(full_path.join("/").as_str(), primitive)
        }
    }

    fn enter_keyed_object(&mut self, _path: &[String], _name: &str) -> bool {
        self.depth += 1;
        false
    }

    fn leave_keyed_object(&mut self, _path: &[String], _name: &str) {
        self.depth -= 1;
    }
}

/// An extractor that extracts an object structure and returns it as a `serde_json::Value`.
pub struct ExtractJsonValue {
    stack: Vec<serde_json::Value>,
}

impl ExtractJsonValue {
    /// Creates a new `ExtractJsonValue` extractor that will extract an object structure and
    /// return it as a `serde_json::Value`.
    pub fn new() -> Self {
        ExtractJsonValue {
            stack: vec![serde_json::Value::Object(serde_json::Map::new())],
        }
    }
}

impl Default for ExtractJsonValue {
    fn default() -> Self {
        Self::new()
    }
}

impl Extractor for ExtractJsonValue {
    fn keyed_primitive(&mut self, _path: &[String], name: &str, primitive: &serde_json::Value) {
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

    fn enter_keyed_object(&mut self, _path: &[String], _name: &str) -> bool {
        self.stack.push(serde_json::Value::Object(serde_json::Map::new()));
        true
    }

    fn leave_keyed_object(&mut self, _path: &[String], name: &str) {
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

    fn enter_keyed_array(&mut self, _path: &[String], _name: &str) -> bool {
        self.stack.push(serde_json::Value::Array(vec![]));
        true
    }

    fn leave_keyed_array(&mut self, path: &[String], name: &str) {
        self.leave_keyed_object(path, name);
    }

    fn indexed_primitive(&mut self, _path: &[String], _index: usize, primitive: &serde_json::Value) {
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

    fn enter_indexed_object(&mut self, path: &[String], index: usize) -> bool {
        self.enter_keyed_object(path, index.to_string().as_str())
    }

    fn leave_indexed_object(&mut self, _path: &[String], _index: usize) {
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

    fn enter_indexed_array(&mut self, path: &[String], index: usize) -> bool {
        self.enter_keyed_array(path, index.to_string().as_str())
    }

    fn leave_indexed_array(&mut self, path: &[String], index: usize) {
        self.leave_indexed_object(path, index);
    }
}

impl CanExtract<Option<serde_json::Value>> for ExtractJsonValue {
    fn extract(&mut self) -> Option<serde_json::Value> {
        if self.stack.len() == 1 {
            let result = self.stack.pop();
            self.stack.push(serde_json::Value::Object(serde_json::Map::new()));
            result
        } else {
            None
        }
    }
}
