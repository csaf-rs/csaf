use std::path::PathBuf;

use csaf_rs::schema::csaf2_1::schema::{self as csaf2_1_schema};

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

    #[command(flatten, next_help_heading = "Model number options")]
    pub(super) model_numbers: ModelNumberArgs,

    #[command(flatten, next_help_heading = "Serial number options")]
    pub(super) serial_numbers: SerialNumberArgs,

    #[command(flatten, next_help_heading = "Stock keeping unit options")]
    pub(super) skus: StockKeepingUnitArgs,
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

/// Values for the publisher contact if the document does not provide them. The CSAF 2.1 schema
/// types validate the given values, so that invalid arguments are already rejected at parse time
#[derive(clap::Args, Debug)]
pub(super) struct PublisherContactArgs {
    /// Let these values take precedence over the ones extracted from the document
    #[arg(long = "publisher-contact-overwrite-extracted")]
    pub(super) overwrite_extracted: bool,

    /// Value for the contact details, e.g. web sites, phone numbers, and postal mail addresses
    #[arg(long = "publisher-contact-details")]
    pub(super) details: Option<csaf2_1_schema::ContactDetails>,

    /// Email address to contact the publisher
    #[arg(long = "publisher-contact-email", value_parser = parse_email)]
    pub(super) email: Option<csaf2_1_schema::Email>,

    /// URL pointing to a public OpenPGP key of the publisher (HTTPS, at least 11 characters)
    #[arg(
        long = "publisher-contact-public-openpgp-key-url",
        value_parser = parse_public_openpgp_key_url
    )]
    pub(super) public_openpgp_key_url: Option<csaf2_1_schema::PublicOpenPgpKeyUrl>,
}

#[derive(clap::Args, Debug)]
pub(super) struct ModelNumberArgs {
    /// Interpret all model numbers as complete values
    #[arg(id = "model-numbers-as-complete", long = "model-numbers-as-complete")]
    pub(super) interpret_as_complete: bool,

    /// Interpret question marks in model numbers as literal characters
    #[arg(
        id = "model-numbers-question-mark-literal",
        long = "model-numbers-question-mark-literal"
    )]
    pub(super) question_mark_is_literal: bool,

    /// Interpret asterisks in model numbers as literal characters
    #[arg(id = "model-numbers-asterisk-literal", long = "model-numbers-asterisk-literal")]
    pub(super) asterisk_is_literal: bool,
}

#[derive(clap::Args, Debug)]
pub(super) struct SerialNumberArgs {
    /// Interpret all serial numbers as complete values
    #[arg(id = "serial-numbers-as-complete", long = "serial-numbers-as-complete")]
    pub(super) interpret_as_complete: bool,

    /// Interpret question marks in serial numbers as literal characters
    #[arg(
        id = "serial-numbers-question-mark-literal",
        long = "serial-numbers-question-mark-literal"
    )]
    pub(super) question_mark_is_literal: bool,

    /// Interpret asterisks in serial numbers as literal characters
    #[arg(id = "serial-numbers-asterisk-literal", long = "serial-numbers-asterisk-literal")]
    pub(super) asterisk_is_literal: bool,
}

#[derive(clap::Args, Debug)]
pub(super) struct StockKeepingUnitArgs {
    /// Interpret all stock keeping units as complete values
    #[arg(id = "skus-as-complete", long = "skus-as-complete")]
    pub(super) interpret_as_complete: bool,

    /// Interpret question marks in stock keeping units as literal characters
    #[arg(id = "skus-question-mark-literal", long = "skus-question-mark-literal")]
    pub(super) question_mark_is_literal: bool,

    /// Interpret asterisks in stock keeping units as literal characters
    #[arg(id = "skus-asterisk-literal", long = "skus-asterisk-literal")]
    pub(super) asterisk_is_literal: bool,
}

/// Validates the value against the requirements of the CSAF 2.1 property `email`, including its
/// `format: email` requirement
fn parse_email(value: &str) -> Result<csaf2_1_schema::Email, String> {
    if csaf_rs::converter::is_valid_email(value) {
        value.parse::<csaf2_1_schema::Email>().map_err(|err| err.to_string())
    } else {
        Err("does not fulfill the requirements of the CSAF 2.1 property `email`".to_string())
    }
}

/// Validates the value against the requirements of the CSAF 2.1 property
/// `public_openpgp_key_url`, including its `format: uri` requirement
fn parse_public_openpgp_key_url(value: &str) -> Result<csaf2_1_schema::PublicOpenPgpKeyUrl, String> {
    if csaf_rs::converter::is_valid_public_openpgp_key_url(value) {
        value
            .parse::<csaf2_1_schema::PublicOpenPgpKeyUrl>()
            .map_err(|err| err.to_string())
    } else {
        Err("does not fulfill the requirements of the CSAF 2.1 property `public_openpgp_key_url`".to_string())
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser as _;

    use super::ConverterArgs;

    const INPUT: &str = "input.json";

    #[test]
    fn parses_valid_publisher_contact_args() {
        let args = ConverterArgs::try_parse_from([
            "csaf-converter",
            INPUT,
            "--publisher-contact-details",
            "Call me, maybe",
            "--publisher-contact-email",
            "security@example.com",
            "--publisher-contact-public-openpgp-key-url",
            "https://example.com/key",
            "--publisher-contact-overwrite-extracted",
        ])
        .unwrap();

        let contact = args.publisher_contact;
        assert!(contact.overwrite_extracted);
        assert_eq!(&*contact.details.unwrap(), "Call me, maybe");
        assert_eq!(&*contact.email.unwrap(), "security@example.com");
        assert_eq!(&*contact.public_openpgp_key_url.unwrap(), "https://example.com/key");
    }

    #[test]
    fn rejects_email_below_minimum_length_at_parse_time() {
        let result = ConverterArgs::try_parse_from(["csaf-converter", INPUT, "--publisher-contact-email", "a@b.c"]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_non_https_openpgp_key_url_at_parse_time() {
        let result = ConverterArgs::try_parse_from([
            "csaf-converter",
            INPUT,
            "--publisher-contact-public-openpgp-key-url",
            "http://example.com/key",
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_non_format_email_at_parse_time() {
        let result =
            ConverterArgs::try_parse_from(["csaf-converter", INPUT, "--publisher-contact-email", "not-an-email"]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_dot_atom_violating_email_at_parse_time() {
        let result = ConverterArgs::try_parse_from([
            "csaf-converter",
            INPUT,
            "--publisher-contact-email",
            "security..alerts@example.org",
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_non_uri_public_openpgp_key_url_at_parse_time() {
        for value in [
            "https://example.com/a b",
            "https://example.com/[key",
            "https://example.org/key%ZZ.asc",
            "https://example.de/schl\u{fc}ssel.asc",
        ] {
            let result = ConverterArgs::try_parse_from([
                "csaf-converter",
                INPUT,
                "--publisher-contact-public-openpgp-key-url",
                value,
            ]);

            assert!(result.is_err(), "{value}");
        }
    }
}
