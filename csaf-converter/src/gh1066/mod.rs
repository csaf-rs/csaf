use csaf::csaf::raw::WipDocument;
use csaf::csaf_traits::{CsafTrait as _, CsafVersion, DocumentTrait as _};

use crate::{ConversionOptions, Convert, Diagnostics};

pub struct Conversion;

impl Convert for Conversion {
    fn convert(&self, wip: &mut WipDocument, _opts: &ConversionOptions, _diagnostics: &mut dyn Diagnostics) {
        // sanity check
        debug_assert_eq!(CsafVersion::X20, wip.parsed().get_document().get_csaf_version());

        wip.raw()["document"]["csaf_version"] = "2.1".into();
    }
}

#[cfg(test)]
mod tests {
    use crate::DiagnosticsSpy;

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let mut wip = crate::load_csaf2_0(include_str!("input.json"))?;
        let output: serde_json::Value = serde_json::from_str(include_str!("output.json"))?;

        Conversion.convert(&mut wip, &Default::default(), &mut DiagnosticsSpy::default());
        assert_eq!(output, *wip.raw());
        Ok(())
    }
}
