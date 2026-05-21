use crate::schema::csaf2_0::schema::{CategoryOfPublisher as CategoryOfPublisher20, Publisher as Publisher20};
use crate::schema::csaf2_1::schema::{CategoryOfPublisher as CategoryOfPublisher21, Publisher as Publisher21};

/// Trait representing publisher information
pub trait PublisherTrait {
    fn get_category(&self) -> CategoryOfPublisher21;
    fn get_issuing_authority(&self) -> Option<&str>;
    fn get_name(&self) -> &str;
    fn get_namespace(&self) -> &str;
    fn get_contact_details(&self) -> Option<&str>;
}

impl PublisherTrait for Publisher20 {
    fn get_category(&self) -> CategoryOfPublisher21 {
        match self.category {
            CategoryOfPublisher20::Coordinator => CategoryOfPublisher21::Coordinator,
            CategoryOfPublisher20::Discoverer => CategoryOfPublisher21::Discoverer,
            CategoryOfPublisher20::Other => CategoryOfPublisher21::Other,
            CategoryOfPublisher20::Translator => CategoryOfPublisher21::Translator,
            CategoryOfPublisher20::Vendor => CategoryOfPublisher21::Vendor,
            CategoryOfPublisher20::User => CategoryOfPublisher21::User,
        }
    }

    fn get_issuing_authority(&self) -> Option<&str> {
        self.issuing_authority.as_deref().map(String::as_str)
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_namespace(&self) -> &str {
        self.namespace.as_str()
    }

    fn get_contact_details(&self) -> Option<&str> {
        self.contact_details.as_deref().map(String::as_str)
    }
}

impl PublisherTrait for Publisher21 {
    fn get_category(&self) -> CategoryOfPublisher21 {
        self.category
    }

    fn get_issuing_authority(&self) -> Option<&str> {
        self.issuing_authority.as_deref().map(String::as_str)
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_namespace(&self) -> &str {
        self.namespace.as_str()
    }

    fn get_contact_details(&self) -> Option<&str> {
        self.contact_details.as_deref().map(String::as_str)
    }
}
