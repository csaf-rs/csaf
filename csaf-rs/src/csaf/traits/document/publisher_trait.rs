use crate::csaf::traits::util::not_present_20::NotPresentInCsaf20;
use crate::csaf::traits::util::{impl_optional_str_field_getter, impl_str_field_getter};
use crate::csaf_traits::ContactTrait;
use crate::schema::csaf2_0::schema::{CategoryOfPublisher as CategoryOfPublisher20, Publisher as Publisher20};
use crate::schema::csaf2_1::schema::{
    CategoryOfPublisher as CategoryOfPublisher21, Contact as Contact21, Publisher as Publisher21,
};

/// Trait representing publisher information
pub trait PublisherTrait {
    type ContactType: ContactTrait;
    fn get_category(&self) -> CategoryOfPublisher21;
    fn get_issuing_authority(&self) -> Option<&str>;
    fn get_name(&self) -> &str;
    fn get_namespace(&self) -> &str;
    fn get_contact_details_20(&self) -> Option<&str>;
    fn get_contact_21(&self) -> Option<&Self::ContactType>;
}

impl PublisherTrait for Publisher20 {
    type ContactType = NotPresentInCsaf20;
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

    impl_optional_str_field_getter!(get_issuing_authority, issuing_authority);
    impl_str_field_getter!(get_name, name);
    impl_str_field_getter!(get_namespace, namespace);
    impl_optional_str_field_getter!(get_contact_details_20, contact_details);

    // The contact entity from CSAF 2.1 does not exist on CSAF 2.0, we implement NotPresentInCsaf20
    // for it.
    fn get_contact_21(&self) -> Option<&Self::ContactType> {
        None
    }
}

impl PublisherTrait for Publisher21 {
    type ContactType = Contact21;
    fn get_category(&self) -> CategoryOfPublisher21 {
        self.category
    }

    impl_optional_str_field_getter!(get_issuing_authority, issuing_authority);
    impl_str_field_getter!(get_name, name);
    impl_str_field_getter!(get_namespace, namespace);

    // The CSAF 2.0 syntax can still be fulfilled in CSAF 2.1, its just own layer nested now.
    fn get_contact_details_20(&self) -> Option<&str> {
        self.contact.as_ref()?.details.as_ref().map(|d| d.as_str())
    }

    fn get_contact_21(&self) -> Option<&Self::ContactType> {
        self.contact.as_ref()
    }
}
