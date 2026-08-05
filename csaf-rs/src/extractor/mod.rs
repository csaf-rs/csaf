//! This module defines extractors which are used to traverse and extract data from JSON structures.

pub mod collect;
pub mod collect_tree;
pub mod combine;
pub mod convert;
pub mod extract;
pub mod navigate;
pub mod traits;
pub mod visit_json;
pub mod visit_stream;

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::extractor::{
        collect::CollectArray,
        collect_tree::CollectTree,
        extract::{ExtractJsonValue, ExtractPrimitive},
        navigate::AtPath,
        traits::CanExtract,
        visit_json::visit_json_value,
        visit_stream::visit_stream,
    };

    #[test]
    fn extract_string_at_path() {
        let json = json!({
            "x": {
                "y": "hello"
            },
            "y": "world"
        });

        let mut collector = AtPath::new_path(&["x", "y"], ExtractPrimitive::new_string());
        visit_json_value(&json, &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(result, Some("hello".into()));
    }

    #[test]
    fn extract_json_at_path() {
        let interesting_object = json!({
                "p": "hello",
                "q": [1, 2],
        });
        let json = json!({
            "x": interesting_object,
            "y": false
        });

        let mut collector = AtPath::new("x", ExtractJsonValue::new());
        visit_json_value(&json, &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(result, Some(("/x".into(), interesting_object)));
    }

    #[test]
    fn extract_array_of_strings() {
        let json = json!([{"x": "a"}, {"x": "b"}]);

        let mut collector = CollectArray::new(AtPath::new("x", ExtractPrimitive::new_string()));
        visit_json_value(&json, &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(result, vec![Some("a".into()), Some("b".into())]);
    }

    #[test]
    fn extract_strings_from_tree() {
        let document = json!({
            "value": "a",
            "children": [
                {"value": "b"},
                {"children": [{"value": "c"}], "value": "d"}
            ]
        });

        let mut collector = CollectTree::new(
            "children",
            AtPath::new("value", ExtractPrimitive::new_string_with_path()),
        );
        visit_json_value(&document, &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(
            result,
            vec![
                Some(("/value".into(), "a".into())),
                Some(("/children/0/value".into(), "b".into())),
                Some(("/children/1/value".into(), "d".into())),
                Some(("/children/1/children/0/value".into(), "c".into()))
            ]
        );
    }

    #[test]
    fn extract_from_truncated_json() {
        let json = br#"{"p": "x", "a":false, "b": [], "c": //"#;

        let mut collector = AtPath::new("a", ExtractPrimitive::new_bool());
        let parse_result = visit_stream(&json[..], &mut [&mut collector]);
        parse_result.expect_err("parsing should fail");

        let result = collector.extract();
        assert_eq!(result, Some(false));
    }
}
