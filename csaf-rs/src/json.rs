//! Helpers to work with JSON
//!
//! ## JSON source
//!
//! In order to avoid creating multiple combinations of functions reading from different JSON
//! sources (file, buffer, string, …) and still missing some, we use [`JsonSource`], which
//! implements whatever has to be done to parse a JSON document from the source.
//!
//! ```rust
//! # use std::fs;
//! # use std::path::Path;
//! # use csaf_rs::json::JsonSource;
//! # use csaf_rs::validation::CsafError;
//! # use serde_json::json;
//! #
//! # #[derive(serde::Deserialize)] struct MyDoc;
//! fn load_doc(s: impl JsonSource) -> Result<MyDoc, CsafError> {
//!   s.parse()
//! }
//!
//! fn examples() -> Result<(), CsafError> {
//!   let _ = load_doc(r#"{}"#)?; // load from JSON string
//!   let _ = load_doc(Path::new("file.json"))?; // load from file
//!   let _ = load_doc(json!({}))?; // load from serde_json::Value
//!   let _ = load_doc(&fs::read("file.json").map_err(CsafError::from)?)?; // load from byte array
//!
//!   Ok(())
//! }
//! ```
use crate::validation::CsafError;
use serde::de::DeserializeOwned;
use std::path::PathBuf;
use std::{fs, io::BufReader, path::Path};

/// Substring `serde_json` uses (as of this writing) to report that a string it was parsing
/// contained a byte sequence that isn't valid UTF-8. There is no public, stable API to
/// distinguish this specific failure from other syntax errors, so we match on the message
/// text; see `ErrorCode::InvalidUnicodeCodePoint` in `serde_json`'s source.
const INVALID_UNICODE_MARKER: &str = "invalid unicode code point";

fn map_json_error(path: String, error: serde_json::Error) -> CsafError {
    let message = error.to_string();
    if error.is_io() {
        CsafError::LoadError { message }
    } else if message.contains(INVALID_UNICODE_MARKER) {
        CsafError::WrongEncoding { path, message }
    } else {
        CsafError::InvalidJson {
            message: format!("{path}: {message}"),
        }
    }
}

/// Deserializes `T` from `deserializer`, converting any failure into a [`CsafError`] that
/// includes the JSON path (e.g. `document.tracking.id`) that was being deserialized when the
/// error occurred. This makes diagnosing malformed CSAF documents (invalid JSON, invalid
/// UTF-8, type mismatches, ...) far easier than the bare line/column serde_json otherwise
/// provides, especially for large documents.
///
/// CSAF documents must be valid JSON encoded using UTF-8. Invalid UTF-8 content is reported
/// distinctly as [`CsafError::WrongEncoding`], while all other parsing failures are reported
/// as [`CsafError::InvalidJson`].
fn deserialize_with_path<'de, R, T>(
    deserializer: &mut serde_json::Deserializer<R>,
) -> Result<T, CsafError>
where
    R: serde_json::de::Read<'de>,
    T: DeserializeOwned,
{
    let value = serde_path_to_error::deserialize(&mut *deserializer).map_err(|err| {
        map_json_error(err.path().to_string(), err.into_inner())
    })?;
    deserializer
        .end()
        .map_err(|error| map_json_error(String::new(), error))?;
    Ok(value)
}

fn deserialize_value_with_path<T>(value: serde_json::Value) -> Result<T, CsafError>
where
    T: DeserializeOwned,
{
    serde_path_to_error::deserialize(value).map_err(|err| {
        map_json_error(err.path().to_string(), err.into_inner())
    })
}

/// A source of JSON based information.
///
/// This trait allows to parse a JSON document, independent of the source. Avoiding repeated
/// implementations for "parsing from file", "parsing from string", "parsing from Value", ...
pub trait JsonSource {
    /// Parse the source
    fn parse<T: DeserializeOwned>(self) -> Result<T, CsafError>;
}

impl JsonSource for &Path {
    fn parse<T: DeserializeOwned>(self) -> Result<T, CsafError> {
        let mut de = serde_json::Deserializer::from_reader(BufReader::new(fs::File::open(self)?));
        deserialize_with_path(&mut de)
    }
}

