use anyhow::Result;
use clap::Parser;
use csaf_converter::cli::{ConverterArgs, run_converter};

fn main() -> Result<(), anyhow::Error> {
    let args = ConverterArgs::parse();
    run_converter(args)
}
