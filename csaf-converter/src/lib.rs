use core::fmt;

use anyhow::anyhow;
use csaf::csaf::raw::{HasParsed as _, WipDocument};

mod gh1066;
mod gh1067;

pub fn convert(
    input: &str,
    options: &ConversionOptions,
    diagnostics: &mut dyn Diagnostics,
) -> anyhow::Result<serde_json::Value> {
    let mut wip = load_csaf2_0(input)?;

    let mut conversions = CONVERSIONS.to_vec();
    conversions.sort_by_key(|&conversion| conversion.priority());
    for conversion in conversions {
        conversion.convert(&mut wip, options, diagnostics)
    }

    let output = wip.into_2_1();
    let schema_mismatch = output.get_parsed().is_err();
    if schema_mismatch {
        diagnostics.error(Error::Csaf21Mismatch)
    }

    Ok(output.get_json().clone())
}

static CONVERSIONS: &[&(dyn Convert + Sync)] = &[&gh1066::Conversion, &gh1067::Conversion];

#[derive(Default)]
pub struct ConversionOptions {
    // TODO
}

trait Convert {
    // TODO is the transforms are completely independent and can be executed in any order then
    // this can be removed
    fn priority(&self) -> u8 {
        100
    }
    // TODO do we need "priority" metadata to ensure some conversions are performed in
    // a certain order?
    fn convert(&self, wip: &mut WipDocument, opts: &ConversionOptions, diagnostics: &mut dyn Diagnostics);
}

pub trait Diagnostics {
    fn warning(&mut self, warning: Warning);
    fn error(&mut self, error: Error);
}

#[derive(Debug, PartialEq)]
pub enum Warning {
    NoTlpLabel,
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Warning::NoTlpLabel => "no TLP label was given; default value was used",
        };
        f.write_str(s)
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    Csaf21Mismatch,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Error::Csaf21Mismatch => "output does not conform to the CSAF 2.1 schema",
        };
        f.write_str(s)
    }
}

fn load_csaf2_0(input: &str) -> anyhow::Result<WipDocument> {
    csaf::csaf2_0::loader::load_document(input)?
        .into_wip()
        .map_err(|s| anyhow!("{s}"))
}

#[derive(Default)]
pub struct DiagnosticsSpy {
    warnings: Vec<Warning>,
    errors: Vec<Error>,
}

impl DiagnosticsSpy {
    #[cfg(test)]
    pub fn contains_warning(&self, warning: &Warning) -> bool {
        self.warnings.contains(warning)
    }

    pub fn contains_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn warnings(&self) -> &[Warning] {
        &self.warnings
    }

    pub fn errors(&self) -> &[Error] {
        &self.errors
    }
}

impl Diagnostics for DiagnosticsSpy {
    fn warning(&mut self, warning: Warning) {
        self.warnings.push(warning)
    }

    fn error(&mut self, error: Error) {
        self.errors.push(error)
    }
}
