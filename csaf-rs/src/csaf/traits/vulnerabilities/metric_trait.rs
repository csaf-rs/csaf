use crate::csaf_traits::ContentTrait;
use crate::schema::csaf2_0::schema::Score as Score20;
use crate::schema::csaf2_1::schema::{Content as Content21, Metric as Metric21};
use std::ops::Deref;

/// Trait representing an abstract metric in a CSAF document.
pub trait MetricTrait {
    type ContentType: ContentTrait;

    /// Retrieves an iterator over product IDs associated with this metric.
    fn get_products(&self) -> impl Iterator<Item = &String> + '_;

    /// Retrieves the "content" (i.e., actual metrics) of this metric.
    fn get_content(&self) -> &Self::ContentType;

    /// Retrieves the "source" (i.e., description of the metrics' origin) of this metric.
    fn get_source(&self) -> &Option<String>;
}

impl MetricTrait for Score20 {
    type ContentType = Score20;

    fn get_products(&self) -> impl Iterator<Item = &String> + '_ {
        self.products.iter().map(|x| x.deref())
    }

    fn get_content(&self) -> &Self::ContentType {
        self
    }

    fn get_source(&self) -> &Option<String> {
        &None
    }
}

impl MetricTrait for Metric21 {
    type ContentType = Content21;

    fn get_products(&self) -> impl Iterator<Item = &String> + '_ {
        self.products.deref().iter().map(|p| p.deref())
    }

    fn get_content(&self) -> &Self::ContentType {
        &self.content
    }

    fn get_source(&self) -> &Option<String> {
        &self.source
    }
}
