use crate::csaf2x;

mod t6_1_12;

macro_rules! validate {
    ($doc:expr, $version:expr, $diagnostics:expr; $($test:path,)+) => {
        $(
            execute(&$test, $version, &$doc, $diagnostics);
        )+
    }
}

pub fn validate(raw: serde_json::Value, stated_version: Option<StatedVersion>, diagnostics: &mut dyn Diagnostics) {
    let doc = csaf2x::Csaf::new(raw);

    let Some(version) = stated_version.or_else(|| {
        // no stated version; get it from the document itself
        let csaf_version = doc.v2x_document()?.v2x_csaf_version()?;
        let csaf_version = if csaf_version.v20_get().is_ok() {
            StatedVersion::V20
        } else if csaf_version.v20_get().is_ok() {
            StatedVersion::V21
        } else {
            return None;
        };

        Some(csaf_version)
    }) else {
        diagnostics.error(Error::UnknownCsafVersion);
        return;
    };

    validate!(&doc, version, diagnostics;
              t6_1_12::T6_1_12,
    );
}

/// A single CSAF test
pub trait Validate {
    /// Which CSAF version, or versions, does this validation apply to?
    fn applies_to(&self) -> VersionSelector;

    /// Validation logic
    ///
    /// Only runs if the stated version matches the selector returned by `applies_to`
    fn validate(&self, doc: &csaf2x::Csaf, version: StatedVersion, diagnostics: &mut dyn Diagnostics);
}

fn execute(validation: &impl Validate, version: StatedVersion, doc: &csaf2x::Csaf, diagnostics: &mut dyn Diagnostics) {
    if !matches!(
        (validation.applies_to(), version),
        (VersionSelector::V20 | VersionSelector::V2x, StatedVersion::V20)
            | (VersionSelector::V21 | VersionSelector::V2x, StatedVersion::V21)
    ) {
        // validation does not apply
        return;
    }

    validation.validate(doc, version, diagnostics);
}

/// Stated version
///
/// Either specified via the command-line interface or extracted from the CSAF document
#[derive(Clone, Copy)]
pub enum StatedVersion {
    V20,
    V21,
}

#[derive(Clone, Copy)]
pub enum VersionSelector {
    /// 2.0 Only
    V20,
    /// 2.1 Only
    V21,
    /// Both 2.0 and 2.1
    V2x,
}

pub trait Diagnostics {
    fn error(&mut self, error: Error);
    fn warning(&mut self, warning: Warning);
}

#[derive(Debug, PartialEq)]
pub enum Warning {
    Todo,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    UnknownCsafVersion,
    InvalidLanguageCode,
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
