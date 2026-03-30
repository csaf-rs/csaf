use crate::csaf::traits::util::not_present_20::NotPresentInCsaf20;
use crate::csaf_traits::{SharingGroupTrait, TlpTrait};
use crate::schema::csaf2_0::schema::{
    RulesForSharingDocument as RulesForSharingDocument20, TrafficLightProtocolTlp as TrafficLightProtocolTlp20,
};
use crate::schema::csaf2_1::schema::{
    RulesForDocumentSharing as RulesForDocumentSharing21, SharingGroup as SharingGroup21,
    TrafficLightProtocolTlp as TrafficLightProtocolTlp21,
};
use crate::validation::ValidationError;

/// Trait representing distribution information for a document
pub trait DistributionTrait {
    /// Type representing sharing group information
    type SharingGroupType: SharingGroupTrait;

    /// Type representing TLP (Traffic Light Protocol) information
    type TlpType: TlpTrait;

    /// Returns the sharing group for this distribution
    fn get_sharing_group(&self) -> &Option<Self::SharingGroupType>;

    /// Returns the TLP information for this distribution with CSAF 2.0 semantics
    fn get_tlp_20(&self) -> Option<&Self::TlpType>;

    /// Returns the TLP information for this distribution with CSAF 2.1 semantics
    fn get_tlp_21(&self) -> Result<&Self::TlpType, ValidationError>;
}

impl DistributionTrait for RulesForSharingDocument20 {
    type SharingGroupType = NotPresentInCsaf20;
    type TlpType = TrafficLightProtocolTlp20;

    fn get_sharing_group(&self) -> &Option<Self::SharingGroupType> {
        &None
    }

    /// Return TLP as ref Option, it is an option anyway
    fn get_tlp_20(&self) -> Option<&Self::TlpType> {
        self.tlp.as_ref()
    }

    /// Return TLP or a ValidationError to satisfy CSAF 2.1 semantics
    fn get_tlp_21(&self) -> Result<&Self::TlpType, ValidationError> {
        match self.tlp.as_ref() {
            None => Err(ValidationError {
                message: "CSAF 2.1 requires the TLP property, but it is not set.".to_string(),
                instance_path: "/document/distribution/tlp".to_string(),
            }),
            Some(tlp) => Ok(tlp),
        }
    }
}

impl DistributionTrait for RulesForDocumentSharing21 {
    type SharingGroupType = SharingGroup21;
    type TlpType = TrafficLightProtocolTlp21;

    fn get_sharing_group(&self) -> &Option<Self::SharingGroupType> {
        &self.sharing_group
    }

    /// We normalize to Option here because property was optional in CSAF 2.0
    fn get_tlp_20(&self) -> Option<&Self::TlpType> {
        Some(&self.tlp)
    }

    /// Always return the value because it is mandatory
    fn get_tlp_21(&self) -> Result<&Self::TlpType, ValidationError> {
        Ok(&self.tlp)
    }
}
