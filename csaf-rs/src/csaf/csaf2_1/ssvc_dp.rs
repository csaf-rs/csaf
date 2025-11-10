/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///This schema defines the structure to represent an SSVC DecisionPoint object.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$id": "https://certcc.github.io/SSVC/data/schema/v2/DecisionPoint_2_0_0.schema.json",
///  "title": "DecisionPoint",
///  "description": "This schema defines the structure to represent an SSVC DecisionPoint object.",
///  "type": "object",
///  "required": [
///    "definition",
///    "key",
///    "name",
///    "namespace",
///    "schemaVersion",
///    "values"
///  ],
///  "properties": {
///    "definition": {
///      "title": "Definition",
///      "type": "string",
///      "minLength": 1
///    },
///    "key": {
///      "title": "Key",
///      "description": "A short, non-empty string identifier for the object. Keys must start with an alphanumeric, contain only alphanumerics and `_`, and end with an alphanumeric.(`T*` is explicitly grandfathered in as a valid key, but should not be used for new objects.)",
///      "examples": [
///        "E",
///        "A",
///        "SI",
///        "L",
///        "M",
///        "H",
///        "Mixed_case_OK",
///        "alph4num3ric"
///      ],
///      "type": "string",
///      "minLength": 1,
///      "pattern": "^(([a-zA-Z0-9])|([a-zA-Z0-9][a-zA-Z0-9_]*[a-zA-Z0-9])|(T\\*))$"
///    },
///    "name": {
///      "title": "Name",
///      "type": "string",
///      "minLength": 1
///    },
///    "namespace": {
///      "title": "Namespace",
///      "description": "The namespace of the SSVC object.",
///      "examples": [
///        "ssvc",
///        "cisa",
///        "x_example.test#test//.example.test#private-extension",
///        "ssvc/de-DE/.example.organization#reference-arch-1"
///      ],
///      "type": "string",
///      "maxLength": 1000,
///      "minLength": 3,
///      "pattern": "^(x_([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?(\\.([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?)+#(([a-z]|[0-9]))+((\\.|-)(([a-z]|[0-9]))+)*|[a-z]([a-z]|[0-9])(((\\.|-))?(([a-z]|[0-9]))+)+(#(([a-z]|[0-9]))+((\\.|-)(([a-z]|[0-9]))+)*)?)((/|/(([a-zA-Z]{2,3}(-[a-zA-Z]{3}(-[a-zA-Z]{3}){0,2})?|[a-zA-Z]{4,8})(-[a-zA-Z]{4})?(-([a-zA-Z]{2}|[0-9]{3}))?(-(([a-zA-Z0-9]){5,8}|[0-9]([a-zA-Z0-9]){3}))*(-[0-9A-WY-Za-wy-z](-([a-zA-Z0-9]){2,8})+)*(-[xX](-([a-zA-Z0-9]){2,8})+)?|[xX](-([a-zA-Z0-9]){2,8})+|i-default|i-mingo))((/((([a-zA-Z]{2,3}(-[a-zA-Z]{3}(-[a-zA-Z]{3}){0,2})?|[a-zA-Z]{4,8})(-[a-zA-Z]{4})?(-([a-zA-Z]{2}|[0-9]{3}))?(-(([a-zA-Z0-9]){5,8}|[0-9]([a-zA-Z0-9]){3}))*(-[0-9A-WY-Za-wy-z](-([a-zA-Z0-9]){2,8})+)*(-[xX](-([a-zA-Z0-9]){2,8})+)?|[xX](-([a-zA-Z0-9]){2,8})+|i-default|i-mingo)|\\.([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?(\\.([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?)+#(([a-z]|[0-9]))+((\\.|-)(([a-z]|[0-9]))+)*|\\.(([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?(\\.([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?)+|([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?(\\.([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?)+#(([a-z]|[0-9]))+((\\.|-)(([a-z]|[0-9]))+)*)\\$(([a-zA-Z]{2,3}(-[a-zA-Z]{3}(-[a-zA-Z]{3}){0,2})?|[a-zA-Z]{4,8})(-[a-zA-Z]{4})?(-([a-zA-Z]{2}|[0-9]{3}))?(-(([a-zA-Z0-9]){5,8}|[0-9]([a-zA-Z0-9]){3}))*(-[0-9A-WY-Za-wy-z](-([a-zA-Z0-9]){2,8})+)*(-[xX](-([a-zA-Z0-9]){2,8})+)?|[xX](-([a-zA-Z0-9]){2,8})+|i-default|i-mingo)))+)?)?$"
///    },
///    "schemaVersion": {
///      "title": "Schemaversion",
///      "type": "string",
///      "const": "2.0.0"
///    },
///    "values": {
///      "title": "Values",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/DecisionPointValue"
///      }
///    },
///    "version": {
///      "title": "Version",
///      "description": "The version of the SSVC object. This must be a valid semantic version string.",
///      "default": "0.0.1",
///      "examples": [
///        "1.0.0",
///        "2.1.3"
///      ],
///      "type": "string",
///      "minLength": 5,
///      "pattern": "^(0|[1-9]\\d*)\\.(0|[1-9]\\d*)\\.(0|[1-9]\\d*)(?:-((?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\\.(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\\+([0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*))?$"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, PartialEq)]
pub struct DecisionPoint {
    pub definition: Definition,
    ///A short, non-empty string identifier for the object. Keys must start with an alphanumeric, contain only alphanumerics and `_`, and end with an alphanumeric.(`T*` is explicitly grandfathered in as a valid key, but should not be used for new objects.)
    pub key: Key,
    pub name: Name,
    ///The namespace of the SSVC object.
    pub namespace: Namespace,
    #[serde(rename = "schemaVersion")]
    pub schema_version: ::std::string::String,
    pub values: ::std::vec::Vec<DecisionPointValue>,
    ///The version of the SSVC object. This must be a valid semantic version string.
    #[serde(default = "defaults::decision_point_version")]
    pub version: Version,
}
impl ::std::convert::From<&DecisionPoint> for DecisionPoint {
    fn from(value: &DecisionPoint) -> Self {
        value.clone()
    }
}
impl DecisionPoint {
    pub fn builder() -> builder::DecisionPoint {
        Default::default()
    }
}
/**Models a single value option for a decision point.

Each value should have the following attributes:

- name (str): A name
- description (str): A description
- key (str): A key (a short, unique string) that can be used to identify the value in a shorthand way
- _comment (str): An optional comment that will be included in the object.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "DecisionPointValue",
///  "description": "Models a single value option for a decision point.\n\nEach value should have the following attributes:\n\n- name (str): A name\n- description (str): A description\n- key (str): A key (a short, unique string) that can be used to identify the value in a shorthand way\n- _comment (str): An optional comment that will be included in the object.",
///  "type": "object",
///  "required": [
///    "definition",
///    "key",
///    "name"
///  ],
///  "properties": {
///    "definition": {
///      "title": "Definition",
///      "type": "string",
///      "minLength": 1
///    },
///    "key": {
///      "title": "Key",
///      "description": "A short, non-empty string identifier for the object. Keys must start with an alphanumeric, contain only alphanumerics and `_`, and end with an alphanumeric.(`T*` is explicitly grandfathered in as a valid key, but should not be used for new objects.)",
///      "examples": [
///        "E",
///        "A",
///        "SI",
///        "L",
///        "M",
///        "H",
///        "Mixed_case_OK",
///        "alph4num3ric"
///      ],
///      "type": "string",
///      "minLength": 1,
///      "pattern": "^(([a-zA-Z0-9])|([a-zA-Z0-9][a-zA-Z0-9_]*[a-zA-Z0-9])|(T\\*))$"
///    },
///    "name": {
///      "title": "Name",
///      "type": "string",
///      "minLength": 1
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, PartialEq)]
pub struct DecisionPointValue {
    pub definition: Definition,
    ///A short, non-empty string identifier for the object. Keys must start with an alphanumeric, contain only alphanumerics and `_`, and end with an alphanumeric.(`T*` is explicitly grandfathered in as a valid key, but should not be used for new objects.)
    pub key: Key,
    pub name: Name,
}
impl ::std::convert::From<&DecisionPointValue> for DecisionPointValue {
    fn from(value: &DecisionPointValue) -> Self {
        value.clone()
    }
}
impl DecisionPointValue {
    pub fn builder() -> builder::DecisionPointValue {
        Default::default()
    }
}
///`Definition`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Definition",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Definition(::std::string::String);
impl ::std::ops::Deref for Definition {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Definition> for ::std::string::String {
    fn from(value: Definition) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Definition> for Definition {
    fn from(value: &Definition) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Definition {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Definition {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Definition {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Definition {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Definition {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///A short, non-empty string identifier for the object. Keys must start with an alphanumeric, contain only alphanumerics and `_`, and end with an alphanumeric.(`T*` is explicitly grandfathered in as a valid key, but should not be used for new objects.)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Key",
///  "description": "A short, non-empty string identifier for the object. Keys must start with an alphanumeric, contain only alphanumerics and `_`, and end with an alphanumeric.(`T*` is explicitly grandfathered in as a valid key, but should not be used for new objects.)",
///  "examples": [
///    "E",
///    "A",
///    "SI",
///    "L",
///    "M",
///    "H",
///    "Mixed_case_OK",
///    "alph4num3ric"
///  ],
///  "type": "string",
///  "minLength": 1,
///  "pattern": "^(([a-zA-Z0-9])|([a-zA-Z0-9][a-zA-Z0-9_]*[a-zA-Z0-9])|(T\\*))$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Key(::std::string::String);
impl ::std::ops::Deref for Key {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Key> for ::std::string::String {
    fn from(value: Key) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Key> for Key {
    fn from(value: &Key) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Key {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^(([a-zA-Z0-9])|([a-zA-Z0-9][a-zA-Z0-9_]*[a-zA-Z0-9])|(T\\*))$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^(([a-zA-Z0-9])|([a-zA-Z0-9][a-zA-Z0-9_]*[a-zA-Z0-9])|(T\\*))$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Key {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Key {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Key {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Key {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`Name`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Name",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Name(::std::string::String);
impl ::std::ops::Deref for Name {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Name> for ::std::string::String {
    fn from(value: Name) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Name> for Name {
    fn from(value: &Name) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Name {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Name {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Name {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Name {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///The namespace of the SSVC object.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Namespace",
///  "description": "The namespace of the SSVC object.",
///  "examples": [
///    "ssvc",
///    "cisa",
///    "x_example.test#test//.example.test#private-extension",
///    "ssvc/de-DE/.example.organization#reference-arch-1"
///  ],
///  "type": "string",
///  "maxLength": 1000,
///  "minLength": 3,
///  "pattern": "^(x_([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?(\\.([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?)+#(([a-z]|[0-9]))+((\\.|-)(([a-z]|[0-9]))+)*|[a-z]([a-z]|[0-9])(((\\.|-))?(([a-z]|[0-9]))+)+(#(([a-z]|[0-9]))+((\\.|-)(([a-z]|[0-9]))+)*)?)((/|/(([a-zA-Z]{2,3}(-[a-zA-Z]{3}(-[a-zA-Z]{3}){0,2})?|[a-zA-Z]{4,8})(-[a-zA-Z]{4})?(-([a-zA-Z]{2}|[0-9]{3}))?(-(([a-zA-Z0-9]){5,8}|[0-9]([a-zA-Z0-9]){3}))*(-[0-9A-WY-Za-wy-z](-([a-zA-Z0-9]){2,8})+)*(-[xX](-([a-zA-Z0-9]){2,8})+)?|[xX](-([a-zA-Z0-9]){2,8})+|i-default|i-mingo))((/((([a-zA-Z]{2,3}(-[a-zA-Z]{3}(-[a-zA-Z]{3}){0,2})?|[a-zA-Z]{4,8})(-[a-zA-Z]{4})?(-([a-zA-Z]{2}|[0-9]{3}))?(-(([a-zA-Z0-9]){5,8}|[0-9]([a-zA-Z0-9]){3}))*(-[0-9A-WY-Za-wy-z](-([a-zA-Z0-9]){2,8})+)*(-[xX](-([a-zA-Z0-9]){2,8})+)?|[xX](-([a-zA-Z0-9]){2,8})+|i-default|i-mingo)|\\.([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?(\\.([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?)+#(([a-z]|[0-9]))+((\\.|-)(([a-z]|[0-9]))+)*|\\.(([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?(\\.([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?)+|([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?(\\.([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?)+#(([a-z]|[0-9]))+((\\.|-)(([a-z]|[0-9]))+)*)\\$(([a-zA-Z]{2,3}(-[a-zA-Z]{3}(-[a-zA-Z]{3}){0,2})?|[a-zA-Z]{4,8})(-[a-zA-Z]{4})?(-([a-zA-Z]{2}|[0-9]{3}))?(-(([a-zA-Z0-9]){5,8}|[0-9]([a-zA-Z0-9]){3}))*(-[0-9A-WY-Za-wy-z](-([a-zA-Z0-9]){2,8})+)*(-[xX](-([a-zA-Z0-9]){2,8})+)?|[xX](-([a-zA-Z0-9]){2,8})+|i-default|i-mingo)))+)?)?$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Namespace(::std::string::String);
impl ::std::ops::Deref for Namespace {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Namespace> for ::std::string::String {
    fn from(value: Namespace) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Namespace> for Namespace {
    fn from(value: &Namespace) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Namespace {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1000usize {
            return Err("longer than 1000 characters".into());
        }
        if value.chars().count() < 3usize {
            return Err("shorter than 3 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^(x_([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?(\\.([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?)+#(([a-z]|[0-9]))+((\\.|-)(([a-z]|[0-9]))+)*|[a-z]([a-z]|[0-9])(((\\.|-))?(([a-z]|[0-9]))+)+(#(([a-z]|[0-9]))+((\\.|-)(([a-z]|[0-9]))+)*)?)((/|/(([a-zA-Z]{2,3}(-[a-zA-Z]{3}(-[a-zA-Z]{3}){0,2})?|[a-zA-Z]{4,8})(-[a-zA-Z]{4})?(-([a-zA-Z]{2}|[0-9]{3}))?(-(([a-zA-Z0-9]){5,8}|[0-9]([a-zA-Z0-9]){3}))*(-[0-9A-WY-Za-wy-z](-([a-zA-Z0-9]){2,8})+)*(-[xX](-([a-zA-Z0-9]){2,8})+)?|[xX](-([a-zA-Z0-9]){2,8})+|i-default|i-mingo))((/((([a-zA-Z]{2,3}(-[a-zA-Z]{3}(-[a-zA-Z]{3}){0,2})?|[a-zA-Z]{4,8})(-[a-zA-Z]{4})?(-([a-zA-Z]{2}|[0-9]{3}))?(-(([a-zA-Z0-9]){5,8}|[0-9]([a-zA-Z0-9]){3}))*(-[0-9A-WY-Za-wy-z](-([a-zA-Z0-9]){2,8})+)*(-[xX](-([a-zA-Z0-9]){2,8})+)?|[xX](-([a-zA-Z0-9]){2,8})+|i-default|i-mingo)|\\.([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?(\\.([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?)+#(([a-z]|[0-9]))+((\\.|-)(([a-z]|[0-9]))+)*|\\.(([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?(\\.([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?)+|([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?(\\.([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?)+#(([a-z]|[0-9]))+((\\.|-)(([a-z]|[0-9]))+)*)\\$(([a-zA-Z]{2,3}(-[a-zA-Z]{3}(-[a-zA-Z]{3}){0,2})?|[a-zA-Z]{4,8})(-[a-zA-Z]{4})?(-([a-zA-Z]{2}|[0-9]{3}))?(-(([a-zA-Z0-9]){5,8}|[0-9]([a-zA-Z0-9]){3}))*(-[0-9A-WY-Za-wy-z](-([a-zA-Z0-9]){2,8})+)*(-[xX](-([a-zA-Z0-9]){2,8})+)?|[xX](-([a-zA-Z0-9]){2,8})+|i-default|i-mingo)))+)?)?$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^(x_([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?(\\.([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?)+#(([a-z]|[0-9]))+((\\.|-)(([a-z]|[0-9]))+)*|[a-z]([a-z]|[0-9])(((\\.|-))?(([a-z]|[0-9]))+)+(#(([a-z]|[0-9]))+((\\.|-)(([a-z]|[0-9]))+)*)?)((/|/(([a-zA-Z]{2,3}(-[a-zA-Z]{3}(-[a-zA-Z]{3}){0,2})?|[a-zA-Z]{4,8})(-[a-zA-Z]{4})?(-([a-zA-Z]{2}|[0-9]{3}))?(-(([a-zA-Z0-9]){5,8}|[0-9]([a-zA-Z0-9]){3}))*(-[0-9A-WY-Za-wy-z](-([a-zA-Z0-9]){2,8})+)*(-[xX](-([a-zA-Z0-9]){2,8})+)?|[xX](-([a-zA-Z0-9]){2,8})+|i-default|i-mingo))((/((([a-zA-Z]{2,3}(-[a-zA-Z]{3}(-[a-zA-Z]{3}){0,2})?|[a-zA-Z]{4,8})(-[a-zA-Z]{4})?(-([a-zA-Z]{2}|[0-9]{3}))?(-(([a-zA-Z0-9]){5,8}|[0-9]([a-zA-Z0-9]){3}))*(-[0-9A-WY-Za-wy-z](-([a-zA-Z0-9]){2,8})+)*(-[xX](-([a-zA-Z0-9]){2,8})+)?|[xX](-([a-zA-Z0-9]){2,8})+|i-default|i-mingo)|\\.([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?(\\.([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?)+#(([a-z]|[0-9]))+((\\.|-)(([a-z]|[0-9]))+)*|\\.(([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?(\\.([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?)+|([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?(\\.([a-z]|[0-9])(((([a-z]|[0-9])|-)){0,61}([a-z]|[0-9]))?)+#(([a-z]|[0-9]))+((\\.|-)(([a-z]|[0-9]))+)*)\\$(([a-zA-Z]{2,3}(-[a-zA-Z]{3}(-[a-zA-Z]{3}){0,2})?|[a-zA-Z]{4,8})(-[a-zA-Z]{4})?(-([a-zA-Z]{2}|[0-9]{3}))?(-(([a-zA-Z0-9]){5,8}|[0-9]([a-zA-Z0-9]){3}))*(-[0-9A-WY-Za-wy-z](-([a-zA-Z0-9]){2,8})+)*(-[xX](-([a-zA-Z0-9]){2,8})+)?|[xX](-([a-zA-Z0-9]){2,8})+|i-default|i-mingo)))+)?)?$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Namespace {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Namespace {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Namespace {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Namespace {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///The version of the SSVC object. This must be a valid semantic version string.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Version",
///  "description": "The version of the SSVC object. This must be a valid semantic version string.",
///  "default": "0.0.1",
///  "examples": [
///    "1.0.0",
///    "2.1.3"
///  ],
///  "type": "string",
///  "minLength": 5,
///  "pattern": "^(0|[1-9]\\d*)\\.(0|[1-9]\\d*)\\.(0|[1-9]\\d*)(?:-((?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\\.(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\\+([0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*))?$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Version(::std::string::String);
impl ::std::ops::Deref for Version {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Version> for ::std::string::String {
    fn from(value: Version) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Version> for Version {
    fn from(value: &Version) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Version {
    fn default() -> Self {
        Version("0.0.1".to_string())
    }
}
impl ::std::str::FromStr for Version {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 5usize {
            return Err("shorter than 5 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^(0|[1-9]\\d*)\\.(0|[1-9]\\d*)\\.(0|[1-9]\\d*)(?:-((?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\\.(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\\+([0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*))?$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^(0|[1-9]\\d*)\\.(0|[1-9]\\d*)\\.(0|[1-9]\\d*)(?:-((?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\\.(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\\+([0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*))?$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Version {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Version {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Version {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct DecisionPoint {
        definition: ::std::result::Result<super::Definition, ::std::string::String>,
        key: ::std::result::Result<super::Key, ::std::string::String>,
        name: ::std::result::Result<super::Name, ::std::string::String>,
        namespace: ::std::result::Result<super::Namespace, ::std::string::String>,
        schema_version: ::std::result::Result<
            ::std::string::String,
            ::std::string::String,
        >,
        values: ::std::result::Result<
            ::std::vec::Vec<super::DecisionPointValue>,
            ::std::string::String,
        >,
        version: ::std::result::Result<super::Version, ::std::string::String>,
    }
    impl ::std::default::Default for DecisionPoint {
        fn default() -> Self {
            Self {
                definition: Err("no value supplied for definition".to_string()),
                key: Err("no value supplied for key".to_string()),
                name: Err("no value supplied for name".to_string()),
                namespace: Err("no value supplied for namespace".to_string()),
                schema_version: Err("no value supplied for schema_version".to_string()),
                values: Err("no value supplied for values".to_string()),
                version: Ok(super::defaults::decision_point_version()),
            }
        }
    }
    impl DecisionPoint {
        pub fn definition<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Definition>,
            T::Error: ::std::fmt::Display,
        {
            self.definition = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for definition: {}", e)
                });
            self
        }
        pub fn key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Key>,
            T::Error: ::std::fmt::Display,
        {
            self.key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for key: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Name>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn namespace<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Namespace>,
            T::Error: ::std::fmt::Display,
        {
            self.namespace = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for namespace: {}", e)
                });
            self
        }
        pub fn schema_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.schema_version = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for schema_version: {}", e)
                });
            self
        }
        pub fn values<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::DecisionPointValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.values = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for values: {}", e)
                });
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Version>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for version: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<DecisionPoint> for super::DecisionPoint {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DecisionPoint,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                definition: value.definition?,
                key: value.key?,
                name: value.name?,
                namespace: value.namespace?,
                schema_version: value.schema_version?,
                values: value.values?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::DecisionPoint> for DecisionPoint {
        fn from(value: super::DecisionPoint) -> Self {
            Self {
                definition: Ok(value.definition),
                key: Ok(value.key),
                name: Ok(value.name),
                namespace: Ok(value.namespace),
                schema_version: Ok(value.schema_version),
                values: Ok(value.values),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DecisionPointValue {
        definition: ::std::result::Result<super::Definition, ::std::string::String>,
        key: ::std::result::Result<super::Key, ::std::string::String>,
        name: ::std::result::Result<super::Name, ::std::string::String>,
    }
    impl ::std::default::Default for DecisionPointValue {
        fn default() -> Self {
            Self {
                definition: Err("no value supplied for definition".to_string()),
                key: Err("no value supplied for key".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl DecisionPointValue {
        pub fn definition<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Definition>,
            T::Error: ::std::fmt::Display,
        {
            self.definition = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for definition: {}", e)
                });
            self
        }
        pub fn key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Key>,
            T::Error: ::std::fmt::Display,
        {
            self.key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for key: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Name>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<DecisionPointValue> for super::DecisionPointValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DecisionPointValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                definition: value.definition?,
                key: value.key?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::DecisionPointValue> for DecisionPointValue {
        fn from(value: super::DecisionPointValue) -> Self {
            Self {
                definition: Ok(value.definition),
                key: Ok(value.key),
                name: Ok(value.name),
            }
        }
    }
}
/// Generation of default values for serde.
pub mod defaults {
    pub(super) fn decision_point_version() -> super::Version {
        super::Version("0.0.1".to_string())
    }
}
