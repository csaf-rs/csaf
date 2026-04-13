use crate::csaf::types::csaf_datetime::CsafDateTime;
use crate::csaf_traits::{WithOptionalDate, WithOptionalGroupIds, WithOptionalProductIds};
use crate::schema::csaf2_0::schema::{Flag as Flag20, LabelOfTheFlag as LabelOfTheFlag20};
use crate::schema::csaf2_1::schema::{Flag as Flag21, LabelOfTheFlag as LabelOfTheFlag21};
use std::ops::Deref;

/// Trait for accessing vulnerability flags information
pub trait FlagTrait: WithOptionalGroupIds + WithOptionalProductIds + WithOptionalDate {
    /// Returns the label of the vulnerability flag
    fn get_label(&self) -> LabelOfTheFlag21;
}

impl WithOptionalGroupIds for Flag20 {
    fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.group_ids.as_ref().map(|g| (*g).iter().map(|x| x.deref()))
    }
}

impl WithOptionalProductIds for Flag20 {
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.product_ids.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }
}

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

impl WithOptionalDate for Flag20 {
    fn get_date(&self) -> Option<CsafDateTime> {
        self.date.as_ref().map(CsafDateTime::from)
    }
}

impl WithOptionalGroupIds for Flag21 {
    fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.group_ids.as_ref().map(|g| (*g).iter().map(|x| x.deref()))
    }
}

impl WithOptionalProductIds for Flag21 {
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.product_ids.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }
}

impl FlagTrait for Flag21 {
    fn get_label(&self) -> LabelOfTheFlag21 {
        self.label
    }
}

impl WithOptionalDate for Flag21 {
    fn get_date(&self) -> Option<CsafDateTime> {
        self.date.as_ref().map(CsafDateTime::from)
    }
}
