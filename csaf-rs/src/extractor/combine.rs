use crate::extractor::traits::{CanExtract, Extractor, WrappedExtractor};

/// An extractor that combines the results of two other extractors using a provided function.
pub struct Combine<First, Second, FirstType, SecondType, ResultType> {
    first: First,
    second: Second,
    combine: fn(FirstType, SecondType) -> ResultType,
}

impl<FirstType, SecondType, First: CanExtract<FirstType>, Second: CanExtract<SecondType>, ResultType>
    Combine<First, Second, FirstType, SecondType, ResultType>
{
    /// Creates a new `Combine` extractor that will combine the results of the provided `first` and
    /// `second` extractors using the provided `combine` function.
    pub fn new(first: First, second: Second, combine: fn(FirstType, SecondType) -> ResultType) -> Self {
        Combine { first, second, combine }
    }
}

impl<FirstType, SecondType, First: CanExtract<FirstType>, Second: CanExtract<SecondType>>
    Combine<First, Second, FirstType, SecondType, (FirstType, SecondType)>
{
    /// Creates a new `Combine` extractor that will combine the results of the provided `first` and
    /// `second` extractors into a tuple.
    pub fn new_pair(first: First, second: Second) -> Self {
        Combine::new(first, second, |first, second| (first, second))
    }
}

impl<First: Extractor, Second: Extractor, FirstType, SecondType, ResultType> WrappedExtractor
    for Combine<First, Second, FirstType, SecondType, ResultType>
{
    fn apply_inner_bool<F: FnMut(&mut dyn Extractor) -> bool>(&mut self, mut function: F) -> bool {
        (function)(&mut self.first) | (function)(&mut self.second)
    }

    fn apply_inner_unit<F: FnMut(&mut dyn Extractor)>(&mut self, mut function: F) {
        (function)(&mut self.first);
        (function)(&mut self.second);
    }
}

impl<FirstType, SecondType, First: CanExtract<FirstType>, Second: CanExtract<SecondType>, ResultType>
    CanExtract<ResultType> for Combine<First, Second, FirstType, SecondType, ResultType>
{
    fn extract(&mut self) -> ResultType {
        (self.combine)(self.first.extract(), self.second.extract())
    }
}
