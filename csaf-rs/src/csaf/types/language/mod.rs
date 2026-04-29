pub mod csaf_language;
pub mod invalid_language;
pub mod valid_language;

#[path = "language_subtags.generated.rs"]
mod language_subtags;

pub use csaf_language::CsafLanguage;
