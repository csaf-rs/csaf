use crate::csaf::types::csaf_datetime::CsafDateTime;
use crate::csaf_traits::WithOptionalDate;
use crate::schema::csaf2_0::schema::DocumentGenerator as DocumentGenerator20;
use crate::schema::csaf2_1::schema::DocumentGenerator as DocumentGenerator21;

/// Trait for accessing document generator information
pub trait GeneratorTrait: WithOptionalDate {
    fn get_engine_name(&self) -> &str;
    fn get_engine_version(&self) -> Option<&str>;
}

impl GeneratorTrait for DocumentGenerator20 {
    fn get_engine_name(&self) -> &str {
        &self.engine.name
    }

    fn get_engine_version(&self) -> Option<&str> {
        self.engine.version.as_deref().map(String::as_str)
    }
}

impl GeneratorTrait for DocumentGenerator21 {
    fn get_engine_name(&self) -> &str {
        &self.engine.name
    }

    fn get_engine_version(&self) -> Option<&str> {
        self.engine.version.as_deref().map(String::as_str)
    }
}

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
