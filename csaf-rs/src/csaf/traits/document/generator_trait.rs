use crate::csaf::traits::document::engine_trait::EngineTrait;
use crate::csaf_traits::WithOptionalDate;
use crate::schema::csaf2_0::schema::{
    DocumentGenerator as DocumentGenerator20, EngineOfDocumentGeneration as Engine20,
};
use crate::schema::csaf2_1::schema::{
    DocumentGenerator as DocumentGenerator21, EngineOfDocumentGeneration as Engine21,
};

/// Trait for accessing document generator information
pub trait GeneratorTrait: WithOptionalDate {
    /// Type representing the generation engine
    type EngineType: EngineTrait;

    /// Returns the engine that generated this document
    fn get_engine(&self) -> &Self::EngineType;
}

impl GeneratorTrait for DocumentGenerator20 {
    type EngineType = Engine20;

    fn get_engine(&self) -> &Self::EngineType {
        &self.engine
    }
}

impl GeneratorTrait for DocumentGenerator21 {
    type EngineType = Engine21;

    fn get_engine(&self) -> &Self::EngineType {
        &self.engine
    }
}

crate::csaf::traits::impl_with_optional_date!(DocumentGenerator20);
crate::csaf::traits::impl_with_optional_date!(DocumentGenerator21);
