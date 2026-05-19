use crate::build_errors::BuildError;
use crate::utils::read_write_fs::read_file_to_string;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

use super::Translations;

/// The raw JSON structure of `translations.json`, used for deserialization
#[derive(Deserialize)]
struct JsonTranslationsFile {
    translation: BTreeMap<String, JsonTermTranslation>,
}

/// A single language's term translations, used for deserialization
#[derive(Deserialize)]
struct JsonTermTranslation {
    license: String,
    product_description: String,
    reasoning_for_supersession: String,
    reasoning_for_withdrawal: String,
    superseding_document: String,
}

/// Reads `translations.json` and returns a [`Translations`] struct
/// with per-term language lookups
pub(crate) fn extract_translations(path: &str) -> Result<Translations, BuildError> {
    let content = read_file_to_string(Path::new(path))?;
    let file: JsonTranslationsFile = serde_json::from_str(&content)?;

    // Reorder the lookup from lang -> term -> translation to term -> lang -> translation
    let mut translations = Translations {
        license: BTreeMap::new(),
        product_description: BTreeMap::new(),
        reasoning_for_supersession: BTreeMap::new(),
        reasoning_for_withdrawal: BTreeMap::new(),
        superseding_document: BTreeMap::new(),
    };

    for (lang, term) in file.translation {
        translations.license.insert(lang.clone(), term.license);
        translations
            .product_description
            .insert(lang.clone(), term.product_description);
        translations
            .reasoning_for_supersession
            .insert(lang.clone(), term.reasoning_for_supersession);
        translations
            .reasoning_for_withdrawal
            .insert(lang.clone(), term.reasoning_for_withdrawal);
        translations
            .superseding_document
            .insert(lang, term.superseding_document);
    }

    Ok(translations)
}
