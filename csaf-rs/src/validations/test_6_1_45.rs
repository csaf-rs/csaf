use crate::csaf_traits::{
    CsafTrait, DistributionTrait, DocumentTrait, RevisionTrait, TlpTrait, TrackingTrait, VulnerabilityTrait,
};
use crate::csaf2_1::schema::{DocumentStatus, LabelOfTlp};
use crate::validation::ValidationError;
use chrono::{DateTime, FixedOffset};

pub fn test_6_1_45_inconsistent_disclosure_date(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    // Only check if document is TLP:CLEAR and status is final or interim
    let document = doc.get_document();
    let status = document.get_tracking().get_status();

    if status != DocumentStatus::Final && status != DocumentStatus::Interim {
        return Ok(());
    }

    let is_tlp_clear = match document.get_distribution_21() {
        Ok(distribution) => match distribution.get_tlp_21() {
            Ok(tlp) => tlp.get_label() == LabelOfTlp::Clear,
            Err(_) => false,
        },
        Err(_) => false,
    };

    if !is_tlp_clear {
        return Ok(());
    }

    // Get the newest revision history date
    let mut newest_revision_date: Option<DateTime<FixedOffset>> = None;
    let revision_history = document.get_tracking().get_revision_history();
    for (i_rev, rev) in revision_history.iter().enumerate() {
        chrono::DateTime::parse_from_rfc3339(rev.get_date())
            .map(|rev_datetime| {
                println!(
                    "rev_datetime: {:?}, newest_revision_date: {:?}",
                    rev_datetime, newest_revision_date
                );
                newest_revision_date = match newest_revision_date {
                    None => Some(rev_datetime),
                    Some(prev_max) => Some(prev_max.max(rev_datetime)),
                }
            })
            .map_err(|_| {
                vec![ValidationError {
                    message: format!("Invalid date format in revision history: {}", rev.get_date()),
                    instance_path: format!("/document/tracking/revision_history/{}", i_rev),
                }]
            })?;
    }

    if let Some(newest_date) = newest_revision_date {
        // Check each vulnerability's disclosure date
        for (i_v, v) in doc.get_vulnerabilities().iter().enumerate() {
            if let Some(disclosure_date) = v.get_disclosure_date() {
                match chrono::DateTime::parse_from_rfc3339(disclosure_date) {
                    Ok(disclosure_datetime) => {
                        println!(
                            "disclosure_datetime: {:?}, newest_date: {:?}",
                            disclosure_datetime, newest_date
                        );
                        if disclosure_datetime > newest_date {
                            return Err(vec![ValidationError {
                                message: "Disclosure date must not be later than the newest revision history date for TLP:CLEAR documents with final or interim status".to_string(),
                                instance_path: format!("/vulnerabilities/{}/discovery_date", i_v),
                            }]);
                        }
                    },
                    Err(_) => {
                        return Err(vec![ValidationError {
                            message: format!("Invalid disclosure date format: {}", disclosure_date),
                            instance_path: format!("/vulnerabilities/{}/discovery_date", i_v),
                        }]);
                    },
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_helper::run_csaf21_tests;
    use crate::validation::ValidationError;
    use crate::validations::test_6_1_45::test_6_1_45_inconsistent_disclosure_date;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_45() {
        let expected_error = ValidationError {
            message: "Disclosure date must not be later than the newest revision history date for TLP:CLEAR documents with final or interim status".to_string(),
            instance_path: "/vulnerabilities/0/discovery_date".to_string(),
        };

        run_csaf21_tests(
            "45",
            test_6_1_45_inconsistent_disclosure_date,
            HashMap::from([
                ("01", vec![expected_error.clone()]),
                ("02", vec![expected_error.clone()]),
                ("03", vec![expected_error.clone()]),
            ]),
        );
    }
}
