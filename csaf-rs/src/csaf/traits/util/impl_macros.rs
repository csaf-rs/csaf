/// Macro to implement a getter method that returns a struct field as `&str`.
macro_rules! impl_str_field_getter {
    ($method:ident, $field:ident) => {
        fn $method(&self) -> &str {
            &self.$field
        }
    };
}

/// Macro to implement a getter method that returns `Option<&str>` from an `Option`-wrapped struct field.
macro_rules! impl_optional_str_field_getter {
    ($method:ident, $field:ident) => {
        fn $method(&self) -> Option<&str> {
            self.$field.as_ref().map(|s| s.as_str())
        }
    };
}

pub(crate) use impl_optional_str_field_getter;
pub(crate) use impl_str_field_getter;
