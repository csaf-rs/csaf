use crate::csaf::types::csaf_datetime::CsafDateTime::Valid;
use crate::csaf_traits::{CsafTrait, FirstKnownExploitationDatesTrait, VulnerabilityTrait, WithDate};
use crate::validation::ValidationError;

fn create_inconsistent_exploitation_date_error(
    exploitation_date: &str,
    date: &str,
    v_i: usize,
    f_i: usize,
) -> ValidationError {
    ValidationError {
        message: format!(
            "The exploitation date '{exploitation_date}' must not be later than the date '{date}' of a first known exploitation date"
        ),
        instance_path: format!("/vulnerabilities/{v_i}/first_known_exploitation_dates/{f_i}/exploitation_date"),
    }
}

/// 6.1.53 Inconsistent Exploitation Date
///
/// For each `/vulnerabilities[]/first_known_exploitation_dates[]`, it is tested that `exploitation_date`
/// is earlier or equal to the `date`.
pub fn test_6_1_53_inconsistent_exploitation_date(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;
    for (v_i, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(first_known_exploitation_dates) = vulnerability.get_first_known_exploitation_dates() {
            for (f_i, first_known_exploitation_date) in first_known_exploitation_dates.iter().enumerate() {
                if let Valid(exploitation_date) = first_known_exploitation_date.get_exploitation_date()
                    && let Valid(date) = first_known_exploitation_date.get_date()
                {
                    if exploitation_date > date {
                        errors
                            .get_or_insert_default()
                            .push(create_inconsistent_exploitation_date_error(
                                exploitation_date.get_raw_string(),
                                date.get_raw_string(),
                                v_i,
                                f_i,
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

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_53, test_6_1_53_inconsistent_exploitation_date);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_53() {
        let case_01_exp_date_after_date = Err(vec![create_inconsistent_exploitation_date_error(
            "2024-02-25T11:00:22.987Z",
            "2024-01-24T10:00:00.000Z",
            0,
            0,
        )]);

        // Case 02: 2 vulns, 3 first_known_exploitation_dates with timezones
        // in the second vuln both have exploitation_date after date
        let case_02_exp_date_after_date_timezones = Err(vec![
            create_inconsistent_exploitation_date_error(
                "2024-03-25T22:30:00.000-18:00",
                "2024-03-27T10:00:00.000+18:00",
                1,
                0,
            ),
            create_inconsistent_exploitation_date_error(
                "2024-03-27T08:04:59.9999999+23:50",
                "2024-03-25T20:29:59.99999-11:45",
                1,
                1,
            ),
        ]);

        // Case 11: exploitation_date is before date
        // Case 12: exploitation_date after, but with timezones

        TESTS_2_1.test_6_1_53.expect(
            case_01_exp_date_after_date,
            case_02_exp_date_after_date_timezones,
            Ok(()),
            Ok(()),
        );
    }
}
