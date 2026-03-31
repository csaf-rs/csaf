use crate::csaf_traits::{WithOptionalGroupIds, WithOptionalProductIds};
use crate::schema::csaf2_0::schema::{Note as Note20, NoteCategory as NoteCategory20};
use crate::schema::csaf2_1::schema::{Note as Note21, NoteCategory as NoteCategory21};
use std::ops::Deref;

pub trait NoteTrait: WithOptionalGroupIds + WithOptionalProductIds {
    fn get_category(&self) -> NoteCategory21;
    fn get_title(&self) -> Option<&String>;
}

// CSAF 2.0 implementation
impl WithOptionalGroupIds for Note20 {
    fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        None::<std::iter::Empty<&String>>
    }
}

impl WithOptionalProductIds for Note20 {
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        None::<std::iter::Empty<&String>>
    }
}

impl NoteTrait for Note20 {
    fn get_category(&self) -> NoteCategory21 {
        match self.category {
            NoteCategory20::Summary => NoteCategory21::Summary,
            NoteCategory20::Details => NoteCategory21::Details,
            NoteCategory20::Other => NoteCategory21::Other,
            NoteCategory20::Description => NoteCategory21::Description,
            NoteCategory20::Faq => NoteCategory21::Faq,
            NoteCategory20::General => NoteCategory21::General,
            NoteCategory20::LegalDisclaimer => NoteCategory21::LegalDisclaimer,
        }
    }

    fn get_title(&self) -> Option<&String> {
        self.title.as_deref()
    }
}

// CSAF 2.1 implementation
impl WithOptionalGroupIds for Note21 {
    fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.group_ids.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }
}

impl WithOptionalProductIds for Note21 {
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.product_ids.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }
}

impl NoteTrait for Note21 {
    fn get_category(&self) -> NoteCategory21 {
        self.category
    }

    fn get_title(&self) -> Option<&String> {
        self.title.as_deref()
    }
}
