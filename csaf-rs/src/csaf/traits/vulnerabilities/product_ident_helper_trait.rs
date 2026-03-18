use crate::csaf::types::csaf_product_id_helper_number::{CsafModelNumber, CsafSerialNumber};
use crate::csaf_traits::HashTrait;
use crate::schema::csaf2_0::schema::{
    CryptographicHashes as CryptographicHashes20, HelperToIdentifyTheProduct as HelperToIdentifyTheProduct20,
};
use crate::schema::csaf2_1::schema::{
    CryptographicHashes as CryptographicHashes21, HelperToIdentifyTheProduct as HelperToIdentifyTheProduct21,
};

/// Trait representing an abstract product identification helper of a full product name.
pub trait ProductIdentificationHelperTrait {
    type HashType: HashTrait;

    /// Returns the PURLs identifying the associated product.
    fn get_purls(&self) -> Option<&[String]>;

    fn get_model_numbers(&self) -> Option<Vec<CsafModelNumber>>;

    fn get_serial_numbers(&self) -> Option<Vec<CsafSerialNumber>>;

    fn get_hashes(&self) -> &Vec<Self::HashType>;
}

impl ProductIdentificationHelperTrait for HelperToIdentifyTheProduct20 {
    type HashType = CryptographicHashes20;

    fn get_purls(&self) -> Option<&[String]> {
        self.purl.as_ref().map(std::slice::from_ref)
    }

    fn get_model_numbers(&self) -> Option<Vec<CsafModelNumber>> {
        self.model_numbers
            .as_ref()
            .map(|v| v.iter().map(CsafModelNumber::from).collect())
    }

    fn get_serial_numbers(&self) -> Option<Vec<CsafSerialNumber>> {
        self.serial_numbers
            .as_ref()
            .map(|v| v.iter().map(CsafSerialNumber::from).collect())
    }

    fn get_hashes(&self) -> &Vec<Self::HashType> {
        self.hashes.as_ref()
    }
}

impl ProductIdentificationHelperTrait for HelperToIdentifyTheProduct21 {
    type HashType = CryptographicHashes21;

    fn get_purls(&self) -> Option<&[String]> {
        self.purls.as_deref()
    }

    fn get_model_numbers(&self) -> Option<Vec<CsafModelNumber>> {
        self.model_numbers
            .as_ref()
            .map(|v| v.iter().map(CsafModelNumber::from).collect())
    }

    fn get_serial_numbers(&self) -> Option<Vec<CsafSerialNumber>> {
        self.serial_numbers
            .as_ref()
            .map(|v| v.iter().map(CsafSerialNumber::from).collect())
    }

    fn get_hashes(&self) -> &Vec<Self::HashType> {
        self.hashes.as_ref()
    }
}
