use crate::{csaf21, csaf2x};

use super::{Convert, Diagnostics, Settings};

// csaf-rs/csaf#1066
pub struct UpdateCsafVersion;

impl Convert for UpdateCsafVersion {
    type Scope = csaf2x::CsafVersion;

    fn scope<'doc>(&self, doc: &'doc mut crate::csaf2x::Csaf) -> Option<&'doc mut Self::Scope> {
        doc.v2x_document_mut()?.v2x_csaf_version_mut()
    }

    fn convert(&self, csaf_version: &mut Self::Scope, _settings: &Settings, _diagnostics: &mut dyn Diagnostics) {
        if csaf_version.v20_get().is_err() {
            // XXX error/warning? invalid value as per CSAF 2.0 schema
        }

        csaf_version.v21_set(csaf21::CsafVersion::X21);
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::convert::{self, tests::NoDiagnostics};

    use super::*;

    #[test]
    fn it_works() -> serde_json::Result<()> {
        let input = json!({
            "document": {
                "csaf_version": "2.0",
            }
        });
        let output = json!({
            "document": {
                "csaf_version": "2.1",
            }
        });
        let mut doc = csaf2x::Csaf::new(input);
        convert::execute(&UpdateCsafVersion, &mut doc, &Settings::default(), &mut NoDiagnostics);
        assert_eq!(output, doc.into_raw());

        Ok(())
    }

    #[test]
    fn no_scope() -> serde_json::Result<()> {
        let input = json!({
            "document": {}
        });
        let output = input.clone();
        let mut doc = csaf2x::Csaf::new(input);
        convert::execute(&UpdateCsafVersion, &mut doc, &Settings::default(), &mut NoDiagnostics);
        assert_eq!(output, doc.into_raw());

        Ok(())
    }
}
