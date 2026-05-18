mod extract;
mod generate;

pub(crate) use extract::*;
pub(crate) use generate::*;

use crate::build_errors::BuildError;
use crate::utils::codegen_snippets::{
    add_generated_code_header, add_ignore_clippy, add_ignore_dead_code, add_ignore_rustfmt,
};
use crate::utils::read_write_fs::write_generated_file;
use std::collections::BTreeMap;

/// All term translations, lookup via term -> lang -> translation
#[derive(Debug)]
pub(crate) struct Translations {
    pub license: LanguageTranslationLookup,
    pub product_description: LanguageTranslationLookup,
    pub reasoning_for_supersession: LanguageTranslationLookup,
    pub reasoning_for_withdrawal: LanguageTranslationLookup,
    pub superseding_document: LanguageTranslationLookup,
}

pub(crate) type LanguageTranslationLookup = BTreeMap<String, String>;

/// Generates the language specific translations lookup
pub fn generate_translations(target_folder: &str) -> Result<(), BuildError> {
    let translations = extract_translations("assets/language_specific_translations/translations.json")?;

    let tokens = generate_translation_lookups(&translations);

    let mut file: syn::File = syn::parse2(tokens)?;
    add_generated_code_header(&mut file);
    add_ignore_rustfmt(&mut file);
    add_ignore_clippy(&mut file);
    add_ignore_dead_code(&mut file);

    let code = prettyplease::unparse(&file);

    write_generated_file(
        target_folder,
        "src/validations/utils/language_specific_translations/generated.rs",
        &code,
        "generated language specific translations lookup",
    )?;

    Ok(())
}
