#![allow(dead_code)]
mod generated;
use generated::*;

use std::collections::HashMap;
use std::sync::LazyLock;

use crate::validation::{TestFinding, TestFindingData};

static LICENSE_MAP: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| LICENSE_TRANSLATIONS.iter().copied().collect());
static PRODUCT_DESCRIPTION_MAP: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| PRODUCT_DESCRIPTION_TRANSLATIONS.iter().copied().collect());
static REASONING_FOR_SUPERSESSION_MAP: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| REASONING_FOR_SUPERSESSION_TRANSLATIONS.iter().copied().collect());
static REASONING_FOR_WITHDRAWAL_MAP: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| REASONING_FOR_WITHDRAWAL_TRANSLATIONS.iter().copied().collect());
static SUPERSEDING_DOCUMENT_MAP: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| SUPERSEDING_DOCUMENT_TRANSLATIONS.iter().copied().collect());

/// Returns the translation of the term `license` for the given lang tag, if available.
pub fn get_translation_for_term_license(lang: &str) -> Option<&'static str> {
    LICENSE_MAP.get(lang).copied()
}

/// Returns the translation of the term `product_description` for the given lang tag, if available
pub fn get_translation_for_term_product_description(lang: &str) -> Option<&'static str> {
    PRODUCT_DESCRIPTION_MAP.get(lang).copied()
}

/// Returns the translation of the term `reasoning_for_supersession` for the given lang tag, if available
pub fn get_translation_for_term_reasoning_for_supersession(lang: &str) -> Option<&'static str> {
    REASONING_FOR_SUPERSESSION_MAP.get(lang).copied()
}

/// Returns the translation of the term `reasoning_for_withdrawal` for the given lang tag, if available
pub fn get_translation_for_term_reasoning_for_withdrawal(lang: &str) -> Option<&'static str> {
    REASONING_FOR_WITHDRAWAL_MAP.get(lang).copied()
}

/// Returns the translation of the term `superseding_document` for the given lang tag, if available
pub fn get_translation_for_term_superseding_document(lang: &str) -> Option<&'static str> {
    SUPERSEDING_DOCUMENT_MAP.get(lang).copied()
}

/// Creates a [`TestFinding::Information`] indicating that no translation for `term` is known for
/// the given language tag, and that the test was therefore skipped.
pub(crate) fn create_no_translation_known_info(term: &str, lang: &str) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: format!("No translation for '{term}' known for language '{lang}'. Test skipped."),
        instance_path: "/document/notes".to_string(),
    })
}
