use anyhow::{anyhow, bail, Result};
use csaf_lib::csaf::csaf2_0::loader::load_document as load_document_2_0;
use csaf_lib::csaf::csaf2_1::loader::load_document as load_document_2_1;
use csaf_lib::csaf::validation::{validate_by_profile, Validatable, ValidationProfile};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let path = match args.get(1) {
        None => return Err(anyhow!("Please specify a file to validate")),
        Some(v) => v,
    };

    let profile = ValidationProfile::Basic;

    // TODO: it would be nice to return the validatable from this match, but this is beyond my
    //  rust generics knowledge, so a little bit of duplicate code here
    let version = args.get(2).cloned().unwrap_or("2.0".to_string());
    let result = match version.as_str() {
        "2.0" => {
            validate_by_profile(load_document_2_0(path)?, profile)
        }
        "2.1" => {
            validate_by_profile(load_document_2_1(path)?, profile)
        }
        _ => bail!("invalid version"),
    };

    Ok(result)
}
