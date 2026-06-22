/// An extractor that traverses a JSON structure and extracts data based on specific rules.
pub trait Extractor {
    /// Called when a primitive value is encountered at a specific key of a JSON object.
    fn keyed_primitive(&mut self, name: &str, primitive: &serde_json::Value);

    /// Called when entering an object at a specific key of a JSON object.
    /// Returns `true` if the extractor wants to continue traversing the object.
    fn enter_keyed_object(&mut self, name: &str) -> bool;

    /// Called when leaving an object at a specific key of a JSON object.
    fn leave_keyed_object(&mut self, name: &str);

    /// Called when entering an array at a specific key of a JSON object.
    /// Returns `true` if the extractor wants to continue traversing the array.
    fn enter_keyed_array(&mut self, name: &str) -> bool {
        self.enter_keyed_object(name)
    }

    /// Called when leaving an array at a specific key of a JSON object.
    fn leave_keyed_array(&mut self, name: &str) {
        self.leave_keyed_object(name);
    }

    /// Called when a primitive value is encountered at a specific index of a JSON array.
    fn indexed_primitive(&mut self, index: usize, primitive: &serde_json::Value) {
        self.keyed_primitive(index.to_string().as_str(), primitive);
    }

    /// Called when entering an object at a specific index of a JSON array.
    /// Returns `true` if the extractor wants to continue traversing the object.
    fn enter_indexed_object(&mut self, index: usize) -> bool {
        self.enter_keyed_object(index.to_string().as_str())
    }

    /// Called when leaving an object at a specific index of a JSON array.
    fn leave_indexed_object(&mut self, index: usize) {
        self.leave_keyed_object(index.to_string().as_str());
    }

    /// Called when entering an array at a specific index of a JSON array.
    /// Returns `true` if the extractor wants to continue traversing the array.
    fn enter_indexed_array(&mut self, index: usize) -> bool {
        self.enter_keyed_array(index.to_string().as_str())
    }

    /// Called when leaving an array at a specific index of a JSON array.
    fn leave_indexed_array(&mut self, index: usize) {
        self.leave_keyed_array(index.to_string().as_str());
    }
}

/// A trait for extractors that can produce a final extracted value of a specific type.
pub trait CanExtract<Type> {
    /// Extracts the final value from the extractor after traversal is complete.
    fn extract(&mut self) -> Type;
}

/// A trait for extractors that wrap another extractor and delegate calls to it.
pub trait WrappedExtractor {
    /// Applies a function to the inner extractor that returns a boolean value.
    fn apply_inner_bool<F: FnMut(&mut dyn Extractor) -> bool>(&mut self, function: F) -> bool;

    /// Applies a function to the inner extractor that returns no value.
    fn apply_inner_unit<F: FnMut(&mut dyn Extractor)>(&mut self, function: F);
}

impl<Wrapper: WrappedExtractor> Extractor for Wrapper {
    fn keyed_primitive(&mut self, name: &str, primitive: &serde_json::Value) {
        self.apply_inner_unit(|inner| inner.keyed_primitive(name, primitive))
    }

    fn enter_keyed_object(&mut self, name: &str) -> bool {
        self.apply_inner_bool(|inner| inner.enter_keyed_object(name))
    }

    fn leave_keyed_object(&mut self, name: &str) {
        self.apply_inner_unit(|inner| inner.leave_keyed_object(name))
    }

    fn enter_keyed_array(&mut self, name: &str) -> bool {
        self.apply_inner_bool(|inner| inner.enter_keyed_array(name))
    }

    fn leave_keyed_array(&mut self, name: &str) {
        self.apply_inner_unit(|inner| inner.leave_keyed_array(name))
    }

    fn indexed_primitive(&mut self, index: usize, primitive: &serde_json::Value) {
        self.apply_inner_unit(|inner| inner.indexed_primitive(index, primitive))
    }

    fn enter_indexed_object(&mut self, index: usize) -> bool {
        self.apply_inner_bool(|inner| inner.enter_indexed_object(index))
    }

    fn leave_indexed_object(&mut self, index: usize) {
        self.apply_inner_unit(|inner| inner.leave_indexed_object(index))
    }

    fn enter_indexed_array(&mut self, index: usize) -> bool {
        self.apply_inner_bool(|inner| inner.enter_indexed_array(index))
    }

    fn leave_indexed_array(&mut self, index: usize) {
        self.apply_inner_unit(|inner| inner.leave_indexed_array(index))
    }
}
