mod build_errors;
mod language_tags;
mod schema;
mod testcases;
mod utils;

use crate::build_errors::BuildError;
use crate::language_tags::generate_language_tags;
use crate::schema::build_schema;
use crate::schema::config::{get_schemas, get_testcases_schemas};
use crate::testcases::{generate_testcases, get_testcase_configs};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Whether to include generation of test schema definitions
    #[arg(short, long, default_value_t = false)]
    include_test_schema: bool,

    /// Whether to include generation of test definitions
    #[arg(short, long, default_value_t = false)]
    create_test_definitions: bool,

    /// Whether to include generation of language tags
    #[arg(short = 'l', long, default_value_t = false)]
    generate_language_tags: bool,

    /// Target folder for generated code, ../csaf-rs by default
    #[arg(short, long, default_value = "../csaf-rs")]
    target_folder: String,
}

fn main() -> Result<(), BuildError> {
    let args = Args::parse();

    // Execute all listed schema builds
    for schema in &get_schemas() {
        build_schema(schema, &args.target_folder)?;
    }

    if args.include_test_schema {
        for schema in &get_testcases_schemas() {
            build_schema(schema, &args.target_folder)?;
        }
    }

    if args.create_test_definitions {
        for config in &get_testcase_configs() {
            generate_testcases(config, &args.target_folder)?;
        }
    }

    if args.generate_language_tags {
        generate_language_tags(&args.target_folder)?;
    }

    Ok(())
}
