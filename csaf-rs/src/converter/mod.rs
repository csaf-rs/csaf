use crate::schema::csaf2x;

mod update_csaf_version;
mod update_tlp_label;

pub fn convert(raw: serde_json::Value, settings: &Settings, diagnostics: &mut dyn Diagnostics) -> csaf2x::Document {
    let mut doc = csaf2x::Document::new(raw);

    for conversion in CONVERSIONS {
        conversion.convert(&mut doc, settings, diagnostics)
    }

    doc
}

const CONVERSIONS: &[&dyn Convert] = &[
    &update_csaf_version::UpdateCsafVersion,
    &update_tlp_label::UpdateTlpLabel,
];

/// Incremental v2.0 to v2.1 conversion
pub trait Convert {
    fn convert(&self, scope: &mut csaf2x::Document, settings: &Settings, diagnostics: &mut dyn Diagnostics);
}

pub trait Diagnostics {
    fn error(&mut self, error: Error);
    fn warning(&mut self, warning: Warning);
}

#[derive(Default)]
pub struct Settings {
    // TODO
}

#[derive(Debug, PartialEq)]
pub enum Warning {
    NoTlpLabel,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    Todo,
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct NoDiagnostics;

    impl Diagnostics for NoDiagnostics {
        fn error(&mut self, error: Error) {
            panic!("expected no diagnostics; got {error:?}")
        }

        fn warning(&mut self, warning: Warning) {
            panic!("expected no diagnostics; got {warning:?}")
        }
    }

    #[derive(Default)]
    pub struct DiagnosticsSpy {
        errors: Vec<Error>,
        warnings: Vec<Warning>,
    }

    impl DiagnosticsSpy {
        pub fn errors(&self) -> &[Error] {
            &self.errors
        }

        pub fn warnings(&self) -> &[Warning] {
            &self.warnings
        }
    }

    impl Diagnostics for DiagnosticsSpy {
        fn error(&mut self, error: Error) {
            self.errors.push(error)
        }

        fn warning(&mut self, warning: Warning) {
            self.warnings.push(warning)
        }
    }
}
