use crate::{csaf::raw::RawDocument, json::JsonSource, schema::csaf2_1::schema::CommonSecurityAdvisoryFramework};

pub fn load_document<T: JsonSource>(source: T) -> std::io::Result<RawDocument<CommonSecurityAdvisoryFramework>> {
    Ok(RawDocument::new(source.parse()?))
}

#[cfg(test)]
mod tests {
    use crate::schema::csaf2_1::schema::{
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
                            .date(now)
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
