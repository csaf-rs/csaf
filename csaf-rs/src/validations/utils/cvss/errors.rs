use crate::validation::{TestFinding, TestFindingData};
use cvss_rs::Version;
use strum::{AsRefStr, Display};

/// The type of CVSS score being validated, use for error messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, AsRefStr)]
pub enum ScoreType {
    Base,
    Temporal,
    Environmental,
}

pub fn create_vector_parse_error(
    vector_string: &str,
    version: Version,
    parse_error: &cvss_rs::ParseError,
    instance_path: &str,
) -> TestFinding {
    let version_str = match version {
        Version::V2 => "2.0",
        Version::V4 => "4.0",
        _ => "3.x",
    };
    TestFinding::Error(TestFindingData {
        message: format!("Could not parse vector string \"{vector_string}\" as CVSS {version_str}: {parse_error}"),
        instance_path: instance_path.to_string(),
    })
}

pub fn create_deserialization_finding_data(error_message: String, instance_path: String) -> TestFindingData {
    TestFindingData {
        message: format!("Error deserializing CVSS metric: {error_message}"),
        instance_path,
    }
}

pub fn create_deserialized_cvss_expected_version_mismatch_finding_data(
    found_version: Version,
    expected_version: Version,
    instance_path: &str,
) -> TestFindingData {
    TestFindingData {
        message: format!("Deserialized CVSS metric {found_version} does not match expected version {expected_version}"),
        instance_path: instance_path.to_string(),
    }
}
