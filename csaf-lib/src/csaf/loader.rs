use std::{fs::File, io::BufReader};

use chrono::DateTime;

use super::schema::CommonSecurityAdvisoryFramework;

pub fn load_document(path: &str) -> std::io::Result<CommonSecurityAdvisoryFramework> {
    /*let metadata: DocumentLevelMetaData = DocumentLevelMetaData::builder()
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
    String::from(doc.document.title)*/
    println!("Trying to load document {}", path);

    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let doc: CommonSecurityAdvisoryFramework = serde_json::from_reader(reader)?;
    println!("Successfully parsed document '{}'", doc.document.title.to_string());

    Ok(doc)
}
