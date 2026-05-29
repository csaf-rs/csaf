use crate::csaf::traits::util::impl_optional_str_field_getter;
use crate::csaf_traits::{WithOptionalGroupIds, WithOptionalProductIds};
use crate::schema::csaf2_0::schema::{Note as Note20, NoteCategory as NoteCategory20};
use crate::schema::csaf2_1::schema::{Note as Note21, NoteCategory as NoteCategory21};

pub trait NoteTrait: WithOptionalGroupIds + WithOptionalProductIds {
    fn get_category(&self) -> NoteCategory21;
    fn get_title(&self) -> Option<&str>;

    fn get_audience(&self) -> Option<&str>;

    fn get_text(&self) -> &str;
}

// CSAF 2.0 implementation
crate::csaf::traits::impl_optional_ids!(Note20, WithOptionalGroupIds, ReturnsEmpty);
crate::csaf::traits::impl_optional_ids!(Note20, WithOptionalProductIds, ReturnsEmpty);

impl NoteTrait for Note20 {
    fn get_category(&self) -> NoteCategory21 {
        match &self.category {
            NoteCategory20::Summary => NoteCategory21::Summary,
            NoteCategory20::Details => NoteCategory21::Details,
            NoteCategory20::Other => NoteCategory21::Other,
            NoteCategory20::Description => NoteCategory21::Description,
            NoteCategory20::Faq => NoteCategory21::Faq,
            NoteCategory20::General => NoteCategory21::General,
            NoteCategory20::LegalDisclaimer => NoteCategory21::LegalDisclaimer,
        }
    }

    impl_optional_str_field_getter!(get_title, title);

    fn get_audience(&self) -> Option<&str> {
        self.audience.as_deref().map(String::as_str)
    }

    fn get_text(&self) -> &str {
        &self.text
    }
}

// CSAF 2.1 implementation
crate::csaf::traits::impl_optional_ids!(Note21, WithOptionalGroupIds, ReturnsValues);
crate::csaf::traits::impl_optional_ids!(Note21, WithOptionalProductIds, ReturnsValues);

impl NoteTrait for Note21 {
    fn get_category(&self) -> NoteCategory21 {
        self.category
    }

    impl_optional_str_field_getter!(get_title, title);

    fn get_audience(&self) -> Option<&str> {
        self.audience.as_deref().map(String::as_str)
    }

    fn get_text(&self) -> &str {
        &self.text
    }
}
