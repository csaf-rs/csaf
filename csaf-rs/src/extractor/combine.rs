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
    fn combine_pair() {
        let mut pair = Combine::new_pair(
            AtPath::new("x", ExtractPrimitive::new_string()),
            AtPath::new("y", ExtractJsonValue::new()),
        );

        let document = json!({"z": false, "y": [], "x": "hello"});
        visit_json_value(&document, &mut [&mut pair]);

        let result = pair.extract();
        assert_eq!(
            result,
            (
                Some(("/x".to_string(), "hello".to_string())),
                Some(("/y".to_string(), json!([])))
            )
        );
    }

    #[test]
    fn combine_func() {
        let mut added = Combine::new(
            AtPath::new_path(&["x"], ExtractPrimitive::new_number()),
            AtPath::new_path(&["y"], ExtractPrimitive::new_number()),
            |x, y| x.unwrap().1.as_i64().unwrap() + y.unwrap().1.as_i64().unwrap(),
        );

        let document = json!({"x": 1, "y": 2, "z": "hello"});
        visit_json_value(&document, &mut [&mut added]);

        let result = added.extract();
        assert_eq!(result, 3);
    }
}
