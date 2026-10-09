//! A JSON object that holds a, potentially schema-invalid, CSAF document, in either version 2.0 or 2.1
// NOTE all the content in this file should be tool-generated

use bytemuck::TransparentWrapper;

use crate::{csaf20, csaf21};

/// path: `/`
pub struct Csaf {
    raw: serde_json::Value,
}

// NOTE all accessors are "lenient" and return an `Option` as the JSON document may not match any schema
impl Csaf {
    pub fn new(raw: serde_json::Value) -> Self {
        Self { raw }
    }

    pub fn into_v20(self) -> serde_json::Result<csaf20::Csaf> {
        serde_json::from_value(self.raw)
    }

    pub fn into_v21(self) -> serde_json::Result<csaf21::Csaf> {
        serde_json::from_value(self.raw)
    }

    pub fn into_raw(self) -> serde_json::Value {
        self.raw
    }

    // NOTE `v2x` indicates this getter is available on both v2.0 and v2.1
    pub fn v2x_document(&self) -> Option<&Document> {
        self.raw.get("document").map(TransparentWrapper::wrap_ref)
    }

    pub fn v2x_document_mut(&mut self) -> Option<&mut Document> {
        self.raw.get_mut("document").map(TransparentWrapper::wrap_mut)
    }
}

/// path: `/document`
#[derive(bytemuck::TransparentWrapper)]
#[repr(transparent)]
pub struct Document(serde_json::Value);

impl Document {
    pub fn v2x_lang(&self) -> Option<&Lang> {
        self.0.get("lang").map(TransparentWrapper::wrap_ref)
    }

    pub fn v2x_lang_mut(&mut self) -> Option<&mut Lang> {
        self.0.get_mut("lang").map(TransparentWrapper::wrap_mut)
    }

    pub fn v2x_source_lang(&self) -> Option<&Lang> {
        self.0.get("source_lang").map(TransparentWrapper::wrap_ref)
    }

    pub fn v2x_source_lang_mut(&mut self) -> Option<&mut Lang> {
        self.0.get_mut("source_lang").map(TransparentWrapper::wrap_mut)
    }

    pub fn v2x_distribution(&self) -> Option<&RulesForSharingDocument> {
        self.0.get("distribution").map(TransparentWrapper::wrap_ref)
    }

    pub fn v2x_distribution_mut(&mut self) -> Option<&mut RulesForSharingDocument> {
        self.0.get_mut("distribution").map(TransparentWrapper::wrap_mut)
    }

    pub fn v2x_csaf_version(&self) -> Option<&CsafVersion> {
        self.0.get("csaf_version").map(TransparentWrapper::wrap_ref)
    }

    pub fn v2x_csaf_version_mut(&mut self) -> Option<&mut CsafVersion> {
        self.0.get_mut("csaf_version").map(TransparentWrapper::wrap_mut)
    }
}

/// path: `/document/lang` or `/document/source_lang`
#[derive(bytemuck::TransparentWrapper)]
#[repr(transparent)]
pub struct Lang(serde_json::Value);

impl Lang {
    pub fn v2x_get(&self) -> serde_json::Result<String> {
        serde_json::from_value(TransparentWrapper::peel_ref(self).clone())
    }
}

/// path: `/document/csaf_version`
#[derive(bytemuck::TransparentWrapper)]
#[repr(transparent)]
pub struct CsafVersion(serde_json::Value);

impl CsafVersion {
    pub fn v20_get(&self) -> serde_json::Result<csaf20::CsafVersion> {
        serde_json::from_value(TransparentWrapper::peel_ref(self).clone())
    }

    pub fn v21_set(&mut self, value: csaf21::CsafVersion) {
        *TransparentWrapper::peel_mut(self) = value.into();
    }
}

/// path: `/document/distribution`
#[derive(bytemuck::TransparentWrapper)]
#[repr(transparent)]
pub struct RulesForSharingDocument(serde_json::Value);

impl RulesForSharingDocument {
    // NOTE `v21` indicates this getter is only available on v2.1
    pub fn v21_sharing_group(&self) -> Option<&SharingGroup> {
        self.0.get("sharing_group").map(TransparentWrapper::wrap_ref)
    }

    pub fn v21_sharing_group_mut(&mut self) -> Option<&mut SharingGroup> {
        self.0.get_mut("sharing_group").map(TransparentWrapper::wrap_mut)
    }

    pub fn v2x_tlp(&self) -> Option<&TrafficLightProtocolTlp> {
        self.0.get("tlp").map(TransparentWrapper::wrap_ref)
    }

    pub fn v2x_tlp_mut(&mut self) -> Option<&mut TrafficLightProtocolTlp> {
        self.0.get_mut("tlp").map(TransparentWrapper::wrap_mut)
    }
}

/// path: `/document/distribution/sharing_group`
#[derive(bytemuck::TransparentWrapper)]
#[repr(transparent)]
pub struct SharingGroup(serde_json::Value);

/// path: `/document/distribution/tlp`
#[derive(bytemuck::TransparentWrapper)]
#[repr(transparent)]
pub struct TrafficLightProtocolTlp(serde_json::Value);

impl TrafficLightProtocolTlp {
    pub fn v2x_label(&self) -> Option<&LabelOfTlp> {
        self.0.get("label").map(TransparentWrapper::wrap_ref)
    }

    pub fn v2x_label_mut(&mut self) -> Option<&mut LabelOfTlp> {
        self.0.get_mut("label").map(TransparentWrapper::wrap_mut)
    }

    pub fn as_object_mut(&mut self) -> Option<&mut Object> {
        TransparentWrapper::peel_mut(self).as_object_mut()
    }
}

pub type Object = serde_json::Map<String, serde_json::Value>;

/// path: `/document/distribution/tlp/label`
#[derive(bytemuck::TransparentWrapper)]
#[repr(transparent)]
pub struct LabelOfTlp(serde_json::Value);

impl LabelOfTlp {
    pub fn v20_get(&self) -> serde_json::Result<csaf20::LabelOfTlp> {
        serde_json::from_value(TransparentWrapper::peel_ref(self).clone())
    }

    pub fn v21_set(&mut self, value: csaf21::LabelOfTlp) {
        *TransparentWrapper::peel_mut(self) = value.into();
    }
}
