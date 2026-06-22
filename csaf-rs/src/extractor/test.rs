use serde_json::json;

use crate::extractor::{
    collect::CollectArray,
    combine::Combine,
    convert::Convert,
    extract::{ExtractJsonValue, ExtractPrimitive},
    navigate::AtPath,
    traits::{CanExtract, Extractor},
    visit_json::visit_json_value,
    visit_stream::visit_stream,
};

struct JsonPointerCollector {
    path: Vec<String>,
    pointers: Option<Vec<String>>,
}

impl JsonPointerCollector {
    fn new() -> Self {
        JsonPointerCollector {
            path: vec!["".to_string()],
            pointers: None,
        }
    }
    fn get_json_pointer(&self) -> String {
        self.path.join("/")
    }
}

impl Default for JsonPointerCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl Extractor for JsonPointerCollector {
    fn keyed_primitive(&mut self, name: &str, _primitive: &serde_json::Value) {
        let pointer = self.get_json_pointer();
        self.pointers.get_or_insert_default().push(format!("{pointer}/{name}"));
    }

    fn enter_keyed_object(&mut self, name: &str) -> bool {
        let pointer = self.get_json_pointer();
        self.pointers.get_or_insert_default().push(format!("{pointer}/{name}"));
        self.path.push(name.to_string());
        true
    }

    fn leave_keyed_object(&mut self, _name: &str) {
        self.path.pop();
    }
}

impl CanExtract<Vec<String>> for JsonPointerCollector {
    fn extract(&mut self) -> Vec<String> {
        self.pointers.take().unwrap_or_default()
    }
}

#[derive(Default, Debug, PartialEq)]
struct XAndY {
    x: Option<String>,
    y: Option<bool>,
}

#[test]
fn test_walk_json() {
    let mut my_number = AtPath::new("x", ExtractPrimitive::new_number("y"));
    let mut names = Convert::new(
        AtPath::new_path(&["y", "z"], CollectArray::new(ExtractPrimitive::new_string("x"))),
        |x| x.unwrap_or_default(),
    );
    let mut pointers_and_json = Combine::new_pair(
        AtPath::new("x", JsonPointerCollector::new()),
        AtPath::new("x", ExtractJsonValue::new()),
    );

    let mut x_and_y = Combine::new(
        AtPath::new_path(&["y", "z", "2"], ExtractPrimitive::new_string("x")),
        AtPath::new_path(&["y", "z", "0"], ExtractPrimitive::new_bool("y")),
        |x, y| XAndY { x, y },
    );

    let document = json!({
        "a": false,
        "y": {
            "z": [
                    {"x": "Hallo", "y": true, "z": "Welt"},
                    [1, 2, 3],
                    {"x": "Welt"},
                    3,
                    {"y": 1, "x": "aa"},
                ]
        },
        "x": {
            "y": 1,
            "z": ["a", 1, null, [null], {}]
        }
    });
    visit_json_value(
        document.as_object().unwrap(),
        &mut [&mut my_number, &mut names, &mut pointers_and_json, &mut x_and_y],
    );

    assert_eq!(my_number.extract(), serde_json::Number::from_i128(1));
    assert_eq!(
        names.extract(),
        vec![
            Some("Hallo".to_string()),
            None,
            Some("Welt".to_string()),
            Some("aa".to_string())
        ]
    );
    assert_eq!(
        pointers_and_json.extract(),
        (
            vec![
                "/y".to_string(),
                "/z".to_string(),
                "/z/0".to_string(),
                "/z/1".to_string(),
                "/z/2".to_string(),
                "/z/3".to_string(),
                "/z/3/0".to_string(),
                "/z/4".to_string()
            ],
            Some(json!({
                "y": 1,
                "z": ["a", 1, null, [null], {}]
            }))
        )
    );
    assert_eq!(
        x_and_y.extract(),
        XAndY {
            x: Some("Welt".to_string()),
            y: Some(true),
        }
    );
}

#[test]
fn test_walk_stream() {
    let mut my_number = AtPath::new("x", ExtractPrimitive::new_number("y"));
    let mut names = Convert::new(
        AtPath::new_path(&["y", "z"], CollectArray::new(ExtractPrimitive::new_string("x"))),
        |x| x.unwrap_or_default(),
    );
    let mut pointers_and_json = Combine::new_pair(
        AtPath::new("x", JsonPointerCollector::new()),
        AtPath::new("x", ExtractJsonValue::new()),
    );

    let mut x_and_y = Combine::new(
        AtPath::new_path(&["y", "z", "2"], ExtractPrimitive::new_string("x")),
        AtPath::new_path(&["y", "z", "0"], ExtractPrimitive::new_bool("y")),
        |x, y| XAndY { x, y },
    );

    let document = br#"
        {
            "a": false,
            "y": {
                "z": [
                        {"x": "Hallo", "y": true, "z": "Welt"},
                        [1, 2, 3],
                        {"x": "Welt"},
                        3,
                        {"y": 1, "x": "aa"}
                    ]
            },
            "x": {
                "y": 1,
                "z": ["a", 1, null, [null], {}]
            }
        }"#;

    let parse_result = visit_stream(
        &document[..],
        &mut [&mut my_number, &mut names, &mut pointers_and_json, &mut x_and_y],
    );

    assert_eq!(my_number.extract(), serde_json::Number::from_i128(1));
    assert_eq!(
        names.extract(),
        vec![
            Some("Hallo".to_string()),
            None,
            Some("Welt".to_string()),
            Some("aa".to_string())
        ]
    );
    assert_eq!(
        pointers_and_json.extract(),
        (
            vec![
                "/y".to_string(),
                "/z".to_string(),
                "/z/0".to_string(),
                "/z/1".to_string(),
                "/z/2".to_string(),
                "/z/3".to_string(),
                "/z/3/0".to_string(),
                "/z/4".to_string()
            ],
            Some(json!({
                "y": 1,
                "z": ["a", 1, null, [null], {}]
            }))
        )
    );
    assert_eq!(
        x_and_y.extract(),
        XAndY {
            x: Some("Welt".to_string()),
            y: Some(true),
        }
    );
    assert!(parse_result.is_ok());
}
