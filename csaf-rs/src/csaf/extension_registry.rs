//! A registry of CSAF 2.1 extensions known to this process.
//!
//! The tests 6.1.60.2 (Extension Schema) and 6.1.60.3 (Extension Metadata) need the extension schema and the extension
//! metadata of every extension that is used in a document. Neither is part of the document, and fetching them from the
//! `$schema` URL would require network access. Therefore, extensions have to be registered *before* validation, e.g.
//! from a local directory with [`register_extensions_from_dir`].
//!
//! Extensions that are not registered are not supported by the implementation, which results in a warning
//! (as required by the specification) instead of an error.
//!
//! The registry is process-wide and can be extended at any time.

use jsonschema::{Draft, Retrieve, Uri, Validator};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Arc, LazyLock, RwLock};

/// Value of `$schema` in a CSAF Extension Schema.
const EXTENSION_SCHEMA_META_URL: &str = "https://docs.oasis-open.org/csaf/csaf/v2.1/schema/extension-metaschema.json";
/// Value of `$schema` in a CSAF Extension Metadata file.
const EXTENSION_METADATA_META_URL: &str = "https://docs.oasis-open.org/csaf/csaf/v2.1/schema/extension-metadata.json";

/// Schemas that extension schemas may reference, so that no network access is needed to resolve them.
const LOCAL_SCHEMAS: [(&str, &str); 9] = [
    (
        "https://docs.oasis-open.org/csaf/csaf/v2.1/schema/csaf.json",
        include_str!("../../assets/csaf_2.1_json_schema.json"),
    ),
    (
        "https://docs.oasis-open.org/csaf/csaf/v2.1/schema/meta.json",
        include_str!("../../assets/meta.json"),
    ),
    (
        EXTENSION_SCHEMA_META_URL,
        include_str!("../../assets/extension-metaschema.json"),
    ),
    (
        "https://docs.oasis-open.org/csaf/csaf/v2.1/schema/extension-content.json",
        include_str!("../../assets/extension-content.json"),
    ),
    (
        "https://www.first.org/cvss/cvss-v2.0.json",
        include_str!("../../assets/cvss-v2.0.json"),
    ),
    (
        "https://www.first.org/cvss/cvss-v3.0.json",
        include_str!("../../assets/cvss-v3.0.json"),
    ),
    (
        "https://www.first.org/cvss/cvss-v3.1.json",
        include_str!("../../assets/cvss-v3.1.json"),
    ),
    (
        "https://www.first.org/cvss/cvss-v4.0.json",
        include_str!("../../assets/cvss-v4.0.json"),
    ),
    (
        "https://certcc.github.io/SSVC/data/schema/v2/SelectionList_2_0_0.schema.json",
        include_str!("../../assets/SelectionList_2_0_0.schema.json"),
    ),
];

/// Resolves references of extension schemas to the schemas embedded in this crate, never over the network.
struct LocalRetriever;

