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
//! # use csaf::json::JsonSource;
//! # use serde_json::json;
//! #
//! # #[derive(serde::Deserialize)] struct MyDoc;
//! fn load_doc(s: impl JsonSource) -> std::io::Result<MyDoc> {
//!   s.parse()
//! }
//!
//! fn examples() -> std::io::Result<()> {
//!   let _ = load_doc(r#"{}"#)?; // load from JSON string
//!   let _ = load_doc(Path::new("file.json"))?; // load from file
//!   let _ = load_doc(json!({}))?; // load from serde_json::Value
//!   let _ = load_doc(&fs::read("file.json")?)?; // load from byte array
//!
//!   Ok(())
//! }
//! ```
use serde::de::DeserializeOwned;
use std::path::PathBuf;
use std::{fs, io::BufReader, path::Path};

/// A source of JSON based information.
///
/// This trait allows to parse a JSON document, independent of the source. Avoiding repeated
/// implementations for "parsing from file", "parsing from string", "parsing from Value", ...
pub trait JsonSource {
    /// Parse the source
    fn parse<T: DeserializeOwned>(self) -> Result<T, std::io::Error>;
}

impl JsonSource for &Path {
    fn parse<T: DeserializeOwned>(self) -> Result<T, std::io::Error> {
        Ok(serde_json::from_reader(BufReader::new(fs::File::open(self)?))?)
    }
}

impl JsonSource for &PathBuf {
    fn parse<T: DeserializeOwned>(self) -> Result<T, std::io::Error> {
        Ok(serde_json::from_reader(BufReader::new(fs::File::open(self)?))?)
    }
}

impl JsonSource for fs::File {
    fn parse<T: DeserializeOwned>(self) -> Result<T, std::io::Error> {
        Ok(serde_json::from_reader(BufReader::new(self))?)
    }
}

impl<R: std::io::Read> JsonSource for BufReader<R> {
    fn parse<T: DeserializeOwned>(self) -> Result<T, std::io::Error> {
        Ok(serde_json::from_reader(self)?)
    }
}

impl JsonSource for &Vec<u8> {
    fn parse<T: DeserializeOwned>(self) -> Result<T, std::io::Error> {
        Ok(serde_json::from_slice(self)?)
    }
}

impl JsonSource for &[u8] {
    fn parse<T: DeserializeOwned>(self) -> Result<T, std::io::Error> {
        Ok(serde_json::from_slice(self)?)
    }
}

impl<const N: usize> JsonSource for &[u8; N] {
    fn parse<T: DeserializeOwned>(self) -> Result<T, std::io::Error> {
        Ok(serde_json::from_slice(self)?)
    }
}

impl JsonSource for serde_json::Value {
    fn parse<T: DeserializeOwned>(self) -> Result<T, std::io::Error> {
        Ok(serde_json::from_value(self)?)
    }
}

impl JsonSource for &str {
    fn parse<T: DeserializeOwned>(self) -> Result<T, std::io::Error> {
        Ok(serde_json::from_str(self)?)
    }
}

/// Convenience implementation for directly referencing `String`.
impl JsonSource for &String {
    fn parse<T: DeserializeOwned>(self) -> Result<T, std::io::Error> {
        Ok(serde_json::from_str(self)?)
    }
}

/// A new-type to allow using [`JsonSource`] with anything that implements [`std::io::Read`].
pub struct Reader<R: std::io::Read>(pub R);

impl<R: std::io::Read> JsonSource for Reader<R> {
    fn parse<T: DeserializeOwned>(self) -> Result<T, std::io::Error> {
        Ok(serde_json::from_reader(self.0)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::{Cursor, Seek, SeekFrom, Write};

    #[derive(Debug, serde::Deserialize)]
    struct TestDocument {}

    fn load<T: JsonSource>(source: T) -> std::io::Result<TestDocument> {
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
