use crate::csaf::traits::util::impl_optional_str_field_getter;
use crate::schema::csaf2_0::schema::Acknowledgment as Acknowledgment20;
use crate::schema::csaf2_1::schema::Acknowledgment as Acknowledgment21;

pub trait AcknowledgmentTrait {
    fn get_names(&self) -> Vec<&str>;

    fn get_organization(&self) -> Option<&str>;

    fn get_summary(&self) -> Option<&str>;

    fn get_urls(&self) -> Vec<&str>;
}

impl AcknowledgmentTrait for Acknowledgment20 {
    fn get_names(&self) -> Vec<&str> {
        self.names.iter().map(|n| n.as_str()).collect()
    }

    impl_optional_str_field_getter!(get_organization, organization);
    impl_optional_str_field_getter!(get_summary, summary);

    fn get_urls(&self) -> Vec<&str> {
        self.urls.iter().map(|u| u.as_str()).collect()
    }
}

impl AcknowledgmentTrait for Acknowledgment21 {
    fn get_names(&self) -> Vec<&str> {
        self.names.iter().map(|n| n.as_str()).collect()
    }

    impl_optional_str_field_getter!(get_organization, organization);
    impl_optional_str_field_getter!(get_summary, summary);

    fn get_urls(&self) -> Vec<&str> {
        self.urls.iter().map(|u| u.as_str()).collect()
    }
}
