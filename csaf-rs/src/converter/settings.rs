#[derive(Default)]
pub struct Settings {
    pub publisher_category_other: PublisherCategoryOtherSettings,
}

#[derive(Default)]
pub struct PublisherCategoryOtherSettings {
    pub suppress_warning: bool,
    pub convert_to_multiplier: bool,
}
