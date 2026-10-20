use super::{Convert, Diagnostics, Settings};
use crate::schema::loose;

pub struct ConvertReleaseDateToDisclosureDate;

impl Convert for ConvertReleaseDateToDisclosureDate {
    fn convert(&self, doc: &mut loose::Document, _settings: &Settings, _diagnostics: &mut dyn Diagnostics) {
        // if there are any vulnerabilities
        let Some(vulnerabilities) = doc
            .v20_view_mut()
            .vulnerabilities_mut()
            .and_then(|vulnerabilities| vulnerabilities.as_slice_mut().ok())
        else {
            return;
        };

        for vulnerability in vulnerabilities {
            // get object
            let Ok(object) = vulnerability.caution_as_object_mut() else {
                continue;
            };

            // remove release date if present
            if let Some(release_date) = object.remove("release_date") {
                // add disclosure date with same value
                object.insert("disclosure_date".to_string(), release_date);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::converter::tests::NoDiagnostics;

    use super::*;

    #[test]
    fn converts_release_date_to_disclosure_date() {
        let input = json!({
            "vulnerabilities": [
                {
                    "release_date": "2025-03-11T15:12:00.000Z"
                },
                {},
                {
                    "release_date": "2025-03-11T15:12:00.000Z"
                },
            ]
        });
        let mut doc = loose::Document::new(input);

        ConvertReleaseDateToDisclosureDate.convert(&mut doc, &Settings::default(), &mut NoDiagnostics);

        assert_eq!(
            json!({
                "vulnerabilities": [
                    {
                        "disclosure_date": "2025-03-11T15:12:00.000Z"
                    },
                    {},
                    {
                        "disclosure_date": "2025-03-11T15:12:00.000Z"
                    },
                ]
            }),
            doc.into_raw()
        );
    }
}
