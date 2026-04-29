mod build_errors;
mod language_tags;
mod schema;
mod testcases;
mod utils;
mod validation_schemas;

use crate::build_errors::BuildError;
use crate::language_tags::generate_language_tags;
use crate::schema::build_schema;
use crate::schema::config::{get_schemas, get_testcases_schemas};
use crate::testcases::{generate_testcases, get_testcase_configs};
use crate::validation_schemas::generate_validation_schemas;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Generate schema definitions
    #[arg(long, default_value_t = false)]
    schema: bool,

    /// Generate test schema definitions
    #[arg(long, default_value_t = false)]
    test_schema: bool,

    /// Generate test definitions
    #[arg(long, default_value_t = false)]
    test_definitions: bool,

    /// Generate language tags
    #[arg(long, default_value_t = false)]
    language_tags: bool,

    /// Generate validation schemas
    #[arg(long, default_value_t = false)]
    validation_schemas: bool,

    /// Target folder for generated code, ../csaf-rs by default
    #[arg(long, default_value = "../csaf-rs")]
    target_folder: String,
}

fn main() -> Result<(), BuildError> {
    let args = Args::parse();

    // If no specific generation is requested, run all
    let run_all = !args.schema
        && !args.test_schema
        && !args.test_definitions
        && !args.language_tags
        && !args.validation_schemas;

    if run_all || args.schema {
        for schema in &get_schemas() {
            build_schema(schema, &args.target_folder)?;
        }
    }

    if run_all || args.test_schema {
        for schema in &get_testcases_schemas() {
            build_schema(schema, &args.target_folder)?;
        }
    }

    if run_all || args.test_definitions {
        for config in &get_testcase_configs() {
            generate_testcases(config, &args.target_folder)?;
        }
    }

    if run_all || args.language_tags {
        generate_language_tags(&args.target_folder)?;
    }

    if run_all || args.validation_schemas {
        generate_validation_schemas(&args.target_folder)?;
    }

    Ok(())
}
