use clap::Parser;
use csaf::{
    converter::{Diagnostics, Error, Warning},
    json::JsonSource,
};

/// A converter for CSAF 2.0 to 2.1 documents
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to CSAF document to convert
    #[arg()]
    path: String,

    /// Path to the output file, stdout when omitted
    #[arg(short = 'o')]
    output: Option<String>,
}

pub struct VerboseDiagnostics;
impl Diagnostics for VerboseDiagnostics {
    fn error(&mut self, error: Error) {
        eprintln!("Error: {error:?}");
    }

    fn warning(&mut self, warning: Warning) {
        eprintln!("Warning: {warning:?}");
    }
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let raw = std::path::Path::new(&args.path).parse()?;
    let mut diag = VerboseDiagnostics {};
    let result = csaf::converter::convert(raw, &csaf::converter::Settings::default(), &mut diag);
    let result = serde_json::to_string_pretty(&result.into_raw())?;
    if let Some(path) = args.output {
        std::fs::write(path.clone(), result)?;
        println!("Wrote converted file to {path}.");
    } else {
        println!("{}", result);
    }
    Ok(())
}
