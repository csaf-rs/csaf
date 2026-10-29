use crate::validation::{TestFinding, TestFindingData};
use cvss_rs::Version;

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
