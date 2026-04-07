use crate::csaf::types::csaf_datetime::CsafDateTime::Valid;
use crate::csaf::types::csaf_datetime::ValidCsafDateTime;
use crate::csaf_traits::{
    CsafTrait, DocumentTrait, FirstKnownExploitationDatesTrait, RevisionHistorySortable, TrackingTrait,
    VulnerabilityTrait, WithDate,
};
use crate::schema::csaf2_1::schema::DocumentStatus;
use crate::validation::ValidationError;
use std::fmt;

enum DateProperty {
    Date,
    ExploitationDate,
}

impl fmt::Display for DateProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DateProperty::Date => write!(f, "date"),
            DateProperty::ExploitationDate => write!(f, "exploitation_date"),
        }
    }
}

fn create_date_too_new_error(
    date: &ValidCsafDateTime,
    newest_revision_date: &ValidCsafDateTime,
    v_i: usize,
    f_i: usize,
    property: DateProperty,
) -> ValidationError {
    ValidationError {
        message: format!(
            "The {property} '{date}' of the first known exploitation date is newer than the newest revision date ({newest_revision_date})"
        ),
        instance_path: format!("/vulnerabilities/{v_i}/first_known_exploitation_dates/{f_i}/{property}"),
    }
}

/// 6.1.52 Inconsistent First Known Exploitation Dates
///
/// For each first known exploitation dates item, it is tested that the values of its `date` and
/// `exploitation_date` properties are both earlier than or equal to the `date` of the newest item
/// of the `revision_history` (taking timezones into consideration) if the document 
/// status is `final` or `interim`.
pub fn test_6_1_52_inconsistent_first_known_exploitation_dates(
    doc: &impl CsafTrait,
) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();
    let tracking = document.get_tracking();
    let status = tracking.get_status();

    // Check if the document status is "final" or "interim"
    if status != DocumentStatus::Final && status != DocumentStatus::Interim {
        return Ok(());
    }

    // Get sorted revision history and find the newest entry
    let mut revision_history = tracking.get_revision_history_tuples();
    revision_history.inplace_sort_by_date_then_number();

    let newest_revision = match revision_history.last() {
        Some(rev) => rev,
        None => return Ok(()), // TODO this should be a #409 precondition failed
    };

    // Check each vulnerability's first known exploitation dates
    let mut errors: Option<Vec<ValidationError>> = None;
    // TODO: #409 no data
    for (v_i, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(first_known_exploitation_dates) = vulnerability.get_first_known_exploitation_dates() {
            for (f_i, first_known_exploitation_date) in first_known_exploitation_dates.iter().enumerate() {
                if let Valid(date) = first_known_exploitation_date.get_date() {
                    if date > newest_revision.valid_date {
                        errors.get_or_insert_default().push(create_date_too_new_error(
                            &date,
                            &newest_revision.valid_date,
                            v_i,
                            f_i,
                            DateProperty::Date,
                        ));
                    }
                } else {
                    // TODO: This will be a NonDeterminable (#409) later
                }

                if let Valid(exploitation_date) = first_known_exploitation_date.get_exploitation_date() {
                    if exploitation_date > newest_revision.valid_date {
                        errors.get_or_insert_default().push(create_date_too_new_error(
                            &exploitation_date,
                            &newest_revision.valid_date,
                            v_i,
                            f_i,
                            DateProperty::ExploitationDate,
                        ));
                    }
                } else {
                    // TODO: This will be a NonDeterminable (#409) later
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_52,
    test_6_1_52_inconsistent_first_known_exploitation_dates
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use std::str::FromStr;

    #[test]
    fn test_test_6_1_52() {
        let case_01_newest_revision_date = ValidCsafDateTime::from_str("2024-01-24T10:00:00.000Z").unwrap();
        let case_01_date_and_exploit_date_after_newest_rev_date = Err(vec![
            create_date_too_new_error(
                &ValidCsafDateTime::from_str("2024-01-24T13:00:00.000Z").unwrap(),
                &case_01_newest_revision_date,
                0,
                0,
                DateProperty::Date,
            ),
            create_date_too_new_error(
                &ValidCsafDateTime::from_str("2024-01-24T12:34:56.789Z").unwrap(),
                &case_01_newest_revision_date,
                0,
                0,
                DateProperty::ExploitationDate,
            ),
        ]);

        let case_02_newest_revision_date = ValidCsafDateTime::from_str("2024-03-26T09:59:59.999998-07:00").unwrap();
        let case_02_multiple_vulns_multiple_first_exploit_dates_also_timezones = Err(vec![
            create_date_too_new_error(
                &ValidCsafDateTime::from_str("2024-03-27T10:00:00.000+17:00").unwrap(),
                &case_02_newest_revision_date,
                1,
                0,
                DateProperty::Date,
            ),
            create_date_too_new_error(
                &ValidCsafDateTime::from_str("2024-03-25T23:00:00.000-18:00").unwrap(),
                &case_02_newest_revision_date,
                1,
                0,
                DateProperty::ExploitationDate,
            ),
            create_date_too_new_error(
                &ValidCsafDateTime::from_str("2024-03-25T23:29:59.999999-17:30").unwrap(),
                &case_02_newest_revision_date,
                1,
                1,
                DateProperty::Date,
            ),
            create_date_too_new_error(
                &ValidCsafDateTime::from_str("2024-03-27T07:59:59.999999+15:00").unwrap(),
                &case_02_newest_revision_date,
                1,
                1,
                DateProperty::ExploitationDate,
            ),
        ]);

        // Case 11: dates are before or equal to newest revision date
        // Case 12: dates with timezones are to before or equal to newest revision date

        TESTS_2_1.test_6_1_52.expect(
            case_01_date_and_exploit_date_after_newest_rev_date,
            case_02_multiple_vulns_multiple_first_exploit_dates_also_timezones,
            Ok(()),
            Ok(()),
        );
    }
}
