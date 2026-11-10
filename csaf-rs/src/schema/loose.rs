mod cast;
pub mod v20;
pub mod v21;

/// A potentially schema-invalid CSAF document
pub struct Document {
    inner: serde_json::Value,
}

impl Document {
    /// Treats the given `json_value` as a `csaf2x::Document`
    ///
    /// This constructor does not perform any parsing or validation
    pub fn new(json_value: serde_json::Value) -> Self {
        Self { inner: json_value }
    }

    /// Immutably views the document as if it were a 2.0 document
    ///
    /// This can be thought of as a zero-cost cast
    pub fn v20_view(&self) -> &v20::Node0 {
        bytemuck::TransparentWrapper::wrap_ref(&self.inner)
    }

    /// Mutably views the document as if it were a 2.0 document
    ///
    /// This can be thought of as a zero-cost cast
    pub fn v20_view_mut(&mut self) -> &mut v20::Node0 {
        bytemuck::TransparentWrapper::wrap_mut(&mut self.inner)
    }

    /// Immutably views the document as if it were a 2.1 document
    ///
    /// This can be thought of as a zero-cost cast
    pub fn v21_view(&self) -> &v21::Node0 {
        bytemuck::TransparentWrapper::wrap_ref(&self.inner)
    }

    /// Mutably views the document as if it were a 2.1 document
    ///
    /// This can be thought of as a zero-cost cast
    pub fn v21_view_mut(&mut self) -> &mut v21::Node0 {
        bytemuck::TransparentWrapper::wrap_mut(&mut self.inner)
    }

    pub fn into_raw(self) -> serde_json::Value {
        self.inner
    }
}

#[derive(Debug, PartialEq)]
pub struct SchemaError;

type Object = serde_json::Map<String, serde_json::Value>;
