use serde_json::Value;
use std::sync::LazyLock;

pub static SSVC_2_SCHEMA: LazyLock<Value> = LazyLock::new(|| {
    let schema_str = ssvc::assets::SELECTION_LIST_SCHEMA;
    serde_json::from_str(schema_str).unwrap()
});
