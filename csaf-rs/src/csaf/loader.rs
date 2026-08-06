use crate::json::JsonSource;

/// Detected version and parsed data.
pub struct VersionAndData {
    pub version: String,
    pub data: serde_json::Value,
}

/// Detect the version and return only the version.
pub fn detect_version<T: JsonSource>(source: T) -> std::io::Result<String> {
    detect_version_with(source).map(|r| r.version)
}

/// Detect the version and return it with the parsed raw JSON.
///
/// This can help improve re-using the already parsed data.
pub fn detect_version_with<T: JsonSource>(source: T) -> std::io::Result<VersionAndData> {
    // First, try to parse the JSON
    let json_value: serde_json::Value = source.parse()?;

    // Then, try to get the CSAF version from the document
    let version = json_value
        .get("document")
        .and_then(|doc| doc.get("csaf_version"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Could not detect CSAF version. Make sure the document has a 'document.csaf_version' field",
            )
        })?;

    Ok(VersionAndData {
        version,
        data: json_value,
    })
}
