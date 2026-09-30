use crate::csaf::traits::util::impl_str_field_getter;
use crate::csaf::traits::util::not_present_20::NotPresentInCsaf20;
use crate::csaf_traits::{ExtensionsTrait, ProductIdentificationHelperTrait};
use crate::schema::csaf2_0::schema::{
    FullProductNameT as FullProductNameT20, HelperToIdentifyTheProduct as HelperToIdentifyTheProduct20,
};
use crate::schema::csaf2_1::schema::{
    ExtensionsT as Extensions21, FullProductNameT as FullProductNameT21,
    HelperToIdentifyTheProduct as HelperToIdentifyTheProduct21,
};

/// Trait representing an abstract full product name in a CSAF document.
pub trait ProductTrait {
    /// The associated type representing a product identification helper.
    type ProductIdentificationHelperType: ProductIdentificationHelperTrait;

    type ExtensionsType: ExtensionsTrait;

    /// Returns the product ID from the full product name.
    fn get_product_id(&self) -> &str;

    /// Returns the textual description of the product
    fn get_name(&self) -> &str;

    /// Returns the product identification helper associated with the full product name.
    fn get_product_identification_helper(&self) -> Option<&Self::ProductIdentificationHelperType>;

    fn get_extensions(&self) -> Option<&Self::ExtensionsType>;
}

impl ProductTrait for FullProductNameT20 {
    type ProductIdentificationHelperType = HelperToIdentifyTheProduct20;
    type ExtensionsType = NotPresentInCsaf20;

    impl_str_field_getter!(get_product_id, product_id);
    impl_str_field_getter!(get_name, name);

    fn get_product_identification_helper(&self) -> Option<&Self::ProductIdentificationHelperType> {
        self.product_identification_helper.as_ref()
    }

    fn get_extensions(&self) -> Option<&Self::ExtensionsType> {
        None
    }
}

impl ProductTrait for FullProductNameT21 {
    type ProductIdentificationHelperType = HelperToIdentifyTheProduct21;
    type ExtensionsType = Extensions21;

    impl_str_field_getter!(get_product_id, product_id);
    impl_str_field_getter!(get_name, name);

    fn get_product_identification_helper(&self) -> Option<&Self::ProductIdentificationHelperType> {
        self.product_identification_helper.as_ref()
    }

    fn get_extensions(&self) -> Option<&Self::ExtensionsType> {
        self.x_extensions.as_ref()
    }
}
