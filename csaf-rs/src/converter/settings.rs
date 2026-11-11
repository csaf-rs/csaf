use crate::schema::csaf2_1;

#[derive(Default)]
pub struct Settings {
    pub publisher_category_other: PublisherCategoryOtherSettings,
    pub publisher_contact: PublisherContactSettings,
    pub model_numbers: ModelNumberSettings,
    pub serial_numbers: SerialNumberSettings,
    pub skus: StockKeepingUnitSettings,
}

#[derive(Default)]
pub struct ProductIdentificationNumberSettings {
    /// Interpret model numbers as complete values instead of partial prefixes.
    pub interpret_as_complete: bool,
    /// Interpret question marks as literal characters.
    pub question_mark_is_literal: bool,
    /// Interpret asterisks as literal characters.
    pub asterisk_is_literal: bool,
}

pub type ModelNumberSettings = ProductIdentificationNumberSettings;
pub type SerialNumberSettings = ProductIdentificationNumberSettings;
pub type StockKeepingUnitSettings = ProductIdentificationNumberSettings;

#[derive(Default)]
pub struct PublisherCategoryOtherSettings {
    pub suppress_warning: bool,
    pub convert_to_multiplier: bool,
}

/// Values for the publisher contact if the document does not provide them.
///
/// The typed values already fulfill the pattern and length requirements of their CSAF 2.1 schema
/// types (e.g. via `FromStr`, as clap does for command line arguments when using the types
/// directly as argument values). Their `format` requirements (e.g. `format: email`) are checked
/// by the conversion itself, because the generated types do not assert them.
#[derive(Default)]
pub struct PublisherContactSettings {
    /// Value used for `$.document.publisher.contact.details` if the document does not provide one
    pub details: Option<csaf2_1::schema::ContactDetails>,
    /// Value used for `$.document.publisher.contact.email` if the document does not provide one
    pub email: Option<csaf2_1::schema::Email>,
    /// Value used for `$.document.publisher.contact.public_openpgp_key_url` if the document does not provide one
    pub public_openpgp_key_url: Option<csaf2_1::schema::PublicOpenPgpKeyUrl>,
    /// Let the configured values take precedence over the values found in the document
    pub overwrite_extracted: bool,
}
