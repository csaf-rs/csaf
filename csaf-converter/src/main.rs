use std::{env, fs};

use csaf_converter::{ConversionOptions, DiagnosticsSpy};

fn main() -> anyhow::Result<()> {
    let arg = env::args().nth(1).expect("expected exactly one argument");
    let contents = fs::read_to_string(arg)?;

    // TODO get options from CL arguments
    let opts = ConversionOptions::default();
    // TODO proper diagnostic reporting
    let mut diagnostics = DiagnosticsSpy::default();
    let output = csaf_converter::convert(&contents, &opts, &mut diagnostics)?;

    println!("{}", serde_json::to_string_pretty(&output)?);

    Ok(())
}
