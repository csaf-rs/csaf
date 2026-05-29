use crate::csaf::traits::util::impl_str_field_getter;
use crate::schema::csaf2_0::schema::{CategoryOfReference as CategoryOfReference20, Reference as Reference20};
use crate::schema::csaf2_1::schema::{CategoryOfReference as CategoryOfReference21, Reference as Reference21};

/// Trait representing document references
pub trait ReferenceTrait {
    /// Returns the category of the document reference as enum
    fn get_category(&self) -> CategoryOfReference21;
    /// Returns the summary of the document reference
    fn get_summary(&self) -> &str;
    /// Returns the URL of the document reference
    fn get_url(&self) -> &str;
}

impl ReferenceTrait for Reference20 {
    fn get_category(&self) -> CategoryOfReference21 {
        match &self.category {
            CategoryOfReference20::External => CategoryOfReference21::External,
            CategoryOfReference20::Self_ => CategoryOfReference21::Self_,
        }
    }

    impl_str_field_getter!(get_summary, summary);
    impl_str_field_getter!(get_url, url);
}

impl ReferenceTrait for Reference21 {
    fn get_category(&self) -> CategoryOfReference21 {
        self.category
    }

    impl_str_field_getter!(get_summary, summary);
    impl_str_field_getter!(get_url, url);
}
