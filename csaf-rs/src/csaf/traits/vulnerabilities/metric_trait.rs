use crate::csaf::traits::util::{impl_optional_str_field_getter, impl_str_iter_field_getter};
use crate::csaf_traits::ContentTrait;
use crate::schema::csaf2_0::schema::Score as Score20;
use crate::schema::csaf2_1::schema::{Content as Content21, Metric as Metric21};

/// Trait representing an abstract metric in a CSAF document.
pub trait MetricTrait {
    type ContentType: ContentTrait;

    /// Retrieves an iterator over product IDs associated with this metric.
    fn get_products(&self) -> impl Iterator<Item = &str> + '_;

    /// Retrieves the "content" (i.e., actual metrics) of this metric.
    fn get_content(&self) -> &Self::ContentType;

    /// Retrieves the "source" (i.e., description of the metrics' origin) of this metric.
    fn get_source(&self) -> Option<&str>;
}

impl MetricTrait for Score20 {
    type ContentType = Score20;

    impl_str_iter_field_getter!(get_products, products);

    fn get_content(&self) -> &Self::ContentType {
        self
    }

    fn get_source(&self) -> Option<&str> {
        None
    }
}

impl MetricTrait for Metric21 {
    type ContentType = Content21;

    impl_str_iter_field_getter!(get_products, products);

    fn get_content(&self) -> &Self::ContentType {
        &self.content
    }

    impl_optional_str_field_getter!(get_source, source);
}
