use std::fmt::Display;

use oxilangtag::LanguageTag;

use crate::{
    csaf::types::language::language_subtags::{is_language_subtag, is_region_subtag, is_script_subtag},
    validation::{IntoTestFindingError, TestFinding, TestFindingData},
};

/// Represents an error that occurred while parsing or validating a language tag in a CSAF document.
#[derive(Debug, PartialEq, Clone)]
pub enum CsafLanguageError {
    /// Grandfathered language tags that were deprecated before CSAF was specified are invalid
    GrandfatheredTagDeprecatedBeforeCSAF(String),
    /// Oxilangtag parsing failed
    ParserError(String, String),
    /// One or more subtags do not exist in the IANA registry or are in the wrong position
    InvalidSubtag {
        language_tag: String,
        lang_subtag_err: TagValidity,
        script_subtag_err: TagValidity,
        region_subtag_err: TagValidity,
    },
}

impl CsafLanguageError {
    pub(crate) fn subtag_builder(language_tag: LanguageTag<String>) -> InvalidSubtagErrorBuilder {
        InvalidSubtagErrorBuilder::new(language_tag)
    }
}

impl IntoTestFindingError for CsafLanguageError {
    fn into_test_finding_error(self, instance_path: &str) -> TestFinding {
        let message = match self {
            Self::ParserError(invalid_lang_tag, parser_error) => {
                format!("Invalid language code '{invalid_lang_tag}': parser failed with error: {parser_error}")
            },
            Self::InvalidSubtag {
                language_tag,
                lang_subtag_err,
                script_subtag_err,
                region_subtag_err,
            } => {
                let mut message = format!("Invalid language code: '{language_tag}'.");
                match lang_subtag_err {
                    TagValidity::Valid => (),
                    err => message = format!("{message} Language {err}."),
                }
                match script_subtag_err {
                    TagValidity::Valid => (),
                    err => message = format!("{message} Script {err}."),
                }
                match region_subtag_err {
                    TagValidity::Valid => (),
                    err => message = format!("{message} Region {err}."),
                }
                message
            },
            Self::GrandfatheredTagDeprecatedBeforeCSAF(invalid_lang_tag) => {
                format!(
                    "Invalid language code '{invalid_lang_tag}': grandfathered tag was deprecated before CSAF was specified"
                )
            },
        };
        TestFinding::Error(TestFindingData {
            message,
            instance_path: instance_path.to_string(),
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum TagValidity {
    #[default]
    Valid,
    InRegistryWrongPosition(String, &'static str),
    NotInRegistry(String),
}

impl Display for TagValidity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TagValidity::Valid => write!(f, "Valid"),
            TagValidity::InRegistryWrongPosition(subtag, kind) => {
                write!(f, "subtag is actually a {kind} subtag: '{subtag}'")
            },
            TagValidity::NotInRegistry(subtag) => write!(f, "subtag is not in the IANA registry: '{subtag}'"),
        }
    }
}

#[derive(Debug)]
pub(crate) struct InvalidSubtagErrorBuilder {
    lang_tag: LanguageTag<String>,
    primary_lang: TagValidity,
    script: TagValidity,
    region: TagValidity,
}

impl InvalidSubtagErrorBuilder {
    fn new(language_tag: LanguageTag<String>) -> Self {
        Self {
            lang_tag: language_tag,
            primary_lang: TagValidity::default(),
            script: TagValidity::default(),
            region: TagValidity::default(),
        }
    }

    pub(crate) fn check_language(mut self) -> Self {
        let language = self.lang_tag.primary_language();
        if is_language_subtag(language) {
            self.primary_lang = TagValidity::Valid;
        } else if is_script_subtag(language) {
            self.primary_lang = TagValidity::InRegistryWrongPosition(language.to_string(), "script");
        } else if is_region_subtag(language) {
            self.primary_lang = TagValidity::InRegistryWrongPosition(language.to_string(), "region");
        } else {
            self.primary_lang = TagValidity::NotInRegistry(language.to_string());
        }
        self
    }

    pub(crate) fn check_script(mut self) -> Self {
        let Some(script) = self.lang_tag.script() else {
            return self;
        };
        if is_script_subtag(script) {
            self.script = TagValidity::Valid;
        } else if is_language_subtag(script) {
            self.script = TagValidity::InRegistryWrongPosition(script.to_string(), "language");
        } else if is_region_subtag(script) {
            self.script = TagValidity::InRegistryWrongPosition(script.to_string(), "region");
        } else {
            self.script = TagValidity::NotInRegistry(script.to_string());
        }
        self
    }

    pub(crate) fn check_region(mut self) -> Self {
        let Some(region) = self.lang_tag.region() else {
            return self;
        };
        if is_region_subtag(region) {
            self.region = TagValidity::Valid;
        } else if is_language_subtag(region) {
            self.region = TagValidity::InRegistryWrongPosition(region.to_string(), "language");
        } else if is_script_subtag(region) {
            self.region = TagValidity::InRegistryWrongPosition(region.to_string(), "script");
        } else {
            self.region = TagValidity::NotInRegistry(region.to_string());
        }
        self
    }

    #[allow(clippy::result_large_err)]
    pub(crate) fn build(self) -> Result<(), CsafLanguageError> {
        match (self.primary_lang, self.script, self.region) {
            (TagValidity::Valid, TagValidity::Valid, TagValidity::Valid) => Ok(()),
            (lang, scr, reg) => Err(CsafLanguageError::InvalidSubtag {
                language_tag: self.lang_tag.to_string(),
                lang_subtag_err: lang,
                script_subtag_err: scr,
                region_subtag_err: reg,
            }),
        }
    }
}