impl JsonSource for &PathBuf {
    fn parse<T: DeserializeOwned>(self) -> Result<T, CsafError> {
        let mut de = serde_json::Deserializer::from_reader(BufReader::new(fs::File::open(self)?));
        deserialize_with_path(&mut de)
    }
}

impl JsonSource for fs::File {
    fn parse<T: DeserializeOwned>(self) -> Result<T, CsafError> {
        let mut de = serde_json::Deserializer::from_reader(BufReader::new(self));
        deserialize_with_path(&mut de)
    }
}

impl<R: std::io::Read> JsonSource for BufReader<R> {
    fn parse<T: DeserializeOwned>(self) -> Result<T, CsafError> {
        let mut de = serde_json::Deserializer::from_reader(self);
        deserialize_with_path(&mut de)
    }
}

impl JsonSource for &Vec<u8> {
    fn parse<T: DeserializeOwned>(self) -> Result<T, CsafError> {
        let mut de = serde_json::Deserializer::from_slice(self);
        deserialize_with_path(&mut de)
    }
}

impl JsonSource for &[u8] {
    fn parse<T: DeserializeOwned>(self) -> Result<T, CsafError> {
        let mut de = serde_json::Deserializer::from_slice(self);
        deserialize_with_path(&mut de)
    }
}

impl<const N: usize> JsonSource for &[u8; N] {
    fn parse<T: DeserializeOwned>(self) -> Result<T, CsafError> {
        let mut de = serde_json::Deserializer::from_slice(self);
        deserialize_with_path(&mut de)
    }
}

impl JsonSource for serde_json::Value {
    fn parse<T: DeserializeOwned>(self) -> Result<T, CsafError> {
        deserialize_value_with_path(self)
    }
}

impl JsonSource for &str {
    fn parse<T: DeserializeOwned>(self) -> Result<T, CsafError> {
        let mut de = serde_json::Deserializer::from_str(self);
        deserialize_with_path(&mut de)
    }
}

/// Convenience implementation for directly referencing `String`.
impl JsonSource for &String {
    fn parse<T: DeserializeOwned>(self) -> Result<T, CsafError> {
        let mut de = serde_json::Deserializer::from_str(self);
        deserialize_with_path(&mut de)
    }
}

/// A new-type to allow using [`JsonSource`] with anything that implements [`std::io::Read`].
pub struct Reader<R: std::io::Read>(pub R);

impl<R: std::io::Read> JsonSource for Reader<R> {
    fn parse<T: DeserializeOwned>(self) -> Result<T, CsafError> {
        let mut de = serde_json::Deserializer::from_reader(self.0);
        deserialize_with_path(&mut de)
    }
}

#[cfg(test)]
mod load_test {
    use super::*;
    use std::io::{Cursor, Seek, SeekFrom, Write};

    #[derive(Debug, serde::Deserialize)]
    struct TestDocument {}

    fn load<T: JsonSource>(source: T) -> Result<TestDocument, CsafError> {
        source.parse()
    }

    /// Test we can load a string
    #[test]
    fn from_str() {
        let _ = load("{}").expect("must load");
    }

    /// Test we can load a slice
    #[test]
    fn from_slice() {
        let _ = load(&b"{}"[..]).expect("must load");
    }

    /// Test an array (with a size) works like a slice
    #[test]
    fn from_array() {
        let _ = load(b"{}").expect("must load");
    }

    /// Test from a file handle
    #[test]
    fn from_file() {
        let mut t = tempfile::tempfile().expect("must create temp file");
        t.write_all(br#"{}"#).expect("must write to temp file");
        t.seek(SeekFrom::Start(0)).expect("must seek");

        let _ = load(t).expect("must load");
    }

    /// Test from a path to a file
    #[test]
    fn from_path() {
        let t = tempfile::tempdir().expect("must create temp dir");

        let path = t.path().join("test.json");
        fs::write(&path, br#"{}"#).expect("must write to temp file");

        let _ = load(&path).expect("must load");
        let _ = load(&path as &Path).expect("must load");

        t.close().expect("must close file");
    }

    /// Test a simple reader trait
    #[test]
    fn from_any_reader() {
        let s = br#"{}"#;
        let c = Cursor::new(s);

        let _ = load(Reader(c)).expect("must load");
    }
}
