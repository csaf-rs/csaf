use std::path::PathBuf;

/// A converter for CSAF 2.0 to 2.1 documents
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ConverterArgs {
    /// Path to CSAF document to convert
    #[arg()]
    pub(super) path: PathBuf,

    /// Path to the output file, stdout when omitted
    #[arg(long, short = 'o')]
    pub(super) output: Option<PathBuf>,

    /// Allow (partial) processing and output of invalid CSAF documents
    #[arg(long)]
    pub(super) allow_invalid: bool,

    #[command(flatten, next_help_heading = "Publisher category options")]
    pub(super) publisher_category_other: PublisherCategoryOtherArgs,

    #[command(flatten, next_help_heading = "Publisher contact options")]
    pub(super) publisher_contact: PublisherContactArgs,
}

#[derive(clap::Args, Debug)]
pub(super) struct PublisherCategoryOtherArgs {
    /// Suppress the warning when publisher category is 'other'
    #[arg(long = "publisher-category-other-suppress-warning")]
    pub(super) suppress_warning: bool,

    /// Convert publisher category 'other' to 'multiplier'
    #[arg(long = "publisher-category-other-convert-to-multiplier")]
    pub(super) convert_to_multiplier: bool,
}

/// Values for the publisher contact if the document does not provide them
#[derive(clap::Args, Debug)]
pub(super) struct PublisherContactArgs {
    /// Let these values take precedence over the ones extracted from the document
    #[arg(long = "publisher-contact-overwrite-extracted")]
    pub(super) overwrite_extracted: bool,

    /// Value for the contact details, e.g. web sites, phone numbers, and postal mail addresses
    #[arg(long = "publisher-contact-details")]
    pub(super) details: Option<String>,

    /// Email address to contact the publisher
    #[arg(long = "publisher-contact-email")]
    pub(super) email: Option<String>,

    /// URL pointing to a public OpenPGP key of the publisher (HTTPS, at least 11 characters)
    #[arg(long = "publisher-contact-public-openpgp-key-url")]
    pub(super) public_openpgp_key_url: Option<String>,
}
