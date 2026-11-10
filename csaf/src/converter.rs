use std::borrow::Cow;

pub use csaf_converter::{Diagnostics, Error, Settings, Warning, convert};

#[derive(Default)]
pub struct InMemoryDiagnosticCollector {
    pub errors: Vec<(Cow<'static, str>, Error)>,
    pub warnings: Vec<(Cow<'static, str>, Warning)>,
}
impl InMemoryDiagnosticCollector {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Diagnostics for InMemoryDiagnosticCollector {
    fn error(&mut self, path: Cow<'static, str>, error: Error) {
        self.errors.push((path, error));
    }

    fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn warning(&mut self, path: Cow<'static, str>, warning: Warning) {
        self.warnings.push((path, warning));
    }
}
