use serde_json::Value;

/// Result enum specifying if and how a JSON property is present and set.
pub(crate) enum JsonValuePresence {
    /// Property is not contained
    Missing,
    /// Property is contained, but set to `null`
    Unset,
    /// Property is contained but empty, e.g. String: "", Array: [], Object: {}
    Empty,
    /// Property is contained and has a non-empty value
    Set,
}

/// Checks whether the property specified by `path` is present and set in the provided JSON data.
/// Returns a [JsonValuePresence].
pub(crate) fn is_present_and_set(path: &str, json: &Value) -> JsonValuePresence {
    match json.pointer(path) {
        Some(Value::Null) => JsonValuePresence::Unset,
        Some(Value::Array(t)) if t.is_empty() => JsonValuePresence::Empty,
        Some(Value::String(t)) if t.is_empty() => JsonValuePresence::Empty,
        Some(Value::Object(t)) if t.is_empty() => JsonValuePresence::Empty,
        Some(_) => JsonValuePresence::Set,
        None => JsonValuePresence::Missing,
    }
}

/// Returns `true`, if the provided JSON `path` contains a String object with the `expected` value.
/// Returns `false` in all other cases.
pub(crate) fn property_string_value_is(path: &str, expected: &str, json: &Value) -> bool {
    matches!(json.pointer(path), Some(Value::String(value)) if value == expected)
}
