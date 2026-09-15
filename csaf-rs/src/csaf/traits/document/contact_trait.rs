use crate::csaf::traits::util::impl_optional_str_field_getter;
use crate::csaf::traits::util::not_present_20::NotPresentInCsaf20;
use crate::schema::csaf2_1::schema::Contact as Contact21;

/// Trait representing contact information (CSAF 2.1)
/// Note: In CSAF 2.0, contact details are stored as a string on publisher.contact_details
/// We therefore implement [`NotPresentInCsaf20`] for this.
pub trait ContactTrait {
    /// Returns contact details (e.g., web sites, phone numbers, postal mail addresses)
    fn get_details(&self) -> Option<&str>;

    /// Returns the email address for contacting the publisher
    fn get_email(&self) -> Option<&str>;

    /// Returns the URL pointing to a public OpenPGP key
    fn get_public_openpgp_key_url(&self) -> Option<&str>;
}

impl ContactTrait for NotPresentInCsaf20 {
    fn get_details(&self) -> Option<&str> {
        self.into_any()
    }

    fn get_email(&self) -> Option<&str> {
        self.into_any()
    }

    fn get_public_openpgp_key_url(&self) -> Option<&str> {
        self.into_any()
    }
}

impl ContactTrait for Contact21 {
    impl_optional_str_field_getter!(get_details, details);
    impl_optional_str_field_getter!(get_email, email);
    impl_optional_str_field_getter!(get_public_openpgp_key_url, public_openpgp_key_url);
}
