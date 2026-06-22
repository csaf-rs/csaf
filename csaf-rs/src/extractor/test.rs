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
        self.pointers
            .get_or_insert_default()
            .push(format!("{}/{name}", pointer));
    }

    fn enter_keyed_object(&mut self, name: &str) -> bool {
        let pointer = self.get_json_pointer();
        self.pointers
            .get_or_insert_default()
            .push(format!("{}/{name}", pointer));
        self.path.push(name.to_string());
        true
    }

    fn leave_keyed_object(&mut self, _name: &str) {
        self.path.pop();
    }
}

impl CanExtract<Vec<String>> for JsonPointerCollector {
    fn extract(&mut self) -> Vec<String> {
        self.pointers.take().unwrap_or(vec![])
    }
}

#[derive(Default, Debug)]
struct XAndY {
    x: Option<String>,
    y: Option<bool>,
}

#[test]
fn test_walk_json() {
    let mut my_number = AtPath::new("x", ExtractPrimitive::new_number("y"));
    let mut names = Convert::new(
        AtPath::new_path(&["y", "z"], CollectArray::new(ExtractPrimitive::new_string("x"))),
        |x| x.unwrap_or(vec![]),
    );
    let mut pointers_and_json = Combine::new_pair(
        AtPath::new("x", JsonPointerCollector::new()),
        AtPath::new("x", ExtractJsonValue::new()),
    );

    let mut x_and_y = Combine::new(
        AtPath::new_path(&["y", "z", "2"], ExtractPrimitive::new_string("x")),
        AtPath::new_path(&["y", "z", "0"], ExtractPrimitive::new_bool("y")),
        |x, y| XAndY { x: x, y: y },
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

    println!("number {:?}", my_number.extract());
    println!("names {:?}", names.extract());
    println!("combined {:?}", pointers_and_json.extract());
    println!("x and y: {:?}", x_and_y.extract());
}

#[test]
fn test_walk_stream() {
    let mut get_my_number = AtPath::new("x", ExtractPrimitive::new_number("y"));
    let mut collect_names = Convert::new(
        AtPath::new_path(&["y", "z"], CollectArray::new(ExtractPrimitive::new_string("x"))),
        |x| x.unwrap_or(vec![]),
    );
    let mut combined = Combine::new_pair(
        AtPath::new("x", JsonPointerCollector::new()),
        AtPath::new("x", ExtractJsonValue::new()),
    );

    let mut x_and_y = Combine::new(
        AtPath::new_path(&["y", "z", "2"], ExtractPrimitive::new_string("x")),
        AtPath::new_path(&["y", "z", "0"], ExtractPrimitive::new_bool("y")),
        |x, y| XAndY { x: x, y: y },
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
        &mut [&mut get_my_number, &mut collect_names, &mut combined, &mut x_and_y],
    );

    println!("number {:?}", get_my_number.extract());
    println!("names {:?}", collect_names.extract());
    println!("combined {:?}", combined.extract());
    println!("x and y: {:?}", x_and_y.extract());
    println!("parse result {parse_result:?}");
}
