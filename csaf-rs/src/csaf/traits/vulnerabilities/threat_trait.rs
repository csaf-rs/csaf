use crate::csaf::types::csaf_datetime::CsafDateTime;
use crate::csaf_traits::{WithOptionalDate, WithOptionalGroupIds, WithOptionalProductIds};
use crate::schema::csaf2_0::schema::{CategoryOfTheThreat as CategoryOfTheThreat20, Threat as Threat20};
use crate::schema::csaf2_1::schema::{CategoryOfTheThreat as CategoryOfTheThreat21, Threat as Threat21};
use std::ops::Deref;

/// Trait representing an abstract threat in a CSAF document.
pub trait ThreatTrait: WithOptionalGroupIds + WithOptionalProductIds + WithOptionalDate {
    /// Returns the category of the threat
    fn get_category(&self) -> CategoryOfTheThreat21;
}

impl WithOptionalGroupIds for Threat20 {
    fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.group_ids.as_ref().map(|g| (*g).iter().map(|x| x.deref()))
    }
}

impl WithOptionalProductIds for Threat20 {
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.product_ids.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }
}

impl WithOptionalDate for Threat20 {
    fn get_date(&self) -> Option<CsafDateTime> {
        self.date.as_ref().map(CsafDateTime::from)
    }
}

impl ThreatTrait for Threat20 {
    fn get_category(&self) -> CategoryOfTheThreat21 {
        match self.category {
            CategoryOfTheThreat20::ExploitStatus => CategoryOfTheThreat21::ExploitStatus,
            CategoryOfTheThreat20::Impact => CategoryOfTheThreat21::Impact,
            CategoryOfTheThreat20::TargetSet => CategoryOfTheThreat21::TargetSet,
        }
    }
}

impl WithOptionalGroupIds for Threat21 {
    fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.group_ids.as_ref().map(|g| (*g).iter().map(|x| x.deref()))
    }
}

impl WithOptionalProductIds for Threat21 {
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_> {
        self.product_ids.as_ref().map(|p| (*p).iter().map(|x| x.deref()))
    }
}

impl WithOptionalDate for Threat21 {
    fn get_date(&self) -> Option<CsafDateTime> {
        self.date.as_ref().map(CsafDateTime::from)
    }
}

impl ThreatTrait for Threat21 {
    fn get_category(&self) -> CategoryOfTheThreat21 {
        self.category
    }
}
