use crate::csaf_traits::{WithOptionalDate, WithOptionalGroupIds, WithOptionalProductIds};
use crate::schema::csaf2_0::schema::{Flag as Flag20, LabelOfTheFlag as LabelOfTheFlag20};
use crate::schema::csaf2_1::schema::{Flag as Flag21, LabelOfTheFlag as LabelOfTheFlag21};

/// Trait for accessing vulnerability flags information
pub trait FlagTrait: WithOptionalGroupIds + WithOptionalProductIds + WithOptionalDate {
    /// Returns the label of the vulnerability flag
    fn get_label(&self) -> LabelOfTheFlag21;
}

crate::csaf::traits::impl_optional_ids!(Flag20, WithOptionalGroupIds, ReturnsValues);
crate::csaf::traits::impl_optional_ids!(Flag20, WithOptionalProductIds, ReturnsValues);
crate::csaf::traits::impl_with_optional_date!(Flag20);

crate::csaf::traits::impl_optional_ids!(Flag21, WithOptionalGroupIds, ReturnsValues);
crate::csaf::traits::impl_optional_ids!(Flag21, WithOptionalProductIds, ReturnsValues);
crate::csaf::traits::impl_with_optional_date!(Flag21);

impl FlagTrait for Flag20 {
    fn get_label(&self) -> LabelOfTheFlag21 {
        match self.label {
            LabelOfTheFlag20::ComponentNotPresent => LabelOfTheFlag21::ComponentNotPresent,
            LabelOfTheFlag20::InlineMitigationsAlreadyExist => LabelOfTheFlag21::InlineMitigationsAlreadyExist,
            LabelOfTheFlag20::VulnerableCodeCannotBeControlledByAdversary => {
                LabelOfTheFlag21::VulnerableCodeCannotBeControlledByAdversary
            },
            LabelOfTheFlag20::VulnerableCodeNotInExecutePath => LabelOfTheFlag21::VulnerableCodeNotInExecutePath,
            LabelOfTheFlag20::VulnerableCodeNotPresent => LabelOfTheFlag21::VulnerableCodeNotPresent,
        }
    }
}

impl FlagTrait for Flag21 {
    fn get_label(&self) -> LabelOfTheFlag21 {
        self.label
    }
}
