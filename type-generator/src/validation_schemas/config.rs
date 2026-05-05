use super::ValidationSchemaConfig;

/// All validation schemas that are to be embedded in the `csaf-rs` binary
pub const VALIDATION_SCHEMAS: &[ValidationSchemaConfig] = &[
    ValidationSchemaConfig {
        var_name: "CSAF_2_0_SCHEMA",
        source_url: "https://docs.oasis-open.org/csaf/csaf/v2.0/csaf_json_schema.json",
        relative_asset_path: "assets/csaf_2.0_json_schema.json",
    },
    ValidationSchemaConfig {
        var_name: "CSAF_2_1_SCHEMA",
        source_url: "https://docs.oasis-open.org/csaf/csaf/v2.1/schema/csaf.json",
        relative_asset_path: "assets/csaf_2.1_json_schema.json",
    },
    ValidationSchemaConfig {
        var_name: "CVSS_V2_SCHEMA",
        source_url: "https://www.first.org/cvss/cvss-v2.0.json",
        relative_asset_path: "assets/cvss-v2.0.json",
    },
    ValidationSchemaConfig {
        var_name: "CVSS_V3_0_SCHEMA",
        source_url: "https://www.first.org/cvss/cvss-v3.0.json",
        relative_asset_path: "assets/cvss-v3.0.json",
    },
    ValidationSchemaConfig {
        var_name: "CVSS_V3_1_SCHEMA",
        source_url: "https://www.first.org/cvss/cvss-v3.1.json",
        relative_asset_path: "assets/cvss-v3.1.json",
    },
    ValidationSchemaConfig {
        var_name: "CVSS_V4_0_2_SCHEMA",
        source_url: "https://www.first.org/cvss/cvss-v4.0.2.json",
        relative_asset_path: "assets/cvss-v4.0.2.json",
    },
    ValidationSchemaConfig {
        var_name: "EXTENSION_METASCHEMA",
        source_url: "https://docs.oasis-open.org/csaf/csaf/v2.1/schema/extension-metaschema.json",
        relative_asset_path: "assets/extension-metaschema.json",
    },
    ValidationSchemaConfig {
        var_name: "EXTENSION_SCHEMA",
        source_url: "https://docs.oasis-open.org/csaf/csaf/v2.1/schema/extension-content.json",
        relative_asset_path: "assets/extension-content.json",
    },
];
