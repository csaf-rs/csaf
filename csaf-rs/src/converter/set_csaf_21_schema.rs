use crate::schema::{csaf2_1, loose};

use super::{Convert, Diagnostics, Settings};

pub struct SetCsaf21Schema;

impl Convert for SetCsaf21Schema {
    fn convert(&self, doc: &mut loose::Document, _settings: &Settings, _diagnostics: &mut dyn Diagnostics) {
        doc.v21_view_mut()
            .caution_get_or_insert_schema()
            .set_enum(csaf2_1::schema::JsonSchema::HttpsDocsOasisOpenOrgCsafCsafV21SchemaCsafJson);
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::converter::tests::NoDiagnostics;

    use super::*;

    #[test]
    fn converter_sets_csaf_21_schema() {
        let mut doc = loose::Document::new(json!({ "document": {} }));

        SetCsaf21Schema.convert(&mut doc, &Settings::default(), &mut NoDiagnostics);

        assert_eq!(
            json!({
                "$schema": "https://docs.oasis-open.org/csaf/csaf/v2.1/schema/csaf.json",
                "document": {}
            }),
            doc.into_raw()
        );
    }
}
