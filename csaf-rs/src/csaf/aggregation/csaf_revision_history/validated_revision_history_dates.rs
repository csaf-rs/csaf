use crate::csaf::aggregation::csaf_revision_history::revision_history::{RevisionHistory};
use crate::csaf::types::csaf_datetime::{CsafDateTime, CsafDateTimeParseError, ValidCsafDateTime};
use crate::validation::ValidationError;

pub enum ValidatedRevisionHistoryDates<'a> {
    Valid(ValidRevisionHistoryDates<'a>),
    Invalid(RevisionHistoryDateErrors<'a>)
}
impl<'a> From<&'a RevisionHistory> for ValidatedRevisionHistoryDates<'a> {
    fn from(value: &'a RevisionHistory) -> Self {

        let mut valid_dates: Option<ValidRevisionHistoryDates> = None;
        let mut date_errors: Option<RevisionHistoryDateErrors> = None;
        for item in &value.items {
            match &item.date {
                CsafDateTime::Valid(valid) => {
                    valid_dates.get_or_insert_default().0.push(ValidRevisionHistoryDate {
                        revision_index: item.path_index,
                        date_time: valid,
                    });
                }
                CsafDateTime::Invalid(err) => {
                    date_errors.get_or_insert_default().0.push(RevisionHistoryDateError {
                        revision_index: item.path_index,
                        error: err,
                    });
                }
            }
        }

        match date_errors {
            None => {
                match valid_dates{
                    None => {unreachable!("This should not be able to happen!")}
                    Some(valid_dates) => {
                        ValidatedRevisionHistoryDates::Valid(valid_dates)
                    }
                }
            }
            Some(errors) => {
                ValidatedRevisionHistoryDates::Invalid(errors)
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ValidRevisionHistoryDates<'a>(pub Vec<ValidRevisionHistoryDate<'a>>);

impl<'a> ValidRevisionHistoryDates<'a> {
    pub fn sort(&mut self) {
        self.0.sort_unstable_by_key(|a| a.date_time);
    }

    pub fn get_newest(&self) -> &ValidRevisionHistoryDate<'a> {
        self.0.last().unwrap_or_else(|| unreachable!("At this point there should be at least one valid revision date here."))
    }

    pub fn get_oldest(&self) -> &ValidRevisionHistoryDate<'a> {
        self.0.first().unwrap_or_else(|| unreachable!("At this point there should be at least one valid revision date here."))
    }
}

/// Error type for revision history date parsing failures
#[derive(Debug, Clone)]
pub struct RevisionHistoryDateError<'a> {
    /// The index of the revision that failed to parse
    pub revision_index: usize,
    /// The RFC3339 parsing error
    pub error: &'a CsafDateTimeParseError,
}

#[derive(Debug, Clone)]
pub struct ValidRevisionHistoryDate<'a> {
    /// The index of the revision parsed
    pub revision_index: usize,
    /// The successfully parsed date
    pub date_time: &'a ValidCsafDateTime,
}

/// Helper methods for RevisionHistoryDateError
impl<'a> RevisionHistoryDateError<'a> {
    fn get_instance_path(&self) -> String {
        format!("/document/tracking/revision_history/{}/date", self.revision_index)
    }
}

/// Convert RevisionHistoryDateError to ValidationError
impl<'a> From<RevisionHistoryDateError<'a>> for ValidationError {
    fn from(value: RevisionHistoryDateError) -> Self {
        ValidationError {
            message: value.error.to_string(),
            instance_path: value.get_instance_path(),
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct RevisionHistoryDateErrors<'a>(pub Vec<RevisionHistoryDateError<'a>>);
/// Convert Vec\<RevisionHistoryDateError\> to Vec\<ValidationError\>
impl<'a> From<RevisionHistoryDateErrors<'a>> for Vec<ValidationError> {
    fn from(value: RevisionHistoryDateErrors<'a>) -> Self {
        value.0.into_iter().map(|e| e.into()).collect()
    }
}
