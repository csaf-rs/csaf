use crate::schema::loose;

use super::{Convert, Diagnostics, Settings};
pub struct ConvertReleaseDateToDisclosureDate;

impl Convert for ConvertReleaseDateToDisclosureDate {
    fn convert(&self, doc: &mut loose::Document, _settings: &Settings, _diagnostics: &mut dyn Diagnostics) {
        // get the vulns from the CSAF 2.0 view, skip if there aren't any
        let Some(vulnerabilities_20) = doc
            .v20_view()
            .vulnerabilities()
            .and_then(|vulnerabilities| vulnerabilities.as_slice().ok())
        else {
            return;
        };

        // collect the optional release dates from the vulnerabilities
        let release_dates = vulnerabilities_20
            .iter()
            .map(|vulnerability| {
                vulnerability
                    .release_date()
                    .and_then(|release_date| release_date.get_str().ok())
                    .map(str::to_owned)
            })
            .collect::<Vec<_>>();

        // get mut vulns from the CSAF 2.1 view, this should always succeed if the CSAF 2.0 view had vulns
        let Some(vulnerabilities_21) = doc
            .v21_view_mut()
            .vulnerabilities_mut()
            .and_then(|vulnerabilities| vulnerabilities.as_slice_mut().ok())
        else {
            unreachable!("Vulnerabilities should be present in CSAF 2.1 if they were present in CSAF 2.0");
        };

        // sanity check, this should not happen
        if vulnerabilities_21.len() != release_dates.len() {
            unreachable!("Vulnerabilities CSAF 2.1 and extracted release dates should have the same length");
        }

        // zip the
        for (vulnerability, release_date) in vulnerabilities_21.iter_mut().zip(release_dates) {
            if let Some(release_date) = release_date {
                vulnerability
                    .caution_get_or_insert_disclosure_date()
                    .set_str(release_date);
                if let Ok(object) = vulnerability.caution_as_object_mut() {
                    object.remove("release_date");
                }
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
