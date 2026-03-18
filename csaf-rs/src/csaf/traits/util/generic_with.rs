use crate::csaf::types::csaf_datetime::CsafDateTime;

pub trait WithOptionalGroupIds {
    /// Returns the product group IDs associated with this entity
    fn get_group_ids(&self) -> Option<impl Iterator<Item = &String> + '_>;
}

pub trait WithOptionalProductIds {
    /// Returns the product IDs associated with this entity
    fn get_product_ids(&self) -> Option<impl Iterator<Item = &String> + '_>;
}

pub trait WithDate {
    /// Returns the date associated with this entity
    fn get_date(&self) -> CsafDateTime;
}

pub trait WithOptionalDate {
    /// Returns the date associated with this entity
    fn get_date(&self) -> Option<CsafDateTime>;
}
