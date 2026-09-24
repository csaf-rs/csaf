use csaf::csaf::raw::WipDocument;
use csaf::csaf_traits::{CsafTrait as _, DistributionTrait as _, DocumentTrait as _};
use csaf::schema::csaf2_0;
use csaf::schema::csaf2_1;

use crate::{ConversionOptions, Convert, Diagnostics, Warning};

pub struct Conversion;

impl Convert for Conversion {
    fn convert(&self, wip: &mut WipDocument, _opts: &ConversionOptions, diagnostics: &mut dyn Diagnostics) {
        if let Some(distribution) = wip.parsed().get_document().get_distribution_20() {
            if let Some(tlp) = distribution.get_tlp_20() {
                if matches!(tlp.label, csaf2_0::schema::LabelOfTlp::White) {
                    set_clear_label(wip);
                }
                return;
            }
        }

        diagnostics.warning(Warning::NoTlpLabel);
        set_clear_label(wip);
    }
}

fn set_clear_label(wip: &mut WipDocument) {
    wip.raw()["document"]["distribution"]["tlp"]["label"] = csaf2_1::schema::LabelOfTlp::Clear.to_string().into();
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    use crate::DiagnosticsSpy;

    #[test]
    fn has_tlp_label() -> anyhow::Result<()> {
        let mut wip = crate::load_csaf2_0(include_str!("case1-input.json"))?;
        let output: serde_json::Value = serde_json::from_str(include_str!("case1-output.json"))?;

        Conversion.convert(&mut wip, &Default::default(), &mut DiagnosticsSpy::default());
        assert_eq!(output, *wip.raw());
        Ok(())
    }

    #[test]
    fn no_tlp_label() -> anyhow::Result<()> {
        let mut wip = crate::load_csaf2_0(include_str!("case2-input.json"))?;
        let output: serde_json::Value = serde_json::from_str(include_str!("case2-output.json"))?;

        let mut warnings = DiagnosticsSpy::default();
        Conversion.convert(&mut wip, &Default::default(), &mut warnings);
        assert_eq!(output, *wip.raw());
        assert!(warnings.contains_warning(&Warning::NoTlpLabel));
        Ok(())
    }

    // TODO SHOULD case when a certain `ConversionOpts` is set
}
