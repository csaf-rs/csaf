use crate::csaf_traits::{WithOptionalDate, WithOptionalGroupIds, WithOptionalProductIds};
use crate::schema::csaf2_0::schema::{CategoryOfTheThreat as CategoryOfTheThreat20, Threat as Threat20};
use crate::schema::csaf2_1::schema::{CategoryOfTheThreat as CategoryOfTheThreat21, Threat as Threat21};

/// Trait representing an abstract threat in a CSAF document.
pub trait ThreatTrait: WithOptionalGroupIds + WithOptionalProductIds + WithOptionalDate {
    /// Returns the category of the threat
    fn get_category(&self) -> CategoryOfTheThreat21;
}

crate::csaf::traits::impl_with_optional_group_ids!(Threat20);
crate::csaf::traits::impl_with_optional_product_ids!(Threat20);
crate::csaf::traits::impl_with_optional_date!(Threat20);

crate::csaf::traits::impl_with_optional_group_ids!(Threat21);
crate::csaf::traits::impl_with_optional_product_ids!(Threat21);
crate::csaf::traits::impl_with_optional_date!(Threat21);

impl ThreatTrait for Threat20 {
    fn get_category(&self) -> CategoryOfTheThreat21 {
        match self.category {
            CategoryOfTheThreat20::ExploitStatus => CategoryOfTheThreat21::ExploitStatus,
            CategoryOfTheThreat20::Impact => CategoryOfTheThreat21::Impact,
            CategoryOfTheThreat20::TargetSet => CategoryOfTheThreat21::TargetSet,
        }
    }
}

impl ThreatTrait for Threat21 {
    fn get_category(&self) -> CategoryOfTheThreat21 {
        self.category
    }
}
