use anyhow::Result;
use clap::Parser;
use csaf_validator::cli::{ValidatorArgs, run_validator};

fn main() -> Result<(), anyhow::Error> {
    let args = ValidatorArgs::parse();
    run_validator(args)
}
