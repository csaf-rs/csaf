use crate::csaf::traits::util::{impl_optional_str_field_getter, impl_str_iter_field_getter};
use crate::schema::csaf2_0::schema::Acknowledgment as Acknowledgment20;
use crate::schema::csaf2_1::schema::Acknowledgment as Acknowledgment21;

pub trait AcknowledgmentTrait {
    fn get_names(&self) -> impl Iterator<Item = &str> + '_;

    fn get_organization(&self) -> Option<&str>;

    fn get_summary(&self) -> Option<&str>;

    fn get_urls(&self) -> impl Iterator<Item = &str> + '_;
}

impl AcknowledgmentTrait for Acknowledgment20 {
    impl_str_iter_field_getter!(get_names, names);
    impl_optional_str_field_getter!(get_organization, organization);
    impl_optional_str_field_getter!(get_summary, summary);
    impl_str_iter_field_getter!(get_urls, urls);
}

impl AcknowledgmentTrait for Acknowledgment21 {
    impl_str_iter_field_getter!(get_names, names);
    impl_optional_str_field_getter!(get_organization, organization);
    impl_optional_str_field_getter!(get_summary, summary);
    impl_str_iter_field_getter!(get_urls, urls);
}
