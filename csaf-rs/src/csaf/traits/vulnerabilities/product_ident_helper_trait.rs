use crate::csaf::types::csaf_product_id_helper_number::{CsafModelNumber, CsafSerialNumber, CsafStockKeepingUnit};
use crate::csaf::types::purl::csaf_purl::CsafPurl;
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
    fn get_purls(&self) -> Option<Vec<CsafPurl>>;

    /// Constructs a JSON path string to `purl` / `purls` field.
    ///
    /// # Parameters
    ///
    /// * `path` - The JSON path to the parent product (e.g. `/product_tree/full_product_names/0`)
    /// * `purl_idx` - The zero-based index of the PURL within the PURLs array.
    ///   Ignored for CSAF 2.0 since `purl` is a single value, not an array.
    ///
    /// # Returns
    ///
    /// A `String` containing the JSON path to the PURL, e.g.:
    /// - CSAF 2.0: `/product_tree/full_product_names/0/product_identification_helper/purl`
    /// - CSAF 2.1: `/product_tree/full_product_names/0/product_identification_helper/purls/0`
    fn get_purls_json_path(&self, path: &str, purl_idx: usize) -> String;

    /// Returns the stock keeping units associated with this product.
    fn get_skus(&self) -> Vec<CsafStockKeepingUnit>;

    /// Returns the model numbers associated with this product.
    fn get_model_numbers(&self) -> Option<Vec<CsafModelNumber>>;

    /// Returns the serial numbers associated with this product.
    fn get_serial_numbers(&self) -> Option<Vec<CsafSerialNumber>>;

    /// Returns the hashes associated with this product.
    fn get_hashes(&self) -> &Vec<Self::HashType>;
}

impl ProductIdentificationHelperTrait for HelperToIdentifyTheProduct20 {
    type HashType = CryptographicHashes20;

    fn get_purls(&self) -> Option<Vec<CsafPurl>> {
        self.purl.as_ref().map(|s| vec![CsafPurl::from(s)])
    }

    fn get_purls_json_path(&self, product_path: &str, _purl_idx: usize) -> String {
        format!("{product_path}/product_identification_helper/purl")
    }

    fn get_skus(&self) -> Vec<CsafStockKeepingUnit> {
        self.skus.iter().map(CsafStockKeepingUnit::from).collect()
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

    fn get_purls(&self) -> Option<Vec<CsafPurl>> {
        self.purls.as_ref().map(|v| v.iter().map(CsafPurl::from).collect())
    }

    fn get_purls_json_path(&self, product_path: &str, purl_idx: usize) -> String {
        format!("{product_path}/product_identification_helper/purls/{purl_idx}")
    }

    fn get_skus(&self) -> Vec<CsafStockKeepingUnit> {
        self.skus.iter().map(CsafStockKeepingUnit::from).collect()
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
