use anyhow::{anyhow, bail, Result};
use csaf_lib::csaf::csaf2_0::loader::load_document as load_document_2_0;
use csaf_lib::csaf::csaf2_1::loader::load_document as load_document_2_1;
use csaf_lib::csaf::validation::Validate;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let path = match args.get(1) {
        None => {
            return Err(anyhow!("Please specify a file to validate"))
        }
        Some(v) => v,
    };

    let version = args.get(2).cloned().unwrap_or("2.0".to_string());
    let v: Box<dyn Validate> = match version.as_str() {
        "2.0" => {
            Box::new(load_document_2_0(path)?)
        }
        "2.1" => {
            Box::new(load_document_2_1(path)?)
        }
        _ => bail!("invalid version")
    };

    v.validate();

    Ok(())
}
