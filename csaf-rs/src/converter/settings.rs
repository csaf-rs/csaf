#[derive(Default)]
pub struct Settings {
    pub publisher_category_other: PublisherCategoryOtherSettings,
    pub publisher_contact: PublisherContactSettings,
}

#[derive(Default)]
pub struct PublisherCategoryOtherSettings {
    pub suppress_warning: bool,
    pub convert_to_multiplier: bool,
}

#[derive(Default)]
pub struct PublisherContactSettings {
    /// Value used for `$.document.publisher.contact.details` if the document does not provide one
    pub details: Option<String>,
    /// Value used for `$.document.publisher.contact.email` if the document does not provide one
    pub email: Option<String>,
    /// Value used for `$.document.publisher.contact.public_openpgp_key_url` if the document does not provide one
    pub public_openpgp_key_url: Option<String>,
    /// Let the configured values take precedence over the values found in the document
    pub overwrite_extracted: bool,
}
