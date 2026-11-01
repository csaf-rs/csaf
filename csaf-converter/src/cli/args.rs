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
