#[derive(Default)]
pub struct Settings {
    pub publisher_category_other: PublisherCategoryOtherSettings,
    /// Prefer the first valid TLP v2.0 label found in
    /// document.distribution.text over the standard conversion.
    pub allow_tlp_from_distribution_text: bool,
}

#[derive(Default)]
pub struct PublisherCategoryOtherSettings {
    pub suppress_warning: bool,
    pub convert_to_multiplier: bool,
}
