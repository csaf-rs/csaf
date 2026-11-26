use crate::csaf::traits::util::not_present_20::NotPresentInCsaf20;
use crate::csaf::traits::util::{impl_into_any_getter, impl_optional_str_field_getter, impl_str_field_getter};
use crate::csaf_traits::ContactTrait;
use crate::schema::csaf2_1::schema::{ContactT as Contact21, Entity as Entity21, EntityCategory as EntityCategory21};

/// Provides access to involvement entity information.
///
/// As this involvement entity information is a CSAF 2.1 feature,
/// the CSAF 2.0 implementation provides [`NotPresentInCsaf20`] for this trait.
pub trait EntityTrait {
    type ContactType: ContactTrait;

    /// Returns the category describing the entity.
    fn get_category(&self) -> EntityCategory21;

    /// Returns the entity's contact information.
    fn get_contact(&self) -> Option<&Self::ContactType>;

    /// Returns the identifier used to reference the entity.
    fn get_entity_id(&self) -> &str;

    /// Returns the entity's display name.
    fn get_name(&self) -> Option<&str>;
}

impl EntityTrait for NotPresentInCsaf20 {
    type ContactType = NotPresentInCsaf20;

    impl_into_any_getter!(get_category, EntityCategory21);
    impl_into_any_getter!(get_contact, Option<&Self::ContactType>);
    impl_into_any_getter!(get_entity_id, &str);
    impl_into_any_getter!(get_name, Option<&str>);
}

impl EntityTrait for Entity21 {
    type ContactType = Contact21;

    fn get_category(&self) -> EntityCategory21 {
        self.category
    }

    fn get_contact(&self) -> Option<&Self::ContactType> {
        self.contact.as_ref()
    }

    impl_str_field_getter!(get_entity_id, entity_id);
    impl_optional_str_field_getter!(get_name, name);
}
