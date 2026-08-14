use crate::extractor::traits::{CanExtract, Extractor};

/// An extractor that collects the results of another extractor for each element of an array,
/// and returns the collected results as a vector.
pub struct CollectArray<Element, ElementType>
where
    Element: CanExtract<ElementType> + Extractor,
{
    matched: bool,
    depth: usize,
    source: Element,
    result: Vec<ElementType>,
}

impl<ElementType, Element: CanExtract<ElementType> + Extractor> CollectArray<Element, ElementType> {
    /// Creates a new `CollectArray` extractor that will collect the results of the provided `source`
    /// extractor for each element of an array.
    pub fn new(source: Element) -> Self {
        CollectArray {
            matched: false,
            depth: 0,
            source,
            result: Vec::new(),
        }
    }

    fn enter_array(&mut self) -> bool {
        self.depth += 1;
        if self.depth == 1 {
            self.matched = true;
            true
        } else {
            false
        }
    }

    fn enter_object(&mut self) {
        self.depth += 1;
    }

    fn leave(&mut self) {
        self.depth -= 1;
        if self.depth == 0 {
            if self.matched {
                self.result.push(self.source.extract())
            }
            self.matched = false;
        }
    }

    fn should_descend(&self) -> bool {
        self.depth == 1
    }
}

impl<ElementType, Element: CanExtract<ElementType> + Extractor> CanExtract<Vec<ElementType>>
    for CollectArray<Element, ElementType>
{
    fn extract(&mut self) -> Vec<ElementType> {
        std::mem::take(&mut self.result)
    }
}

impl<ElementType, Element: CanExtract<ElementType> + Extractor> Extractor for CollectArray<Element, ElementType> {
    fn keyed_primitive(&mut self, json_pointer: &str, name: &str, primitive: &serde_json::Value) {
        if self.matched {
            self.source.keyed_primitive(json_pointer, name, primitive);
        }
    }

    fn enter_keyed_object(&mut self, json_pointer: &str, name: &str) -> bool {
        self.enter_object();
        if self.matched {
            self.source.enter_keyed_object(json_pointer, name)
        } else {
            self.should_descend()
        }
    }

    fn leave_keyed_object(&mut self, json_pointer: &str, name: &str) {
        self.leave();
        if self.matched {
            self.source.leave_keyed_object(json_pointer, name);
        }
    }

    fn enter_keyed_array(&mut self, json_pointer: &str, name: &str) -> bool {
        self.enter_object();
        if self.matched {
            self.source.enter_keyed_array(json_pointer, name)
        } else {
            self.should_descend()
        }
    }

    fn leave_keyed_array(&mut self, json_pointer: &str, name: &str) {
        self.leave();
        if self.matched {
            self.source.leave_keyed_array(json_pointer, name);
        }
    }

    fn indexed_primitive(&mut self, json_pointer: &str, index: usize, primitive: &serde_json::Value) {
        if self.depth == 0 {
            self.source.init_primitive(json_pointer, primitive);
            self.result.push(self.source.extract())
        } else if self.matched {
            self.source.indexed_primitive(json_pointer, index, primitive);
        }
    }

    fn enter_indexed_object(&mut self, json_pointer: &str, index: usize) -> bool {
        if self.enter_array() {
            self.source.init_object(json_pointer);
            true
        } else if self.matched {
            self.source.enter_indexed_object(json_pointer, index)
        } else {
            self.should_descend()
        }
    }

    fn leave_indexed_object(&mut self, json_pointer: &str, index: usize) {
        self.leave();
        if self.matched {
            self.source.leave_indexed_object(json_pointer, index);
        }
    }

    fn enter_indexed_array(&mut self, json_pointer: &str, index: usize) -> bool {
        if self.enter_array() {
            self.source.init_array(json_pointer);
            true
        } else if self.matched {
            self.source.enter_indexed_array(json_pointer, index)
        } else {
            self.should_descend()
        }
    }

    fn leave_indexed_array(&mut self, json_pointer: &str, index: usize) {
        self.leave();
        if self.matched {
            self.source.leave_indexed_array(json_pointer, index);
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
    fn collect_array_of_primitives() {
        let mut collector = CollectArray::new(AtPath::new("x", ExtractPrimitive::new_string_with_path()));

        let document = json!([{"x": "a"}, {"x": "b"}]);
        visit_json_value(&document, &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(
            result,
            vec![Some(("/0/x".into(), "a".into())), Some(("/1/x".into(), "b".into()))]
        );
    }

    #[test]
    fn collect_array_of_json() {
        let mut collector = CollectArray::new(ExtractJsonValue::new());
        let interesting_object = json!({
            "p": null,
            "o": {},
            "a": [null, {}, []]}
        );
        let document = json!([interesting_object, {}, [], null]);
        visit_json_value(&document, &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(
            result,
            vec![
                Some(("/0".into(), interesting_object)),
                Some(("/1".into(), json!({}))),
                Some(("/2".into(), json!([]))),
                Some(("/3".into(), json!(null)))
            ]
        );
    }

    #[test]
    fn collect_array_of_array_of_primitive() {
        let mut collector = CollectArray::new(CollectArray::new(ExtractPrimitive::new_number_with_path()));

        let document = json!([[1], [2, 3], 4]);
        visit_json_value(&document, &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(
            result,
            vec![
                vec![Some(("/0/0".into(), 1.into()))],
                vec![Some(("/1/0".into(), 2.into())), Some(("/1/1".into(), 3.into()))],
                vec![]
            ]
        );
    }

    #[test]
    fn collect_from_object() {
        let mut collector = CollectArray::new(ExtractPrimitive::new_string_with_path());

        let document = json!({"a": "a", "b": "b"});
        visit_json_value(&document, &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn collect_from_primitive() {
        let mut collector = CollectArray::new(ExtractPrimitive::new_string_with_path());

        visit_json_value(&json!("hello"), &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(result, vec![]);
    }
}
