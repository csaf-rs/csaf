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

    fn get_organization(&self) -> Option<&str> {
        self.organization.as_deref().map(String::as_str)
    }

    fn get_summary(&self) -> Option<&str> {
        self.summary.as_deref().map(String::as_str)
    }

    fn get_urls(&self) -> Vec<&str> {
        self.urls.iter().map(|u| u.as_str()).collect()
    }
}

impl AcknowledgmentTrait for Acknowledgment21 {
    fn get_names(&self) -> Vec<&str> {
        self.names.iter().map(|n| n.as_str()).collect()
    }

    fn get_organization(&self) -> Option<&str> {
        self.organization.as_deref().map(String::as_str)
    }

    fn get_summary(&self) -> Option<&str> {
        self.summary.as_deref().map(String::as_str)
    }

    fn get_urls(&self) -> Vec<&str> {
        self.urls.iter().map(|u| u.as_str()).collect()
    }
}
