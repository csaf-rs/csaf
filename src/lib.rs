use chrono::DateTime;
use crate::codegen::{builder, CategoryOfPublisher, CommonSecurityAdvisoryFramework, DocumentCategory, Publisher, Revision, Tracking};
use crate::codegen::DocumentLevelMetaData;

mod codegen;

fn load_document() -> String {
    let metadata: DocumentLevelMetaData = DocumentLevelMetaData::builder()
        .title("Test")
        .category("csaf_base")
        .csaf_version("2.0")
        .publisher(Publisher::builder()
            .category(CategoryOfPublisher::Coordinator)
            .name("test")
            .namespace("http://example.com"))
        .tracking(
            Tracking::builder()
                .id("test")
                .current_release_date(chrono::Utc::now())
                .initial_release_date(chrono::Utc::now())
                .status("final")
                .version("1")
                .revision_history(vec![
                    Revision::builder().number("1").date(chrono::Utc::now())
                        .summary("test")
                        .try_into().unwrap()
                ])
        )
        .try_into().unwrap();
    let doc: CommonSecurityAdvisoryFramework = CommonSecurityAdvisoryFramework::builder()
        .document(
            metadata
        )
        .try_into()
        .unwrap();
    String::from(doc.document.title)
}

#[cfg(test)]
mod tests {
    use crate::load_document;

    #[test]
    fn it_works() {
        let result = load_document();
        assert_eq!(result, "Test")
    }
}
