/// Validate that the SSVC selection list schema (used in custom.rs) is valid JSON, similar to the checks in the type-generator
pub fn validate_ssvc_lib_json() {
    serde_json::from_str::<serde_json::Value>(ssvc::assets::SELECTION_LIST_SCHEMA)
        .expect("ssvc::assets::SELECTION_LIST_SCHEMA is not valid JSON");
}
