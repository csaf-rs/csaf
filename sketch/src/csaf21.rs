//! Schema-valid v2.1 CSAF document
// NOTE all the content in this file should be tool-generated

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Csaf {
    pub document: Document,
    // ..
}

#[derive(Deserialize, Serialize)]
pub struct Document {
    pub distribution: RulesForSharingDocument,
    pub csaf_version: CsafVersion,
    // ..
}

#[derive(Deserialize, Serialize)]
pub enum CsafVersion {
    #[serde(rename = "2.1")]
    X21,
}

impl From<CsafVersion> for serde_json::Value {
    fn from(value: CsafVersion) -> Self {
        serde_json::to_value(value).expect("unreachable")
    }
}

#[derive(Deserialize, Serialize)]
pub struct RulesForSharingDocument {
    pub sharing_group: Option<SharingGroup>,
    pub tlp: TrafficLightProtocolTlp,
    // ..
}

#[derive(Deserialize, Serialize)]
pub struct SharingGroup {
    // ..
}

#[derive(Deserialize, Serialize)]
pub struct TrafficLightProtocolTlp {
    pub label: LabelOfTlp,
    // ..
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LabelOfTlp {
    Amber,
    AmberStrict,
    Clear,
    Green,
    Red,
}

impl From<LabelOfTlp> for serde_json::Value {
    fn from(value: LabelOfTlp) -> Self {
        serde_json::to_value(value).expect("unreachable")
    }
}