impl Retrieve for LocalRetriever {
    fn retrieve(&self, uri: &Uri<String>) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        LOCAL_SCHEMAS
            .iter()
            .find(|(known, _)| *known == uri.as_str())
            .map(|(_, content)| {
                let mut schema: Value = serde_json::from_str(content)?;
                // the custom metaschema of CVSS 4.0 is not available either, the schema is a plain 2020-12 schema
                if schema.get("$schema").and_then(Value::as_str) == Some("https://www.first.org/cvss/meta.json")
                    && let Some(object) = schema.as_object_mut()
                {
                    object.remove("$schema");
                }
                Ok(schema)
            })
            .unwrap_or_else(|| Err(format!("`{uri}` is not available offline").into()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ExtensionRegistryError {
    #[error("The extension schema has no `$id`")]
    MissingSchemaId,
    #[error("The extension metadata has no `extension_schema`")]
    MissingExtensionSchema,
    #[error("The extension metadata of `{0}` is malformed: {1}")]
    MalformedMetadata(String, String),
    #[error("The extension schema `{0}` can't be compiled: {1}")]
    InvalidSchema(String, String),
    #[error("Can't read `{0}`: {1}")]
    Io(String, String),
}

/// The metadata of an extension, as far as it is needed for validation.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExtensionMetadata {
    /// The `$id`s of the extension schemas that must not be used together with this extension.
    pub incompatible_extensions: Vec<String>,
    /// Maximum number of occurrences of the extension, by JSONPath of the location, e.g. `$.x_extensions`.
    pub max_occurrence: BTreeMap<String, u64>,
}

#[derive(Default)]
struct KnownExtensions {
    validators: BTreeMap<String, Arc<Validator>>,
    metadata: BTreeMap<String, Arc<ExtensionMetadata>>,
}

static REGISTRY: LazyLock<RwLock<KnownExtensions>> = LazyLock::new(RwLock::default);

/// Registers a CSAF Extension Schema (the "content" schema). Returns its `$id`, i.e. the value that extension instances
/// carry in `$schema`. Registering the same `$id` again replaces the earlier registration.
pub fn register_extension_schema(schema: &Value) -> Result<String, ExtensionRegistryError> {
    let id = schema
        .get("$id")
        .and_then(Value::as_str)
        .ok_or(ExtensionRegistryError::MissingSchemaId)?
        .to_string();

    // The `$schema` of an extension schema is the CSAF extension metaschema, which can't be resolved by the validator
    // without network access. All extension schemas are JSON Schema 2020-12 schemas, so it is dropped here.
    let mut schema = schema.clone();
    if let Some(object) = schema.as_object_mut() {
        object.remove("$schema");
    }
    let validator = jsonschema::options()
        .with_draft(Draft::Draft202012)
        .with_retriever(LocalRetriever)
        .should_validate_formats(true)
        .build(&schema)
        .map_err(|e| ExtensionRegistryError::InvalidSchema(id.clone(), e.to_string()))?;

    REGISTRY
        .write()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .validators
        .insert(id.clone(), Arc::new(validator));
    Ok(id)
}

/// Registers the CSAF Extension Metadata of an extension. Returns the `$id` of the extension schema it belongs to.
pub fn register_extension_metadata(metadata: &Value) -> Result<String, ExtensionRegistryError> {
    let extension_schema = metadata
        .get("extension_schema")
        .and_then(Value::as_str)
        .ok_or(ExtensionRegistryError::MissingExtensionSchema)?
        .to_string();
    let malformed = |msg: &str| ExtensionRegistryError::MalformedMetadata(extension_schema.clone(), msg.to_string());

    let incompatible_extensions = metadata
        .get("incompatible_extensions")
        .map(|v| {
            v.as_array()
                .ok_or_else(|| malformed("`incompatible_extensions` is not an array"))?
                .iter()
                .map(|e| {
                    e.as_str()
                        .map(str::to_string)
                        .ok_or_else(|| malformed("`incompatible_extensions` contains a non-string"))
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?
        .unwrap_or_default();

    let max_occurrence = metadata
        .get("requirement")
        .and_then(Value::as_object)
        .ok_or_else(|| malformed("`requirement` is missing"))?
        .iter()
        .map(|(path, requirement)| {
            requirement
                .get("max_occurrence")
                .and_then(Value::as_u64)
                .map(|max| (path.clone(), max))
                .ok_or_else(|| malformed("`max_occurrence` is missing or not a number"))
        })
        .collect::<Result<BTreeMap<_, _>, _>>()?;

    REGISTRY
        .write()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .metadata
        .insert(
            extension_schema.clone(),
            Arc::new(ExtensionMetadata {
                incompatible_extensions,
                max_occurrence,
            }),
        );
    Ok(extension_schema)
}

/// Registers all extension schemas and extension metadata files found (recursively) in `dir`. Files are recognized by
/// their `$schema` value, all other JSON files are ignored. Returns the number of registered files, or the first error: a file that is recognized but can't be registered is not skipped silently.
pub fn register_extensions_from_dir(dir: &Path) -> Result<usize, ExtensionRegistryError> {
    let io_error =
        |path: &Path, e: std::io::Error| ExtensionRegistryError::Io(path.display().to_string(), e.to_string());
    let mut registered = 0;
    let mut entries = std::fs::read_dir(dir)
        .map_err(|e| io_error(dir, e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| io_error(dir, e))?;
    entries.sort_by_key(std::fs::DirEntry::path);

    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            registered += register_extensions_from_dir(&path)?;
        } else if path.extension().is_some_and(|e| e == "json") {
            let content = std::fs::read_to_string(&path).map_err(|e| io_error(&path, e))?;
            let Ok(json) = serde_json::from_str::<Value>(&content) else {
                continue;
            };
            match json.get("$schema").and_then(Value::as_str) {
                Some(EXTENSION_SCHEMA_META_URL) => {
                    register_extension_schema(&json)?;
                    registered += 1;
                },
                Some(EXTENSION_METADATA_META_URL) => {
                    register_extension_metadata(&json)?;
                    registered += 1;
                },
                _ => {},
            }
        }
    }
    Ok(registered)
}

/// The compiled extension schema registered under `schema_id`, if any.
pub fn get_extension_validator(schema_id: &str) -> Option<Arc<Validator>> {
    REGISTRY
        .read()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .validators
        .get(schema_id)
        .cloned()
}

/// The extension metadata registered for the extension schema `schema_id`, if any.
pub fn get_extension_metadata(schema_id: &str) -> Option<Arc<ExtensionMetadata>> {
    REGISTRY
        .read()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .metadata
        .get(schema_id)
        .cloned()
}
