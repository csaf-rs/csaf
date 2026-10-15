use serde_json::Value;

/// Result enum specifying if and how a JSON property is present and set.
#[derive(Debug, PartialEq)]
pub(crate) enum JsonValuePresence {
    /// Property is not contained
    Missing,
    /// Property is contained, but set to `null`
    Unset,
    /// Property is contained, but an empty string (`""`)
    EmptyString,
    /// Property is contained, but an empty array (`[]`)
    EmptyArray,
    /// Property is contained, but an empty object (`{}`)
    EmptyObject,
    /// Property is contained and has a non-empty value
    Set,
}

impl JsonValuePresence {
    /// Returns `true` if the property is not contained in the JSON data.
    pub(crate) fn is_missing(&self) -> bool {
        matches!(self, JsonValuePresence::Missing)
    }

    /// Returns `true` if the property is contained in the JSON data, regardless of its value.
    pub(crate) fn is_present(&self) -> bool {
        !matches!(self, JsonValuePresence::Missing)
    }

    /// Returns `true` if the property is contained in the JSON data and is empty
    /// (i.e., an empty string, array, or object).
    pub(crate) fn is_empty(&self) -> bool {
        matches!(
            self,
            JsonValuePresence::EmptyString
                | JsonValuePresence::EmptyArray
                | JsonValuePresence::EmptyObject
        )
    }

    /// Returns `true` if the property is contained in the JSON data and has a non-empty value.
    pub(crate) fn is_present_and_set(&self) -> bool {
        matches!(self, JsonValuePresence::Set)
    }
}

/// Checks whether the property specified by `path` is present and set in the provided JSON data.
/// Returns a [`JsonValuePresence`].
pub(crate) fn get_json_presence(path: &str, json: &Value) -> JsonValuePresence {
    match json.pointer(path) {
        Some(Value::Null) => JsonValuePresence::Unset,
        Some(Value::Array(t)) if t.is_empty() => JsonValuePresence::EmptyArray,
        Some(Value::String(t)) if t.is_empty() => JsonValuePresence::EmptyString,
        Some(Value::Object(t)) if t.is_empty() => JsonValuePresence::EmptyObject,
        Some(_) => JsonValuePresence::Set,
        None => JsonValuePresence::Missing,
    }
}

/// Returns `true`, if the provided JSON `path` contains a String object with the `expected` value.
/// Returns `false` in all other cases.
pub(crate) fn property_string_value_is(path: &str, expected: &str, json: &Value) -> bool {
    matches!(json.pointer(path), Some(Value::String(value)) if value == expected)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};
    use serde_json::json;

    #[fixture]
    #[once]
    fn test_json() -> Value {
        json!({
            "null": null,
            "empty_array": [],
            "empty_object": {},
            "empty_string": "",
            "with_value": {
                "array": [1],
                "object": {"foo": "bar"},
                "string": "foobar",
                "number": 0,
                "bool": false
            }
        })
    }

    #[rstest]
    #[case("/missing", JsonValuePresence::Missing)]
    #[case("/null", JsonValuePresence::Unset)]
    #[case("/empty_array", JsonValuePresence::EmptyArray)]
    #[case("/empty_object", JsonValuePresence::EmptyObject)]
    #[case("/empty_string", JsonValuePresence::EmptyString)]
    #[case("/with_value/array", JsonValuePresence::Set)]
    #[case("/with_value/object", JsonValuePresence::Set)]
    #[case("/with_value/string", JsonValuePresence::Set)]
    #[case("/with_value/number", JsonValuePresence::Set)]
    #[case("/with_value/bool", JsonValuePresence::Set)]
    fn is_present_and_set_generates_json_value_presence(
        test_json: &Value,
        #[case] path: &str,
        #[case] expected: JsonValuePresence,
    ) {
        assert_eq!(get_json_presence(path, test_json), expected);
    }
}
