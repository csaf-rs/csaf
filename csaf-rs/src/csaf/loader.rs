use std::{fs::File, io::BufReader};

#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, PartialEq)]
struct CommonSecurityAdvisoryFramework {
    pub document: DocumentLevelMetaData,
}

#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, PartialEq)]
struct DocumentLevelMetaData {
    ///Gives the version of the CSAF specification which the document was generated for.
    pub csaf_version: String,
}

pub fn detect_version(path: &str) -> std::io::Result<String> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let doc: CommonSecurityAdvisoryFramework = serde_json::from_reader(reader)?;
    Ok(doc.document.csaf_version)
}
