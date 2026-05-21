use crate::csaf_traits::{WithOptionalGroupIds, WithOptionalProductIds};
use crate::schema::csaf2_0::schema::{Note as Note20, NoteCategory as NoteCategory20};
use crate::schema::csaf2_1::schema::{Note as Note21, NoteCategory as NoteCategory21};

pub trait NoteTrait: WithOptionalGroupIds + WithOptionalProductIds {
    fn get_category(&self) -> NoteCategory21;
    fn get_title(&self) -> Option<&str>;
}

// CSAF 2.0 implementation
crate::csaf::traits::impl_without_group_ids!(Note20);
crate::csaf::traits::impl_without_product_ids!(Note20);

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

    fn get_title(&self) -> Option<&str> {
        self.title.as_deref().map(String::as_str)
    }
}

// CSAF 2.1 implementation
crate::csaf::traits::impl_with_optional_group_ids!(Note21);
crate::csaf::traits::impl_with_optional_product_ids!(Note21);

impl NoteTrait for Note21 {
    fn get_category(&self) -> NoteCategory21 {
        self.category
    }

    fn get_title(&self) -> Option<&str> {
        self.title.as_deref().map(String::as_str)
    }
}
