mod build_helper;
mod file_helper;
mod schema_generator;
mod testcase_generator;

use crate::build_helper::BuildError;
use crate::schema_generator::build_from_schema;
use crate::testcase_generator::{CsafVersion, generate_testcases};
use clap::Parser;
use json_dotpath::DotPaths;
use serde_json::{Value, json};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Whether to include generation of test schema definitions
    #[arg(short, long, default_value_t = false)]
    include_test_schema: bool,

    /// Whether to include generation of test definitions
    #[arg(short, long, default_value_t = false)]
    create_test_definitions: bool,

    /// Target folder for generated code, ../csaf-rs by default
    #[arg(short, long, default_value = "../csaf-rs")]
    target_folder: String,
}

fn main() -> Result<(), BuildError> {
    let args = Args::parse();

    // Execute all listed schema builds
    for (input, output, schema_patch) in &get_schemas() {
        build_from_schema(input, output, schema_patch, args.target_folder.clone());
    }

    if args.include_test_schema {
        for (input, output, schema_patch) in &get_testcases_schemas() {
            build_from_schema(input, output, schema_patch, args.target_folder.clone());
        }
    }

    if args.create_test_definitions {
        _ = generate_testcases(
            "../csaf/csaf_2.0/test/validator/data/testcases.json",
            "assets/tests/csaf_2.0/testcases.json",
            "csaf2_0/testcases.generated.rs",
            CsafVersion::V2_0,
            &args.target_folder,
        );
        _ = generate_testcases(
            "../csaf/csaf_2.1/test/validator/data/testcases.json",
            "assets/tests/csaf_2.1/testcases.json",
            "csaf2_1/testcases.generated.rs",
            CsafVersion::V2_1,
            &args.target_folder,
        );
    }
    Ok(())
}

type SchemaInfo = (&'static str, &'static str, Option<&'static dyn Fn(&mut Value)>);

fn get_schemas() -> Vec<SchemaInfo> {
    vec![
        (
            "assets/csaf_2.0_json_schema.json",
            "csaf2_0/schema.rs",
            Some(&fix_2_0_schema as &dyn Fn(&mut Value)),
        ),
        (
            "assets/csaf_2.1_json_schema.json",
            "csaf2_1/schema.rs",
            Some(&fix_2_1_schema),
        ),
    ]
}

fn get_testcases_schemas() -> Vec<SchemaInfo> {
    vec![
        (
            "assets/csaf_2.0_testcases_json_schema.json",
            "csaf2_0/testcases_schema.rs",
            None,
        ),
        (
            "assets/csaf_2.1_testcases_json_schema.json",
            "csaf2_1/testcases_schema.rs",
            None,
        ),
    ]
}

/// Patches (unsupported) external schemas to the plain object type for CSAF 2.0.
fn fix_2_0_schema(value: &mut Value) {
    let prefix = "properties.vulnerabilities.items.properties.scores.items.properties";
    let fix_paths = [format!("{prefix}.cvss_v2"), format!("{prefix}.cvss_v3")];
    for path in fix_paths {
        value.dot_set(path.as_str(), json!({"type": "object"})).unwrap();
    }
    remove_datetime_formats(value);
}

/// Patches (unsupported) external schemas to the plain object type for CSAF 2.1.
fn fix_2_1_schema(value: &mut Value) {
    let prefix = "properties.vulnerabilities.items.properties.metrics.items.properties.content.properties";
    let fix_paths = [
        format!("{prefix}.cvss_v2"),
        format!("{prefix}.cvss_v3"),
        format!("{prefix}.cvss_v4"),
        format!("{prefix}.ssvc_v1"),
        format!("{prefix}.ssvc_v2"),
    ];
    for path in fix_paths {
        value.dot_set(path.as_str(), json!({"type": "object"})).unwrap();
    }
    remove_datetime_formats(value);
}

/// Recursively searches for "format": "date-time" and removes this format.
fn remove_datetime_formats(value: &mut Value) {
    if let Value::Object(map) = value {
        if let Some(format) = map.get("format")
            && format.as_str() == Some("date-time")
        {
            // Remove the format property entirely
            map.remove("format");
        }

        // Recursively process all values in the object
        for (_, v) in map.iter_mut() {
            remove_datetime_formats(v);
        }
    } else if let Value::Array(arr) = value {
        for item in arr.iter_mut() {
            remove_datetime_formats(item);
        }
    }
}
