use std::{borrow::Cow, path::PathBuf};

use clap::CommandFactory;
use csaf_rs::{
    converter::{
        Diagnostics,
        Error::{self},
        Warning::{self},
    },
    json::JsonSource,
};

/// A converter for CSAF 2.0 to 2.1 documents
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ConverterArgs {
    /// Path to CSAF document to convert
    #[arg()]
    path: PathBuf,

    /// Path to the output file, stdout when omitted
    #[arg(short = 'o')]
    output: Option<PathBuf>,

    /// Allow (partial) processing and output of invalid CSAF documents
    #[arg(long)]
    allow_invalid: bool,
}

pub struct VerboseDiagnostics {
    error_count: u32,
}

impl Diagnostics for VerboseDiagnostics {
    fn error(&mut self, path: Cow<'static, str>, error: Error) {
        self.error_count += 1;
        eprintln!("Error: {error} ({path})");
    }

    fn has_errors(&self) -> bool {
        self.error_count > 0
    }

    fn warning(&mut self, path: Cow<'static, str>, warning: Warning) {
        eprintln!("Warning: {warning} ({path})");
    }
}

pub fn run_converter(args: ConverterArgs) -> Result<(), anyhow::Error> {
    let path = args.path.as_path();
    if !path.exists() {
        ConverterArgs::command().print_help()?;
        return Ok(());
    }

    let mut diag = VerboseDiagnostics { error_count: 0 };

    let raw: serde_json::Value = path.parse()?;
    let converted = super::convert(
        raw,
        &csaf_rs::converter::Settings::default(),
        &mut diag,
        args.allow_invalid,
    )?;
    let result = serde_json::to_string_pretty(&converted)?;

    if let Some(output) = args.output {
        std::fs::write(output, result)?;
    } else {
        println!("{result}");
    }

    Ok(())
}
