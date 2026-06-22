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
