use crate::csaf::types::csaf_datetime::CsafDateTime;
use crate::csaf_traits::WithOptionalDate;
use crate::schema::csaf2_0::schema::DocumentGenerator as DocumentGenerator20;
use crate::schema::csaf2_1::schema::DocumentGenerator as DocumentGenerator21;

/// Trait for accessing document generator information
pub trait GeneratorTrait: WithOptionalDate {}

impl GeneratorTrait for DocumentGenerator20 {}

impl GeneratorTrait for DocumentGenerator21 {}

impl WithOptionalDate for DocumentGenerator20 {
    fn get_date(&self) -> Option<CsafDateTime> {
        self.date.as_ref().map(CsafDateTime::from)
    }
}

impl WithOptionalDate for DocumentGenerator21 {
    fn get_date(&self) -> Option<CsafDateTime> {
        self.date.as_ref().map(CsafDateTime::from)
    }
}
