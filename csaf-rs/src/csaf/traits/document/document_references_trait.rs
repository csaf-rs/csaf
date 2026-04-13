use crate::schema::csaf2_0::schema::{CategoryOfReference as CategoryOfReference20, Reference as Reference20};
use crate::schema::csaf2_1::schema::{CategoryOfReference as CategoryOfReference21, Reference as Reference21};

/// Trait representing document references
pub trait DocumentReferenceTrait {
    /// Returns the category of the document reference as enum
    fn get_category(&self) -> &CategoryOfReference21;
    /// Returns the summary of the document reference
    fn get_summary(&self) -> &String;
    /// Returns the URL of the document reference
    fn get_url(&self) -> &String;
}

impl DocumentReferenceTrait for Reference20 {
    fn get_category(&self) -> &CategoryOfReference21 {
        match &self.category {
            CategoryOfReference20::External => &CategoryOfReference21::External,
            CategoryOfReference20::Self_ => &CategoryOfReference21::Self_,
        }
    }

    fn get_summary(&self) -> &String {
        &self.summary
    }

    fn get_url(&self) -> &String {
        &self.url
    }
}

impl DocumentReferenceTrait for Reference21 {
    fn get_category(&self) -> &CategoryOfReference21 {
        &self.category
    }

    fn get_summary(&self) -> &String {
        &self.summary
    }

    fn get_url(&self) -> &String {
        &self.url
    }
}
