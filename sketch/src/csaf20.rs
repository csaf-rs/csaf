//! Schema-valid v2.0 CSAF document
// NOTE all the content in this file should be tool-generated

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Csaf {
    pub document: Document,
    // ..
}

#[derive(Deserialize, Serialize)]
pub struct Document {
    pub csaf_version: CsafVersion,
    pub distribution: RulesForSharingDocument,
    // ..
}

#[derive(Deserialize, Serialize)]
pub enum CsafVersion {
    #[serde(rename = "2.0")]
    X20,
}

#[derive(Deserialize, Serialize)]
pub struct RulesForSharingDocument {
    pub tlp: TrafficLightProtocolTlp,
    // ..
}

#[derive(Deserialize, Serialize)]
pub struct TrafficLightProtocolTlp {
    pub label: LabelOfTlp,
    // ..
}

#[derive(Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum LabelOfTlp {
    Amber,
    Green,
    Red,
    White,
}
