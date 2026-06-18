use crate::{csaf::raw::RawDocument, json::JsonSource, schema::csaf2_0::schema::CommonSecurityAdvisoryFramework};

pub fn load_document<T: JsonSource>(source: T) -> std::io::Result<RawDocument<CommonSecurityAdvisoryFramework>> {
    Ok(RawDocument::new(source.parse()?))
}

#[cfg(test)]
mod tests {
    use crate::schema::csaf2_0::schema::{
        CategoryOfPublisher, CommonSecurityAdvisoryFramework, DocumentLevelMetaData, Publisher, Revision, Tracking,
    };

    fn mock_document() -> CommonSecurityAdvisoryFramework {
        let now = chrono::Utc::now().to_string();
        let metadata: DocumentLevelMetaData = DocumentLevelMetaData::builder()
            .title("Test")
            .category("csaf_base")
            .csaf_version("2.0")
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
            .try_into()
            .unwrap()
    }

    #[test]
    fn it_works() {
        mock_document();
    }
}
