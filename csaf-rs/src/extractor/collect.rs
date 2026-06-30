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
    result: Option<Vec<ElementType>>,
}

impl<ElementType, Element: CanExtract<ElementType> + Extractor> CollectArray<Element, ElementType> {
    /// Creates a new `CollectArray` extractor that will collect the results of the provided `source`
    /// extractor for each element of an array.
    pub fn new(source: Element) -> Self {
        CollectArray {
            matched: false,
            depth: 0,
            source,
            result: None,
        }
    }

    fn enter(&mut self, is_array: bool) -> bool {
        self.depth += 1;
        if self.depth == 1 {
            self.matched = is_array;
            false
        } else {
            true
        }
    }

    fn leave(&mut self) {
        self.depth -= 1;
        if self.depth == 0 {
            if self.matched {
                self.result.get_or_insert_default().push(self.source.extract())
            }
            self.matched = false;
        }
    }

    fn should_descend(&self) -> bool {
        self.depth == 0
    }
}

impl<ElementType, Element: CanExtract<ElementType> + Extractor> CanExtract<Option<Vec<ElementType>>>
    for CollectArray<Element, ElementType>
{
    fn extract(&mut self) -> Option<Vec<ElementType>> {
        self.result.take()
    }
}

impl<ElementType, Element: CanExtract<ElementType> + Extractor> Extractor for CollectArray<Element, ElementType> {
    fn keyed_primitive(&mut self, path: &[String], name: &str, primitive: &serde_json::Value) {
        if self.matched {
            self.source.keyed_primitive(path, name, primitive);
        }
    }

    fn enter_keyed_object(&mut self, path: &[String], name: &str) -> bool {
        if self.enter(false) && self.matched {
            self.source.enter_keyed_object(path, name)
        } else {
            self.should_descend()
        }
    }

    fn leave_keyed_object(&mut self, path: &[String], name: &str) {
        self.leave();
        if self.matched {
            self.source.leave_keyed_object(path, name);
        }
    }

    fn enter_keyed_array(&mut self, path: &[String], name: &str) -> bool {
        if self.enter(false) && self.matched {
            self.source.enter_keyed_array(path, name)
        } else {
            self.should_descend()
        }
    }

    fn leave_keyed_array(&mut self, path: &[String], name: &str) {
        self.leave();
        if self.matched {
            self.source.leave_keyed_array(path, name);
        }
    }

    fn indexed_primitive(&mut self, path: &[String], index: usize, primitive: &serde_json::Value) {
        if self.matched {
            self.source.indexed_primitive(path, index, primitive);
        }
    }

    fn enter_indexed_object(&mut self, path: &[String], index: usize) -> bool {
        if self.enter(true) && self.matched {
            self.source.enter_indexed_object(path, index)
        } else {
            self.should_descend()
        }
    }

    fn leave_indexed_object(&mut self, path: &[String], index: usize) {
        self.leave();
        if self.matched {
            self.source.leave_indexed_object(path, index);
        }
    }

    fn enter_indexed_array(&mut self, path: &[String], index: usize) -> bool {
        if self.enter(true) && self.matched {
            self.source.enter_indexed_array(path, index)
        } else {
            self.should_descend()
        }
    }

    fn leave_indexed_array(&mut self, path: &[String], index: usize) {
        self.leave();
        if self.matched {
            self.source.leave_indexed_array(path, index);
        }
    }
}
