use crate::csaf_traits::WithOptionalDate;
use crate::schema::csaf2_0::schema::DocumentGenerator as DocumentGenerator20;
use crate::schema::csaf2_1::schema::DocumentGenerator as DocumentGenerator21;

/// Trait for accessing document generator information
pub trait GeneratorTrait: WithOptionalDate {}

impl GeneratorTrait for DocumentGenerator20 {}

impl GeneratorTrait for DocumentGenerator21 {}

crate::csaf::traits::impl_with_optional_date!(DocumentGenerator20);
crate::csaf::traits::impl_with_optional_date!(DocumentGenerator21);
