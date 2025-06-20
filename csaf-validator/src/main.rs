use std::str::FromStr;
use anyhow::{bail, Result};
use csaf_rs::csaf::csaf2_0::loader::load_document as load_document_2_0;
use csaf_rs::csaf::csaf2_1::loader::load_document as load_document_2_1;
use csaf_rs::csaf::validation::{validate_by_preset, validate_by_test, Validatable, ValidationPreset};
use clap::Parser;

/// A validator for CSAF documents
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    path: String,

    /// Version of CSAF to use
    #[arg(short, long, default_value = "2.0")]
    csaf_version: String,

    /// The validation preset to use
    #[arg(short, long, default_value = "basic")]
    preset: String,

    /// Run only the selected tests, may be specified multiple times
    #[arg(short, long, action = clap::ArgAction::Append)]
    test_id: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.csaf_version.as_str() {
        "2.0" => {
            process_document(load_document_2_0(args.path.as_str())?, &args)
        }
        "2.1" => {
            process_document(load_document_2_1(args.path.as_str())?, &args)
        }
        _ => bail!(format!("Invalid CSAF version: {}", args.csaf_version)),
    }
}

fn process_document<T>(document: T, args: &Args) -> Result<()>
where
    T: Validatable<T>,
{
    if !args.test_id.is_empty() {
        for test_id in &args.test_id {
            println!("\nExecuting Test {}... ", test_id);
            validate_by_test(&document, test_id.as_str());
        }
        Ok(())
    } else {
        let preset = match ValidationPreset::from_str(args.preset.as_str()) {
            Ok(preset) => preset,
            Err(_) => bail!(format!("Invalid validation preset: {}", args.preset)),
        };
        validate_by_preset(&document, preset);
        Ok(())
    }
}
