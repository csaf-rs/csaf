use std::mem::{discriminant, Discriminant};
use crate::csaf::aggregation::csaf_revision_history::revision_history::{RevisionHistory};
use crate::csaf::types::csaf_version_number::{CsafVersionNumber, ValidVersionNumber, VersionNumberParsingError};
use crate::validation::ValidationError;

pub enum ValidatedRevisionHistoryNumbers<'a> {
    Valid(ValidRevisionHistoryNumbers<'a>),
    Invalid(RevisionHistoryNumberErrors<'a>)
}

type ValidRevisionHistoryNumbers<'a> = Vec<&'a ValidVersionNumber>;
#[derive(Clone, Debug)]
pub struct RevisionHistoryNumberErrors<'a> {
    pub parsing_errors: Option<RevisionHistoryVersionParsingErrors<'a>>,
    pub type_mismatch_errors: Option<RevisionHistoryVersionTypeMismatchErrors>
}

impl<'a> From<&'a RevisionHistory> for ValidatedRevisionHistoryNumbers<'a> {
    fn from(value: &'a RevisionHistory) -> Self {
        let mut valid_numbers: Option<ValidRevisionHistoryNumbers<'a>> = None;
        let mut parsing_errors: Option<RevisionHistoryVersionParsingErrors<'a>> = None;
        let mut type_mismatch_errors: Option<RevisionHistoryVersionTypeMismatchErrors> = None;
        let mut first_encountered_version_disc: Option<Discriminant<ValidVersionNumber>> = None;
        for item in &value.items {

            match &item.number {
                CsafVersionNumber::Valid(valid) => {
                    match first_encountered_version_disc {
                        None => { first_encountered_version_disc = Some(discriminant(valid)); }
                        Some(disc) => {
                            if disc != discriminant(valid) {
                                // Mixed versioning schemes detected, mark as error
                                type_mismatch_errors.get_or_insert_default().0.push(RevisionHistoryVersionTypeMismatchError::new(
                                    item.path_index,
                                    disc,
                                    discriminant(valid)
                                ));
                            }
                        }
                    };
                    valid_numbers.get_or_insert_default().push(valid);
                }
                CsafVersionNumber::Invalid(err) => {
                    parsing_errors.get_or_insert_default().0.push(RevisionHistoryVersionParsingError {
                        revision_index: item.path_index,
                        error: err,
                    });
                }
            }
        }

        match parsing_errors {
            None => {
                match valid_numbers {
                    None => {unreachable!("This should not be able to happen!")}
                    Some(valid_numbers) => {
                        ValidatedRevisionHistoryNumbers::Valid(valid_numbers)
                    }
                }
            }
            Some(errors) => {
                ValidatedRevisionHistoryNumbers::Invalid({
                    RevisionHistoryNumberErrors {
                        parsing_errors: Some(errors),
                        type_mismatch_errors,
                    }
                })
            }
        }
    }
}

/// Convert RevisionHistoryNumberErrors to Vec<ValidationError>
impl<'a> From<RevisionHistoryNumberErrors<'a>> for Vec<ValidationError> {
    fn from(value: RevisionHistoryNumberErrors<'a>) -> Self {
        value.parsing_errors
            .into_iter()
            .flat_map(|errs| errs.0.into_iter().map(|e| e.into()))
            .chain(
                value.type_mismatch_errors
                    .into_iter()
                    .flat_map(|errs| errs.0.into_iter().map(|e| e.into()))
            )
            .collect()
    }
}


/// Error type for revision history version number parsing failures
#[derive(Debug, Clone)]
pub struct RevisionHistoryVersionParsingError<'a> {
    /// The index of the revision that failed to parse
    pub revision_index: usize,
    /// The version number parsing error
    pub error: &'a VersionNumberParsingError,
}

/// Newtype wrapper to allow From implementation for Vec<ValidationError>
#[derive(Debug, Clone, Default)]
pub struct RevisionHistoryVersionParsingErrors<'a>(pub Vec<RevisionHistoryVersionParsingError<'a>>);

/// Helper methods for RevisionHistoryVersionError
impl<'a> RevisionHistoryVersionParsingError<'a> {
    fn get_instance_path(&self) -> String {
        format!("/document/tracking/revision_history/{}/number", self.revision_index)
    }
}

/// Convert RevisionHistoryVersionError to ValidationError
impl<'a> From<RevisionHistoryVersionParsingError<'a>> for ValidationError {
    fn from(value: RevisionHistoryVersionParsingError) -> Self {
        ValidationError {
            message: value.error.to_string(),
            instance_path: value.get_instance_path(),
        }
    }
}



/// Error type for revision history version number type mismatches
#[derive(Debug, Clone)]
pub struct RevisionHistoryVersionTypeMismatchError {
    /// The index of the revision that has a version type mismatch
    pub revision_index: usize,

    pub error: VersionTypeMismatchError,
}

/// Newtype wrapper to allow From implementation for Vec<ValidationError>
#[derive(Debug, Clone, Default)]
pub struct RevisionHistoryVersionTypeMismatchErrors(pub Vec<RevisionHistoryVersionTypeMismatchError>);

impl RevisionHistoryVersionTypeMismatchError {
    pub fn new(revision_index: usize, a: Discriminant<ValidVersionNumber>, b: Discriminant<ValidVersionNumber>) -> Self {
        RevisionHistoryVersionTypeMismatchError {
            revision_index,
            error: VersionTypeMismatchError { a, b },
        }
    }
}

#[derive(Clone, Debug)]
pub struct VersionTypeMismatchError {
    pub a: Discriminant<ValidVersionNumber>,
    pub b: Discriminant<ValidVersionNumber>
}

impl RevisionHistoryVersionTypeMismatchError {
    fn get_instance_path(&self) -> String {
        format!("/document/tracking/revision_history/{}/number", self.revision_index)
    }
}

/// Convert RevisionHistoryVersionTypeMismatchError to ValidationError
impl From<RevisionHistoryVersionTypeMismatchError> for ValidationError {
    fn from(value: RevisionHistoryVersionTypeMismatchError) -> Self {
        ValidationError {
            // TODO fix this
            message: format!("Mixed versioning schemes detected in revision history between {:?} and {:?}", value.error.a, value.error.b).to_string(),
            instance_path: value.get_instance_path(),
        }
    }
}