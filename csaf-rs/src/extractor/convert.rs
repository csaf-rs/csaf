use crate::extractor::traits::{CanExtract, Extractor, WrappedExtractor};

/// An extractor that converts the result of another extractor from one type to another using a
/// provided function.
pub struct Convert<SourceType, TargetType, Source: CanExtract<SourceType>> {
    inner: Source,
    convert: fn(value: SourceType) -> TargetType,
}

impl<SourceType, TargetType, Source: CanExtract<SourceType>> Convert<SourceType, TargetType, Source> {
    /// Creates a new `Convert` extractor that will convert the result of the provided `inner` extractor
    /// from `SourceType` to `TargetType` using the provided `convert` function
    pub fn new(inner: Source, convert: fn(value: SourceType) -> TargetType) -> Self {
        Convert { inner, convert }
    }
}

impl<SourceType, TargetType, Source: CanExtract<SourceType> + Extractor> WrappedExtractor
    for Convert<SourceType, TargetType, Source>
{
    fn apply_inner_bool<F: FnMut(&mut dyn Extractor) -> bool>(&mut self, mut function: F) -> bool {
        (function)(&mut self.inner)
    }

    fn apply_inner_unit<F: FnMut(&mut dyn Extractor)>(&mut self, mut function: F) {
        (function)(&mut self.inner)
    }
}

impl<SourceType, TargetType, Source: CanExtract<SourceType>> CanExtract<TargetType>
    for Convert<SourceType, TargetType, Source>
{
    fn extract(&mut self) -> TargetType {
        (self.convert)(self.inner.extract())
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
    fn convert_drop_path_primitive() {
        let mut value = Convert::new(AtPath::new("x", ExtractPrimitive::new_string()), |x| x.map(|x| x.1));

        let document = json!({"x": "hello", "y": [], "z": "hello"});
        visit_json_value(&document, &mut [&mut value]);

        let result = value.extract();
        assert_eq!(result, Some("hello".into()));
    }

    #[test]
    fn convert_drop_path_json() {
        let mut collector = Convert::new(AtPath::new("x", ExtractJsonValue::new()), |x| x.map(|x| x.1));
        let interesting_object = json!({"p": null, "o": {}, "a": [null, {}, []]});
        let document = json!({"x": interesting_object, "y": false});
        visit_json_value(&document, &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(result, Some(interesting_object));
    }
}
