use crate::csaf_traits::ProductIdentificationHelperTrait;
use crate::schema::csaf2_0::schema::{
    FullProductNameT as FullProductNameT20, HelperToIdentifyTheProduct as HelperToIdentifyTheProduct20,
};
use crate::schema::csaf2_1::schema::{
    FullProductNameT as FullProductNameT21, HelperToIdentifyTheProduct as HelperToIdentifyTheProduct21,
};
use std::ops::Deref;

/// Trait representing an abstract full product name in a CSAF document.
pub trait ProductTrait {
    /// The associated type representing a product identification helper.
    type ProductIdentificationHelperType: ProductIdentificationHelperTrait;

    /// Returns the product ID from the full product name.
    fn get_product_id(&self) -> &String;

    /// Returns the textual description of the product
    fn get_name(&self) -> &str;

    /// Returns the product identification helper associated with the full product name.
    fn get_product_identification_helper(&self) -> &Option<Self::ProductIdentificationHelperType>;
}

impl ProductTrait for FullProductNameT20 {
    type ProductIdentificationHelperType = HelperToIdentifyTheProduct20;

    fn get_product_id(&self) -> &String {
        self.product_id.deref()
    }

    fn get_name(&self) -> &str {
        self.name.deref()
    }

    fn get_product_identification_helper(&self) -> &Option<Self::ProductIdentificationHelperType> {
        &self.product_identification_helper
    }
}

impl ProductTrait for FullProductNameT21 {
    type ProductIdentificationHelperType = HelperToIdentifyTheProduct21;

    fn get_product_id(&self) -> &String {
        self.product_id.deref()
    }

    fn get_name(&self) -> &str {
        self.name.deref()
    }

    fn get_product_identification_helper(&self) -> &Option<Self::ProductIdentificationHelperType> {
        &self.product_identification_helper
    }
}
