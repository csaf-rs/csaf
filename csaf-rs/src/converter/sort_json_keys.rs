use crate::{
    converter::{Convert, Diagnostics, Settings},
    schema::loose,
};

/// The keys within a CSAF document SHOULD be sorted alphabetically.
pub struct SortJsonKeys;

impl Convert for SortJsonKeys {
    fn convert(&self, doc: &mut loose::Document, _settings: &Settings, _diagnostics: &mut dyn Diagnostics) {
        if let Ok(map) = doc.v21_view_mut().caution_as_object_mut() {
            sort_object_deep(map);
        }
    }
}

fn sort_object_deep(map: &mut serde_json::Map<String, serde_json::Value>) {
    for value in map.values_mut() {
        if let serde_json::Value::Object(inner_map) = value {
            sort_object_deep(inner_map);
        }
    }
    let mut keys: Vec<_> = map.keys().cloned().collect();
    keys.sort();
    let mut sorted_map = serde_json::Map::new();
    for key in keys {
        if let Some(value) = map.remove(&key) {
            sorted_map.insert(key, value);
        }
    }
    *map = sorted_map;
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::converter::tests::NoDiagnostics;

    use super::*;

    #[test]
    fn converter_sort_json_keys() {
        let mut doc = loose::Document::new(json!({ "b": {"c": true, "a": false}, "a": 1 }));

        SortJsonKeys.convert(&mut doc, &Settings::default(), &mut NoDiagnostics);

        assert_eq!(json!({"a": 1, "b": {"a": false, "c": true}}), doc.into_raw());
    }
}
