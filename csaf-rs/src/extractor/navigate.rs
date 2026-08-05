use crate::extractor::traits::{CanExtract, Extractor};

/// An extractor that navigates to a specified path and applies another extractor there.
#[derive(Clone)]
pub struct AtPath<Source: Extractor> {
    path: Vec<String>,
    source: Source,
    depth: usize,
    matched: usize,
}

impl<Source: Extractor> AtPath<Source> {
    /// Creates a new `AtPath` extractor that will navigate to the specified key and apply
    /// the provided `payload` extractor there.
    pub fn new(key: &str, payload: Source) -> Self {
        AtPath {
            path: vec![key.to_string()],
            source: payload,
            depth: 0,
            matched: 0,
        }
    }

    /// Creates a new `AtPath` extractor that will navigate to the specified json_pointer and apply the
    /// provided `payload` extractor there.
    pub fn new_path(path: &[&str], payload: Source) -> Self {
        AtPath {
            path: path.iter().map(|s| s.to_string()).collect(),
            source: payload,
            depth: 0,
            matched: 0,
        }
    }

    fn enter(&mut self, name: &str) -> bool {
        self.depth += 1;
        if self.matched + 1 == self.depth
            && let Some(element) = self.path.get(self.matched)
            && element == name
        {
            self.matched += 1;
            self.matched == self.path.len()
        } else {
            false
        }
    }

    fn leave(&mut self) {
        self.depth -= 1;
        if self.matched > self.depth {
            self.matched = self.depth;
        }
    }

    fn should_forward(&self) -> bool {
        self.matched == self.path.len()
    }

    fn should_descend(&self) -> bool {
        self.depth <= self.path.len()
    }
}

impl<Source: Extractor> Extractor for AtPath<Source> {
    fn keyed_primitive(&mut self, json_pointer: &str, name: &str, primitive: &serde_json::Value) {
        if self.enter(name) {
            self.source.init_primitive(json_pointer, primitive);
        }
        self.leave();
        if self.should_forward() {
            self.source.keyed_primitive(json_pointer, name, primitive);
        }
    }

    fn enter_keyed_object(&mut self, json_pointer: &str, name: &str) -> bool {
        if self.enter(name) {
            self.source.init_object(json_pointer);
            true
        } else if self.should_forward() {
            self.source.enter_keyed_object(json_pointer, name)
        } else {
            self.should_descend()
        }
    }

    fn leave_keyed_object(&mut self, json_pointer: &str, name: &str) {
        self.leave();
        if self.should_forward() {
            self.source.leave_keyed_object(json_pointer, name)
        }
    }

    fn enter_keyed_array(&mut self, json_pointer: &str, name: &str) -> bool {
        if self.enter(name) {
            self.source.init_array(json_pointer);
            true
        } else if self.should_forward() {
            self.source.enter_keyed_array(json_pointer, name)
        } else {
            self.should_descend()
        }
    }

    fn leave_keyed_array(&mut self, json_pointer: &str, name: &str) {
        self.leave();
        if self.should_forward() {
            self.source.leave_keyed_array(json_pointer, name)
        }
    }

    fn indexed_primitive(&mut self, json_pointer: &str, index: usize, primitive: &serde_json::Value) {
        if self.enter(index.to_string().as_str()) {
            self.source.init_primitive(json_pointer, primitive);
        }
        self.leave();
        if self.should_forward() {
            self.source.indexed_primitive(json_pointer, index, primitive);
        }
    }

    fn enter_indexed_object(&mut self, json_pointer: &str, index: usize) -> bool {
        if self.enter(index.to_string().as_str()) {
            self.source.init_object(json_pointer);
            true
        } else if self.should_forward() {
            self.source.enter_indexed_object(json_pointer, index)
        } else {
            self.should_descend()
        }
    }

    fn leave_indexed_object(&mut self, json_pointer: &str, index: usize) {
        self.leave();
        if self.should_forward() {
            self.source.leave_indexed_object(json_pointer, index)
        }
    }

    fn enter_indexed_array(&mut self, json_pointer: &str, index: usize) -> bool {
        if self.enter(index.to_string().as_str()) {
            self.source.init_array(json_pointer);
            true
        } else if self.should_forward() {
            self.source.enter_indexed_array(json_pointer, index)
        } else {
            self.should_descend()
        }
    }

    fn leave_indexed_array(&mut self, json_pointer: &str, index: usize) {
        self.leave();
        if self.should_forward() {
            self.source.leave_indexed_array(json_pointer, index)
        }
    }
}

impl<R, T: CanExtract<R> + Extractor> CanExtract<R> for AtPath<T> {
    fn extract(&mut self) -> R {
        self.source.extract()
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
    fn object_in_object() {
        let interesting_object = json!({"p": null, "o": {}, "a": [null, {}, []]});
        let mut collector = AtPath::new("x", ExtractJsonValue::new());

        let document = json!({"x": interesting_object, "y": false});
        visit_json_value(&document, &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(result, Some(("/x".into(), interesting_object)));
    }

    #[test]
    fn object_in_array() {
        let interesting_object = json!({"p": null, "o": {}, "a": [null, {}, []]});
        let mut collector = AtPath::new("0", ExtractJsonValue::new());

        let document = json!([interesting_object, false]);
        visit_json_value(&document, &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(result, Some(("/0".into(), interesting_object)));
    }

    #[test]
    fn array_in_object() {
        let interesting_array = json!([null, {"x": {}}, [null, {}, [null]]]);
        let mut collector = AtPath::new("x", ExtractJsonValue::new());

        let document = json!({"x": interesting_array, "y": false});
        visit_json_value(&document, &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(result, Some(("/x".into(), interesting_array)));
    }

    #[test]
    fn array_in_array() {
        let interesting_array = json!([null, {"x": {}}, [null, {}, [null]]]);
        let mut collector = AtPath::new("0", ExtractJsonValue::new());

        let document = json!([interesting_array, false]);
        visit_json_value(&document, &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(result, Some(("/0".into(), interesting_array)));
    }

    #[test]
    fn primitive_at_path() {
        let mut collector = AtPath::new("x", ExtractPrimitive::new_string_with_path());

        let document = json!({"x": "hello", "y": false});
        visit_json_value(&document, &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(result, Some(("/x".into(), "hello".into())));
    }

    #[test]
    fn deep_path() {
        let mut collector = AtPath::new_path(&["x", "0", "x"], ExtractPrimitive::new_bool_with_path());

        let document = json!({"x": [{"y": "z", "x": false}], "y": null});
        visit_json_value(&document, &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(result, Some(("/x/0/x".into(), false)));
    }
}
