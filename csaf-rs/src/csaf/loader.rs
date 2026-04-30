use std::{fs::File, io::BufReader};

/// Detect the CSAF version of a document by reading the "document.csaf_version" field from the JSON.
pub fn detect_version(path: &str) -> std::io::Result<String> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    // First, try to parse the JSON
    let json_value: serde_json::Value = serde_json::from_reader(reader)?;

    detect_version_from_json(&json_value)
}

/// Detect the CSAF version from a JSON value by looking for "document.csaf_version".
pub fn detect_version_from_json(json_value: &serde_json::Value) -> std::io::Result<String> {
    json_value
        .get("document")
        .and_then(|doc| doc.get("csaf_version"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Could not detect CSAF version. Make sure the document has a 'document.csaf_version' field",
            )
        })
}
