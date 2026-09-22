use super::StrictSchemaConfig;

pub fn get_strict_schemas() -> Vec<StrictSchemaConfig> {
    vec![
        StrictSchemaConfig {
            input: "assets/csaf_2.0_json_schema.json",
            output: "assets/csaf_2.0_json_schema.strict.json",
        },
        StrictSchemaConfig {
            input: "assets/csaf_2.1_json_schema.json",
            output: "assets/csaf_2.1_json_schema.strict.json",
        },
        StrictSchemaConfig {
            input: "assets/cvss-v2.0.json",
            output: "assets/cvss-v2.0.strict.json",
        },
        StrictSchemaConfig {
            input: "assets/cvss-v4.0.json",
            output: "assets/cvss-v4.0.strict.json",
        },
        StrictSchemaConfig {
            input: "assets/extension-metaschema.json",
            output: "assets/extension-metaschema.strict.json",
        },
        StrictSchemaConfig {
            input: "assets/extension-content.json",
            output: "assets/extension-content.strict.json",
        },
        StrictSchemaConfig {
            input: "assets/SelectionList_2_0_0.schema.json",
            output: "assets/SelectionList_2_0_0.schema.strict.json",
        },
    ]
}
