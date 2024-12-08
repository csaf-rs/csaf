use anyhow::{bail, Result};
use csaf_lib::csaf::csaf2_0::loader::load_document as load_document_2_0;
use csaf_lib::csaf::csaf2_1::loader::load_document as load_document_2_1;
use csaf_lib::csaf::validation::{validate_by_profile, validate_by_test, ValidationProfile};
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

    /// The profile to use
    #[arg(short, long, default_value = "basic")]
    profile: String,

    /// Run only the selected test
    #[arg(short, long)]
    only_test: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let profile = ValidationProfile::Basic;

    // TODO: it would be nice to return the validatable from this match, but this is beyond my
    //  rust generics knowledge, so a little bit of duplicate code here
    if let Some(test_id) = args.only_test {
        let result = match args.csaf_version.as_str() {
            "2.0" => {
                validate_by_test(&load_document_2_0(args.path.as_str())?, test_id.as_str())
            }
            "2.1" => {
                validate_by_test(&load_document_2_1(args.path.as_str())?, test_id.as_str())
            }
            _ => bail!("invalid version"),
        };

        Ok(result)
    } else {
        let result = match args.csaf_version.as_str() {
            "2.0" => {
                validate_by_profile(&load_document_2_0(args.path.as_str())?, profile)
            }
            "2.1" => {
                validate_by_profile(&load_document_2_1(args.path.as_str())?, profile)
            }
            _ => bail!("invalid version"),
        };

        Ok(result)
    }
}
