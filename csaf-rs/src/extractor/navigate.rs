use crate::extractor::traits::{CanExtract, Extractor};

/// An extractor that navigates to a specified path and applies another extractor there.
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

    /// Creates a new `AtPath` extractor that will navigate to the specified path and apply the
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
            false
        } else {
            true
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
    fn keyed_primitive(&mut self, name: &str, primitive: &serde_json::Value) {
        if self.should_forward() {
            self.source.keyed_primitive(name, primitive);
        }
    }

    fn enter_keyed_object(&mut self, name: &str) -> bool {
        if self.enter(name) && self.should_forward() {
            self.source.enter_keyed_object(name)
        } else {
            self.should_descend()
        }
    }

    fn leave_keyed_object(&mut self, name: &str) {
        self.leave();
        if self.should_forward() {
            self.source.leave_keyed_object(name)
        }
    }

    fn enter_keyed_array(&mut self, name: &str) -> bool {
        if self.enter(name) && self.should_forward() {
            self.source.enter_keyed_array(name)
        } else {
            self.should_descend()
        }
    }

    fn leave_keyed_array(&mut self, name: &str) {
        self.leave();
        if self.should_forward() {
            self.source.leave_keyed_array(name)
        }
    }

    fn indexed_primitive(&mut self, index: usize, primitive: &serde_json::Value) {
        if self.should_forward() {
            self.source.indexed_primitive(index, primitive);
        }
    }

    fn enter_indexed_object(&mut self, index: usize) -> bool {
        if self.enter(index.to_string().as_str()) && self.should_forward() {
            self.source.enter_indexed_object(index)
        } else {
            self.should_descend()
        }
    }

    fn leave_indexed_object(&mut self, index: usize) {
        self.leave();
        if self.should_forward() {
            self.source.leave_indexed_object(index)
        }
    }

    fn enter_indexed_array(&mut self, index: usize) -> bool {
        if self.enter(index.to_string().as_str()) && self.should_forward() {
            self.source.enter_indexed_array(index)
        } else {
            self.should_descend()
        }
    }

    fn leave_indexed_array(&mut self, index: usize) {
        self.leave();
        if self.should_forward() {
            self.source.leave_indexed_array(index)
        }
    }
}

impl<R, T: CanExtract<R> + Extractor> CanExtract<R> for AtPath<T> {
    fn extract(&mut self) -> R {
        self.source.extract()
    }
}
