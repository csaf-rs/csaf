use crate::schema::csaf2_0::schema::{
    LabelOfTlp as LabelOfTlp20, TrafficLightProtocolTlp as TrafficLightProtocolTlp20,
};
use crate::schema::csaf2_1::schema::{
    LabelOfTlp as LabelOfTlp21, TrafficLightProtocolTlp as TrafficLightProtocolTlp21,
};

/// Trait representing TLP (Traffic Light Protocol) information
pub trait TlpTrait {
    /// Returns the TLP label
    fn get_label(&self) -> LabelOfTlp21;
}

impl TlpTrait for TrafficLightProtocolTlp20 {
    /// Normalizes the TLP (Traffic Light Protocol) labels from CSAF 2.0 to those of CSAF 2.1.
    ///
    /// # Explanation
    /// In CSAF 2.1, the TLP labeling scheme was updated to align with the official TLP 2.0 standard,
    /// which renamed "WHITE" to "CLEAR". This function ensures that TLP labels from CSAF 2.0
    /// are converted to their corresponding labels in CSAF 2.1.
    ///
    /// # Returns
    /// A CSAF 2.1 `Tlp21` value that corresponds to the TLP label of the current object.
    fn get_label(&self) -> LabelOfTlp21 {
        match self.label {
            LabelOfTlp20::Amber => LabelOfTlp21::Amber,
            LabelOfTlp20::Green => LabelOfTlp21::Green,
            LabelOfTlp20::Red => LabelOfTlp21::Red,
            LabelOfTlp20::White => LabelOfTlp21::Clear,
        }
    }
}

impl TlpTrait for TrafficLightProtocolTlp21 {
    fn get_label(&self) -> LabelOfTlp21 {
        self.label
    }
}
