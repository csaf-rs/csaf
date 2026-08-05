use crate::csaf::traits::util::impl_str_field_getter;
use crate::schema::csaf2_0::schema::{CategoryOfReference as CategoryOfReference20, Reference as Reference20};
use crate::schema::csaf2_1::schema::{CategoryOfReference as CategoryOfReference21, Reference as Reference21};

/// Trait representing document or vulnerability references
pub trait ReferenceTrait {
    /// Returns the category of the reference as enum
    fn get_category(&self) -> CategoryOfReference21;
    /// Returns the summary of the reference
    fn get_summary(&self) -> &str;
    /// Returns the URL of the reference
    fn get_url(&self) -> &str;
}

/// Filters a slice of references by category, returning each match paired with its original index.
fn filter_by_category<R: ReferenceTrait>(references: &[R], category: CategoryOfReference21) -> Vec<(usize, &R)> {
    references
        .iter()
        .enumerate()
        .filter(|(_, r)| r.get_category() == category)
        .collect()
}

/// Returns only the self references from the given slice (filtered by category).
pub fn get_self_references<R: ReferenceTrait>(references: &[R]) -> Vec<(usize, &R)> {
    filter_by_category(references, CategoryOfReference21::Self_)
}

/// Returns only the external references from the given slice (filtered by category).
pub fn get_external_references<R: ReferenceTrait>(references: &[R]) -> Vec<(usize, &R)> {
    filter_by_category(references, CategoryOfReference21::External)
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
