use std::str::FromStr;

use crate::validation::TestFindingData;
use cvss_rs::{ParseError, Version};

pub(crate) fn deserialize_vector<T>(
    vector_string: &str,
    version: Version,
    instance_path: &str,
) -> Result<T, TestFindingData>
where
    T: FromStr<Err = ParseError>,
{
    match vector_string.parse() {
        Ok(parsed) => Ok(parsed),
        Err(error) => Err(create_vector_parse_error(vector_string, version, &error, instance_path)),
    }
}

pub fn create_vector_parse_error(
    vector_string: &str,
    version: Version,
    parse_error: &cvss_rs::ParseError,
    instance_path: &str,
) -> TestFindingData {
    let version_str = match version {
        Version::V2 => "2.0",
        Version::V4 => "4.0",
        _ => "3.x",
    };
    TestFindingData {
        message: format!("Could not parse vector string \"{vector_string}\" as CVSS {version_str}: {parse_error}"),
        instance_path: instance_path.to_string(),
    }
}
