use super::schema::CommonSecurityAdvisoryFramework;
use std::{fs::File, io::BufReader};

pub fn load_document(path: &str) -> std::io::Result<CommonSecurityAdvisoryFramework> {
    println!("Trying to load document {}", path);

    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let doc: CommonSecurityAdvisoryFramework = serde_json::from_reader(reader)?;
    println!("Successfully parsed document '{}'", doc.document.title.to_string());

    Ok(doc)
}

/// Load a CSAF document from a JSON string
pub fn load_document_from_str(json_str: &str) -> std::io::Result<CommonSecurityAdvisoryFramework> {
    let doc: CommonSecurityAdvisoryFramework = serde_json::from_str(json_str)?;
    Ok(doc)
}

#[cfg(test)]
mod tests {
    use crate::csaf2_1::schema::{
        CategoryOfPublisher, CommonSecurityAdvisoryFramework, DocumentLevelMetaData, JsonSchema, LabelOfTlp, Publisher,
        Revision, RulesForDocumentSharing, Tracking, TrafficLightProtocolTlp,
    };

    fn mock_document() -> CommonSecurityAdvisoryFramework {
        let now = chrono::Utc::now().to_string();
        let metadata: DocumentLevelMetaData = DocumentLevelMetaData::builder()
            .title("Test")
            .category("csaf_base")
            .csaf_version("2.1")
            .distribution(
                RulesForDocumentSharing::builder().tlp(TrafficLightProtocolTlp::builder().label(LabelOfTlp::Clear)),
            )
            .publisher(
                Publisher::builder()
                    .category(CategoryOfPublisher::Coordinator)
                    .name("test")
                    .namespace("http://example.com"),
            )
            .tracking(
                Tracking::builder()
                    .id("test")
                    .current_release_date(now.clone())
                    .initial_release_date(now.clone())
                    .status("final")
                    .version("1")
                    .revision_history(vec![
                        Revision::builder()
                            .number("1")
                            .date(now.clone())
                            .summary("test")
                            .try_into()
                            .unwrap(),
                    ]),
            )
            .try_into()
            .unwrap();
        CommonSecurityAdvisoryFramework::builder()
            .document(metadata)
            .schema(JsonSchema::HttpsDocsOasisOpenOrgCsafCsafV21SchemaCsafJson)
            .try_into()
            .unwrap()
    }

    #[test]
    fn it_works() {
        mock_document();
    }
}
