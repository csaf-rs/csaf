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

/// Macro to implement a getter method that returns an `Option<impl Iterator<Item = &str>>`
/// from an `Option<Vec<T>>` or `Option<Newtype>` that derefs to `Vec<T>, where `T` derefs to `str`-like.
macro_rules! impl_optional_str_iter_field_getter {
    ($method:ident, $field:ident) => {
        fn $method(&self) -> Option<impl Iterator<Item = &str> + '_> {
            self.$field.as_ref().map(|v| v.iter().map(|x| x.as_str()))
        }
    };
}

/// Macro to implement a getter method that returns `impl Iterator<Item = &str>`
/// from a `Vec<T>` or `Newtype` field that derefs to `Vec<T>`, where `T` derefs to `str`-like.
macro_rules! impl_str_iter_field_getter {
    ($method:ident, $field:ident) => {
        fn $method(&self) -> impl Iterator<Item = &str> + '_ {
            self.$field.iter().map(|x| x.as_str())
        }
    };
}

pub(crate) use impl_optional_str_field_getter;
pub(crate) use impl_optional_str_iter_field_getter;
pub(crate) use impl_str_field_getter;
pub(crate) use impl_str_iter_field_getter;
