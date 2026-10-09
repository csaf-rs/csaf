use crate::csaf2x;

mod update_csaf_version;
mod update_tlp_label;

macro_rules! convert {
    ($doc:expr, $settings:expr, $diagnostics:expr; $($convert:path,)+) => {
        $(
            execute(&$convert, &mut $doc, $settings, $diagnostics);
        )+
    }
}

pub fn convert(raw: serde_json::Value, settings: &Settings, diagnostics: &mut dyn Diagnostics) -> csaf2x::Csaf {
    let mut doc = csaf2x::Csaf::new(raw);

    convert!(&mut doc, settings, diagnostics;
             update_tlp_label::UpdateTlpLabel,
             update_csaf_version::UpdateCsafVersion,
    );

    doc
}

/// Incremental v2.0 to v2.1 conversion
pub trait Convert {
    type Scope;

    /// Reduces the scope of the operation to avoid unintendedly mutating unrelated parts
    fn scope<'doc>(&self, doc: &'doc mut csaf2x::Csaf) -> Option<&'doc mut Self::Scope>;

    fn convert(&self, scope: &mut Self::Scope, settings: &Settings, diagnostics: &mut dyn Diagnostics);
}

fn execute(convert: &impl Convert, doc: &mut csaf2x::Csaf, settings: &Settings, diagnostics: &mut dyn Diagnostics) {
    if let Some(scope) = convert.scope(doc) {
        convert.convert(scope, settings, diagnostics);
    }
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
