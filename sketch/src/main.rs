use std::io::Write;
use std::{array, env, fs, io};

use anyhow::bail;
use sketch::{convert, validate};

fn main() -> anyhow::Result<()> {
    let mut args = env::args().skip(1);
    let [Some(command), Some(input), None] = array::from_fn(|_| args.next()) else {
        bail!("expected exactly 2 arguments")
    };

    match command.as_str() {
        "convert" => convert(&input)?,
        "validate" => validate(&input)?,
        _ => bail!("unknown command: {command}"),
    }

    Ok(())
}

fn convert(path: &str) -> anyhow::Result<()> {
    let input = fs::read(path)?;
    let raw = serde_json::from_slice(&input)?;
    // TODO get settings from CLI
    let settings = convert::Settings::default();
    let output = convert::convert(raw, &settings, &mut ConvertDiagnostics).into_raw();

    // TODO check against CSAF 2.1 schema
    // TODO run CSAF 2.1 tests

    println!("{}", serde_json::to_string_pretty(&output)?);

    Ok(())
}

fn validate(path: &str) -> anyhow::Result<()> {
    let input = fs::read(path)?;
    let raw = serde_json::from_slice(&input)?;
    // TODO get settings from CLI
    // TODO run a subset of tests as per CL flags
    validate::validate(raw, None, &mut ValidatorDiagnostics);

    Ok(())
}

pub struct ConvertDiagnostics;

impl convert::Diagnostics for ConvertDiagnostics {
    fn error(&mut self, error: convert::Error) {
        eprintln!("error: {error:?}")
    }

    fn warning(&mut self, warning: convert::Warning) {
        eprintln!("warning: {warning:?}")
    }
}

pub struct ValidatorDiagnostics;

impl validate::Diagnostics for ValidatorDiagnostics {
    fn error(&mut self, error: validate::Error) {
        eprintln!("error: {error:?}")
    }

    fn warning(&mut self, warning: validate::Warning) {
        eprintln!("warning: {warning:?}")
    }
}
