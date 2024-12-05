/// Error types.
pub mod error {
    /// Error from a TryFrom or FromStr implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
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
///Acknowledges contributions by describing those that contributed.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Acknowledgment",
///  "description": "Acknowledges contributions by describing those that contributed.",
///  "type": "object",
///  "minProperties": 1,
///  "properties": {
///    "names": {
///      "title": "List of acknowledged names",
///      "description": "Contains the names of contributors being recognized.",
///      "type": "array",
///      "items": {
///        "title": "Name of the contributor",
///        "description": "Contains the name of a single contributor being recognized.",
///        "examples": [
///          "Albert Einstein",
///          "Johann Sebastian Bach"
///        ],
///        "type": "string",
///        "minLength": 1
///      },
///      "minItems": 1
///    },
///    "organization": {
///      "title": "Contributing organization",
///      "description": "Contains the name of a contributing organization being recognized.",
///      "examples": [
///        "CISA",
///        "Google Project Zero",
///        "Talos"
///      ],
///      "type": "string",
///      "minLength": 1
///    },
///    "summary": {
///      "title": "Summary of the acknowledgment",
///      "description": "SHOULD represent any contextual details the document producers wish to make known about the acknowledgment or acknowledged parties.",
///      "examples": [
///        "First analysis of Coordinated Multi-Stream Attack (CMSA)"
///      ],
///      "type": "string",
///      "minLength": 1
///    },
///    "urls": {
///      "title": "List of URLs",
///      "description": "Specifies a list of URLs or location of the reference to be acknowledged.",
///      "type": "array",
///      "items": {
///        "title": "URL of acknowledgment",
///        "description": "Contains the URL or location of the reference to be acknowledged.",
///        "type": "string",
///        "format": "uri"
///      },
///      "minItems": 1
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Acknowledgment {
    ///Contains the names of contributors being recognized.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<NameOfTheContributor>,
    ///Contains the name of a contributing organization being recognized.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<ContributingOrganization>,
    ///SHOULD represent any contextual details the document producers wish to make known about the acknowledgment or acknowledged parties.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<SummaryOfTheAcknowledgment>,
    ///Specifies a list of URLs or location of the reference to be acknowledged.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub urls: Vec<String>,
}
impl From<&Acknowledgment> for Acknowledgment {
    fn from(value: &Acknowledgment) -> Self {
        value.clone()
    }
}
impl Acknowledgment {
    pub fn builder() -> builder::Acknowledgment {
        Default::default()
    }
}
///Contains a list of acknowledgment elements.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "List of acknowledgments",
///  "description": "Contains a list of acknowledgment elements.",
///  "type": "array",
///  "items": {
///    "title": "Acknowledgment",
///    "description": "Acknowledges contributions by describing those that contributed.",
///    "type": "object",
///    "minProperties": 1,
///    "properties": {
///      "names": {
///        "title": "List of acknowledged names",
///        "description": "Contains the names of contributors being recognized.",
///        "type": "array",
///        "items": {
///          "title": "Name of the contributor",
///          "description": "Contains the name of a single contributor being recognized.",
///          "examples": [
///            "Albert Einstein",
///            "Johann Sebastian Bach"
///          ],
///          "type": "string",
///          "minLength": 1
///        },
///        "minItems": 1
///      },
///      "organization": {
///        "title": "Contributing organization",
///        "description": "Contains the name of a contributing organization being recognized.",
///        "examples": [
///          "CISA",
///          "Google Project Zero",
///          "Talos"
///        ],
///        "type": "string",
///        "minLength": 1
///      },
///      "summary": {
///        "title": "Summary of the acknowledgment",
///        "description": "SHOULD represent any contextual details the document producers wish to make known about the acknowledgment or acknowledged parties.",
///        "examples": [
///          "First analysis of Coordinated Multi-Stream Attack (CMSA)"
///        ],
///        "type": "string",
///        "minLength": 1
///      },
///      "urls": {
///        "title": "List of URLs",
///        "description": "Specifies a list of URLs or location of the reference to be acknowledged.",
///        "type": "array",
///        "items": {
///          "title": "URL of acknowledgment",
///          "description": "Contains the URL or location of the reference to be acknowledged.",
///          "type": "string",
///          "format": "uri"
///        },
///        "minItems": 1
///      }
///    }
///  },
///  "minItems": 1
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AcknowledgmentsT(pub Vec<Acknowledgment>);
impl ::std::ops::Deref for AcknowledgmentsT {
    type Target = Vec<Acknowledgment>;
    fn deref(&self) -> &Vec<Acknowledgment> {
        &self.0
    }
}
impl From<AcknowledgmentsT> for Vec<Acknowledgment> {
    fn from(value: AcknowledgmentsT) -> Self {
        value.0
    }
}
impl From<&AcknowledgmentsT> for AcknowledgmentsT {
    fn from(value: &AcknowledgmentsT) -> Self {
        value.clone()
    }
}
impl From<Vec<Acknowledgment>> for AcknowledgmentsT {
    fn from(value: Vec<Acknowledgment>) -> Self {
        Self(value)
    }
}
///Provides additional information for the restart. This can include details on procedures, scope or impact.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Additional restart information",
///  "description": "Provides additional information for the restart. This can include details on procedures, scope or impact.",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AdditionalRestartInformation(String);
impl ::std::ops::Deref for AdditionalRestartInformation {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<AdditionalRestartInformation> for String {
    fn from(value: AdditionalRestartInformation) -> Self {
        value.0
    }
}
impl From<&AdditionalRestartInformation> for AdditionalRestartInformation {
    fn from(value: &AdditionalRestartInformation) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for AdditionalRestartInformation {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AdditionalRestartInformation {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for AdditionalRestartInformation {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for AdditionalRestartInformation {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AdditionalRestartInformation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Is a vehicle that is provided by the document producer to convey the urgency and criticality with which the one or more vulnerabilities reported should be addressed. It is a document-level metric and applied to the document as a whole — not any specific vulnerability. The range of values in this field is defined according to the document producer's policies and procedures.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Aggregate severity",
///  "description": "Is a vehicle that is provided by the document producer to convey the urgency and criticality with which the one or more vulnerabilities reported should be addressed. It is a document-level metric and applied to the document as a whole — not any specific vulnerability. The range of values in this field is defined according to the document producer's policies and procedures.",
///  "type": "object",
///  "required": [
///    "text"
///  ],
///  "properties": {
///    "namespace": {
///      "title": "Namespace of aggregate severity",
///      "description": "Points to the namespace so referenced.",
///      "type": "string",
///      "format": "uri"
///    },
///    "text": {
///      "title": "Text of aggregate severity",
///      "description": "Provides a severity which is independent of - and in addition to - any other standard metric for determining the impact or severity of a given vulnerability (such as CVSS).",
///      "examples": [
///        "Critical",
///        "Important",
///        "Moderate"
///      ],
///      "type": "string",
///      "minLength": 1
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AggregateSeverity {
    ///Points to the namespace so referenced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    ///Provides a severity which is independent of - and in addition to - any other standard metric for determining the impact or severity of a given vulnerability (such as CVSS).
    pub text: TextOfAggregateSeverity,
}
impl From<&AggregateSeverity> for AggregateSeverity {
    fn from(value: &AggregateSeverity) -> Self {
        value.clone()
    }
}
impl AggregateSeverity {
    pub fn builder() -> builder::AggregateSeverity {
        Default::default()
    }
}
///Contains the name of the cryptographic hash algorithm used to calculate the value.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Algorithm of the cryptographic hash",
///  "description": "Contains the name of the cryptographic hash algorithm used to calculate the value.",
///  "default": "sha256",
///  "examples": [
///    "blake2b512",
///    "sha256",
///    "sha3-512",
///    "sha384",
///    "sha512"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AlgorithmOfTheCryptographicHash(String);
impl ::std::ops::Deref for AlgorithmOfTheCryptographicHash {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<AlgorithmOfTheCryptographicHash> for String {
    fn from(value: AlgorithmOfTheCryptographicHash) -> Self {
        value.0
    }
}
impl From<&AlgorithmOfTheCryptographicHash> for AlgorithmOfTheCryptographicHash {
    fn from(value: &AlgorithmOfTheCryptographicHash) -> Self {
        value.clone()
    }
}
impl Default for AlgorithmOfTheCryptographicHash {
    fn default() -> Self {
        AlgorithmOfTheCryptographicHash("sha256".to_string())
    }
}
impl ::std::str::FromStr for AlgorithmOfTheCryptographicHash {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AlgorithmOfTheCryptographicHash {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for AlgorithmOfTheCryptographicHash {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for AlgorithmOfTheCryptographicHash {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AlgorithmOfTheCryptographicHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Specifies a non-empty string that represents a distinct optional alternative ID used to refer to the document.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Alternate name",
///  "description": "Specifies a non-empty string that represents a distinct optional alternative ID used to refer to the document.",
///  "examples": [
///    "CVE-2019-12345"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AlternateName(String);
impl ::std::ops::Deref for AlternateName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<AlternateName> for String {
    fn from(value: AlternateName) -> Self {
        value.0
    }
}
impl From<&AlternateName> for AlternateName {
    fn from(value: &AlternateName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for AlternateName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AlternateName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for AlternateName {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for AlternateName {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AlternateName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Indicates who is intended to read it.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Audience of note",
///  "description": "Indicates who is intended to read it.",
///  "examples": [
///    "all",
///    "executives",
///    "operational management and system administrators",
///    "safety engineers"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AudienceOfNote(String);
impl ::std::ops::Deref for AudienceOfNote {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<AudienceOfNote> for String {
    fn from(value: AudienceOfNote) -> Self {
        value.0
    }
}
impl From<&AudienceOfNote> for AudienceOfNote {
    fn from(value: &AudienceOfNote) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for AudienceOfNote {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AudienceOfNote {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for AudienceOfNote {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for AudienceOfNote {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AudienceOfNote {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Is a part of the hierarchical structure of the product tree.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Branch",
///  "description": "Is a part of the hierarchical structure of the product tree.",
///  "type": "object",
///  "maxProperties": 3,
///  "minProperties": 3,
///  "required": [
///    "category",
///    "name"
///  ],
///  "properties": {
///    "branches": {
///      "$ref": "#/$defs/branches_t"
///    },
///    "category": {
///      "title": "Category of the branch",
///      "description": "Describes the characteristics of the labeled branch.",
///      "type": "string",
///      "enum": [
///        "architecture",
///        "host_name",
///        "language",
///        "legacy",
///        "patch_level",
///        "product_family",
///        "product_name",
///        "product_version",
///        "product_version_range",
///        "service_pack",
///        "specification",
///        "vendor"
///      ]
///    },
///    "name": {
///      "title": "Name of the branch",
///      "description": "Contains the canonical descriptor or 'friendly name' of the branch.",
///      "examples": [
///        "10",
///        "365",
///        "Microsoft",
///        "Office",
///        "PCS 7",
///        "SIMATIC",
///        "Siemens",
///        "Windows"
///      ],
///      "type": "string",
///      "minLength": 1
///    },
///    "product": {
///      "$ref": "#/$defs/full_product_name_t"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Branch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branches: Option<BranchesT>,
    ///Describes the characteristics of the labeled branch.
    pub category: CategoryOfTheBranch,
    ///Contains the canonical descriptor or 'friendly name' of the branch.
    pub name: NameOfTheBranch,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<FullProductNameT>,
}
impl From<&Branch> for Branch {
    fn from(value: &Branch) -> Self {
        value.clone()
    }
}
impl Branch {
    pub fn builder() -> builder::Branch {
        Default::default()
    }
}
///Contains branch elements as children of the current element.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "List of branches",
///  "description": "Contains branch elements as children of the current element.",
///  "type": "array",
///  "items": {
///    "title": "Branch",
///    "description": "Is a part of the hierarchical structure of the product tree.",
///    "type": "object",
///    "maxProperties": 3,
///    "minProperties": 3,
///    "required": [
///      "category",
///      "name"
///    ],
///    "properties": {
///      "branches": {
///        "$ref": "#/$defs/branches_t"
///      },
///      "category": {
///        "title": "Category of the branch",
///        "description": "Describes the characteristics of the labeled branch.",
///        "type": "string",
///        "enum": [
///          "architecture",
///          "host_name",
///          "language",
///          "legacy",
///          "patch_level",
///          "product_family",
///          "product_name",
///          "product_version",
///          "product_version_range",
///          "service_pack",
///          "specification",
///          "vendor"
///        ]
///      },
///      "name": {
///        "title": "Name of the branch",
///        "description": "Contains the canonical descriptor or 'friendly name' of the branch.",
///        "examples": [
///          "10",
///          "365",
///          "Microsoft",
///          "Office",
///          "PCS 7",
///          "SIMATIC",
///          "Siemens",
///          "Windows"
///        ],
///        "type": "string",
///        "minLength": 1
///      },
///      "product": {
///        "$ref": "#/$defs/full_product_name_t"
///      }
///    }
///  },
///  "minItems": 1
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct BranchesT(pub Vec<Branch>);
impl ::std::ops::Deref for BranchesT {
    type Target = Vec<Branch>;
    fn deref(&self) -> &Vec<Branch> {
        &self.0
    }
}
impl From<BranchesT> for Vec<Branch> {
    fn from(value: BranchesT) -> Self {
        value.0
    }
}
impl From<&BranchesT> for BranchesT {
    fn from(value: &BranchesT) -> Self {
        value.clone()
    }
}
impl From<Vec<Branch>> for BranchesT {
    fn from(value: Vec<Branch>) -> Self {
        Self(value)
    }
}
///Provides information about the category of publisher releasing the document.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Category of publisher",
///  "description": "Provides information about the category of publisher releasing the document.",
///  "type": "string",
///  "enum": [
///    "coordinator",
///    "discoverer",
///    "other",
///    "translator",
///    "user",
///    "vendor"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CategoryOfPublisher {
    #[serde(rename = "coordinator")]
    Coordinator,
    #[serde(rename = "discoverer")]
    Discoverer,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "translator")]
    Translator,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "vendor")]
    Vendor,
}
impl From<&CategoryOfPublisher> for CategoryOfPublisher {
    fn from(value: &CategoryOfPublisher) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CategoryOfPublisher {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Coordinator => write!(f, "coordinator"),
            Self::Discoverer => write!(f, "discoverer"),
            Self::Other => write!(f, "other"),
            Self::Translator => write!(f, "translator"),
            Self::User => write!(f, "user"),
            Self::Vendor => write!(f, "vendor"),
        }
    }
}
impl std::str::FromStr for CategoryOfPublisher {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "coordinator" => Ok(Self::Coordinator),
            "discoverer" => Ok(Self::Discoverer),
            "other" => Ok(Self::Other),
            "translator" => Ok(Self::Translator),
            "user" => Ok(Self::User),
            "vendor" => Ok(Self::Vendor),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for CategoryOfPublisher {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CategoryOfPublisher {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CategoryOfPublisher {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Indicates whether the reference points to the same document or vulnerability in focus (depending on scope) or to an external resource.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Category of reference",
///  "description": "Indicates whether the reference points to the same document or vulnerability in focus (depending on scope) or to an external resource.",
///  "default": "external",
///  "type": "string",
///  "enum": [
///    "external",
///    "self"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CategoryOfReference {
    #[serde(rename = "external")]
    External,
    #[serde(rename = "self")]
    Self_,
}
impl From<&CategoryOfReference> for CategoryOfReference {
    fn from(value: &CategoryOfReference) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CategoryOfReference {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::External => write!(f, "external"),
            Self::Self_ => write!(f, "self"),
        }
    }
}
impl std::str::FromStr for CategoryOfReference {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "external" => Ok(Self::External),
            "self" => Ok(Self::Self_),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for CategoryOfReference {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CategoryOfReference {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CategoryOfReference {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for CategoryOfReference {
    fn default() -> Self {
        CategoryOfReference::External
    }
}
///Specifies what category of restart is required by this remediation to become effective.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Category of restart",
///  "description": "Specifies what category of restart is required by this remediation to become effective.",
///  "type": "string",
///  "enum": [
///    "connected",
///    "dependencies",
///    "machine",
///    "none",
///    "parent",
///    "service",
///    "system",
///    "vulnerable_component",
///    "zone"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CategoryOfRestart {
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "dependencies")]
    Dependencies,
    #[serde(rename = "machine")]
    Machine,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "parent")]
    Parent,
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "vulnerable_component")]
    VulnerableComponent,
    #[serde(rename = "zone")]
    Zone,
}
impl From<&CategoryOfRestart> for CategoryOfRestart {
    fn from(value: &CategoryOfRestart) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CategoryOfRestart {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Connected => write!(f, "connected"),
            Self::Dependencies => write!(f, "dependencies"),
            Self::Machine => write!(f, "machine"),
            Self::None => write!(f, "none"),
            Self::Parent => write!(f, "parent"),
            Self::Service => write!(f, "service"),
            Self::System => write!(f, "system"),
            Self::VulnerableComponent => write!(f, "vulnerable_component"),
            Self::Zone => write!(f, "zone"),
        }
    }
}
impl std::str::FromStr for CategoryOfRestart {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "connected" => Ok(Self::Connected),
            "dependencies" => Ok(Self::Dependencies),
            "machine" => Ok(Self::Machine),
            "none" => Ok(Self::None),
            "parent" => Ok(Self::Parent),
            "service" => Ok(Self::Service),
            "system" => Ok(Self::System),
            "vulnerable_component" => Ok(Self::VulnerableComponent),
            "zone" => Ok(Self::Zone),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for CategoryOfRestart {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CategoryOfRestart {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CategoryOfRestart {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Describes the characteristics of the labeled branch.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Category of the branch",
///  "description": "Describes the characteristics of the labeled branch.",
///  "type": "string",
///  "enum": [
///    "architecture",
///    "host_name",
///    "language",
///    "legacy",
///    "patch_level",
///    "product_family",
///    "product_name",
///    "product_version",
///    "product_version_range",
///    "service_pack",
///    "specification",
///    "vendor"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CategoryOfTheBranch {
    #[serde(rename = "architecture")]
    Architecture,
    #[serde(rename = "host_name")]
    HostName,
    #[serde(rename = "language")]
    Language,
    #[serde(rename = "legacy")]
    Legacy,
    #[serde(rename = "patch_level")]
    PatchLevel,
    #[serde(rename = "product_family")]
    ProductFamily,
    #[serde(rename = "product_name")]
    ProductName,
    #[serde(rename = "product_version")]
    ProductVersion,
    #[serde(rename = "product_version_range")]
    ProductVersionRange,
    #[serde(rename = "service_pack")]
    ServicePack,
    #[serde(rename = "specification")]
    Specification,
    #[serde(rename = "vendor")]
    Vendor,
}
impl From<&CategoryOfTheBranch> for CategoryOfTheBranch {
    fn from(value: &CategoryOfTheBranch) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CategoryOfTheBranch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Architecture => write!(f, "architecture"),
            Self::HostName => write!(f, "host_name"),
            Self::Language => write!(f, "language"),
            Self::Legacy => write!(f, "legacy"),
            Self::PatchLevel => write!(f, "patch_level"),
            Self::ProductFamily => write!(f, "product_family"),
            Self::ProductName => write!(f, "product_name"),
            Self::ProductVersion => write!(f, "product_version"),
            Self::ProductVersionRange => write!(f, "product_version_range"),
            Self::ServicePack => write!(f, "service_pack"),
            Self::Specification => write!(f, "specification"),
            Self::Vendor => write!(f, "vendor"),
        }
    }
}
impl std::str::FromStr for CategoryOfTheBranch {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "architecture" => Ok(Self::Architecture),
            "host_name" => Ok(Self::HostName),
            "language" => Ok(Self::Language),
            "legacy" => Ok(Self::Legacy),
            "patch_level" => Ok(Self::PatchLevel),
            "product_family" => Ok(Self::ProductFamily),
            "product_name" => Ok(Self::ProductName),
            "product_version" => Ok(Self::ProductVersion),
            "product_version_range" => Ok(Self::ProductVersionRange),
            "service_pack" => Ok(Self::ServicePack),
            "specification" => Ok(Self::Specification),
            "vendor" => Ok(Self::Vendor),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for CategoryOfTheBranch {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CategoryOfTheBranch {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CategoryOfTheBranch {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Specifies the category which this remediation belongs to.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Category of the remediation",
///  "description": "Specifies the category which this remediation belongs to.",
///  "type": "string",
///  "enum": [
///    "mitigation",
///    "no_fix_planned",
///    "none_available",
///    "vendor_fix",
///    "workaround"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CategoryOfTheRemediation {
    #[serde(rename = "mitigation")]
    Mitigation,
    #[serde(rename = "no_fix_planned")]
    NoFixPlanned,
    #[serde(rename = "none_available")]
    NoneAvailable,
    #[serde(rename = "vendor_fix")]
    VendorFix,
    #[serde(rename = "workaround")]
    Workaround,
}
impl From<&CategoryOfTheRemediation> for CategoryOfTheRemediation {
    fn from(value: &CategoryOfTheRemediation) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CategoryOfTheRemediation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Mitigation => write!(f, "mitigation"),
            Self::NoFixPlanned => write!(f, "no_fix_planned"),
            Self::NoneAvailable => write!(f, "none_available"),
            Self::VendorFix => write!(f, "vendor_fix"),
            Self::Workaround => write!(f, "workaround"),
        }
    }
}
impl std::str::FromStr for CategoryOfTheRemediation {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "mitigation" => Ok(Self::Mitigation),
            "no_fix_planned" => Ok(Self::NoFixPlanned),
            "none_available" => Ok(Self::NoneAvailable),
            "vendor_fix" => Ok(Self::VendorFix),
            "workaround" => Ok(Self::Workaround),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for CategoryOfTheRemediation {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CategoryOfTheRemediation {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CategoryOfTheRemediation {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Categorizes the threat according to the rules of the specification.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Category of the threat",
///  "description": "Categorizes the threat according to the rules of the specification.",
///  "type": "string",
///  "enum": [
///    "exploit_status",
///    "impact",
///    "target_set"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CategoryOfTheThreat {
    #[serde(rename = "exploit_status")]
    ExploitStatus,
    #[serde(rename = "impact")]
    Impact,
    #[serde(rename = "target_set")]
    TargetSet,
}
impl From<&CategoryOfTheThreat> for CategoryOfTheThreat {
    fn from(value: &CategoryOfTheThreat) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CategoryOfTheThreat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ExploitStatus => write!(f, "exploit_status"),
            Self::Impact => write!(f, "impact"),
            Self::TargetSet => write!(f, "target_set"),
        }
    }
}
impl std::str::FromStr for CategoryOfTheThreat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "exploit_status" => Ok(Self::ExploitStatus),
            "impact" => Ok(Self::Impact),
            "target_set" => Ok(Self::TargetSet),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for CategoryOfTheThreat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CategoryOfTheThreat {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CategoryOfTheThreat {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The Common Platform Enumeration (CPE) attribute refers to a method for naming platforms external to this specification.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Common Platform Enumeration representation",
///  "description": "The Common Platform Enumeration (CPE) attribute refers to a method for naming platforms external to this specification.",
///  "type": "string",
///  "minLength": 5,
///  "pattern": "^(cpe:2\\.3:[aho\\*\\-](:(((\\?*|\\*?)([a-zA-Z0-9\\-\\._]|(\\\\[\\\\\\*\\?!\"#\\$%&'\\(\\)\\+,/:;<=>@\\[\\]\\^`\\{\\|\\}~]))+(\\?*|\\*?))|[\\*\\-])){5}(:(([a-zA-Z]{2,3}(-([a-zA-Z]{2}|[0-9]{3}))?)|[\\*\\-]))(:(((\\?*|\\*?)([a-zA-Z0-9\\-\\._]|(\\\\[\\\\\\*\\?!\"#\\$%&'\\(\\)\\+,/:;<=>@\\[\\]\\^`\\{\\|\\}~]))+(\\?*|\\*?))|[\\*\\-])){4})|([c][pP][eE]:/[AHOaho]?(:[A-Za-z0-9\\._\\-~%]*){0,6})$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CommonPlatformEnumerationRepresentation(String);
impl ::std::ops::Deref for CommonPlatformEnumerationRepresentation {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<CommonPlatformEnumerationRepresentation> for String {
    fn from(value: CommonPlatformEnumerationRepresentation) -> Self {
        value.0
    }
}
impl From<&CommonPlatformEnumerationRepresentation> for CommonPlatformEnumerationRepresentation {
    fn from(value: &CommonPlatformEnumerationRepresentation) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CommonPlatformEnumerationRepresentation {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 5usize {
            return Err("shorter than 5 characters".into());
        }
        if regress::Regex::new(
                "^(cpe:2\\.3:[aho\\*\\-](:(((\\?*|\\*?)([a-zA-Z0-9\\-\\._]|(\\\\[\\\\\\*\\?!\"#\\$%&'\\(\\)\\+,/:;<=>@\\[\\]\\^`\\{\\|\\}~]))+(\\?*|\\*?))|[\\*\\-])){5}(:(([a-zA-Z]{2,3}(-([a-zA-Z]{2}|[0-9]{3}))?)|[\\*\\-]))(:(((\\?*|\\*?)([a-zA-Z0-9\\-\\._]|(\\\\[\\\\\\*\\?!\"#\\$%&'\\(\\)\\+,/:;<=>@\\[\\]\\^`\\{\\|\\}~]))+(\\?*|\\*?))|[\\*\\-])){4})|([c][pP][eE]:/[AHOaho]?(:[A-Za-z0-9\\._\\-~%]*){0,6})$",
            )
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err(
                "doesn't match pattern \"^(cpe:2\\.3:[aho\\*\\-](:(((\\?*|\\*?)([a-zA-Z0-9\\-\\._]|(\\\\[\\\\\\*\\?!\"#\\$%&'\\(\\)\\+,/:;<=>@\\[\\]\\^`\\{\\|\\}~]))+(\\?*|\\*?))|[\\*\\-])){5}(:(([a-zA-Z]{2,3}(-([a-zA-Z]{2}|[0-9]{3}))?)|[\\*\\-]))(:(((\\?*|\\*?)([a-zA-Z0-9\\-\\._]|(\\\\[\\\\\\*\\?!\"#\\$%&'\\(\\)\\+,/:;<=>@\\[\\]\\^`\\{\\|\\}~]))+(\\?*|\\*?))|[\\*\\-])){4})|([c][pP][eE]:/[AHOaho]?(:[A-Za-z0-9\\._\\-~%]*){0,6})$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CommonPlatformEnumerationRepresentation {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for CommonPlatformEnumerationRepresentation {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for CommonPlatformEnumerationRepresentation {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CommonPlatformEnumerationRepresentation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Representation of security advisory information as a JSON document.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$id": "https://docs.oasis-open.org/csaf/csaf/v2.0/csaf_json_schema.json",
///  "title": "Common Security Advisory Framework",
///  "description": "Representation of security advisory information as a JSON document.",
///  "type": "object",
///  "required": [
///    "document"
///  ],
///  "properties": {
///    "document": {
///      "title": "Document level meta-data",
///      "description": "Captures the meta-data about this document describing a particular set of security advisories.",
///      "type": "object",
///      "required": [
///        "category",
///        "csaf_version",
///        "publisher",
///        "title",
///        "tracking"
///      ],
///      "properties": {
///        "acknowledgments": {
///          "title": "Document acknowledgments",
///          "description": "Contains a list of acknowledgment elements associated with the whole document.",
///          "$ref": "#/$defs/acknowledgments_t"
///        },
///        "aggregate_severity": {
///          "title": "Aggregate severity",
///          "description": "Is a vehicle that is provided by the document producer to convey the urgency and criticality with which the one or more vulnerabilities reported should be addressed. It is a document-level metric and applied to the document as a whole — not any specific vulnerability. The range of values in this field is defined according to the document producer's policies and procedures.",
///          "type": "object",
///          "required": [
///            "text"
///          ],
///          "properties": {
///            "namespace": {
///              "title": "Namespace of aggregate severity",
///              "description": "Points to the namespace so referenced.",
///              "type": "string",
///              "format": "uri"
///            },
///            "text": {
///              "title": "Text of aggregate severity",
///              "description": "Provides a severity which is independent of - and in addition to - any other standard metric for determining the impact or severity of a given vulnerability (such as CVSS).",
///              "examples": [
///                "Critical",
///                "Important",
///                "Moderate"
///              ],
///              "type": "string",
///              "minLength": 1
///            }
///          }
///        },
///        "category": {
///          "title": "Document category",
///          "description": "Defines a short canonical name, chosen by the document producer, which will inform the end user as to the category of document.",
///          "examples": [
///            "csaf_base",
///            "csaf_security_advisory",
///            "csaf_vex",
///            "Example Company Security Notice"
///          ],
///          "type": "string",
///          "minLength": 1,
///          "pattern": "^[^\\s\\-_\\.](.*[^\\s\\-_\\.])?$"
///        },
///        "csaf_version": {
///          "title": "CSAF version",
///          "description": "Gives the version of the CSAF specification which the document was generated for.",
///          "type": "string",
///          "enum": [
///            "2.0"
///          ]
///        },
///        "distribution": {
///          "title": "Rules for sharing document",
///          "description": "Describe any constraints on how this document might be shared.",
///          "type": "object",
///          "minProperties": 1,
///          "properties": {
///            "text": {
///              "title": "Textual description",
///              "description": "Provides a textual description of additional constraints.",
///              "examples": [
///                "Copyright 2021, Example Company, All Rights Reserved.",
///                "Distribute freely.",
///                "Share only on a need-to-know-basis only."
///              ],
///              "type": "string",
///              "minLength": 1
///            },
///            "tlp": {
///              "title": "Traffic Light Protocol (TLP)",
///              "description": "Provides details about the TLP classification of the document.",
///              "type": "object",
///              "required": [
///                "label"
///              ],
///              "properties": {
///                "label": {
///                  "title": "Label of TLP",
///                  "description": "Provides the TLP label of the document.",
///                  "type": "string",
///                  "enum": [
///                    "AMBER",
///                    "GREEN",
///                    "RED",
///                    "WHITE"
///                  ]
///                },
///                "url": {
///                  "title": "URL of TLP version",
///                  "description": "Provides a URL where to find the textual description of the TLP version which is used in this document. Default is the URL to the definition by FIRST.",
///                  "default": "https://www.first.org/tlp/",
///                  "examples": [
///                    "https://www.us-cert.gov/tlp",
///                    "https://www.bsi.bund.de/SharedDocs/Downloads/DE/BSI/Kritis/Merkblatt_TLP.pdf"
///                  ],
///                  "type": "string",
///                  "format": "uri"
///                }
///              }
///            }
///          }
///        },
///        "lang": {
///          "title": "Document language",
///          "description": "Identifies the language used by this document, corresponding to IETF BCP 47 / RFC 5646.",
///          "$ref": "#/$defs/lang_t"
///        },
///        "notes": {
///          "title": "Document notes",
///          "description": "Holds notes associated with the whole document.",
///          "$ref": "#/$defs/notes_t"
///        },
///        "publisher": {
///          "title": "Publisher",
///          "description": "Provides information about the publisher of the document.",
///          "type": "object",
///          "required": [
///            "category",
///            "name",
///            "namespace"
///          ],
///          "properties": {
///            "category": {
///              "title": "Category of publisher",
///              "description": "Provides information about the category of publisher releasing the document.",
///              "type": "string",
///              "enum": [
///                "coordinator",
///                "discoverer",
///                "other",
///                "translator",
///                "user",
///                "vendor"
///              ]
///            },
///            "contact_details": {
///              "title": "Contact details",
///              "description": "Information on how to contact the publisher, possibly including details such as web sites, email addresses, phone numbers, and postal mail addresses.",
///              "examples": [
///                "Example Company can be reached at contact_us@example.com, or via our website at https://www.example.com/contact."
///              ],
///              "type": "string",
///              "minLength": 1
///            },
///            "issuing_authority": {
///              "title": "Issuing authority",
///              "description": "Provides information about the authority of the issuing party to release the document, in particular, the party's constituency and responsibilities or other obligations.",
///              "type": "string",
///              "minLength": 1
///            },
///            "name": {
///              "title": "Name of publisher",
///              "description": "Contains the name of the issuing party.",
///              "examples": [
///                "BSI",
///                "Cisco PSIRT",
///                "Siemens ProductCERT"
///              ],
///              "type": "string",
///              "minLength": 1
///            },
///            "namespace": {
///              "title": "Namespace of publisher",
///              "description": "Contains a URL which is under control of the issuing party and can be used as a globally unique identifier for that issuing party.",
///              "examples": [
///                "https://csaf.io",
///                "https://www.example.com"
///              ],
///              "type": "string",
///              "format": "uri"
///            }
///          }
///        },
///        "references": {
///          "title": "Document references",
///          "description": "Holds a list of references associated with the whole document.",
///          "$ref": "#/$defs/references_t"
///        },
///        "source_lang": {
///          "title": "Source language",
///          "description": "If this copy of the document is a translation then the value of this property describes from which language this document was translated.",
///          "$ref": "#/$defs/lang_t"
///        },
///        "title": {
///          "title": "Title of this document",
///          "description": "This SHOULD be a canonical name for the document, and sufficiently unique to distinguish it from similar documents.",
///          "examples": [
///            "Cisco IPv6 Crafted Packet Denial of Service Vulnerability",
///            "Example Company Cross-Site-Scripting Vulnerability in Example Generator"
///          ],
///          "type": "string",
///          "minLength": 1
///        },
///        "tracking": {
///          "title": "Tracking",
///          "description": "Is a container designated to hold all management attributes necessary to track a CSAF document as a whole.",
///          "type": "object",
///          "required": [
///            "current_release_date",
///            "id",
///            "initial_release_date",
///            "revision_history",
///            "status",
///            "version"
///          ],
///          "properties": {
///            "aliases": {
///              "title": "Aliases",
///              "description": "Contains a list of alternate names for the same document.",
///              "type": "array",
///              "items": {
///                "title": "Alternate name",
///                "description": "Specifies a non-empty string that represents a distinct optional alternative ID used to refer to the document.",
///                "examples": [
///                  "CVE-2019-12345"
///                ],
///                "type": "string",
///                "minLength": 1
///              },
///              "minItems": 1,
///              "uniqueItems": true
///            },
///            "current_release_date": {
///              "title": "Current release date",
///              "description": "The date when the current revision of this document was released",
///              "type": "string",
///              "format": "date-time"
///            },
///            "generator": {
///              "title": "Document generator",
///              "description": "Is a container to hold all elements related to the generation of the document. These items will reference when the document was actually created, including the date it was generated and the entity that generated it.",
///              "type": "object",
///              "required": [
///                "engine"
///              ],
///              "properties": {
///                "date": {
///                  "title": "Date of document generation",
///                  "description": "This SHOULD be the current date that the document was generated. Because documents are often generated internally by a document producer and exist for a nonzero amount of time before being released, this field MAY be different from the Initial Release Date and Current Release Date.",
///                  "type": "string",
///                  "format": "date-time"
///                },
///                "engine": {
///                  "title": "Engine of document generation",
///                  "description": "Contains information about the engine that generated the CSAF document.",
///                  "type": "object",
///                  "required": [
///                    "name"
///                  ],
///                  "properties": {
///                    "name": {
///                      "title": "Engine name",
///                      "description": "Represents the name of the engine that generated the CSAF document.",
///                      "examples": [
///                        "Red Hat rhsa-to-cvrf",
///                        "Secvisogram",
///                        "TVCE"
///                      ],
///                      "type": "string",
///                      "minLength": 1
///                    },
///                    "version": {
///                      "title": "Engine version",
///                      "description": "Contains the version of the engine that generated the CSAF document.",
///                      "examples": [
///                        "0.6.0",
///                        "1.0.0-beta+exp.sha.a1c44f85",
///                        "2"
///                      ],
///                      "type": "string",
///                      "minLength": 1
///                    }
///                  }
///                }
///              }
///            },
///            "id": {
///              "title": "Unique identifier for the document",
///              "description": "The ID is a simple label that provides for a wide range of numbering values, types, and schemes. Its value SHOULD be assigned and maintained by the original document issuing authority.",
///              "examples": [
///                "Example Company - 2019-YH3234",
///                "RHBA-2019:0024",
///                "cisco-sa-20190513-secureboot"
///              ],
///              "type": "string",
///              "minLength": 1,
///              "pattern": "^[\\S](.*[\\S])?$"
///            },
///            "initial_release_date": {
///              "title": "Initial release date",
///              "description": "The date when this document was first published.",
///              "type": "string",
///              "format": "date-time"
///            },
///            "revision_history": {
///              "title": "Revision history",
///              "description": "Holds one revision item for each version of the CSAF document, including the initial one.",
///              "type": "array",
///              "items": {
///                "title": "Revision",
///                "description": "Contains all the information elements required to track the evolution of a CSAF document.",
///                "type": "object",
///                "required": [
///                  "date",
///                  "number",
///                  "summary"
///                ],
///                "properties": {
///                  "date": {
///                    "title": "Date of the revision",
///                    "description": "The date of the revision entry",
///                    "type": "string",
///                    "format": "date-time"
///                  },
///                  "legacy_version": {
///                    "title": "Legacy version of the revision",
///                    "description": "Contains the version string used in an existing document with the same content.",
///                    "type": "string",
///                    "minLength": 1
///                  },
///                  "number": {
///                    "$ref": "#/$defs/version_t"
///                  },
///                  "summary": {
///                    "title": "Summary of the revision",
///                    "description": "Holds a single non-empty string representing a short description of the changes.",
///                    "examples": [
///                      "Initial version."
///                    ],
///                    "type": "string",
///                    "minLength": 1
///                  }
///                }
///              },
///              "minItems": 1
///            },
///            "status": {
///              "title": "Document status",
///              "description": "Defines the draft status of the document.",
///              "type": "string",
///              "enum": [
///                "draft",
///                "final",
///                "interim"
///              ]
///            },
///            "version": {
///              "$ref": "#/$defs/version_t"
///            }
///          }
///        }
///      }
///    },
///    "product_tree": {
///      "title": "Product tree",
///      "description": "Is a container for all fully qualified product names that can be referenced elsewhere in the document.",
///      "type": "object",
///      "minProperties": 1,
///      "properties": {
///        "branches": {
///          "$ref": "#/$defs/branches_t"
///        },
///        "full_product_names": {
///          "title": "List of full product names",
///          "description": "Contains a list of full product names.",
///          "type": "array",
///          "items": {
///            "$ref": "#/$defs/full_product_name_t"
///          },
///          "minItems": 1
///        },
///        "product_groups": {
///          "title": "List of product groups",
///          "description": "Contains a list of product groups.",
///          "type": "array",
///          "items": {
///            "title": "Product group",
///            "description": "Defines a new logical group of products that can then be referred to in other parts of the document to address a group of products with a single identifier.",
///            "type": "object",
///            "required": [
///              "group_id",
///              "product_ids"
///            ],
///            "properties": {
///              "group_id": {
///                "$ref": "#/$defs/product_group_id_t"
///              },
///              "product_ids": {
///                "title": "List of Product IDs",
///                "description": "Lists the product_ids of those products which known as one group in the document.",
///                "type": "array",
///                "items": {
///                  "$ref": "#/$defs/product_id_t"
///                },
///                "minItems": 2,
///                "uniqueItems": true
///              },
///              "summary": {
///                "title": "Summary of the product group",
///                "description": "Gives a short, optional description of the group.",
///                "examples": [
///                  "Products supporting Modbus.",
///                  "The x64 versions of the operating system."
///                ],
///                "type": "string",
///                "minLength": 1
///              }
///            }
///          },
///          "minItems": 1
///        },
///        "relationships": {
///          "title": "List of relationships",
///          "description": "Contains a list of relationships.",
///          "type": "array",
///          "items": {
///            "title": "Relationship",
///            "description": "Establishes a link between two existing full_product_name_t elements, allowing the document producer to define a combination of two products that form a new full_product_name entry.",
///            "type": "object",
///            "required": [
///              "category",
///              "full_product_name",
///              "product_reference",
///              "relates_to_product_reference"
///            ],
///            "properties": {
///              "category": {
///                "title": "Relationship category",
///                "description": "Defines the category of relationship for the referenced component.",
///                "type": "string",
///                "enum": [
///                  "default_component_of",
///                  "external_component_of",
///                  "installed_on",
///                  "installed_with",
///                  "optional_component_of"
///                ]
///              },
///              "full_product_name": {
///                "$ref": "#/$defs/full_product_name_t"
///              },
///              "product_reference": {
///                "title": "Product reference",
///                "description": "Holds a Product ID that refers to the Full Product Name element, which is referenced as the first element of the relationship.",
///                "$ref": "#/$defs/product_id_t"
///              },
///              "relates_to_product_reference": {
///                "title": "Relates to product reference",
///                "description": "Holds a Product ID that refers to the Full Product Name element, which is referenced as the second element of the relationship.",
///                "$ref": "#/$defs/product_id_t"
///              }
///            }
///          },
///          "minItems": 1
///        }
///      }
///    },
///    "vulnerabilities": {
///      "title": "Vulnerabilities",
///      "description": "Represents a list of all relevant vulnerability information items.",
///      "type": "array",
///      "items": {
///        "title": "Vulnerability",
///        "description": "Is a container for the aggregation of all fields that are related to a single vulnerability in the document.",
///        "type": "object",
///        "minProperties": 1,
///        "properties": {
///          "acknowledgments": {
///            "title": "Vulnerability acknowledgments",
///            "description": "Contains a list of acknowledgment elements associated with this vulnerability item.",
///            "$ref": "#/$defs/acknowledgments_t"
///          },
///          "cve": {
///            "title": "CVE",
///            "description": "Holds the MITRE standard Common Vulnerabilities and Exposures (CVE) tracking number for the vulnerability.",
///            "type": "string",
///            "pattern": "^CVE-[0-9]{4}-[0-9]{4,}$"
///          },
///          "cwe": {
///            "title": "CWE",
///            "description": "Holds the MITRE standard Common Weakness Enumeration (CWE) for the weakness associated.",
///            "type": "object",
///            "required": [
///              "id",
///              "name"
///            ],
///            "properties": {
///              "id": {
///                "title": "Weakness ID",
///                "description": "Holds the ID for the weakness associated.",
///                "examples": [
///                  "CWE-22",
///                  "CWE-352",
///                  "CWE-79"
///                ],
///                "type": "string",
///                "pattern": "^CWE-[1-9]\\d{0,5}$"
///              },
///              "name": {
///                "title": "Weakness name",
///                "description": "Holds the full name of the weakness as given in the CWE specification.",
///                "examples": [
///                  "Cross-Site Request Forgery (CSRF)",
///                  "Improper Limitation of a Pathname to a Restricted Directory ('Path Traversal')",
///                  "Improper Neutralization of Input During Web Page Generation ('Cross-site Scripting')"
///                ],
///                "type": "string",
///                "minLength": 1
///              }
///            }
///          },
///          "discovery_date": {
///            "title": "Discovery date",
///            "description": "Holds the date and time the vulnerability was originally discovered.",
///            "type": "string",
///            "format": "date-time"
///          },
///          "flags": {
///            "title": "List of flags",
///            "description": "Contains a list of machine readable flags.",
///            "type": "array",
///            "items": {
///              "title": "Flag",
///              "description": "Contains product specific information in regard to this vulnerability as a single machine readable flag.",
///              "type": "object",
///              "required": [
///                "label"
///              ],
///              "properties": {
///                "date": {
///                  "title": "Date of the flag",
///                  "description": "Contains the date when assessment was done or the flag was assigned.",
///                  "type": "string",
///                  "format": "date-time"
///                },
///                "group_ids": {
///                  "$ref": "#/$defs/product_groups_t"
///                },
///                "label": {
///                  "title": "Label of the flag",
///                  "description": "Specifies the machine readable label.",
///                  "type": "string",
///                  "enum": [
///                    "component_not_present",
///                    "inline_mitigations_already_exist",
///                    "vulnerable_code_cannot_be_controlled_by_adversary",
///                    "vulnerable_code_not_in_execute_path",
///                    "vulnerable_code_not_present"
///                  ]
///                },
///                "product_ids": {
///                  "$ref": "#/$defs/products_t"
///                }
///              }
///            },
///            "minItems": 1,
///            "uniqueItems": true
///          },
///          "ids": {
///            "title": "List of IDs",
///            "description": "Represents a list of unique labels or tracking IDs for the vulnerability (if such information exists).",
///            "type": "array",
///            "items": {
///              "title": "ID",
///              "description": "Contains a single unique label or tracking ID for the vulnerability.",
///              "type": "object",
///              "required": [
///                "system_name",
///                "text"
///              ],
///              "properties": {
///                "system_name": {
///                  "title": "System name",
///                  "description": "Indicates the name of the vulnerability tracking or numbering system.",
///                  "examples": [
///                    "Cisco Bug ID",
///                    "GitHub Issue"
///                  ],
///                  "type": "string",
///                  "minLength": 1
///                },
///                "text": {
///                  "title": "Text",
///                  "description": "Is unique label or tracking ID for the vulnerability (if such information exists).",
///                  "examples": [
///                    "CSCso66472",
///                    "oasis-tcs/csaf#210"
///                  ],
///                  "type": "string",
///                  "minLength": 1
///                }
///              }
///            },
///            "minItems": 1,
///            "uniqueItems": true
///          },
///          "involvements": {
///            "title": "List of involvements",
///            "description": "Contains a list of involvements.",
///            "type": "array",
///            "items": {
///              "title": "Involvement",
///              "description": "Is a container, that allows the document producers to comment on the level of involvement (or engagement) of themselves or third parties in the vulnerability identification, scoping, and remediation process.",
///              "type": "object",
///              "required": [
///                "party",
///                "status"
///              ],
///              "properties": {
///                "date": {
///                  "title": "Date of involvement",
///                  "description": "Holds the date and time of the involvement entry.",
///                  "type": "string",
///                  "format": "date-time"
///                },
///                "party": {
///                  "title": "Party category",
///                  "description": "Defines the category of the involved party.",
///                  "type": "string",
///                  "enum": [
///                    "coordinator",
///                    "discoverer",
///                    "other",
///                    "user",
///                    "vendor"
///                  ]
///                },
///                "status": {
///                  "title": "Party status",
///                  "description": "Defines contact status of the involved party.",
///                  "type": "string",
///                  "enum": [
///                    "completed",
///                    "contact_attempted",
///                    "disputed",
///                    "in_progress",
///                    "not_contacted",
///                    "open"
///                  ]
///                },
///                "summary": {
///                  "title": "Summary of the involvement",
///                  "description": "Contains additional context regarding what is going on.",
///                  "type": "string",
///                  "minLength": 1
///                }
///              }
///            },
///            "minItems": 1,
///            "uniqueItems": true
///          },
///          "notes": {
///            "title": "Vulnerability notes",
///            "description": "Holds notes associated with this vulnerability item.",
///            "$ref": "#/$defs/notes_t"
///          },
///          "product_status": {
///            "title": "Product status",
///            "description": "Contains different lists of product_ids which provide details on the status of the referenced product related to the current vulnerability. ",
///            "type": "object",
///            "minProperties": 1,
///            "properties": {
///              "first_affected": {
///                "title": "First affected",
///                "description": "These are the first versions of the releases known to be affected by the vulnerability.",
///                "$ref": "#/$defs/products_t"
///              },
///              "first_fixed": {
///                "title": "First fixed",
///                "description": "These versions contain the first fix for the vulnerability but may not be the recommended fixed versions.",
///                "$ref": "#/$defs/products_t"
///              },
///              "fixed": {
///                "title": "Fixed",
///                "description": "These versions contain a fix for the vulnerability but may not be the recommended fixed versions.",
///                "$ref": "#/$defs/products_t"
///              },
///              "known_affected": {
///                "title": "Known affected",
///                "description": "These versions are known to be affected by the vulnerability.",
///                "$ref": "#/$defs/products_t"
///              },
///              "known_not_affected": {
///                "title": "Known not affected",
///                "description": "These versions are known not to be affected by the vulnerability.",
///                "$ref": "#/$defs/products_t"
///              },
///              "last_affected": {
///                "title": "Last affected",
///                "description": "These are the last versions in a release train known to be affected by the vulnerability. Subsequently released versions would contain a fix for the vulnerability.",
///                "$ref": "#/$defs/products_t"
///              },
///              "recommended": {
///                "title": "Recommended",
///                "description": "These versions have a fix for the vulnerability and are the vendor-recommended versions for fixing the vulnerability.",
///                "$ref": "#/$defs/products_t"
///              },
///              "under_investigation": {
///                "title": "Under investigation",
///                "description": "It is not known yet whether these versions are or are not affected by the vulnerability. However, it is still under investigation - the result will be provided in a later release of the document.",
///                "$ref": "#/$defs/products_t"
///              }
///            }
///          },
///          "references": {
///            "title": "Vulnerability references",
///            "description": "Holds a list of references associated with this vulnerability item.",
///            "$ref": "#/$defs/references_t"
///          },
///          "release_date": {
///            "title": "Release date",
///            "description": "Holds the date and time the vulnerability was originally released into the wild.",
///            "type": "string",
///            "format": "date-time"
///          },
///          "remediations": {
///            "title": "List of remediations",
///            "description": "Contains a list of remediations.",
///            "type": "array",
///            "items": {
///              "title": "Remediation",
///              "description": "Specifies details on how to handle (and presumably, fix) a vulnerability.",
///              "type": "object",
///              "required": [
///                "category",
///                "details"
///              ],
///              "properties": {
///                "category": {
///                  "title": "Category of the remediation",
///                  "description": "Specifies the category which this remediation belongs to.",
///                  "type": "string",
///                  "enum": [
///                    "mitigation",
///                    "no_fix_planned",
///                    "none_available",
///                    "vendor_fix",
///                    "workaround"
///                  ]
///                },
///                "date": {
///                  "title": "Date of the remediation",
///                  "description": "Contains the date from which the remediation is available.",
///                  "type": "string",
///                  "format": "date-time"
///                },
///                "details": {
///                  "title": "Details of the remediation",
///                  "description": "Contains a thorough human-readable discussion of the remediation.",
///                  "type": "string",
///                  "minLength": 1
///                },
///                "entitlements": {
///                  "title": "List of entitlements",
///                  "description": "Contains a list of entitlements.",
///                  "type": "array",
///                  "items": {
///                    "title": "Entitlement of the remediation",
///                    "description": "Contains any possible vendor-defined constraints for obtaining fixed software or hardware that fully resolves the vulnerability.",
///                    "type": "string",
///                    "minLength": 1
///                  },
///                  "minItems": 1
///                },
///                "group_ids": {
///                  "$ref": "#/$defs/product_groups_t"
///                },
///                "product_ids": {
///                  "$ref": "#/$defs/products_t"
///                },
///                "restart_required": {
///                  "title": "Restart required by remediation",
///                  "description": "Provides information on category of restart is required by this remediation to become effective.",
///                  "type": "object",
///                  "required": [
///                    "category"
///                  ],
///                  "properties": {
///                    "category": {
///                      "title": "Category of restart",
///                      "description": "Specifies what category of restart is required by this remediation to become effective.",
///                      "type": "string",
///                      "enum": [
///                        "connected",
///                        "dependencies",
///                        "machine",
///                        "none",
///                        "parent",
///                        "service",
///                        "system",
///                        "vulnerable_component",
///                        "zone"
///                      ]
///                    },
///                    "details": {
///                      "title": "Additional restart information",
///                      "description": "Provides additional information for the restart. This can include details on procedures, scope or impact.",
///                      "type": "string",
///                      "minLength": 1
///                    }
///                  }
///                },
///                "url": {
///                  "title": "URL to the remediation",
///                  "description": "Contains the URL where to obtain the remediation.",
///                  "type": "string",
///                  "format": "uri"
///                }
///              }
///            },
///            "minItems": 1
///          },
///          "scores": {
///            "title": "List of scores",
///            "description": "Contains score objects for the current vulnerability.",
///            "type": "array",
///            "items": {
///              "title": "Score",
///              "description": "Specifies information about (at least one) score of the vulnerability and for which products the given value applies.",
///              "type": "object",
///              "minProperties": 2,
///              "required": [
///                "products"
///              ],
///              "properties": {
///                "cvss_v2": {
///                  "type": "object"
///                },
///                "cvss_v3": {
///                  "type": "object"
///                },
///                "products": {
///                  "$ref": "#/$defs/products_t"
///                }
///              }
///            },
///            "minItems": 1
///          },
///          "threats": {
///            "title": "List of threats",
///            "description": "Contains information about a vulnerability that can change with time.",
///            "type": "array",
///            "items": {
///              "title": "Threat",
///              "description": "Contains the vulnerability kinetic information. This information can change as the vulnerability ages and new information becomes available.",
///              "type": "object",
///              "required": [
///                "category",
///                "details"
///              ],
///              "properties": {
///                "category": {
///                  "title": "Category of the threat",
///                  "description": "Categorizes the threat according to the rules of the specification.",
///                  "type": "string",
///                  "enum": [
///                    "exploit_status",
///                    "impact",
///                    "target_set"
///                  ]
///                },
///                "date": {
///                  "title": "Date of the threat",
///                  "description": "Contains the date when the assessment was done or the threat appeared.",
///                  "type": "string",
///                  "format": "date-time"
///                },
///                "details": {
///                  "title": "Details of the threat",
///                  "description": "Represents a thorough human-readable discussion of the threat.",
///                  "type": "string",
///                  "minLength": 1
///                },
///                "group_ids": {
///                  "$ref": "#/$defs/product_groups_t"
///                },
///                "product_ids": {
///                  "$ref": "#/$defs/products_t"
///                }
///              }
///            },
///            "minItems": 1
///          },
///          "title": {
///            "title": "Title",
///            "description": "Gives the document producer the ability to apply a canonical name or title to the vulnerability.",
///            "type": "string",
///            "minLength": 1
///          }
///        }
///      },
///      "minItems": 1
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CommonSecurityAdvisoryFramework {
    pub document: DocumentLevelMetaData,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_tree: Option<ProductTree>,
    ///Represents a list of all relevant vulnerability information items.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vulnerabilities: Vec<Vulnerability>,
}
impl From<&CommonSecurityAdvisoryFramework> for CommonSecurityAdvisoryFramework {
    fn from(value: &CommonSecurityAdvisoryFramework) -> Self {
        value.clone()
    }
}
impl CommonSecurityAdvisoryFramework {
    pub fn builder() -> builder::CommonSecurityAdvisoryFramework {
        Default::default()
    }
}
///Information on how to contact the publisher, possibly including details such as web sites, email addresses, phone numbers, and postal mail addresses.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Contact details",
///  "description": "Information on how to contact the publisher, possibly including details such as web sites, email addresses, phone numbers, and postal mail addresses.",
///  "examples": [
///    "Example Company can be reached at contact_us@example.com, or via our website at https://www.example.com/contact."
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ContactDetails(String);
impl ::std::ops::Deref for ContactDetails {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<ContactDetails> for String {
    fn from(value: ContactDetails) -> Self {
        value.0
    }
}
impl From<&ContactDetails> for ContactDetails {
    fn from(value: &ContactDetails) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ContactDetails {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ContactDetails {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for ContactDetails {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for ContactDetails {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ContactDetails {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Contains the name of a contributing organization being recognized.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Contributing organization",
///  "description": "Contains the name of a contributing organization being recognized.",
///  "examples": [
///    "CISA",
///    "Google Project Zero",
///    "Talos"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ContributingOrganization(String);
impl ::std::ops::Deref for ContributingOrganization {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<ContributingOrganization> for String {
    fn from(value: ContributingOrganization) -> Self {
        value.0
    }
}
impl From<&ContributingOrganization> for ContributingOrganization {
    fn from(value: &ContributingOrganization) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ContributingOrganization {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ContributingOrganization {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for ContributingOrganization {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for ContributingOrganization {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ContributingOrganization {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Contains all information to identify a file based on its cryptographic hash values.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Cryptographic hashes",
///  "description": "Contains all information to identify a file based on its cryptographic hash values.",
///  "type": "object",
///  "required": [
///    "file_hashes",
///    "filename"
///  ],
///  "properties": {
///    "file_hashes": {
///      "title": "List of file hashes",
///      "description": "Contains a list of cryptographic hashes for this file.",
///      "type": "array",
///      "items": {
///        "title": "File hash",
///        "description": "Contains one hash value and algorithm of the file to be identified.",
///        "type": "object",
///        "required": [
///          "algorithm",
///          "value"
///        ],
///        "properties": {
///          "algorithm": {
///            "title": "Algorithm of the cryptographic hash",
///            "description": "Contains the name of the cryptographic hash algorithm used to calculate the value.",
///            "default": "sha256",
///            "examples": [
///              "blake2b512",
///              "sha256",
///              "sha3-512",
///              "sha384",
///              "sha512"
///            ],
///            "type": "string",
///            "minLength": 1
///          },
///          "value": {
///            "title": "Value of the cryptographic hash",
///            "description": "Contains the cryptographic hash value in hexadecimal representation.",
///            "examples": [
///              "37df33cb7464da5c7f077f4d56a32bc84987ec1d85b234537c1c1a4d4fc8d09dc29e2e762cb5203677bf849a2855a0283710f1f5fe1d6ce8d5ac85c645d0fcb3",
///              "4775203615d9534a8bfca96a93dc8b461a489f69124a130d786b42204f3341cc",
///              "9ea4c8200113d49d26505da0e02e2f49055dc078d1ad7a419b32e291c7afebbb84badfbd46dec42883bea0b2a1fa697c"
///            ],
///            "type": "string",
///            "minLength": 32,
///            "pattern": "^[0-9a-fA-F]{32,}$"
///          }
///        }
///      },
///      "minItems": 1
///    },
///    "filename": {
///      "title": "Filename",
///      "description": "Contains the name of the file which is identified by the hash values.",
///      "examples": [
///        "WINWORD.EXE",
///        "msotadddin.dll",
///        "sudoers.so"
///      ],
///      "type": "string",
///      "minLength": 1
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CryptographicHashes {
    ///Contains a list of cryptographic hashes for this file.
    pub file_hashes: Vec<FileHash>,
    ///Contains the name of the file which is identified by the hash values.
    pub filename: Filename,
}
impl From<&CryptographicHashes> for CryptographicHashes {
    fn from(value: &CryptographicHashes) -> Self {
        value.clone()
    }
}
impl CryptographicHashes {
    pub fn builder() -> builder::CryptographicHashes {
        Default::default()
    }
}
///Gives the version of the CSAF specification which the document was generated for.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "CSAF version",
///  "description": "Gives the version of the CSAF specification which the document was generated for.",
///  "type": "string",
///  "enum": [
///    "2.0"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CsafVersion {
    #[serde(rename = "2.0")]
    _20,
}
impl From<&CsafVersion> for CsafVersion {
    fn from(value: &CsafVersion) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CsafVersion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::_20 => write!(f, "2.0"),
        }
    }
}
impl std::str::FromStr for CsafVersion {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "2.0" => Ok(Self::_20),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for CsafVersion {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CsafVersion {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CsafVersion {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Holds the MITRE standard Common Vulnerabilities and Exposures (CVE) tracking number for the vulnerability.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "CVE",
///  "description": "Holds the MITRE standard Common Vulnerabilities and Exposures (CVE) tracking number for the vulnerability.",
///  "type": "string",
///  "pattern": "^CVE-[0-9]{4}-[0-9]{4,}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Cve(String);
impl ::std::ops::Deref for Cve {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Cve> for String {
    fn from(value: Cve) -> Self {
        value.0
    }
}
impl From<&Cve> for Cve {
    fn from(value: &Cve) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Cve {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^CVE-[0-9]{4}-[0-9]{4,}$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^CVE-[0-9]{4}-[0-9]{4,}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Cve {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for Cve {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for Cve {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Cve {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Holds the MITRE standard Common Weakness Enumeration (CWE) for the weakness associated.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "CWE",
///  "description": "Holds the MITRE standard Common Weakness Enumeration (CWE) for the weakness associated.",
///  "type": "object",
///  "required": [
///    "id",
///    "name"
///  ],
///  "properties": {
///    "id": {
///      "title": "Weakness ID",
///      "description": "Holds the ID for the weakness associated.",
///      "examples": [
///        "CWE-22",
///        "CWE-352",
///        "CWE-79"
///      ],
///      "type": "string",
///      "pattern": "^CWE-[1-9]\\d{0,5}$"
///    },
///    "name": {
///      "title": "Weakness name",
///      "description": "Holds the full name of the weakness as given in the CWE specification.",
///      "examples": [
///        "Cross-Site Request Forgery (CSRF)",
///        "Improper Limitation of a Pathname to a Restricted Directory ('Path Traversal')",
///        "Improper Neutralization of Input During Web Page Generation ('Cross-site Scripting')"
///      ],
///      "type": "string",
///      "minLength": 1
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Cwe {
    ///Holds the ID for the weakness associated.
    pub id: WeaknessId,
    ///Holds the full name of the weakness as given in the CWE specification.
    pub name: WeaknessName,
}
impl From<&Cwe> for Cwe {
    fn from(value: &Cwe) -> Self {
        value.clone()
    }
}
impl Cwe {
    pub fn builder() -> builder::Cwe {
        Default::default()
    }
}
///Contains a thorough human-readable discussion of the remediation.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Details of the remediation",
///  "description": "Contains a thorough human-readable discussion of the remediation.",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DetailsOfTheRemediation(String);
impl ::std::ops::Deref for DetailsOfTheRemediation {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<DetailsOfTheRemediation> for String {
    fn from(value: DetailsOfTheRemediation) -> Self {
        value.0
    }
}
impl From<&DetailsOfTheRemediation> for DetailsOfTheRemediation {
    fn from(value: &DetailsOfTheRemediation) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for DetailsOfTheRemediation {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DetailsOfTheRemediation {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for DetailsOfTheRemediation {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for DetailsOfTheRemediation {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DetailsOfTheRemediation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Represents a thorough human-readable discussion of the threat.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Details of the threat",
///  "description": "Represents a thorough human-readable discussion of the threat.",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DetailsOfTheThreat(String);
impl ::std::ops::Deref for DetailsOfTheThreat {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<DetailsOfTheThreat> for String {
    fn from(value: DetailsOfTheThreat) -> Self {
        value.0
    }
}
impl From<&DetailsOfTheThreat> for DetailsOfTheThreat {
    fn from(value: &DetailsOfTheThreat) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for DetailsOfTheThreat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DetailsOfTheThreat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for DetailsOfTheThreat {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for DetailsOfTheThreat {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DetailsOfTheThreat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Defines a short canonical name, chosen by the document producer, which will inform the end user as to the category of document.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Document category",
///  "description": "Defines a short canonical name, chosen by the document producer, which will inform the end user as to the category of document.",
///  "examples": [
///    "csaf_base",
///    "csaf_security_advisory",
///    "csaf_vex",
///    "Example Company Security Notice"
///  ],
///  "type": "string",
///  "minLength": 1,
///  "pattern": "^[^\\s\\-_\\.](.*[^\\s\\-_\\.])?$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DocumentCategory(String);
impl ::std::ops::Deref for DocumentCategory {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<DocumentCategory> for String {
    fn from(value: DocumentCategory) -> Self {
        value.0
    }
}
impl From<&DocumentCategory> for DocumentCategory {
    fn from(value: &DocumentCategory) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for DocumentCategory {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        if regress::Regex::new("^[^\\s\\-_\\.](.*[^\\s\\-_\\.])?$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^[^\\s\\-_\\.](.*[^\\s\\-_\\.])?$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DocumentCategory {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for DocumentCategory {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for DocumentCategory {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DocumentCategory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Is a container to hold all elements related to the generation of the document. These items will reference when the document was actually created, including the date it was generated and the entity that generated it.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Document generator",
///  "description": "Is a container to hold all elements related to the generation of the document. These items will reference when the document was actually created, including the date it was generated and the entity that generated it.",
///  "type": "object",
///  "required": [
///    "engine"
///  ],
///  "properties": {
///    "date": {
///      "title": "Date of document generation",
///      "description": "This SHOULD be the current date that the document was generated. Because documents are often generated internally by a document producer and exist for a nonzero amount of time before being released, this field MAY be different from the Initial Release Date and Current Release Date.",
///      "type": "string",
///      "format": "date-time"
///    },
///    "engine": {
///      "title": "Engine of document generation",
///      "description": "Contains information about the engine that generated the CSAF document.",
///      "type": "object",
///      "required": [
///        "name"
///      ],
///      "properties": {
///        "name": {
///          "title": "Engine name",
///          "description": "Represents the name of the engine that generated the CSAF document.",
///          "examples": [
///            "Red Hat rhsa-to-cvrf",
///            "Secvisogram",
///            "TVCE"
///          ],
///          "type": "string",
///          "minLength": 1
///        },
///        "version": {
///          "title": "Engine version",
///          "description": "Contains the version of the engine that generated the CSAF document.",
///          "examples": [
///            "0.6.0",
///            "1.0.0-beta+exp.sha.a1c44f85",
///            "2"
///          ],
///          "type": "string",
///          "minLength": 1
///        }
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DocumentGenerator {
    ///This SHOULD be the current date that the document was generated. Because documents are often generated internally by a document producer and exist for a nonzero amount of time before being released, this field MAY be different from the Initial Release Date and Current Release Date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<chrono::DateTime<chrono::offset::Utc>>,
    pub engine: EngineOfDocumentGeneration,
}
impl From<&DocumentGenerator> for DocumentGenerator {
    fn from(value: &DocumentGenerator) -> Self {
        value.clone()
    }
}
impl DocumentGenerator {
    pub fn builder() -> builder::DocumentGenerator {
        Default::default()
    }
}
///Captures the meta-data about this document describing a particular set of security advisories.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Document level meta-data",
///  "description": "Captures the meta-data about this document describing a particular set of security advisories.",
///  "type": "object",
///  "required": [
///    "category",
///    "csaf_version",
///    "publisher",
///    "title",
///    "tracking"
///  ],
///  "properties": {
///    "acknowledgments": {
///      "title": "Document acknowledgments",
///      "description": "Contains a list of acknowledgment elements associated with the whole document.",
///      "$ref": "#/$defs/acknowledgments_t"
///    },
///    "aggregate_severity": {
///      "title": "Aggregate severity",
///      "description": "Is a vehicle that is provided by the document producer to convey the urgency and criticality with which the one or more vulnerabilities reported should be addressed. It is a document-level metric and applied to the document as a whole — not any specific vulnerability. The range of values in this field is defined according to the document producer's policies and procedures.",
///      "type": "object",
///      "required": [
///        "text"
///      ],
///      "properties": {
///        "namespace": {
///          "title": "Namespace of aggregate severity",
///          "description": "Points to the namespace so referenced.",
///          "type": "string",
///          "format": "uri"
///        },
///        "text": {
///          "title": "Text of aggregate severity",
///          "description": "Provides a severity which is independent of - and in addition to - any other standard metric for determining the impact or severity of a given vulnerability (such as CVSS).",
///          "examples": [
///            "Critical",
///            "Important",
///            "Moderate"
///          ],
///          "type": "string",
///          "minLength": 1
///        }
///      }
///    },
///    "category": {
///      "title": "Document category",
///      "description": "Defines a short canonical name, chosen by the document producer, which will inform the end user as to the category of document.",
///      "examples": [
///        "csaf_base",
///        "csaf_security_advisory",
///        "csaf_vex",
///        "Example Company Security Notice"
///      ],
///      "type": "string",
///      "minLength": 1,
///      "pattern": "^[^\\s\\-_\\.](.*[^\\s\\-_\\.])?$"
///    },
///    "csaf_version": {
///      "title": "CSAF version",
///      "description": "Gives the version of the CSAF specification which the document was generated for.",
///      "type": "string",
///      "enum": [
///        "2.0"
///      ]
///    },
///    "distribution": {
///      "title": "Rules for sharing document",
///      "description": "Describe any constraints on how this document might be shared.",
///      "type": "object",
///      "minProperties": 1,
///      "properties": {
///        "text": {
///          "title": "Textual description",
///          "description": "Provides a textual description of additional constraints.",
///          "examples": [
///            "Copyright 2021, Example Company, All Rights Reserved.",
///            "Distribute freely.",
///            "Share only on a need-to-know-basis only."
///          ],
///          "type": "string",
///          "minLength": 1
///        },
///        "tlp": {
///          "title": "Traffic Light Protocol (TLP)",
///          "description": "Provides details about the TLP classification of the document.",
///          "type": "object",
///          "required": [
///            "label"
///          ],
///          "properties": {
///            "label": {
///              "title": "Label of TLP",
///              "description": "Provides the TLP label of the document.",
///              "type": "string",
///              "enum": [
///                "AMBER",
///                "GREEN",
///                "RED",
///                "WHITE"
///              ]
///            },
///            "url": {
///              "title": "URL of TLP version",
///              "description": "Provides a URL where to find the textual description of the TLP version which is used in this document. Default is the URL to the definition by FIRST.",
///              "default": "https://www.first.org/tlp/",
///              "examples": [
///                "https://www.us-cert.gov/tlp",
///                "https://www.bsi.bund.de/SharedDocs/Downloads/DE/BSI/Kritis/Merkblatt_TLP.pdf"
///              ],
///              "type": "string",
///              "format": "uri"
///            }
///          }
///        }
///      }
///    },
///    "lang": {
///      "title": "Document language",
///      "description": "Identifies the language used by this document, corresponding to IETF BCP 47 / RFC 5646.",
///      "$ref": "#/$defs/lang_t"
///    },
///    "notes": {
///      "title": "Document notes",
///      "description": "Holds notes associated with the whole document.",
///      "$ref": "#/$defs/notes_t"
///    },
///    "publisher": {
///      "title": "Publisher",
///      "description": "Provides information about the publisher of the document.",
///      "type": "object",
///      "required": [
///        "category",
///        "name",
///        "namespace"
///      ],
///      "properties": {
///        "category": {
///          "title": "Category of publisher",
///          "description": "Provides information about the category of publisher releasing the document.",
///          "type": "string",
///          "enum": [
///            "coordinator",
///            "discoverer",
///            "other",
///            "translator",
///            "user",
///            "vendor"
///          ]
///        },
///        "contact_details": {
///          "title": "Contact details",
///          "description": "Information on how to contact the publisher, possibly including details such as web sites, email addresses, phone numbers, and postal mail addresses.",
///          "examples": [
///            "Example Company can be reached at contact_us@example.com, or via our website at https://www.example.com/contact."
///          ],
///          "type": "string",
///          "minLength": 1
///        },
///        "issuing_authority": {
///          "title": "Issuing authority",
///          "description": "Provides information about the authority of the issuing party to release the document, in particular, the party's constituency and responsibilities or other obligations.",
///          "type": "string",
///          "minLength": 1
///        },
///        "name": {
///          "title": "Name of publisher",
///          "description": "Contains the name of the issuing party.",
///          "examples": [
///            "BSI",
///            "Cisco PSIRT",
///            "Siemens ProductCERT"
///          ],
///          "type": "string",
///          "minLength": 1
///        },
///        "namespace": {
///          "title": "Namespace of publisher",
///          "description": "Contains a URL which is under control of the issuing party and can be used as a globally unique identifier for that issuing party.",
///          "examples": [
///            "https://csaf.io",
///            "https://www.example.com"
///          ],
///          "type": "string",
///          "format": "uri"
///        }
///      }
///    },
///    "references": {
///      "title": "Document references",
///      "description": "Holds a list of references associated with the whole document.",
///      "$ref": "#/$defs/references_t"
///    },
///    "source_lang": {
///      "title": "Source language",
///      "description": "If this copy of the document is a translation then the value of this property describes from which language this document was translated.",
///      "$ref": "#/$defs/lang_t"
///    },
///    "title": {
///      "title": "Title of this document",
///      "description": "This SHOULD be a canonical name for the document, and sufficiently unique to distinguish it from similar documents.",
///      "examples": [
///        "Cisco IPv6 Crafted Packet Denial of Service Vulnerability",
///        "Example Company Cross-Site-Scripting Vulnerability in Example Generator"
///      ],
///      "type": "string",
///      "minLength": 1
///    },
///    "tracking": {
///      "title": "Tracking",
///      "description": "Is a container designated to hold all management attributes necessary to track a CSAF document as a whole.",
///      "type": "object",
///      "required": [
///        "current_release_date",
///        "id",
///        "initial_release_date",
///        "revision_history",
///        "status",
///        "version"
///      ],
///      "properties": {
///        "aliases": {
///          "title": "Aliases",
///          "description": "Contains a list of alternate names for the same document.",
///          "type": "array",
///          "items": {
///            "title": "Alternate name",
///            "description": "Specifies a non-empty string that represents a distinct optional alternative ID used to refer to the document.",
///            "examples": [
///              "CVE-2019-12345"
///            ],
///            "type": "string",
///            "minLength": 1
///          },
///          "minItems": 1,
///          "uniqueItems": true
///        },
///        "current_release_date": {
///          "title": "Current release date",
///          "description": "The date when the current revision of this document was released",
///          "type": "string",
///          "format": "date-time"
///        },
///        "generator": {
///          "title": "Document generator",
///          "description": "Is a container to hold all elements related to the generation of the document. These items will reference when the document was actually created, including the date it was generated and the entity that generated it.",
///          "type": "object",
///          "required": [
///            "engine"
///          ],
///          "properties": {
///            "date": {
///              "title": "Date of document generation",
///              "description": "This SHOULD be the current date that the document was generated. Because documents are often generated internally by a document producer and exist for a nonzero amount of time before being released, this field MAY be different from the Initial Release Date and Current Release Date.",
///              "type": "string",
///              "format": "date-time"
///            },
///            "engine": {
///              "title": "Engine of document generation",
///              "description": "Contains information about the engine that generated the CSAF document.",
///              "type": "object",
///              "required": [
///                "name"
///              ],
///              "properties": {
///                "name": {
///                  "title": "Engine name",
///                  "description": "Represents the name of the engine that generated the CSAF document.",
///                  "examples": [
///                    "Red Hat rhsa-to-cvrf",
///                    "Secvisogram",
///                    "TVCE"
///                  ],
///                  "type": "string",
///                  "minLength": 1
///                },
///                "version": {
///                  "title": "Engine version",
///                  "description": "Contains the version of the engine that generated the CSAF document.",
///                  "examples": [
///                    "0.6.0",
///                    "1.0.0-beta+exp.sha.a1c44f85",
///                    "2"
///                  ],
///                  "type": "string",
///                  "minLength": 1
///                }
///              }
///            }
///          }
///        },
///        "id": {
///          "title": "Unique identifier for the document",
///          "description": "The ID is a simple label that provides for a wide range of numbering values, types, and schemes. Its value SHOULD be assigned and maintained by the original document issuing authority.",
///          "examples": [
///            "Example Company - 2019-YH3234",
///            "RHBA-2019:0024",
///            "cisco-sa-20190513-secureboot"
///          ],
///          "type": "string",
///          "minLength": 1,
///          "pattern": "^[\\S](.*[\\S])?$"
///        },
///        "initial_release_date": {
///          "title": "Initial release date",
///          "description": "The date when this document was first published.",
///          "type": "string",
///          "format": "date-time"
///        },
///        "revision_history": {
///          "title": "Revision history",
///          "description": "Holds one revision item for each version of the CSAF document, including the initial one.",
///          "type": "array",
///          "items": {
///            "title": "Revision",
///            "description": "Contains all the information elements required to track the evolution of a CSAF document.",
///            "type": "object",
///            "required": [
///              "date",
///              "number",
///              "summary"
///            ],
///            "properties": {
///              "date": {
///                "title": "Date of the revision",
///                "description": "The date of the revision entry",
///                "type": "string",
///                "format": "date-time"
///              },
///              "legacy_version": {
///                "title": "Legacy version of the revision",
///                "description": "Contains the version string used in an existing document with the same content.",
///                "type": "string",
///                "minLength": 1
///              },
///              "number": {
///                "$ref": "#/$defs/version_t"
///              },
///              "summary": {
///                "title": "Summary of the revision",
///                "description": "Holds a single non-empty string representing a short description of the changes.",
///                "examples": [
///                  "Initial version."
///                ],
///                "type": "string",
///                "minLength": 1
///              }
///            }
///          },
///          "minItems": 1
///        },
///        "status": {
///          "title": "Document status",
///          "description": "Defines the draft status of the document.",
///          "type": "string",
///          "enum": [
///            "draft",
///            "final",
///            "interim"
///          ]
///        },
///        "version": {
///          "$ref": "#/$defs/version_t"
///        }
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DocumentLevelMetaData {
    ///Contains a list of acknowledgment elements associated with the whole document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acknowledgments: Option<AcknowledgmentsT>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregate_severity: Option<AggregateSeverity>,
    ///Defines a short canonical name, chosen by the document producer, which will inform the end user as to the category of document.
    pub category: DocumentCategory,
    ///Gives the version of the CSAF specification which the document was generated for.
    pub csaf_version: CsafVersion,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distribution: Option<RulesForSharingDocument>,
    ///Identifies the language used by this document, corresponding to IETF BCP 47 / RFC 5646.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<LangT>,
    ///Holds notes associated with the whole document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<NotesT>,
    pub publisher: Publisher,
    ///Holds a list of references associated with the whole document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub references: Option<ReferencesT>,
    ///If this copy of the document is a translation then the value of this property describes from which language this document was translated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_lang: Option<LangT>,
    ///This SHOULD be a canonical name for the document, and sufficiently unique to distinguish it from similar documents.
    pub title: TitleOfThisDocument,
    pub tracking: Tracking,
}
impl From<&DocumentLevelMetaData> for DocumentLevelMetaData {
    fn from(value: &DocumentLevelMetaData) -> Self {
        value.clone()
    }
}
impl DocumentLevelMetaData {
    pub fn builder() -> builder::DocumentLevelMetaData {
        Default::default()
    }
}
///Defines the draft status of the document.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Document status",
///  "description": "Defines the draft status of the document.",
///  "type": "string",
///  "enum": [
///    "draft",
///    "final",
///    "interim"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DocumentStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "final")]
    Final,
    #[serde(rename = "interim")]
    Interim,
}
impl From<&DocumentStatus> for DocumentStatus {
    fn from(value: &DocumentStatus) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for DocumentStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Draft => write!(f, "draft"),
            Self::Final => write!(f, "final"),
            Self::Interim => write!(f, "interim"),
        }
    }
}
impl std::str::FromStr for DocumentStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "draft" => Ok(Self::Draft),
            "final" => Ok(Self::Final),
            "interim" => Ok(Self::Interim),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for DocumentStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DocumentStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DocumentStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Represents the name of the engine that generated the CSAF document.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Engine name",
///  "description": "Represents the name of the engine that generated the CSAF document.",
///  "examples": [
///    "Red Hat rhsa-to-cvrf",
///    "Secvisogram",
///    "TVCE"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EngineName(String);
impl ::std::ops::Deref for EngineName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<EngineName> for String {
    fn from(value: EngineName) -> Self {
        value.0
    }
}
impl From<&EngineName> for EngineName {
    fn from(value: &EngineName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for EngineName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for EngineName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for EngineName {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for EngineName {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for EngineName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Contains information about the engine that generated the CSAF document.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Engine of document generation",
///  "description": "Contains information about the engine that generated the CSAF document.",
///  "type": "object",
///  "required": [
///    "name"
///  ],
///  "properties": {
///    "name": {
///      "title": "Engine name",
///      "description": "Represents the name of the engine that generated the CSAF document.",
///      "examples": [
///        "Red Hat rhsa-to-cvrf",
///        "Secvisogram",
///        "TVCE"
///      ],
///      "type": "string",
///      "minLength": 1
///    },
///    "version": {
///      "title": "Engine version",
///      "description": "Contains the version of the engine that generated the CSAF document.",
///      "examples": [
///        "0.6.0",
///        "1.0.0-beta+exp.sha.a1c44f85",
///        "2"
///      ],
///      "type": "string",
///      "minLength": 1
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct EngineOfDocumentGeneration {
    ///Represents the name of the engine that generated the CSAF document.
    pub name: EngineName,
    ///Contains the version of the engine that generated the CSAF document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<EngineVersion>,
}
impl From<&EngineOfDocumentGeneration> for EngineOfDocumentGeneration {
    fn from(value: &EngineOfDocumentGeneration) -> Self {
        value.clone()
    }
}
impl EngineOfDocumentGeneration {
    pub fn builder() -> builder::EngineOfDocumentGeneration {
        Default::default()
    }
}
///Contains the version of the engine that generated the CSAF document.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Engine version",
///  "description": "Contains the version of the engine that generated the CSAF document.",
///  "examples": [
///    "0.6.0",
///    "1.0.0-beta+exp.sha.a1c44f85",
///    "2"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EngineVersion(String);
impl ::std::ops::Deref for EngineVersion {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<EngineVersion> for String {
    fn from(value: EngineVersion) -> Self {
        value.0
    }
}
impl From<&EngineVersion> for EngineVersion {
    fn from(value: &EngineVersion) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for EngineVersion {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for EngineVersion {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for EngineVersion {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for EngineVersion {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for EngineVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Contains any possible vendor-defined constraints for obtaining fixed software or hardware that fully resolves the vulnerability.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Entitlement of the remediation",
///  "description": "Contains any possible vendor-defined constraints for obtaining fixed software or hardware that fully resolves the vulnerability.",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EntitlementOfTheRemediation(String);
impl ::std::ops::Deref for EntitlementOfTheRemediation {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<EntitlementOfTheRemediation> for String {
    fn from(value: EntitlementOfTheRemediation) -> Self {
        value.0
    }
}
impl From<&EntitlementOfTheRemediation> for EntitlementOfTheRemediation {
    fn from(value: &EntitlementOfTheRemediation) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for EntitlementOfTheRemediation {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for EntitlementOfTheRemediation {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for EntitlementOfTheRemediation {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for EntitlementOfTheRemediation {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for EntitlementOfTheRemediation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Contains one hash value and algorithm of the file to be identified.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "File hash",
///  "description": "Contains one hash value and algorithm of the file to be identified.",
///  "type": "object",
///  "required": [
///    "algorithm",
///    "value"
///  ],
///  "properties": {
///    "algorithm": {
///      "title": "Algorithm of the cryptographic hash",
///      "description": "Contains the name of the cryptographic hash algorithm used to calculate the value.",
///      "default": "sha256",
///      "examples": [
///        "blake2b512",
///        "sha256",
///        "sha3-512",
///        "sha384",
///        "sha512"
///      ],
///      "type": "string",
///      "minLength": 1
///    },
///    "value": {
///      "title": "Value of the cryptographic hash",
///      "description": "Contains the cryptographic hash value in hexadecimal representation.",
///      "examples": [
///        "37df33cb7464da5c7f077f4d56a32bc84987ec1d85b234537c1c1a4d4fc8d09dc29e2e762cb5203677bf849a2855a0283710f1f5fe1d6ce8d5ac85c645d0fcb3",
///        "4775203615d9534a8bfca96a93dc8b461a489f69124a130d786b42204f3341cc",
///        "9ea4c8200113d49d26505da0e02e2f49055dc078d1ad7a419b32e291c7afebbb84badfbd46dec42883bea0b2a1fa697c"
///      ],
///      "type": "string",
///      "minLength": 32,
///      "pattern": "^[0-9a-fA-F]{32,}$"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FileHash {
    ///Contains the name of the cryptographic hash algorithm used to calculate the value.
    pub algorithm: AlgorithmOfTheCryptographicHash,
    ///Contains the cryptographic hash value in hexadecimal representation.
    pub value: ValueOfTheCryptographicHash,
}
impl From<&FileHash> for FileHash {
    fn from(value: &FileHash) -> Self {
        value.clone()
    }
}
impl FileHash {
    pub fn builder() -> builder::FileHash {
        Default::default()
    }
}
///Contains the name of the file which is identified by the hash values.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Filename",
///  "description": "Contains the name of the file which is identified by the hash values.",
///  "examples": [
///    "WINWORD.EXE",
///    "msotadddin.dll",
///    "sudoers.so"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Filename(String);
impl ::std::ops::Deref for Filename {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Filename> for String {
    fn from(value: Filename) -> Self {
        value.0
    }
}
impl From<&Filename> for Filename {
    fn from(value: &Filename) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Filename {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Filename {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for Filename {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for Filename {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Filename {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Contains product specific information in regard to this vulnerability as a single machine readable flag.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Flag",
///  "description": "Contains product specific information in regard to this vulnerability as a single machine readable flag.",
///  "type": "object",
///  "required": [
///    "label"
///  ],
///  "properties": {
///    "date": {
///      "title": "Date of the flag",
///      "description": "Contains the date when assessment was done or the flag was assigned.",
///      "type": "string",
///      "format": "date-time"
///    },
///    "group_ids": {
///      "$ref": "#/$defs/product_groups_t"
///    },
///    "label": {
///      "title": "Label of the flag",
///      "description": "Specifies the machine readable label.",
///      "type": "string",
///      "enum": [
///        "component_not_present",
///        "inline_mitigations_already_exist",
///        "vulnerable_code_cannot_be_controlled_by_adversary",
///        "vulnerable_code_not_in_execute_path",
///        "vulnerable_code_not_present"
///      ]
///    },
///    "product_ids": {
///      "$ref": "#/$defs/products_t"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Flag {
    ///Contains the date when assessment was done or the flag was assigned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<ProductGroupsT>,
    ///Specifies the machine readable label.
    pub label: LabelOfTheFlag,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_ids: Option<ProductsT>,
}
impl From<&Flag> for Flag {
    fn from(value: &Flag) -> Self {
        value.clone()
    }
}
impl Flag {
    pub fn builder() -> builder::Flag {
        Default::default()
    }
}
///Specifies information about the product and assigns the product_id.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Full product name",
///  "description": "Specifies information about the product and assigns the product_id.",
///  "type": "object",
///  "required": [
///    "name",
///    "product_id"
///  ],
///  "properties": {
///    "name": {
///      "title": "Textual description of the product",
///      "description": "The value should be the product’s full canonical name, including version number and other attributes, as it would be used in a human-friendly document.",
///      "examples": [
///        "Cisco AnyConnect Secure Mobility Client 2.3.185",
///        "Microsoft Host Integration Server 2006 Service Pack 1"
///      ],
///      "type": "string",
///      "minLength": 1
///    },
///    "product_id": {
///      "$ref": "#/$defs/product_id_t"
///    },
///    "product_identification_helper": {
///      "title": "Helper to identify the product",
///      "description": "Provides at least one method which aids in identifying the product in an asset database.",
///      "type": "object",
///      "minProperties": 1,
///      "properties": {
///        "cpe": {
///          "title": "Common Platform Enumeration representation",
///          "description": "The Common Platform Enumeration (CPE) attribute refers to a method for naming platforms external to this specification.",
///          "type": "string",
///          "minLength": 5,
///          "pattern": "^(cpe:2\\.3:[aho\\*\\-](:(((\\?*|\\*?)([a-zA-Z0-9\\-\\._]|(\\\\[\\\\\\*\\?!\"#\\$%&'\\(\\)\\+,/:;<=>@\\[\\]\\^`\\{\\|\\}~]))+(\\?*|\\*?))|[\\*\\-])){5}(:(([a-zA-Z]{2,3}(-([a-zA-Z]{2}|[0-9]{3}))?)|[\\*\\-]))(:(((\\?*|\\*?)([a-zA-Z0-9\\-\\._]|(\\\\[\\\\\\*\\?!\"#\\$%&'\\(\\)\\+,/:;<=>@\\[\\]\\^`\\{\\|\\}~]))+(\\?*|\\*?))|[\\*\\-])){4})|([c][pP][eE]:/[AHOaho]?(:[A-Za-z0-9\\._\\-~%]*){0,6})$"
///        },
///        "hashes": {
///          "title": "List of hashes",
///          "description": "Contains a list of cryptographic hashes usable to identify files.",
///          "type": "array",
///          "items": {
///            "title": "Cryptographic hashes",
///            "description": "Contains all information to identify a file based on its cryptographic hash values.",
///            "type": "object",
///            "required": [
///              "file_hashes",
///              "filename"
///            ],
///            "properties": {
///              "file_hashes": {
///                "title": "List of file hashes",
///                "description": "Contains a list of cryptographic hashes for this file.",
///                "type": "array",
///                "items": {
///                  "title": "File hash",
///                  "description": "Contains one hash value and algorithm of the file to be identified.",
///                  "type": "object",
///                  "required": [
///                    "algorithm",
///                    "value"
///                  ],
///                  "properties": {
///                    "algorithm": {
///                      "title": "Algorithm of the cryptographic hash",
///                      "description": "Contains the name of the cryptographic hash algorithm used to calculate the value.",
///                      "default": "sha256",
///                      "examples": [
///                        "blake2b512",
///                        "sha256",
///                        "sha3-512",
///                        "sha384",
///                        "sha512"
///                      ],
///                      "type": "string",
///                      "minLength": 1
///                    },
///                    "value": {
///                      "title": "Value of the cryptographic hash",
///                      "description": "Contains the cryptographic hash value in hexadecimal representation.",
///                      "examples": [
///                        "37df33cb7464da5c7f077f4d56a32bc84987ec1d85b234537c1c1a4d4fc8d09dc29e2e762cb5203677bf849a2855a0283710f1f5fe1d6ce8d5ac85c645d0fcb3",
///                        "4775203615d9534a8bfca96a93dc8b461a489f69124a130d786b42204f3341cc",
///                        "9ea4c8200113d49d26505da0e02e2f49055dc078d1ad7a419b32e291c7afebbb84badfbd46dec42883bea0b2a1fa697c"
///                      ],
///                      "type": "string",
///                      "minLength": 32,
///                      "pattern": "^[0-9a-fA-F]{32,}$"
///                    }
///                  }
///                },
///                "minItems": 1
///              },
///              "filename": {
///                "title": "Filename",
///                "description": "Contains the name of the file which is identified by the hash values.",
///                "examples": [
///                  "WINWORD.EXE",
///                  "msotadddin.dll",
///                  "sudoers.so"
///                ],
///                "type": "string",
///                "minLength": 1
///              }
///            }
///          },
///          "minItems": 1
///        },
///        "model_numbers": {
///          "title": "List of models",
///          "description": "Contains a list of full or abbreviated (partial) model numbers.",
///          "type": "array",
///          "items": {
///            "title": "Model number",
///            "description": "Contains a full or abbreviated (partial) model number of the component to identify.",
///            "type": "string",
///            "minLength": 1
///          },
///          "minItems": 1,
///          "uniqueItems": true
///        },
///        "purl": {
///          "title": "package URL representation",
///          "description": "The package URL (purl) attribute refers to a method for reliably identifying and locating software packages external to this specification.",
///          "type": "string",
///          "format": "uri",
///          "minLength": 7,
///          "pattern": "^pkg:[A-Za-z\\.\\-\\+][A-Za-z0-9\\.\\-\\+]*/.+"
///        },
///        "sbom_urls": {
///          "title": "List of SBOM URLs",
///          "description": "Contains a list of URLs where SBOMs for this product can be retrieved.",
///          "type": "array",
///          "items": {
///            "title": "SBOM URL",
///            "description": "Contains a URL of one SBOM for this product.",
///            "type": "string",
///            "format": "uri"
///          },
///          "minItems": 1
///        },
///        "serial_numbers": {
///          "title": "List of serial numbers",
///          "description": "Contains a list of full or abbreviated (partial) serial numbers.",
///          "type": "array",
///          "items": {
///            "title": "Serial number",
///            "description": "Contains a full or abbreviated (partial) serial number of the component to identify.",
///            "type": "string",
///            "minLength": 1
///          },
///          "minItems": 1,
///          "uniqueItems": true
///        },
///        "skus": {
///          "title": "List of stock keeping units",
///          "description": "Contains a list of full or abbreviated (partial) stock keeping units.",
///          "type": "array",
///          "items": {
///            "title": "Stock keeping unit",
///            "description": "Contains a full or abbreviated (partial) stock keeping unit (SKU) which is used in the ordering process to identify the component.",
///            "type": "string",
///            "minLength": 1
///          },
///          "minItems": 1
///        },
///        "x_generic_uris": {
///          "title": "List of generic URIs",
///          "description": "Contains a list of identifiers which are either vendor-specific or derived from a standard not yet supported.",
///          "type": "array",
///          "items": {
///            "title": "Generic URI",
///            "description": "Provides a generic extension point for any identifier which is either vendor-specific or derived from a standard not yet supported.",
///            "type": "object",
///            "required": [
///              "namespace",
///              "uri"
///            ],
///            "properties": {
///              "namespace": {
///                "title": "Namespace of the generic URI",
///                "description": "Refers to a URL which provides the name and knowledge about the specification used or is the namespace in which these values are valid.",
///                "type": "string",
///                "format": "uri"
///              },
///              "uri": {
///                "title": "URI",
///                "description": "Contains the identifier itself.",
///                "type": "string",
///                "format": "uri"
///              }
///            }
///          },
///          "minItems": 1
///        }
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FullProductNameT {
    ///The value should be the product’s full canonical name, including version number and other attributes, as it would be used in a human-friendly document.
    pub name: TextualDescriptionOfTheProduct,
    pub product_id: ProductIdT,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_identification_helper: Option<HelperToIdentifyTheProduct>,
}
impl From<&FullProductNameT> for FullProductNameT {
    fn from(value: &FullProductNameT) -> Self {
        value.clone()
    }
}
impl FullProductNameT {
    pub fn builder() -> builder::FullProductNameT {
        Default::default()
    }
}
///Provides a generic extension point for any identifier which is either vendor-specific or derived from a standard not yet supported.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Generic URI",
///  "description": "Provides a generic extension point for any identifier which is either vendor-specific or derived from a standard not yet supported.",
///  "type": "object",
///  "required": [
///    "namespace",
///    "uri"
///  ],
///  "properties": {
///    "namespace": {
///      "title": "Namespace of the generic URI",
///      "description": "Refers to a URL which provides the name and knowledge about the specification used or is the namespace in which these values are valid.",
///      "type": "string",
///      "format": "uri"
///    },
///    "uri": {
///      "title": "URI",
///      "description": "Contains the identifier itself.",
///      "type": "string",
///      "format": "uri"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GenericUri {
    ///Refers to a URL which provides the name and knowledge about the specification used or is the namespace in which these values are valid.
    pub namespace: String,
    ///Contains the identifier itself.
    pub uri: String,
}
impl From<&GenericUri> for GenericUri {
    fn from(value: &GenericUri) -> Self {
        value.clone()
    }
}
impl GenericUri {
    pub fn builder() -> builder::GenericUri {
        Default::default()
    }
}
///Provides at least one method which aids in identifying the product in an asset database.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Helper to identify the product",
///  "description": "Provides at least one method which aids in identifying the product in an asset database.",
///  "type": "object",
///  "minProperties": 1,
///  "properties": {
///    "cpe": {
///      "title": "Common Platform Enumeration representation",
///      "description": "The Common Platform Enumeration (CPE) attribute refers to a method for naming platforms external to this specification.",
///      "type": "string",
///      "minLength": 5,
///      "pattern": "^(cpe:2\\.3:[aho\\*\\-](:(((\\?*|\\*?)([a-zA-Z0-9\\-\\._]|(\\\\[\\\\\\*\\?!\"#\\$%&'\\(\\)\\+,/:;<=>@\\[\\]\\^`\\{\\|\\}~]))+(\\?*|\\*?))|[\\*\\-])){5}(:(([a-zA-Z]{2,3}(-([a-zA-Z]{2}|[0-9]{3}))?)|[\\*\\-]))(:(((\\?*|\\*?)([a-zA-Z0-9\\-\\._]|(\\\\[\\\\\\*\\?!\"#\\$%&'\\(\\)\\+,/:;<=>@\\[\\]\\^`\\{\\|\\}~]))+(\\?*|\\*?))|[\\*\\-])){4})|([c][pP][eE]:/[AHOaho]?(:[A-Za-z0-9\\._\\-~%]*){0,6})$"
///    },
///    "hashes": {
///      "title": "List of hashes",
///      "description": "Contains a list of cryptographic hashes usable to identify files.",
///      "type": "array",
///      "items": {
///        "title": "Cryptographic hashes",
///        "description": "Contains all information to identify a file based on its cryptographic hash values.",
///        "type": "object",
///        "required": [
///          "file_hashes",
///          "filename"
///        ],
///        "properties": {
///          "file_hashes": {
///            "title": "List of file hashes",
///            "description": "Contains a list of cryptographic hashes for this file.",
///            "type": "array",
///            "items": {
///              "title": "File hash",
///              "description": "Contains one hash value and algorithm of the file to be identified.",
///              "type": "object",
///              "required": [
///                "algorithm",
///                "value"
///              ],
///              "properties": {
///                "algorithm": {
///                  "title": "Algorithm of the cryptographic hash",
///                  "description": "Contains the name of the cryptographic hash algorithm used to calculate the value.",
///                  "default": "sha256",
///                  "examples": [
///                    "blake2b512",
///                    "sha256",
///                    "sha3-512",
///                    "sha384",
///                    "sha512"
///                  ],
///                  "type": "string",
///                  "minLength": 1
///                },
///                "value": {
///                  "title": "Value of the cryptographic hash",
///                  "description": "Contains the cryptographic hash value in hexadecimal representation.",
///                  "examples": [
///                    "37df33cb7464da5c7f077f4d56a32bc84987ec1d85b234537c1c1a4d4fc8d09dc29e2e762cb5203677bf849a2855a0283710f1f5fe1d6ce8d5ac85c645d0fcb3",
///                    "4775203615d9534a8bfca96a93dc8b461a489f69124a130d786b42204f3341cc",
///                    "9ea4c8200113d49d26505da0e02e2f49055dc078d1ad7a419b32e291c7afebbb84badfbd46dec42883bea0b2a1fa697c"
///                  ],
///                  "type": "string",
///                  "minLength": 32,
///                  "pattern": "^[0-9a-fA-F]{32,}$"
///                }
///              }
///            },
///            "minItems": 1
///          },
///          "filename": {
///            "title": "Filename",
///            "description": "Contains the name of the file which is identified by the hash values.",
///            "examples": [
///              "WINWORD.EXE",
///              "msotadddin.dll",
///              "sudoers.so"
///            ],
///            "type": "string",
///            "minLength": 1
///          }
///        }
///      },
///      "minItems": 1
///    },
///    "model_numbers": {
///      "title": "List of models",
///      "description": "Contains a list of full or abbreviated (partial) model numbers.",
///      "type": "array",
///      "items": {
///        "title": "Model number",
///        "description": "Contains a full or abbreviated (partial) model number of the component to identify.",
///        "type": "string",
///        "minLength": 1
///      },
///      "minItems": 1,
///      "uniqueItems": true
///    },
///    "purl": {
///      "title": "package URL representation",
///      "description": "The package URL (purl) attribute refers to a method for reliably identifying and locating software packages external to this specification.",
///      "type": "string",
///      "format": "uri",
///      "minLength": 7,
///      "pattern": "^pkg:[A-Za-z\\.\\-\\+][A-Za-z0-9\\.\\-\\+]*/.+"
///    },
///    "sbom_urls": {
///      "title": "List of SBOM URLs",
///      "description": "Contains a list of URLs where SBOMs for this product can be retrieved.",
///      "type": "array",
///      "items": {
///        "title": "SBOM URL",
///        "description": "Contains a URL of one SBOM for this product.",
///        "type": "string",
///        "format": "uri"
///      },
///      "minItems": 1
///    },
///    "serial_numbers": {
///      "title": "List of serial numbers",
///      "description": "Contains a list of full or abbreviated (partial) serial numbers.",
///      "type": "array",
///      "items": {
///        "title": "Serial number",
///        "description": "Contains a full or abbreviated (partial) serial number of the component to identify.",
///        "type": "string",
///        "minLength": 1
///      },
///      "minItems": 1,
///      "uniqueItems": true
///    },
///    "skus": {
///      "title": "List of stock keeping units",
///      "description": "Contains a list of full or abbreviated (partial) stock keeping units.",
///      "type": "array",
///      "items": {
///        "title": "Stock keeping unit",
///        "description": "Contains a full or abbreviated (partial) stock keeping unit (SKU) which is used in the ordering process to identify the component.",
///        "type": "string",
///        "minLength": 1
///      },
///      "minItems": 1
///    },
///    "x_generic_uris": {
///      "title": "List of generic URIs",
///      "description": "Contains a list of identifiers which are either vendor-specific or derived from a standard not yet supported.",
///      "type": "array",
///      "items": {
///        "title": "Generic URI",
///        "description": "Provides a generic extension point for any identifier which is either vendor-specific or derived from a standard not yet supported.",
///        "type": "object",
///        "required": [
///          "namespace",
///          "uri"
///        ],
///        "properties": {
///          "namespace": {
///            "title": "Namespace of the generic URI",
///            "description": "Refers to a URL which provides the name and knowledge about the specification used or is the namespace in which these values are valid.",
///            "type": "string",
///            "format": "uri"
///          },
///          "uri": {
///            "title": "URI",
///            "description": "Contains the identifier itself.",
///            "type": "string",
///            "format": "uri"
///          }
///        }
///      },
///      "minItems": 1
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct HelperToIdentifyTheProduct {
    ///The Common Platform Enumeration (CPE) attribute refers to a method for naming platforms external to this specification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpe: Option<CommonPlatformEnumerationRepresentation>,
    ///Contains a list of cryptographic hashes usable to identify files.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hashes: Vec<CryptographicHashes>,
    ///Contains a list of full or abbreviated (partial) model numbers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_numbers: Option<Vec<ModelNumber>>,
    ///The package URL (purl) attribute refers to a method for reliably identifying and locating software packages external to this specification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purl: Option<String>,
    ///Contains a list of URLs where SBOMs for this product can be retrieved.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sbom_urls: Vec<String>,
    ///Contains a list of full or abbreviated (partial) serial numbers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serial_numbers: Option<Vec<SerialNumber>>,
    ///Contains a list of full or abbreviated (partial) stock keeping units.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skus: Vec<StockKeepingUnit>,
    ///Contains a list of identifiers which are either vendor-specific or derived from a standard not yet supported.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub x_generic_uris: Vec<GenericUri>,
}
impl From<&HelperToIdentifyTheProduct> for HelperToIdentifyTheProduct {
    fn from(value: &HelperToIdentifyTheProduct) -> Self {
        value.clone()
    }
}
impl HelperToIdentifyTheProduct {
    pub fn builder() -> builder::HelperToIdentifyTheProduct {
        Default::default()
    }
}
///Contains a single unique label or tracking ID for the vulnerability.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "ID",
///  "description": "Contains a single unique label or tracking ID for the vulnerability.",
///  "type": "object",
///  "required": [
///    "system_name",
///    "text"
///  ],
///  "properties": {
///    "system_name": {
///      "title": "System name",
///      "description": "Indicates the name of the vulnerability tracking or numbering system.",
///      "examples": [
///        "Cisco Bug ID",
///        "GitHub Issue"
///      ],
///      "type": "string",
///      "minLength": 1
///    },
///    "text": {
///      "title": "Text",
///      "description": "Is unique label or tracking ID for the vulnerability (if such information exists).",
///      "examples": [
///        "CSCso66472",
///        "oasis-tcs/csaf#210"
///      ],
///      "type": "string",
///      "minLength": 1
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Id {
    ///Indicates the name of the vulnerability tracking or numbering system.
    pub system_name: SystemName,
    ///Is unique label or tracking ID for the vulnerability (if such information exists).
    pub text: Text,
}
impl From<&Id> for Id {
    fn from(value: &Id) -> Self {
        value.clone()
    }
}
impl Id {
    pub fn builder() -> builder::Id {
        Default::default()
    }
}
///Is a container, that allows the document producers to comment on the level of involvement (or engagement) of themselves or third parties in the vulnerability identification, scoping, and remediation process.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Involvement",
///  "description": "Is a container, that allows the document producers to comment on the level of involvement (or engagement) of themselves or third parties in the vulnerability identification, scoping, and remediation process.",
///  "type": "object",
///  "required": [
///    "party",
///    "status"
///  ],
///  "properties": {
///    "date": {
///      "title": "Date of involvement",
///      "description": "Holds the date and time of the involvement entry.",
///      "type": "string",
///      "format": "date-time"
///    },
///    "party": {
///      "title": "Party category",
///      "description": "Defines the category of the involved party.",
///      "type": "string",
///      "enum": [
///        "coordinator",
///        "discoverer",
///        "other",
///        "user",
///        "vendor"
///      ]
///    },
///    "status": {
///      "title": "Party status",
///      "description": "Defines contact status of the involved party.",
///      "type": "string",
///      "enum": [
///        "completed",
///        "contact_attempted",
///        "disputed",
///        "in_progress",
///        "not_contacted",
///        "open"
///      ]
///    },
///    "summary": {
///      "title": "Summary of the involvement",
///      "description": "Contains additional context regarding what is going on.",
///      "type": "string",
///      "minLength": 1
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Involvement {
    ///Holds the date and time of the involvement entry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<chrono::DateTime<chrono::offset::Utc>>,
    ///Defines the category of the involved party.
    pub party: PartyCategory,
    ///Defines contact status of the involved party.
    pub status: PartyStatus,
    ///Contains additional context regarding what is going on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<SummaryOfTheInvolvement>,
}
impl From<&Involvement> for Involvement {
    fn from(value: &Involvement) -> Self {
        value.clone()
    }
}
impl Involvement {
    pub fn builder() -> builder::Involvement {
        Default::default()
    }
}
///Provides information about the authority of the issuing party to release the document, in particular, the party's constituency and responsibilities or other obligations.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Issuing authority",
///  "description": "Provides information about the authority of the issuing party to release the document, in particular, the party's constituency and responsibilities or other obligations.",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IssuingAuthority(String);
impl ::std::ops::Deref for IssuingAuthority {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IssuingAuthority> for String {
    fn from(value: IssuingAuthority) -> Self {
        value.0
    }
}
impl From<&IssuingAuthority> for IssuingAuthority {
    fn from(value: &IssuingAuthority) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for IssuingAuthority {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for IssuingAuthority {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for IssuingAuthority {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for IssuingAuthority {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for IssuingAuthority {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Specifies the machine readable label.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Label of the flag",
///  "description": "Specifies the machine readable label.",
///  "type": "string",
///  "enum": [
///    "component_not_present",
///    "inline_mitigations_already_exist",
///    "vulnerable_code_cannot_be_controlled_by_adversary",
///    "vulnerable_code_not_in_execute_path",
///    "vulnerable_code_not_present"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LabelOfTheFlag {
    #[serde(rename = "component_not_present")]
    ComponentNotPresent,
    #[serde(rename = "inline_mitigations_already_exist")]
    InlineMitigationsAlreadyExist,
    #[serde(rename = "vulnerable_code_cannot_be_controlled_by_adversary")]
    VulnerableCodeCannotBeControlledByAdversary,
    #[serde(rename = "vulnerable_code_not_in_execute_path")]
    VulnerableCodeNotInExecutePath,
    #[serde(rename = "vulnerable_code_not_present")]
    VulnerableCodeNotPresent,
}
impl From<&LabelOfTheFlag> for LabelOfTheFlag {
    fn from(value: &LabelOfTheFlag) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for LabelOfTheFlag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ComponentNotPresent => write!(f, "component_not_present"),
            Self::InlineMitigationsAlreadyExist => {
                write!(f, "inline_mitigations_already_exist")
            }
            Self::VulnerableCodeCannotBeControlledByAdversary => {
                write!(f, "vulnerable_code_cannot_be_controlled_by_adversary")
            }
            Self::VulnerableCodeNotInExecutePath => {
                write!(f, "vulnerable_code_not_in_execute_path")
            }
            Self::VulnerableCodeNotPresent => write!(f, "vulnerable_code_not_present"),
        }
    }
}
impl std::str::FromStr for LabelOfTheFlag {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "component_not_present" => Ok(Self::ComponentNotPresent),
            "inline_mitigations_already_exist" => Ok(Self::InlineMitigationsAlreadyExist),
            "vulnerable_code_cannot_be_controlled_by_adversary" => {
                Ok(Self::VulnerableCodeCannotBeControlledByAdversary)
            }
            "vulnerable_code_not_in_execute_path" => Ok(Self::VulnerableCodeNotInExecutePath),
            "vulnerable_code_not_present" => Ok(Self::VulnerableCodeNotPresent),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for LabelOfTheFlag {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LabelOfTheFlag {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LabelOfTheFlag {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Provides the TLP label of the document.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Label of TLP",
///  "description": "Provides the TLP label of the document.",
///  "type": "string",
///  "enum": [
///    "AMBER",
///    "GREEN",
///    "RED",
///    "WHITE"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LabelOfTlp {
    #[serde(rename = "AMBER")]
    Amber,
    #[serde(rename = "GREEN")]
    Green,
    #[serde(rename = "RED")]
    Red,
    #[serde(rename = "WHITE")]
    White,
}
impl From<&LabelOfTlp> for LabelOfTlp {
    fn from(value: &LabelOfTlp) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for LabelOfTlp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Amber => write!(f, "AMBER"),
            Self::Green => write!(f, "GREEN"),
            Self::Red => write!(f, "RED"),
            Self::White => write!(f, "WHITE"),
        }
    }
}
impl std::str::FromStr for LabelOfTlp {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "AMBER" => Ok(Self::Amber),
            "GREEN" => Ok(Self::Green),
            "RED" => Ok(Self::Red),
            "WHITE" => Ok(Self::White),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for LabelOfTlp {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LabelOfTlp {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LabelOfTlp {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Identifies a language, corresponding to IETF BCP 47 / RFC 5646. See IETF language registry: https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Language type",
///  "description": "Identifies a language, corresponding to IETF BCP 47 / RFC 5646. See IETF language registry: https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry",
///  "examples": [
///    "de",
///    "en",
///    "fr",
///    "frc",
///    "jp"
///  ],
///  "type": "string",
///  "pattern": "^(([A-Za-z]{2,3}(-[A-Za-z]{3}(-[A-Za-z]{3}){0,2})?|[A-Za-z]{4,8})(-[A-Za-z]{4})?(-([A-Za-z]{2}|[0-9]{3}))?(-([A-Za-z0-9]{5,8}|[0-9][A-Za-z0-9]{3}))*(-[A-WY-Za-wy-z0-9](-[A-Za-z0-9]{2,8})+)*(-[Xx](-[A-Za-z0-9]{1,8})+)?|[Xx](-[A-Za-z0-9]{1,8})+|[Ii]-[Dd][Ee][Ff][Aa][Uu][Ll][Tt]|[Ii]-[Mm][Ii][Nn][Gg][Oo])$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LangT(String);
impl ::std::ops::Deref for LangT {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<LangT> for String {
    fn from(value: LangT) -> Self {
        value.0
    }
}
impl From<&LangT> for LangT {
    fn from(value: &LangT) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for LangT {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new(
                "^(([A-Za-z]{2,3}(-[A-Za-z]{3}(-[A-Za-z]{3}){0,2})?|[A-Za-z]{4,8})(-[A-Za-z]{4})?(-([A-Za-z]{2}|[0-9]{3}))?(-([A-Za-z0-9]{5,8}|[0-9][A-Za-z0-9]{3}))*(-[A-WY-Za-wy-z0-9](-[A-Za-z0-9]{2,8})+)*(-[Xx](-[A-Za-z0-9]{1,8})+)?|[Xx](-[A-Za-z0-9]{1,8})+|[Ii]-[Dd][Ee][Ff][Aa][Uu][Ll][Tt]|[Ii]-[Mm][Ii][Nn][Gg][Oo])$",
            )
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err(
                "doesn't match pattern \"^(([A-Za-z]{2,3}(-[A-Za-z]{3}(-[A-Za-z]{3}){0,2})?|[A-Za-z]{4,8})(-[A-Za-z]{4})?(-([A-Za-z]{2}|[0-9]{3}))?(-([A-Za-z0-9]{5,8}|[0-9][A-Za-z0-9]{3}))*(-[A-WY-Za-wy-z0-9](-[A-Za-z0-9]{2,8})+)*(-[Xx](-[A-Za-z0-9]{1,8})+)?|[Xx](-[A-Za-z0-9]{1,8})+|[Ii]-[Dd][Ee][Ff][Aa][Uu][Ll][Tt]|[Ii]-[Mm][Ii][Nn][Gg][Oo])$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LangT {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for LangT {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for LangT {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LangT {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Contains the version string used in an existing document with the same content.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Legacy version of the revision",
///  "description": "Contains the version string used in an existing document with the same content.",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LegacyVersionOfTheRevision(String);
impl ::std::ops::Deref for LegacyVersionOfTheRevision {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<LegacyVersionOfTheRevision> for String {
    fn from(value: LegacyVersionOfTheRevision) -> Self {
        value.0
    }
}
impl From<&LegacyVersionOfTheRevision> for LegacyVersionOfTheRevision {
    fn from(value: &LegacyVersionOfTheRevision) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for LegacyVersionOfTheRevision {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LegacyVersionOfTheRevision {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for LegacyVersionOfTheRevision {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for LegacyVersionOfTheRevision {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LegacyVersionOfTheRevision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Contains a full or abbreviated (partial) model number of the component to identify.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Model number",
///  "description": "Contains a full or abbreviated (partial) model number of the component to identify.",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModelNumber(String);
impl ::std::ops::Deref for ModelNumber {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<ModelNumber> for String {
    fn from(value: ModelNumber) -> Self {
        value.0
    }
}
impl From<&ModelNumber> for ModelNumber {
    fn from(value: &ModelNumber) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ModelNumber {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ModelNumber {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for ModelNumber {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for ModelNumber {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ModelNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Contains the name of the issuing party.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Name of publisher",
///  "description": "Contains the name of the issuing party.",
///  "examples": [
///    "BSI",
///    "Cisco PSIRT",
///    "Siemens ProductCERT"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NameOfPublisher(String);
impl ::std::ops::Deref for NameOfPublisher {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<NameOfPublisher> for String {
    fn from(value: NameOfPublisher) -> Self {
        value.0
    }
}
impl From<&NameOfPublisher> for NameOfPublisher {
    fn from(value: &NameOfPublisher) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for NameOfPublisher {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for NameOfPublisher {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for NameOfPublisher {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for NameOfPublisher {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for NameOfPublisher {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Contains the canonical descriptor or 'friendly name' of the branch.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Name of the branch",
///  "description": "Contains the canonical descriptor or 'friendly name' of the branch.",
///  "examples": [
///    "10",
///    "365",
///    "Microsoft",
///    "Office",
///    "PCS 7",
///    "SIMATIC",
///    "Siemens",
///    "Windows"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NameOfTheBranch(String);
impl ::std::ops::Deref for NameOfTheBranch {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<NameOfTheBranch> for String {
    fn from(value: NameOfTheBranch) -> Self {
        value.0
    }
}
impl From<&NameOfTheBranch> for NameOfTheBranch {
    fn from(value: &NameOfTheBranch) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for NameOfTheBranch {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for NameOfTheBranch {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for NameOfTheBranch {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for NameOfTheBranch {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for NameOfTheBranch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Contains the name of a single contributor being recognized.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Name of the contributor",
///  "description": "Contains the name of a single contributor being recognized.",
///  "examples": [
///    "Albert Einstein",
///    "Johann Sebastian Bach"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NameOfTheContributor(String);
impl ::std::ops::Deref for NameOfTheContributor {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<NameOfTheContributor> for String {
    fn from(value: NameOfTheContributor) -> Self {
        value.0
    }
}
impl From<&NameOfTheContributor> for NameOfTheContributor {
    fn from(value: &NameOfTheContributor) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for NameOfTheContributor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for NameOfTheContributor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for NameOfTheContributor {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for NameOfTheContributor {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for NameOfTheContributor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Is a place to put all manner of text blobs related to the current context.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Note",
///  "description": "Is a place to put all manner of text blobs related to the current context.",
///  "type": "object",
///  "required": [
///    "category",
///    "text"
///  ],
///  "properties": {
///    "audience": {
///      "title": "Audience of note",
///      "description": "Indicates who is intended to read it.",
///      "examples": [
///        "all",
///        "executives",
///        "operational management and system administrators",
///        "safety engineers"
///      ],
///      "type": "string",
///      "minLength": 1
///    },
///    "category": {
///      "title": "Note category",
///      "description": "Contains the information of what kind of note this is.",
///      "type": "string",
///      "enum": [
///        "description",
///        "details",
///        "faq",
///        "general",
///        "legal_disclaimer",
///        "other",
///        "summary"
///      ]
///    },
///    "text": {
///      "title": "Note content",
///      "description": "Holds the content of the note. Content varies depending on type.",
///      "type": "string",
///      "minLength": 1
///    },
///    "title": {
///      "title": "Title of note",
///      "description": "Provides a concise description of what is contained in the text of the note.",
///      "examples": [
///        "Details",
///        "Executive summary",
///        "Technical summary",
///        "Impact on safety systems"
///      ],
///      "type": "string",
///      "minLength": 1
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Note {
    ///Indicates who is intended to read it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<AudienceOfNote>,
    ///Contains the information of what kind of note this is.
    pub category: NoteCategory,
    ///Holds the content of the note. Content varies depending on type.
    pub text: NoteContent,
    ///Provides a concise description of what is contained in the text of the note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<TitleOfNote>,
}
impl From<&Note> for Note {
    fn from(value: &Note) -> Self {
        value.clone()
    }
}
impl Note {
    pub fn builder() -> builder::Note {
        Default::default()
    }
}
///Contains the information of what kind of note this is.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Note category",
///  "description": "Contains the information of what kind of note this is.",
///  "type": "string",
///  "enum": [
///    "description",
///    "details",
///    "faq",
///    "general",
///    "legal_disclaimer",
///    "other",
///    "summary"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum NoteCategory {
    #[serde(rename = "description")]
    Description,
    #[serde(rename = "details")]
    Details,
    #[serde(rename = "faq")]
    Faq,
    #[serde(rename = "general")]
    General,
    #[serde(rename = "legal_disclaimer")]
    LegalDisclaimer,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "summary")]
    Summary,
}
impl From<&NoteCategory> for NoteCategory {
    fn from(value: &NoteCategory) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for NoteCategory {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Description => write!(f, "description"),
            Self::Details => write!(f, "details"),
            Self::Faq => write!(f, "faq"),
            Self::General => write!(f, "general"),
            Self::LegalDisclaimer => write!(f, "legal_disclaimer"),
            Self::Other => write!(f, "other"),
            Self::Summary => write!(f, "summary"),
        }
    }
}
impl std::str::FromStr for NoteCategory {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "description" => Ok(Self::Description),
            "details" => Ok(Self::Details),
            "faq" => Ok(Self::Faq),
            "general" => Ok(Self::General),
            "legal_disclaimer" => Ok(Self::LegalDisclaimer),
            "other" => Ok(Self::Other),
            "summary" => Ok(Self::Summary),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for NoteCategory {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for NoteCategory {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for NoteCategory {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Holds the content of the note. Content varies depending on type.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Note content",
///  "description": "Holds the content of the note. Content varies depending on type.",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NoteContent(String);
impl ::std::ops::Deref for NoteContent {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<NoteContent> for String {
    fn from(value: NoteContent) -> Self {
        value.0
    }
}
impl From<&NoteContent> for NoteContent {
    fn from(value: &NoteContent) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for NoteContent {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for NoteContent {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for NoteContent {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for NoteContent {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for NoteContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Contains notes which are specific to the current context.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "List of notes",
///  "description": "Contains notes which are specific to the current context.",
///  "type": "array",
///  "items": {
///    "title": "Note",
///    "description": "Is a place to put all manner of text blobs related to the current context.",
///    "type": "object",
///    "required": [
///      "category",
///      "text"
///    ],
///    "properties": {
///      "audience": {
///        "title": "Audience of note",
///        "description": "Indicates who is intended to read it.",
///        "examples": [
///          "all",
///          "executives",
///          "operational management and system administrators",
///          "safety engineers"
///        ],
///        "type": "string",
///        "minLength": 1
///      },
///      "category": {
///        "title": "Note category",
///        "description": "Contains the information of what kind of note this is.",
///        "type": "string",
///        "enum": [
///          "description",
///          "details",
///          "faq",
///          "general",
///          "legal_disclaimer",
///          "other",
///          "summary"
///        ]
///      },
///      "text": {
///        "title": "Note content",
///        "description": "Holds the content of the note. Content varies depending on type.",
///        "type": "string",
///        "minLength": 1
///      },
///      "title": {
///        "title": "Title of note",
///        "description": "Provides a concise description of what is contained in the text of the note.",
///        "examples": [
///          "Details",
///          "Executive summary",
///          "Technical summary",
///          "Impact on safety systems"
///        ],
///        "type": "string",
///        "minLength": 1
///      }
///    }
///  },
///  "minItems": 1
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct NotesT(pub Vec<Note>);
impl ::std::ops::Deref for NotesT {
    type Target = Vec<Note>;
    fn deref(&self) -> &Vec<Note> {
        &self.0
    }
}
impl From<NotesT> for Vec<Note> {
    fn from(value: NotesT) -> Self {
        value.0
    }
}
impl From<&NotesT> for NotesT {
    fn from(value: &NotesT) -> Self {
        value.clone()
    }
}
impl From<Vec<Note>> for NotesT {
    fn from(value: Vec<Note>) -> Self {
        Self(value)
    }
}
///Defines the category of the involved party.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Party category",
///  "description": "Defines the category of the involved party.",
///  "type": "string",
///  "enum": [
///    "coordinator",
///    "discoverer",
///    "other",
///    "user",
///    "vendor"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PartyCategory {
    #[serde(rename = "coordinator")]
    Coordinator,
    #[serde(rename = "discoverer")]
    Discoverer,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "vendor")]
    Vendor,
}
impl From<&PartyCategory> for PartyCategory {
    fn from(value: &PartyCategory) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for PartyCategory {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Coordinator => write!(f, "coordinator"),
            Self::Discoverer => write!(f, "discoverer"),
            Self::Other => write!(f, "other"),
            Self::User => write!(f, "user"),
            Self::Vendor => write!(f, "vendor"),
        }
    }
}
impl std::str::FromStr for PartyCategory {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "coordinator" => Ok(Self::Coordinator),
            "discoverer" => Ok(Self::Discoverer),
            "other" => Ok(Self::Other),
            "user" => Ok(Self::User),
            "vendor" => Ok(Self::Vendor),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for PartyCategory {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PartyCategory {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PartyCategory {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Defines contact status of the involved party.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Party status",
///  "description": "Defines contact status of the involved party.",
///  "type": "string",
///  "enum": [
///    "completed",
///    "contact_attempted",
///    "disputed",
///    "in_progress",
///    "not_contacted",
///    "open"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PartyStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "contact_attempted")]
    ContactAttempted,
    #[serde(rename = "disputed")]
    Disputed,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "not_contacted")]
    NotContacted,
    #[serde(rename = "open")]
    Open,
}
impl From<&PartyStatus> for PartyStatus {
    fn from(value: &PartyStatus) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for PartyStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => write!(f, "completed"),
            Self::ContactAttempted => write!(f, "contact_attempted"),
            Self::Disputed => write!(f, "disputed"),
            Self::InProgress => write!(f, "in_progress"),
            Self::NotContacted => write!(f, "not_contacted"),
            Self::Open => write!(f, "open"),
        }
    }
}
impl std::str::FromStr for PartyStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "contact_attempted" => Ok(Self::ContactAttempted),
            "disputed" => Ok(Self::Disputed),
            "in_progress" => Ok(Self::InProgress),
            "not_contacted" => Ok(Self::NotContacted),
            "open" => Ok(Self::Open),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for PartyStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PartyStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PartyStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Defines a new logical group of products that can then be referred to in other parts of the document to address a group of products with a single identifier.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Product group",
///  "description": "Defines a new logical group of products that can then be referred to in other parts of the document to address a group of products with a single identifier.",
///  "type": "object",
///  "required": [
///    "group_id",
///    "product_ids"
///  ],
///  "properties": {
///    "group_id": {
///      "$ref": "#/$defs/product_group_id_t"
///    },
///    "product_ids": {
///      "title": "List of Product IDs",
///      "description": "Lists the product_ids of those products which known as one group in the document.",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/product_id_t"
///      },
///      "minItems": 2,
///      "uniqueItems": true
///    },
///    "summary": {
///      "title": "Summary of the product group",
///      "description": "Gives a short, optional description of the group.",
///      "examples": [
///        "Products supporting Modbus.",
///        "The x64 versions of the operating system."
///      ],
///      "type": "string",
///      "minLength": 1
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ProductGroup {
    pub group_id: ProductGroupIdT,
    ///Lists the product_ids of those products which known as one group in the document.
    pub product_ids: Vec<ProductIdT>,
    ///Gives a short, optional description of the group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<SummaryOfTheProductGroup>,
}
impl From<&ProductGroup> for ProductGroup {
    fn from(value: &ProductGroup) -> Self {
        value.clone()
    }
}
impl ProductGroup {
    pub fn builder() -> builder::ProductGroup {
        Default::default()
    }
}
///Token required to identify a group of products so that it can be referred to from other parts in the document. There is no predefined or required format for the product_group_id as long as it uniquely identifies a group in the context of the current document.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Reference token for product group instance",
///  "description": "Token required to identify a group of products so that it can be referred to from other parts in the document. There is no predefined or required format for the product_group_id as long as it uniquely identifies a group in the context of the current document.",
///  "examples": [
///    "CSAFGID-0001",
///    "CSAFGID-0002",
///    "CSAFGID-0020"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProductGroupIdT(String);
impl ::std::ops::Deref for ProductGroupIdT {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<ProductGroupIdT> for String {
    fn from(value: ProductGroupIdT) -> Self {
        value.0
    }
}
impl From<&ProductGroupIdT> for ProductGroupIdT {
    fn from(value: &ProductGroupIdT) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ProductGroupIdT {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ProductGroupIdT {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for ProductGroupIdT {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for ProductGroupIdT {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ProductGroupIdT {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Specifies a list of product_group_ids to give context to the parent item.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "List of product_group_ids",
///  "description": "Specifies a list of product_group_ids to give context to the parent item.",
///  "type": "array",
///  "items": {
///    "$ref": "#/$defs/product_group_id_t"
///  },
///  "minItems": 1,
///  "uniqueItems": true
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ProductGroupsT(pub Vec<ProductGroupIdT>);
impl ::std::ops::Deref for ProductGroupsT {
    type Target = Vec<ProductGroupIdT>;
    fn deref(&self) -> &Vec<ProductGroupIdT> {
        &self.0
    }
}
impl From<ProductGroupsT> for Vec<ProductGroupIdT> {
    fn from(value: ProductGroupsT) -> Self {
        value.0
    }
}
impl From<&ProductGroupsT> for ProductGroupsT {
    fn from(value: &ProductGroupsT) -> Self {
        value.clone()
    }
}
impl From<Vec<ProductGroupIdT>> for ProductGroupsT {
    fn from(value: Vec<ProductGroupIdT>) -> Self {
        Self(value)
    }
}
///Token required to identify a full_product_name so that it can be referred to from other parts in the document. There is no predefined or required format for the product_id as long as it uniquely identifies a product in the context of the current document.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Reference token for product instance",
///  "description": "Token required to identify a full_product_name so that it can be referred to from other parts in the document. There is no predefined or required format for the product_id as long as it uniquely identifies a product in the context of the current document.",
///  "examples": [
///    "CSAFPID-0004",
///    "CSAFPID-0008"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProductIdT(String);
impl ::std::ops::Deref for ProductIdT {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<ProductIdT> for String {
    fn from(value: ProductIdT) -> Self {
        value.0
    }
}
impl From<&ProductIdT> for ProductIdT {
    fn from(value: &ProductIdT) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ProductIdT {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ProductIdT {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for ProductIdT {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for ProductIdT {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ProductIdT {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Contains different lists of product_ids which provide details on the status of the referenced product related to the current vulnerability.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Product status",
///  "description": "Contains different lists of product_ids which provide details on the status of the referenced product related to the current vulnerability. ",
///  "type": "object",
///  "minProperties": 1,
///  "properties": {
///    "first_affected": {
///      "title": "First affected",
///      "description": "These are the first versions of the releases known to be affected by the vulnerability.",
///      "$ref": "#/$defs/products_t"
///    },
///    "first_fixed": {
///      "title": "First fixed",
///      "description": "These versions contain the first fix for the vulnerability but may not be the recommended fixed versions.",
///      "$ref": "#/$defs/products_t"
///    },
///    "fixed": {
///      "title": "Fixed",
///      "description": "These versions contain a fix for the vulnerability but may not be the recommended fixed versions.",
///      "$ref": "#/$defs/products_t"
///    },
///    "known_affected": {
///      "title": "Known affected",
///      "description": "These versions are known to be affected by the vulnerability.",
///      "$ref": "#/$defs/products_t"
///    },
///    "known_not_affected": {
///      "title": "Known not affected",
///      "description": "These versions are known not to be affected by the vulnerability.",
///      "$ref": "#/$defs/products_t"
///    },
///    "last_affected": {
///      "title": "Last affected",
///      "description": "These are the last versions in a release train known to be affected by the vulnerability. Subsequently released versions would contain a fix for the vulnerability.",
///      "$ref": "#/$defs/products_t"
///    },
///    "recommended": {
///      "title": "Recommended",
///      "description": "These versions have a fix for the vulnerability and are the vendor-recommended versions for fixing the vulnerability.",
///      "$ref": "#/$defs/products_t"
///    },
///    "under_investigation": {
///      "title": "Under investigation",
///      "description": "It is not known yet whether these versions are or are not affected by the vulnerability. However, it is still under investigation - the result will be provided in a later release of the document.",
///      "$ref": "#/$defs/products_t"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ProductStatus {
    ///These are the first versions of the releases known to be affected by the vulnerability.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_affected: Option<ProductsT>,
    ///These versions contain the first fix for the vulnerability but may not be the recommended fixed versions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_fixed: Option<ProductsT>,
    ///These versions contain a fix for the vulnerability but may not be the recommended fixed versions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed: Option<ProductsT>,
    ///These versions are known to be affected by the vulnerability.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub known_affected: Option<ProductsT>,
    ///These versions are known not to be affected by the vulnerability.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub known_not_affected: Option<ProductsT>,
    ///These are the last versions in a release train known to be affected by the vulnerability. Subsequently released versions would contain a fix for the vulnerability.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_affected: Option<ProductsT>,
    ///These versions have a fix for the vulnerability and are the vendor-recommended versions for fixing the vulnerability.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommended: Option<ProductsT>,
    ///It is not known yet whether these versions are or are not affected by the vulnerability. However, it is still under investigation - the result will be provided in a later release of the document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub under_investigation: Option<ProductsT>,
}
impl From<&ProductStatus> for ProductStatus {
    fn from(value: &ProductStatus) -> Self {
        value.clone()
    }
}
impl ProductStatus {
    pub fn builder() -> builder::ProductStatus {
        Default::default()
    }
}
///Is a container for all fully qualified product names that can be referenced elsewhere in the document.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Product tree",
///  "description": "Is a container for all fully qualified product names that can be referenced elsewhere in the document.",
///  "type": "object",
///  "minProperties": 1,
///  "properties": {
///    "branches": {
///      "$ref": "#/$defs/branches_t"
///    },
///    "full_product_names": {
///      "title": "List of full product names",
///      "description": "Contains a list of full product names.",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/full_product_name_t"
///      },
///      "minItems": 1
///    },
///    "product_groups": {
///      "title": "List of product groups",
///      "description": "Contains a list of product groups.",
///      "type": "array",
///      "items": {
///        "title": "Product group",
///        "description": "Defines a new logical group of products that can then be referred to in other parts of the document to address a group of products with a single identifier.",
///        "type": "object",
///        "required": [
///          "group_id",
///          "product_ids"
///        ],
///        "properties": {
///          "group_id": {
///            "$ref": "#/$defs/product_group_id_t"
///          },
///          "product_ids": {
///            "title": "List of Product IDs",
///            "description": "Lists the product_ids of those products which known as one group in the document.",
///            "type": "array",
///            "items": {
///              "$ref": "#/$defs/product_id_t"
///            },
///            "minItems": 2,
///            "uniqueItems": true
///          },
///          "summary": {
///            "title": "Summary of the product group",
///            "description": "Gives a short, optional description of the group.",
///            "examples": [
///              "Products supporting Modbus.",
///              "The x64 versions of the operating system."
///            ],
///            "type": "string",
///            "minLength": 1
///          }
///        }
///      },
///      "minItems": 1
///    },
///    "relationships": {
///      "title": "List of relationships",
///      "description": "Contains a list of relationships.",
///      "type": "array",
///      "items": {
///        "title": "Relationship",
///        "description": "Establishes a link between two existing full_product_name_t elements, allowing the document producer to define a combination of two products that form a new full_product_name entry.",
///        "type": "object",
///        "required": [
///          "category",
///          "full_product_name",
///          "product_reference",
///          "relates_to_product_reference"
///        ],
///        "properties": {
///          "category": {
///            "title": "Relationship category",
///            "description": "Defines the category of relationship for the referenced component.",
///            "type": "string",
///            "enum": [
///              "default_component_of",
///              "external_component_of",
///              "installed_on",
///              "installed_with",
///              "optional_component_of"
///            ]
///          },
///          "full_product_name": {
///            "$ref": "#/$defs/full_product_name_t"
///          },
///          "product_reference": {
///            "title": "Product reference",
///            "description": "Holds a Product ID that refers to the Full Product Name element, which is referenced as the first element of the relationship.",
///            "$ref": "#/$defs/product_id_t"
///          },
///          "relates_to_product_reference": {
///            "title": "Relates to product reference",
///            "description": "Holds a Product ID that refers to the Full Product Name element, which is referenced as the second element of the relationship.",
///            "$ref": "#/$defs/product_id_t"
///          }
///        }
///      },
///      "minItems": 1
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ProductTree {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branches: Option<BranchesT>,
    ///Contains a list of full product names.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub full_product_names: Vec<FullProductNameT>,
    ///Contains a list of product groups.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub product_groups: Vec<ProductGroup>,
    ///Contains a list of relationships.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relationships: Vec<Relationship>,
}
impl From<&ProductTree> for ProductTree {
    fn from(value: &ProductTree) -> Self {
        value.clone()
    }
}
impl ProductTree {
    pub fn builder() -> builder::ProductTree {
        Default::default()
    }
}
///Specifies a list of product_ids to give context to the parent item.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "List of product_ids",
///  "description": "Specifies a list of product_ids to give context to the parent item.",
///  "type": "array",
///  "items": {
///    "$ref": "#/$defs/product_id_t"
///  },
///  "minItems": 1,
///  "uniqueItems": true
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ProductsT(pub Vec<ProductIdT>);
impl ::std::ops::Deref for ProductsT {
    type Target = Vec<ProductIdT>;
    fn deref(&self) -> &Vec<ProductIdT> {
        &self.0
    }
}
impl From<ProductsT> for Vec<ProductIdT> {
    fn from(value: ProductsT) -> Self {
        value.0
    }
}
impl From<&ProductsT> for ProductsT {
    fn from(value: &ProductsT) -> Self {
        value.clone()
    }
}
impl From<Vec<ProductIdT>> for ProductsT {
    fn from(value: Vec<ProductIdT>) -> Self {
        Self(value)
    }
}
///Provides information about the publisher of the document.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Publisher",
///  "description": "Provides information about the publisher of the document.",
///  "type": "object",
///  "required": [
///    "category",
///    "name",
///    "namespace"
///  ],
///  "properties": {
///    "category": {
///      "title": "Category of publisher",
///      "description": "Provides information about the category of publisher releasing the document.",
///      "type": "string",
///      "enum": [
///        "coordinator",
///        "discoverer",
///        "other",
///        "translator",
///        "user",
///        "vendor"
///      ]
///    },
///    "contact_details": {
///      "title": "Contact details",
///      "description": "Information on how to contact the publisher, possibly including details such as web sites, email addresses, phone numbers, and postal mail addresses.",
///      "examples": [
///        "Example Company can be reached at contact_us@example.com, or via our website at https://www.example.com/contact."
///      ],
///      "type": "string",
///      "minLength": 1
///    },
///    "issuing_authority": {
///      "title": "Issuing authority",
///      "description": "Provides information about the authority of the issuing party to release the document, in particular, the party's constituency and responsibilities or other obligations.",
///      "type": "string",
///      "minLength": 1
///    },
///    "name": {
///      "title": "Name of publisher",
///      "description": "Contains the name of the issuing party.",
///      "examples": [
///        "BSI",
///        "Cisco PSIRT",
///        "Siemens ProductCERT"
///      ],
///      "type": "string",
///      "minLength": 1
///    },
///    "namespace": {
///      "title": "Namespace of publisher",
///      "description": "Contains a URL which is under control of the issuing party and can be used as a globally unique identifier for that issuing party.",
///      "examples": [
///        "https://csaf.io",
///        "https://www.example.com"
///      ],
///      "type": "string",
///      "format": "uri"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Publisher {
    ///Provides information about the category of publisher releasing the document.
    pub category: CategoryOfPublisher,
    ///Information on how to contact the publisher, possibly including details such as web sites, email addresses, phone numbers, and postal mail addresses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_details: Option<ContactDetails>,
    ///Provides information about the authority of the issuing party to release the document, in particular, the party's constituency and responsibilities or other obligations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuing_authority: Option<IssuingAuthority>,
    ///Contains the name of the issuing party.
    pub name: NameOfPublisher,
    ///Contains a URL which is under control of the issuing party and can be used as a globally unique identifier for that issuing party.
    pub namespace: String,
}
impl From<&Publisher> for Publisher {
    fn from(value: &Publisher) -> Self {
        value.clone()
    }
}
impl Publisher {
    pub fn builder() -> builder::Publisher {
        Default::default()
    }
}
///Holds any reference to conferences, papers, advisories, and other resources that are related and considered related to either a surrounding part of or the entire document and to be of value to the document consumer.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Reference",
///  "description": "Holds any reference to conferences, papers, advisories, and other resources that are related and considered related to either a surrounding part of or the entire document and to be of value to the document consumer.",
///  "type": "object",
///  "required": [
///    "summary",
///    "url"
///  ],
///  "properties": {
///    "category": {
///      "title": "Category of reference",
///      "description": "Indicates whether the reference points to the same document or vulnerability in focus (depending on scope) or to an external resource.",
///      "default": "external",
///      "type": "string",
///      "enum": [
///        "external",
///        "self"
///      ]
///    },
///    "summary": {
///      "title": "Summary of the reference",
///      "description": "Indicates what this reference refers to.",
///      "type": "string",
///      "minLength": 1
///    },
///    "url": {
///      "title": "URL of reference",
///      "description": "Provides the URL for the reference.",
///      "type": "string",
///      "format": "uri"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Reference {
    ///Indicates whether the reference points to the same document or vulnerability in focus (depending on scope) or to an external resource.
    #[serde(default = "defaults::reference_category")]
    pub category: CategoryOfReference,
    ///Indicates what this reference refers to.
    pub summary: SummaryOfTheReference,
    ///Provides the URL for the reference.
    pub url: String,
}
impl From<&Reference> for Reference {
    fn from(value: &Reference) -> Self {
        value.clone()
    }
}
impl Reference {
    pub fn builder() -> builder::Reference {
        Default::default()
    }
}
///Holds a list of references.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "List of references",
///  "description": "Holds a list of references.",
///  "type": "array",
///  "items": {
///    "title": "Reference",
///    "description": "Holds any reference to conferences, papers, advisories, and other resources that are related and considered related to either a surrounding part of or the entire document and to be of value to the document consumer.",
///    "type": "object",
///    "required": [
///      "summary",
///      "url"
///    ],
///    "properties": {
///      "category": {
///        "title": "Category of reference",
///        "description": "Indicates whether the reference points to the same document or vulnerability in focus (depending on scope) or to an external resource.",
///        "default": "external",
///        "type": "string",
///        "enum": [
///          "external",
///          "self"
///        ]
///      },
///      "summary": {
///        "title": "Summary of the reference",
///        "description": "Indicates what this reference refers to.",
///        "type": "string",
///        "minLength": 1
///      },
///      "url": {
///        "title": "URL of reference",
///        "description": "Provides the URL for the reference.",
///        "type": "string",
///        "format": "uri"
///      }
///    }
///  },
///  "minItems": 1
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ReferencesT(pub Vec<Reference>);
impl ::std::ops::Deref for ReferencesT {
    type Target = Vec<Reference>;
    fn deref(&self) -> &Vec<Reference> {
        &self.0
    }
}
impl From<ReferencesT> for Vec<Reference> {
    fn from(value: ReferencesT) -> Self {
        value.0
    }
}
impl From<&ReferencesT> for ReferencesT {
    fn from(value: &ReferencesT) -> Self {
        value.clone()
    }
}
impl From<Vec<Reference>> for ReferencesT {
    fn from(value: Vec<Reference>) -> Self {
        Self(value)
    }
}
///Establishes a link between two existing full_product_name_t elements, allowing the document producer to define a combination of two products that form a new full_product_name entry.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Relationship",
///  "description": "Establishes a link between two existing full_product_name_t elements, allowing the document producer to define a combination of two products that form a new full_product_name entry.",
///  "type": "object",
///  "required": [
///    "category",
///    "full_product_name",
///    "product_reference",
///    "relates_to_product_reference"
///  ],
///  "properties": {
///    "category": {
///      "title": "Relationship category",
///      "description": "Defines the category of relationship for the referenced component.",
///      "type": "string",
///      "enum": [
///        "default_component_of",
///        "external_component_of",
///        "installed_on",
///        "installed_with",
///        "optional_component_of"
///      ]
///    },
///    "full_product_name": {
///      "$ref": "#/$defs/full_product_name_t"
///    },
///    "product_reference": {
///      "title": "Product reference",
///      "description": "Holds a Product ID that refers to the Full Product Name element, which is referenced as the first element of the relationship.",
///      "$ref": "#/$defs/product_id_t"
///    },
///    "relates_to_product_reference": {
///      "title": "Relates to product reference",
///      "description": "Holds a Product ID that refers to the Full Product Name element, which is referenced as the second element of the relationship.",
///      "$ref": "#/$defs/product_id_t"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Relationship {
    ///Defines the category of relationship for the referenced component.
    pub category: RelationshipCategory,
    pub full_product_name: FullProductNameT,
    ///Holds a Product ID that refers to the Full Product Name element, which is referenced as the first element of the relationship.
    pub product_reference: ProductIdT,
    ///Holds a Product ID that refers to the Full Product Name element, which is referenced as the second element of the relationship.
    pub relates_to_product_reference: ProductIdT,
}
impl From<&Relationship> for Relationship {
    fn from(value: &Relationship) -> Self {
        value.clone()
    }
}
impl Relationship {
    pub fn builder() -> builder::Relationship {
        Default::default()
    }
}
///Defines the category of relationship for the referenced component.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Relationship category",
///  "description": "Defines the category of relationship for the referenced component.",
///  "type": "string",
///  "enum": [
///    "default_component_of",
///    "external_component_of",
///    "installed_on",
///    "installed_with",
///    "optional_component_of"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum RelationshipCategory {
    #[serde(rename = "default_component_of")]
    DefaultComponentOf,
    #[serde(rename = "external_component_of")]
    ExternalComponentOf,
    #[serde(rename = "installed_on")]
    InstalledOn,
    #[serde(rename = "installed_with")]
    InstalledWith,
    #[serde(rename = "optional_component_of")]
    OptionalComponentOf,
}
impl From<&RelationshipCategory> for RelationshipCategory {
    fn from(value: &RelationshipCategory) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RelationshipCategory {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::DefaultComponentOf => write!(f, "default_component_of"),
            Self::ExternalComponentOf => write!(f, "external_component_of"),
            Self::InstalledOn => write!(f, "installed_on"),
            Self::InstalledWith => write!(f, "installed_with"),
            Self::OptionalComponentOf => write!(f, "optional_component_of"),
        }
    }
}
impl std::str::FromStr for RelationshipCategory {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "default_component_of" => Ok(Self::DefaultComponentOf),
            "external_component_of" => Ok(Self::ExternalComponentOf),
            "installed_on" => Ok(Self::InstalledOn),
            "installed_with" => Ok(Self::InstalledWith),
            "optional_component_of" => Ok(Self::OptionalComponentOf),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for RelationshipCategory {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RelationshipCategory {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RelationshipCategory {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Specifies details on how to handle (and presumably, fix) a vulnerability.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Remediation",
///  "description": "Specifies details on how to handle (and presumably, fix) a vulnerability.",
///  "type": "object",
///  "required": [
///    "category",
///    "details"
///  ],
///  "properties": {
///    "category": {
///      "title": "Category of the remediation",
///      "description": "Specifies the category which this remediation belongs to.",
///      "type": "string",
///      "enum": [
///        "mitigation",
///        "no_fix_planned",
///        "none_available",
///        "vendor_fix",
///        "workaround"
///      ]
///    },
///    "date": {
///      "title": "Date of the remediation",
///      "description": "Contains the date from which the remediation is available.",
///      "type": "string",
///      "format": "date-time"
///    },
///    "details": {
///      "title": "Details of the remediation",
///      "description": "Contains a thorough human-readable discussion of the remediation.",
///      "type": "string",
///      "minLength": 1
///    },
///    "entitlements": {
///      "title": "List of entitlements",
///      "description": "Contains a list of entitlements.",
///      "type": "array",
///      "items": {
///        "title": "Entitlement of the remediation",
///        "description": "Contains any possible vendor-defined constraints for obtaining fixed software or hardware that fully resolves the vulnerability.",
///        "type": "string",
///        "minLength": 1
///      },
///      "minItems": 1
///    },
///    "group_ids": {
///      "$ref": "#/$defs/product_groups_t"
///    },
///    "product_ids": {
///      "$ref": "#/$defs/products_t"
///    },
///    "restart_required": {
///      "title": "Restart required by remediation",
///      "description": "Provides information on category of restart is required by this remediation to become effective.",
///      "type": "object",
///      "required": [
///        "category"
///      ],
///      "properties": {
///        "category": {
///          "title": "Category of restart",
///          "description": "Specifies what category of restart is required by this remediation to become effective.",
///          "type": "string",
///          "enum": [
///            "connected",
///            "dependencies",
///            "machine",
///            "none",
///            "parent",
///            "service",
///            "system",
///            "vulnerable_component",
///            "zone"
///          ]
///        },
///        "details": {
///          "title": "Additional restart information",
///          "description": "Provides additional information for the restart. This can include details on procedures, scope or impact.",
///          "type": "string",
///          "minLength": 1
///        }
///      }
///    },
///    "url": {
///      "title": "URL to the remediation",
///      "description": "Contains the URL where to obtain the remediation.",
///      "type": "string",
///      "format": "uri"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Remediation {
    ///Specifies the category which this remediation belongs to.
    pub category: CategoryOfTheRemediation,
    ///Contains the date from which the remediation is available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<chrono::DateTime<chrono::offset::Utc>>,
    ///Contains a thorough human-readable discussion of the remediation.
    pub details: DetailsOfTheRemediation,
    ///Contains a list of entitlements.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entitlements: Vec<EntitlementOfTheRemediation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<ProductGroupsT>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_ids: Option<ProductsT>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restart_required: Option<RestartRequiredByRemediation>,
    ///Contains the URL where to obtain the remediation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&Remediation> for Remediation {
    fn from(value: &Remediation) -> Self {
        value.clone()
    }
}
impl Remediation {
    pub fn builder() -> builder::Remediation {
        Default::default()
    }
}
///Provides information on category of restart is required by this remediation to become effective.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Restart required by remediation",
///  "description": "Provides information on category of restart is required by this remediation to become effective.",
///  "type": "object",
///  "required": [
///    "category"
///  ],
///  "properties": {
///    "category": {
///      "title": "Category of restart",
///      "description": "Specifies what category of restart is required by this remediation to become effective.",
///      "type": "string",
///      "enum": [
///        "connected",
///        "dependencies",
///        "machine",
///        "none",
///        "parent",
///        "service",
///        "system",
///        "vulnerable_component",
///        "zone"
///      ]
///    },
///    "details": {
///      "title": "Additional restart information",
///      "description": "Provides additional information for the restart. This can include details on procedures, scope or impact.",
///      "type": "string",
///      "minLength": 1
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RestartRequiredByRemediation {
    ///Specifies what category of restart is required by this remediation to become effective.
    pub category: CategoryOfRestart,
    ///Provides additional information for the restart. This can include details on procedures, scope or impact.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<AdditionalRestartInformation>,
}
impl From<&RestartRequiredByRemediation> for RestartRequiredByRemediation {
    fn from(value: &RestartRequiredByRemediation) -> Self {
        value.clone()
    }
}
impl RestartRequiredByRemediation {
    pub fn builder() -> builder::RestartRequiredByRemediation {
        Default::default()
    }
}
///Contains all the information elements required to track the evolution of a CSAF document.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Revision",
///  "description": "Contains all the information elements required to track the evolution of a CSAF document.",
///  "type": "object",
///  "required": [
///    "date",
///    "number",
///    "summary"
///  ],
///  "properties": {
///    "date": {
///      "title": "Date of the revision",
///      "description": "The date of the revision entry",
///      "type": "string",
///      "format": "date-time"
///    },
///    "legacy_version": {
///      "title": "Legacy version of the revision",
///      "description": "Contains the version string used in an existing document with the same content.",
///      "type": "string",
///      "minLength": 1
///    },
///    "number": {
///      "$ref": "#/$defs/version_t"
///    },
///    "summary": {
///      "title": "Summary of the revision",
///      "description": "Holds a single non-empty string representing a short description of the changes.",
///      "examples": [
///        "Initial version."
///      ],
///      "type": "string",
///      "minLength": 1
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Revision {
    ///The date of the revision entry
    pub date: chrono::DateTime<chrono::offset::Utc>,
    ///Contains the version string used in an existing document with the same content.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legacy_version: Option<LegacyVersionOfTheRevision>,
    pub number: VersionT,
    ///Holds a single non-empty string representing a short description of the changes.
    pub summary: SummaryOfTheRevision,
}
impl From<&Revision> for Revision {
    fn from(value: &Revision) -> Self {
        value.clone()
    }
}
impl Revision {
    pub fn builder() -> builder::Revision {
        Default::default()
    }
}
///Describe any constraints on how this document might be shared.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Rules for sharing document",
///  "description": "Describe any constraints on how this document might be shared.",
///  "type": "object",
///  "minProperties": 1,
///  "properties": {
///    "text": {
///      "title": "Textual description",
///      "description": "Provides a textual description of additional constraints.",
///      "examples": [
///        "Copyright 2021, Example Company, All Rights Reserved.",
///        "Distribute freely.",
///        "Share only on a need-to-know-basis only."
///      ],
///      "type": "string",
///      "minLength": 1
///    },
///    "tlp": {
///      "title": "Traffic Light Protocol (TLP)",
///      "description": "Provides details about the TLP classification of the document.",
///      "type": "object",
///      "required": [
///        "label"
///      ],
///      "properties": {
///        "label": {
///          "title": "Label of TLP",
///          "description": "Provides the TLP label of the document.",
///          "type": "string",
///          "enum": [
///            "AMBER",
///            "GREEN",
///            "RED",
///            "WHITE"
///          ]
///        },
///        "url": {
///          "title": "URL of TLP version",
///          "description": "Provides a URL where to find the textual description of the TLP version which is used in this document. Default is the URL to the definition by FIRST.",
///          "default": "https://www.first.org/tlp/",
///          "examples": [
///            "https://www.us-cert.gov/tlp",
///            "https://www.bsi.bund.de/SharedDocs/Downloads/DE/BSI/Kritis/Merkblatt_TLP.pdf"
///          ],
///          "type": "string",
///          "format": "uri"
///        }
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RulesForSharingDocument {
    ///Provides a textual description of additional constraints.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<TextualDescription>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tlp: Option<TrafficLightProtocolTlp>,
}
impl From<&RulesForSharingDocument> for RulesForSharingDocument {
    fn from(value: &RulesForSharingDocument) -> Self {
        value.clone()
    }
}
impl RulesForSharingDocument {
    pub fn builder() -> builder::RulesForSharingDocument {
        Default::default()
    }
}
///Specifies information about (at least one) score of the vulnerability and for which products the given value applies.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Score",
///  "description": "Specifies information about (at least one) score of the vulnerability and for which products the given value applies.",
///  "type": "object",
///  "minProperties": 2,
///  "required": [
///    "products"
///  ],
///  "properties": {
///    "cvss_v2": {
///      "type": "object"
///    },
///    "cvss_v3": {
///      "type": "object"
///    },
///    "products": {
///      "$ref": "#/$defs/products_t"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Score {
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub cvss_v2: ::serde_json::Map<String, ::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub cvss_v3: ::serde_json::Map<String, ::serde_json::Value>,
    pub products: ProductsT,
}
impl From<&Score> for Score {
    fn from(value: &Score) -> Self {
        value.clone()
    }
}
impl Score {
    pub fn builder() -> builder::Score {
        Default::default()
    }
}
///Contains a full or abbreviated (partial) serial number of the component to identify.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Serial number",
///  "description": "Contains a full or abbreviated (partial) serial number of the component to identify.",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SerialNumber(String);
impl ::std::ops::Deref for SerialNumber {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<SerialNumber> for String {
    fn from(value: SerialNumber) -> Self {
        value.0
    }
}
impl From<&SerialNumber> for SerialNumber {
    fn from(value: &SerialNumber) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for SerialNumber {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SerialNumber {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for SerialNumber {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for SerialNumber {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SerialNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Contains a full or abbreviated (partial) stock keeping unit (SKU) which is used in the ordering process to identify the component.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Stock keeping unit",
///  "description": "Contains a full or abbreviated (partial) stock keeping unit (SKU) which is used in the ordering process to identify the component.",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StockKeepingUnit(String);
impl ::std::ops::Deref for StockKeepingUnit {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<StockKeepingUnit> for String {
    fn from(value: StockKeepingUnit) -> Self {
        value.0
    }
}
impl From<&StockKeepingUnit> for StockKeepingUnit {
    fn from(value: &StockKeepingUnit) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for StockKeepingUnit {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for StockKeepingUnit {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for StockKeepingUnit {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for StockKeepingUnit {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for StockKeepingUnit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///SHOULD represent any contextual details the document producers wish to make known about the acknowledgment or acknowledged parties.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Summary of the acknowledgment",
///  "description": "SHOULD represent any contextual details the document producers wish to make known about the acknowledgment or acknowledged parties.",
///  "examples": [
///    "First analysis of Coordinated Multi-Stream Attack (CMSA)"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SummaryOfTheAcknowledgment(String);
impl ::std::ops::Deref for SummaryOfTheAcknowledgment {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<SummaryOfTheAcknowledgment> for String {
    fn from(value: SummaryOfTheAcknowledgment) -> Self {
        value.0
    }
}
impl From<&SummaryOfTheAcknowledgment> for SummaryOfTheAcknowledgment {
    fn from(value: &SummaryOfTheAcknowledgment) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for SummaryOfTheAcknowledgment {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SummaryOfTheAcknowledgment {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for SummaryOfTheAcknowledgment {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for SummaryOfTheAcknowledgment {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SummaryOfTheAcknowledgment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Contains additional context regarding what is going on.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Summary of the involvement",
///  "description": "Contains additional context regarding what is going on.",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SummaryOfTheInvolvement(String);
impl ::std::ops::Deref for SummaryOfTheInvolvement {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<SummaryOfTheInvolvement> for String {
    fn from(value: SummaryOfTheInvolvement) -> Self {
        value.0
    }
}
impl From<&SummaryOfTheInvolvement> for SummaryOfTheInvolvement {
    fn from(value: &SummaryOfTheInvolvement) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for SummaryOfTheInvolvement {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SummaryOfTheInvolvement {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for SummaryOfTheInvolvement {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for SummaryOfTheInvolvement {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SummaryOfTheInvolvement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Gives a short, optional description of the group.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Summary of the product group",
///  "description": "Gives a short, optional description of the group.",
///  "examples": [
///    "Products supporting Modbus.",
///    "The x64 versions of the operating system."
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SummaryOfTheProductGroup(String);
impl ::std::ops::Deref for SummaryOfTheProductGroup {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<SummaryOfTheProductGroup> for String {
    fn from(value: SummaryOfTheProductGroup) -> Self {
        value.0
    }
}
impl From<&SummaryOfTheProductGroup> for SummaryOfTheProductGroup {
    fn from(value: &SummaryOfTheProductGroup) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for SummaryOfTheProductGroup {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SummaryOfTheProductGroup {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for SummaryOfTheProductGroup {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for SummaryOfTheProductGroup {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SummaryOfTheProductGroup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Indicates what this reference refers to.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Summary of the reference",
///  "description": "Indicates what this reference refers to.",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SummaryOfTheReference(String);
impl ::std::ops::Deref for SummaryOfTheReference {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<SummaryOfTheReference> for String {
    fn from(value: SummaryOfTheReference) -> Self {
        value.0
    }
}
impl From<&SummaryOfTheReference> for SummaryOfTheReference {
    fn from(value: &SummaryOfTheReference) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for SummaryOfTheReference {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SummaryOfTheReference {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for SummaryOfTheReference {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for SummaryOfTheReference {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SummaryOfTheReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Holds a single non-empty string representing a short description of the changes.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Summary of the revision",
///  "description": "Holds a single non-empty string representing a short description of the changes.",
///  "examples": [
///    "Initial version."
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SummaryOfTheRevision(String);
impl ::std::ops::Deref for SummaryOfTheRevision {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<SummaryOfTheRevision> for String {
    fn from(value: SummaryOfTheRevision) -> Self {
        value.0
    }
}
impl From<&SummaryOfTheRevision> for SummaryOfTheRevision {
    fn from(value: &SummaryOfTheRevision) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for SummaryOfTheRevision {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SummaryOfTheRevision {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for SummaryOfTheRevision {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for SummaryOfTheRevision {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SummaryOfTheRevision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Indicates the name of the vulnerability tracking or numbering system.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "System name",
///  "description": "Indicates the name of the vulnerability tracking or numbering system.",
///  "examples": [
///    "Cisco Bug ID",
///    "GitHub Issue"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SystemName(String);
impl ::std::ops::Deref for SystemName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<SystemName> for String {
    fn from(value: SystemName) -> Self {
        value.0
    }
}
impl From<&SystemName> for SystemName {
    fn from(value: &SystemName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for SystemName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SystemName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for SystemName {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for SystemName {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SystemName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Is unique label or tracking ID for the vulnerability (if such information exists).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Text",
///  "description": "Is unique label or tracking ID for the vulnerability (if such information exists).",
///  "examples": [
///    "CSCso66472",
///    "oasis-tcs/csaf#210"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Text(String);
impl ::std::ops::Deref for Text {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Text> for String {
    fn from(value: Text) -> Self {
        value.0
    }
}
impl From<&Text> for Text {
    fn from(value: &Text) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Text {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Text {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for Text {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for Text {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Text {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Provides a severity which is independent of - and in addition to - any other standard metric for determining the impact or severity of a given vulnerability (such as CVSS).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Text of aggregate severity",
///  "description": "Provides a severity which is independent of - and in addition to - any other standard metric for determining the impact or severity of a given vulnerability (such as CVSS).",
///  "examples": [
///    "Critical",
///    "Important",
///    "Moderate"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TextOfAggregateSeverity(String);
impl ::std::ops::Deref for TextOfAggregateSeverity {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<TextOfAggregateSeverity> for String {
    fn from(value: TextOfAggregateSeverity) -> Self {
        value.0
    }
}
impl From<&TextOfAggregateSeverity> for TextOfAggregateSeverity {
    fn from(value: &TextOfAggregateSeverity) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TextOfAggregateSeverity {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TextOfAggregateSeverity {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for TextOfAggregateSeverity {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for TextOfAggregateSeverity {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TextOfAggregateSeverity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Provides a textual description of additional constraints.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Textual description",
///  "description": "Provides a textual description of additional constraints.",
///  "examples": [
///    "Copyright 2021, Example Company, All Rights Reserved.",
///    "Distribute freely.",
///    "Share only on a need-to-know-basis only."
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TextualDescription(String);
impl ::std::ops::Deref for TextualDescription {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<TextualDescription> for String {
    fn from(value: TextualDescription) -> Self {
        value.0
    }
}
impl From<&TextualDescription> for TextualDescription {
    fn from(value: &TextualDescription) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TextualDescription {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TextualDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for TextualDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for TextualDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TextualDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///The value should be the product’s full canonical name, including version number and other attributes, as it would be used in a human-friendly document.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Textual description of the product",
///  "description": "The value should be the product’s full canonical name, including version number and other attributes, as it would be used in a human-friendly document.",
///  "examples": [
///    "Cisco AnyConnect Secure Mobility Client 2.3.185",
///    "Microsoft Host Integration Server 2006 Service Pack 1"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TextualDescriptionOfTheProduct(String);
impl ::std::ops::Deref for TextualDescriptionOfTheProduct {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<TextualDescriptionOfTheProduct> for String {
    fn from(value: TextualDescriptionOfTheProduct) -> Self {
        value.0
    }
}
impl From<&TextualDescriptionOfTheProduct> for TextualDescriptionOfTheProduct {
    fn from(value: &TextualDescriptionOfTheProduct) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TextualDescriptionOfTheProduct {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TextualDescriptionOfTheProduct {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for TextualDescriptionOfTheProduct {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for TextualDescriptionOfTheProduct {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TextualDescriptionOfTheProduct {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Contains the vulnerability kinetic information. This information can change as the vulnerability ages and new information becomes available.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Threat",
///  "description": "Contains the vulnerability kinetic information. This information can change as the vulnerability ages and new information becomes available.",
///  "type": "object",
///  "required": [
///    "category",
///    "details"
///  ],
///  "properties": {
///    "category": {
///      "title": "Category of the threat",
///      "description": "Categorizes the threat according to the rules of the specification.",
///      "type": "string",
///      "enum": [
///        "exploit_status",
///        "impact",
///        "target_set"
///      ]
///    },
///    "date": {
///      "title": "Date of the threat",
///      "description": "Contains the date when the assessment was done or the threat appeared.",
///      "type": "string",
///      "format": "date-time"
///    },
///    "details": {
///      "title": "Details of the threat",
///      "description": "Represents a thorough human-readable discussion of the threat.",
///      "type": "string",
///      "minLength": 1
///    },
///    "group_ids": {
///      "$ref": "#/$defs/product_groups_t"
///    },
///    "product_ids": {
///      "$ref": "#/$defs/products_t"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Threat {
    ///Categorizes the threat according to the rules of the specification.
    pub category: CategoryOfTheThreat,
    ///Contains the date when the assessment was done or the threat appeared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<chrono::DateTime<chrono::offset::Utc>>,
    ///Represents a thorough human-readable discussion of the threat.
    pub details: DetailsOfTheThreat,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<ProductGroupsT>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_ids: Option<ProductsT>,
}
impl From<&Threat> for Threat {
    fn from(value: &Threat) -> Self {
        value.clone()
    }
}
impl Threat {
    pub fn builder() -> builder::Threat {
        Default::default()
    }
}
///Gives the document producer the ability to apply a canonical name or title to the vulnerability.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Title",
///  "description": "Gives the document producer the ability to apply a canonical name or title to the vulnerability.",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Title(String);
impl ::std::ops::Deref for Title {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Title> for String {
    fn from(value: Title) -> Self {
        value.0
    }
}
impl From<&Title> for Title {
    fn from(value: &Title) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Title {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Title {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for Title {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for Title {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Title {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Provides a concise description of what is contained in the text of the note.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Title of note",
///  "description": "Provides a concise description of what is contained in the text of the note.",
///  "examples": [
///    "Details",
///    "Executive summary",
///    "Technical summary",
///    "Impact on safety systems"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TitleOfNote(String);
impl ::std::ops::Deref for TitleOfNote {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<TitleOfNote> for String {
    fn from(value: TitleOfNote) -> Self {
        value.0
    }
}
impl From<&TitleOfNote> for TitleOfNote {
    fn from(value: &TitleOfNote) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TitleOfNote {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TitleOfNote {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for TitleOfNote {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for TitleOfNote {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TitleOfNote {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///This SHOULD be a canonical name for the document, and sufficiently unique to distinguish it from similar documents.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Title of this document",
///  "description": "This SHOULD be a canonical name for the document, and sufficiently unique to distinguish it from similar documents.",
///  "examples": [
///    "Cisco IPv6 Crafted Packet Denial of Service Vulnerability",
///    "Example Company Cross-Site-Scripting Vulnerability in Example Generator"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TitleOfThisDocument(String);
impl ::std::ops::Deref for TitleOfThisDocument {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<TitleOfThisDocument> for String {
    fn from(value: TitleOfThisDocument) -> Self {
        value.0
    }
}
impl From<&TitleOfThisDocument> for TitleOfThisDocument {
    fn from(value: &TitleOfThisDocument) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TitleOfThisDocument {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TitleOfThisDocument {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for TitleOfThisDocument {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for TitleOfThisDocument {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TitleOfThisDocument {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Is a container designated to hold all management attributes necessary to track a CSAF document as a whole.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Tracking",
///  "description": "Is a container designated to hold all management attributes necessary to track a CSAF document as a whole.",
///  "type": "object",
///  "required": [
///    "current_release_date",
///    "id",
///    "initial_release_date",
///    "revision_history",
///    "status",
///    "version"
///  ],
///  "properties": {
///    "aliases": {
///      "title": "Aliases",
///      "description": "Contains a list of alternate names for the same document.",
///      "type": "array",
///      "items": {
///        "title": "Alternate name",
///        "description": "Specifies a non-empty string that represents a distinct optional alternative ID used to refer to the document.",
///        "examples": [
///          "CVE-2019-12345"
///        ],
///        "type": "string",
///        "minLength": 1
///      },
///      "minItems": 1,
///      "uniqueItems": true
///    },
///    "current_release_date": {
///      "title": "Current release date",
///      "description": "The date when the current revision of this document was released",
///      "type": "string",
///      "format": "date-time"
///    },
///    "generator": {
///      "title": "Document generator",
///      "description": "Is a container to hold all elements related to the generation of the document. These items will reference when the document was actually created, including the date it was generated and the entity that generated it.",
///      "type": "object",
///      "required": [
///        "engine"
///      ],
///      "properties": {
///        "date": {
///          "title": "Date of document generation",
///          "description": "This SHOULD be the current date that the document was generated. Because documents are often generated internally by a document producer and exist for a nonzero amount of time before being released, this field MAY be different from the Initial Release Date and Current Release Date.",
///          "type": "string",
///          "format": "date-time"
///        },
///        "engine": {
///          "title": "Engine of document generation",
///          "description": "Contains information about the engine that generated the CSAF document.",
///          "type": "object",
///          "required": [
///            "name"
///          ],
///          "properties": {
///            "name": {
///              "title": "Engine name",
///              "description": "Represents the name of the engine that generated the CSAF document.",
///              "examples": [
///                "Red Hat rhsa-to-cvrf",
///                "Secvisogram",
///                "TVCE"
///              ],
///              "type": "string",
///              "minLength": 1
///            },
///            "version": {
///              "title": "Engine version",
///              "description": "Contains the version of the engine that generated the CSAF document.",
///              "examples": [
///                "0.6.0",
///                "1.0.0-beta+exp.sha.a1c44f85",
///                "2"
///              ],
///              "type": "string",
///              "minLength": 1
///            }
///          }
///        }
///      }
///    },
///    "id": {
///      "title": "Unique identifier for the document",
///      "description": "The ID is a simple label that provides for a wide range of numbering values, types, and schemes. Its value SHOULD be assigned and maintained by the original document issuing authority.",
///      "examples": [
///        "Example Company - 2019-YH3234",
///        "RHBA-2019:0024",
///        "cisco-sa-20190513-secureboot"
///      ],
///      "type": "string",
///      "minLength": 1,
///      "pattern": "^[\\S](.*[\\S])?$"
///    },
///    "initial_release_date": {
///      "title": "Initial release date",
///      "description": "The date when this document was first published.",
///      "type": "string",
///      "format": "date-time"
///    },
///    "revision_history": {
///      "title": "Revision history",
///      "description": "Holds one revision item for each version of the CSAF document, including the initial one.",
///      "type": "array",
///      "items": {
///        "title": "Revision",
///        "description": "Contains all the information elements required to track the evolution of a CSAF document.",
///        "type": "object",
///        "required": [
///          "date",
///          "number",
///          "summary"
///        ],
///        "properties": {
///          "date": {
///            "title": "Date of the revision",
///            "description": "The date of the revision entry",
///            "type": "string",
///            "format": "date-time"
///          },
///          "legacy_version": {
///            "title": "Legacy version of the revision",
///            "description": "Contains the version string used in an existing document with the same content.",
///            "type": "string",
///            "minLength": 1
///          },
///          "number": {
///            "$ref": "#/$defs/version_t"
///          },
///          "summary": {
///            "title": "Summary of the revision",
///            "description": "Holds a single non-empty string representing a short description of the changes.",
///            "examples": [
///              "Initial version."
///            ],
///            "type": "string",
///            "minLength": 1
///          }
///        }
///      },
///      "minItems": 1
///    },
///    "status": {
///      "title": "Document status",
///      "description": "Defines the draft status of the document.",
///      "type": "string",
///      "enum": [
///        "draft",
///        "final",
///        "interim"
///      ]
///    },
///    "version": {
///      "$ref": "#/$defs/version_t"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Tracking {
    ///Contains a list of alternate names for the same document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<AlternateName>>,
    ///The date when the current revision of this document was released
    pub current_release_date: chrono::DateTime<chrono::offset::Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generator: Option<DocumentGenerator>,
    ///The ID is a simple label that provides for a wide range of numbering values, types, and schemes. Its value SHOULD be assigned and maintained by the original document issuing authority.
    pub id: UniqueIdentifierForTheDocument,
    ///The date when this document was first published.
    pub initial_release_date: chrono::DateTime<chrono::offset::Utc>,
    ///Holds one revision item for each version of the CSAF document, including the initial one.
    pub revision_history: Vec<Revision>,
    ///Defines the draft status of the document.
    pub status: DocumentStatus,
    pub version: VersionT,
}
impl From<&Tracking> for Tracking {
    fn from(value: &Tracking) -> Self {
        value.clone()
    }
}
impl Tracking {
    pub fn builder() -> builder::Tracking {
        Default::default()
    }
}
///Provides details about the TLP classification of the document.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Traffic Light Protocol (TLP)",
///  "description": "Provides details about the TLP classification of the document.",
///  "type": "object",
///  "required": [
///    "label"
///  ],
///  "properties": {
///    "label": {
///      "title": "Label of TLP",
///      "description": "Provides the TLP label of the document.",
///      "type": "string",
///      "enum": [
///        "AMBER",
///        "GREEN",
///        "RED",
///        "WHITE"
///      ]
///    },
///    "url": {
///      "title": "URL of TLP version",
///      "description": "Provides a URL where to find the textual description of the TLP version which is used in this document. Default is the URL to the definition by FIRST.",
///      "default": "https://www.first.org/tlp/",
///      "examples": [
///        "https://www.us-cert.gov/tlp",
///        "https://www.bsi.bund.de/SharedDocs/Downloads/DE/BSI/Kritis/Merkblatt_TLP.pdf"
///      ],
///      "type": "string",
///      "format": "uri"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct TrafficLightProtocolTlp {
    ///Provides the TLP label of the document.
    pub label: LabelOfTlp,
    ///Provides a URL where to find the textual description of the TLP version which is used in this document. Default is the URL to the definition by FIRST.
    #[serde(default = "defaults::traffic_light_protocol_tlp_url")]
    pub url: String,
}
impl From<&TrafficLightProtocolTlp> for TrafficLightProtocolTlp {
    fn from(value: &TrafficLightProtocolTlp) -> Self {
        value.clone()
    }
}
impl TrafficLightProtocolTlp {
    pub fn builder() -> builder::TrafficLightProtocolTlp {
        Default::default()
    }
}
///The ID is a simple label that provides for a wide range of numbering values, types, and schemes. Its value SHOULD be assigned and maintained by the original document issuing authority.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Unique identifier for the document",
///  "description": "The ID is a simple label that provides for a wide range of numbering values, types, and schemes. Its value SHOULD be assigned and maintained by the original document issuing authority.",
///  "examples": [
///    "Example Company - 2019-YH3234",
///    "RHBA-2019:0024",
///    "cisco-sa-20190513-secureboot"
///  ],
///  "type": "string",
///  "minLength": 1,
///  "pattern": "^[\\S](.*[\\S])?$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UniqueIdentifierForTheDocument(String);
impl ::std::ops::Deref for UniqueIdentifierForTheDocument {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<UniqueIdentifierForTheDocument> for String {
    fn from(value: UniqueIdentifierForTheDocument) -> Self {
        value.0
    }
}
impl From<&UniqueIdentifierForTheDocument> for UniqueIdentifierForTheDocument {
    fn from(value: &UniqueIdentifierForTheDocument) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for UniqueIdentifierForTheDocument {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        if regress::Regex::new("^[\\S](.*[\\S])?$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^[\\S](.*[\\S])?$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for UniqueIdentifierForTheDocument {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for UniqueIdentifierForTheDocument {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for UniqueIdentifierForTheDocument {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for UniqueIdentifierForTheDocument {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Contains the cryptographic hash value in hexadecimal representation.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Value of the cryptographic hash",
///  "description": "Contains the cryptographic hash value in hexadecimal representation.",
///  "examples": [
///    "37df33cb7464da5c7f077f4d56a32bc84987ec1d85b234537c1c1a4d4fc8d09dc29e2e762cb5203677bf849a2855a0283710f1f5fe1d6ce8d5ac85c645d0fcb3",
///    "4775203615d9534a8bfca96a93dc8b461a489f69124a130d786b42204f3341cc",
///    "9ea4c8200113d49d26505da0e02e2f49055dc078d1ad7a419b32e291c7afebbb84badfbd46dec42883bea0b2a1fa697c"
///  ],
///  "type": "string",
///  "minLength": 32,
///  "pattern": "^[0-9a-fA-F]{32,}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ValueOfTheCryptographicHash(String);
impl ::std::ops::Deref for ValueOfTheCryptographicHash {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<ValueOfTheCryptographicHash> for String {
    fn from(value: ValueOfTheCryptographicHash) -> Self {
        value.0
    }
}
impl From<&ValueOfTheCryptographicHash> for ValueOfTheCryptographicHash {
    fn from(value: &ValueOfTheCryptographicHash) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ValueOfTheCryptographicHash {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 32usize {
            return Err("shorter than 32 characters".into());
        }
        if regress::Regex::new("^[0-9a-fA-F]{32,}$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^[0-9a-fA-F]{32,}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ValueOfTheCryptographicHash {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for ValueOfTheCryptographicHash {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for ValueOfTheCryptographicHash {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ValueOfTheCryptographicHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Specifies a version string to denote clearly the evolution of the content of the document. Format must be either integer or semantic versioning.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Version",
///  "description": "Specifies a version string to denote clearly the evolution of the content of the document. Format must be either integer or semantic versioning.",
///  "examples": [
///    "1",
///    "4",
///    "0.9.0",
///    "1.4.3",
///    "2.40.0+21AF26D3"
///  ],
///  "type": "string",
///  "pattern": "^(0|[1-9][0-9]*)$|^((0|[1-9]\\d*)\\.(0|[1-9]\\d*)\\.(0|[1-9]\\d*)(?:-((?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\\.(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\\+([0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*))?)$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct VersionT(String);
impl ::std::ops::Deref for VersionT {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<VersionT> for String {
    fn from(value: VersionT) -> Self {
        value.0
    }
}
impl From<&VersionT> for VersionT {
    fn from(value: &VersionT) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for VersionT {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new(
                "^(0|[1-9][0-9]*)$|^((0|[1-9]\\d*)\\.(0|[1-9]\\d*)\\.(0|[1-9]\\d*)(?:-((?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\\.(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\\+([0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*))?)$",
            )
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err(
                "doesn't match pattern \"^(0|[1-9][0-9]*)$|^((0|[1-9]\\d*)\\.(0|[1-9]\\d*)\\.(0|[1-9]\\d*)(?:-((?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\\.(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\\+([0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*))?)$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VersionT {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for VersionT {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for VersionT {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VersionT {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Is a container for the aggregation of all fields that are related to a single vulnerability in the document.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Vulnerability",
///  "description": "Is a container for the aggregation of all fields that are related to a single vulnerability in the document.",
///  "type": "object",
///  "minProperties": 1,
///  "properties": {
///    "acknowledgments": {
///      "title": "Vulnerability acknowledgments",
///      "description": "Contains a list of acknowledgment elements associated with this vulnerability item.",
///      "$ref": "#/$defs/acknowledgments_t"
///    },
///    "cve": {
///      "title": "CVE",
///      "description": "Holds the MITRE standard Common Vulnerabilities and Exposures (CVE) tracking number for the vulnerability.",
///      "type": "string",
///      "pattern": "^CVE-[0-9]{4}-[0-9]{4,}$"
///    },
///    "cwe": {
///      "title": "CWE",
///      "description": "Holds the MITRE standard Common Weakness Enumeration (CWE) for the weakness associated.",
///      "type": "object",
///      "required": [
///        "id",
///        "name"
///      ],
///      "properties": {
///        "id": {
///          "title": "Weakness ID",
///          "description": "Holds the ID for the weakness associated.",
///          "examples": [
///            "CWE-22",
///            "CWE-352",
///            "CWE-79"
///          ],
///          "type": "string",
///          "pattern": "^CWE-[1-9]\\d{0,5}$"
///        },
///        "name": {
///          "title": "Weakness name",
///          "description": "Holds the full name of the weakness as given in the CWE specification.",
///          "examples": [
///            "Cross-Site Request Forgery (CSRF)",
///            "Improper Limitation of a Pathname to a Restricted Directory ('Path Traversal')",
///            "Improper Neutralization of Input During Web Page Generation ('Cross-site Scripting')"
///          ],
///          "type": "string",
///          "minLength": 1
///        }
///      }
///    },
///    "discovery_date": {
///      "title": "Discovery date",
///      "description": "Holds the date and time the vulnerability was originally discovered.",
///      "type": "string",
///      "format": "date-time"
///    },
///    "flags": {
///      "title": "List of flags",
///      "description": "Contains a list of machine readable flags.",
///      "type": "array",
///      "items": {
///        "title": "Flag",
///        "description": "Contains product specific information in regard to this vulnerability as a single machine readable flag.",
///        "type": "object",
///        "required": [
///          "label"
///        ],
///        "properties": {
///          "date": {
///            "title": "Date of the flag",
///            "description": "Contains the date when assessment was done or the flag was assigned.",
///            "type": "string",
///            "format": "date-time"
///          },
///          "group_ids": {
///            "$ref": "#/$defs/product_groups_t"
///          },
///          "label": {
///            "title": "Label of the flag",
///            "description": "Specifies the machine readable label.",
///            "type": "string",
///            "enum": [
///              "component_not_present",
///              "inline_mitigations_already_exist",
///              "vulnerable_code_cannot_be_controlled_by_adversary",
///              "vulnerable_code_not_in_execute_path",
///              "vulnerable_code_not_present"
///            ]
///          },
///          "product_ids": {
///            "$ref": "#/$defs/products_t"
///          }
///        }
///      },
///      "minItems": 1,
///      "uniqueItems": true
///    },
///    "ids": {
///      "title": "List of IDs",
///      "description": "Represents a list of unique labels or tracking IDs for the vulnerability (if such information exists).",
///      "type": "array",
///      "items": {
///        "title": "ID",
///        "description": "Contains a single unique label or tracking ID for the vulnerability.",
///        "type": "object",
///        "required": [
///          "system_name",
///          "text"
///        ],
///        "properties": {
///          "system_name": {
///            "title": "System name",
///            "description": "Indicates the name of the vulnerability tracking or numbering system.",
///            "examples": [
///              "Cisco Bug ID",
///              "GitHub Issue"
///            ],
///            "type": "string",
///            "minLength": 1
///          },
///          "text": {
///            "title": "Text",
///            "description": "Is unique label or tracking ID for the vulnerability (if such information exists).",
///            "examples": [
///              "CSCso66472",
///              "oasis-tcs/csaf#210"
///            ],
///            "type": "string",
///            "minLength": 1
///          }
///        }
///      },
///      "minItems": 1,
///      "uniqueItems": true
///    },
///    "involvements": {
///      "title": "List of involvements",
///      "description": "Contains a list of involvements.",
///      "type": "array",
///      "items": {
///        "title": "Involvement",
///        "description": "Is a container, that allows the document producers to comment on the level of involvement (or engagement) of themselves or third parties in the vulnerability identification, scoping, and remediation process.",
///        "type": "object",
///        "required": [
///          "party",
///          "status"
///        ],
///        "properties": {
///          "date": {
///            "title": "Date of involvement",
///            "description": "Holds the date and time of the involvement entry.",
///            "type": "string",
///            "format": "date-time"
///          },
///          "party": {
///            "title": "Party category",
///            "description": "Defines the category of the involved party.",
///            "type": "string",
///            "enum": [
///              "coordinator",
///              "discoverer",
///              "other",
///              "user",
///              "vendor"
///            ]
///          },
///          "status": {
///            "title": "Party status",
///            "description": "Defines contact status of the involved party.",
///            "type": "string",
///            "enum": [
///              "completed",
///              "contact_attempted",
///              "disputed",
///              "in_progress",
///              "not_contacted",
///              "open"
///            ]
///          },
///          "summary": {
///            "title": "Summary of the involvement",
///            "description": "Contains additional context regarding what is going on.",
///            "type": "string",
///            "minLength": 1
///          }
///        }
///      },
///      "minItems": 1,
///      "uniqueItems": true
///    },
///    "notes": {
///      "title": "Vulnerability notes",
///      "description": "Holds notes associated with this vulnerability item.",
///      "$ref": "#/$defs/notes_t"
///    },
///    "product_status": {
///      "title": "Product status",
///      "description": "Contains different lists of product_ids which provide details on the status of the referenced product related to the current vulnerability. ",
///      "type": "object",
///      "minProperties": 1,
///      "properties": {
///        "first_affected": {
///          "title": "First affected",
///          "description": "These are the first versions of the releases known to be affected by the vulnerability.",
///          "$ref": "#/$defs/products_t"
///        },
///        "first_fixed": {
///          "title": "First fixed",
///          "description": "These versions contain the first fix for the vulnerability but may not be the recommended fixed versions.",
///          "$ref": "#/$defs/products_t"
///        },
///        "fixed": {
///          "title": "Fixed",
///          "description": "These versions contain a fix for the vulnerability but may not be the recommended fixed versions.",
///          "$ref": "#/$defs/products_t"
///        },
///        "known_affected": {
///          "title": "Known affected",
///          "description": "These versions are known to be affected by the vulnerability.",
///          "$ref": "#/$defs/products_t"
///        },
///        "known_not_affected": {
///          "title": "Known not affected",
///          "description": "These versions are known not to be affected by the vulnerability.",
///          "$ref": "#/$defs/products_t"
///        },
///        "last_affected": {
///          "title": "Last affected",
///          "description": "These are the last versions in a release train known to be affected by the vulnerability. Subsequently released versions would contain a fix for the vulnerability.",
///          "$ref": "#/$defs/products_t"
///        },
///        "recommended": {
///          "title": "Recommended",
///          "description": "These versions have a fix for the vulnerability and are the vendor-recommended versions for fixing the vulnerability.",
///          "$ref": "#/$defs/products_t"
///        },
///        "under_investigation": {
///          "title": "Under investigation",
///          "description": "It is not known yet whether these versions are or are not affected by the vulnerability. However, it is still under investigation - the result will be provided in a later release of the document.",
///          "$ref": "#/$defs/products_t"
///        }
///      }
///    },
///    "references": {
///      "title": "Vulnerability references",
///      "description": "Holds a list of references associated with this vulnerability item.",
///      "$ref": "#/$defs/references_t"
///    },
///    "release_date": {
///      "title": "Release date",
///      "description": "Holds the date and time the vulnerability was originally released into the wild.",
///      "type": "string",
///      "format": "date-time"
///    },
///    "remediations": {
///      "title": "List of remediations",
///      "description": "Contains a list of remediations.",
///      "type": "array",
///      "items": {
///        "title": "Remediation",
///        "description": "Specifies details on how to handle (and presumably, fix) a vulnerability.",
///        "type": "object",
///        "required": [
///          "category",
///          "details"
///        ],
///        "properties": {
///          "category": {
///            "title": "Category of the remediation",
///            "description": "Specifies the category which this remediation belongs to.",
///            "type": "string",
///            "enum": [
///              "mitigation",
///              "no_fix_planned",
///              "none_available",
///              "vendor_fix",
///              "workaround"
///            ]
///          },
///          "date": {
///            "title": "Date of the remediation",
///            "description": "Contains the date from which the remediation is available.",
///            "type": "string",
///            "format": "date-time"
///          },
///          "details": {
///            "title": "Details of the remediation",
///            "description": "Contains a thorough human-readable discussion of the remediation.",
///            "type": "string",
///            "minLength": 1
///          },
///          "entitlements": {
///            "title": "List of entitlements",
///            "description": "Contains a list of entitlements.",
///            "type": "array",
///            "items": {
///              "title": "Entitlement of the remediation",
///              "description": "Contains any possible vendor-defined constraints for obtaining fixed software or hardware that fully resolves the vulnerability.",
///              "type": "string",
///              "minLength": 1
///            },
///            "minItems": 1
///          },
///          "group_ids": {
///            "$ref": "#/$defs/product_groups_t"
///          },
///          "product_ids": {
///            "$ref": "#/$defs/products_t"
///          },
///          "restart_required": {
///            "title": "Restart required by remediation",
///            "description": "Provides information on category of restart is required by this remediation to become effective.",
///            "type": "object",
///            "required": [
///              "category"
///            ],
///            "properties": {
///              "category": {
///                "title": "Category of restart",
///                "description": "Specifies what category of restart is required by this remediation to become effective.",
///                "type": "string",
///                "enum": [
///                  "connected",
///                  "dependencies",
///                  "machine",
///                  "none",
///                  "parent",
///                  "service",
///                  "system",
///                  "vulnerable_component",
///                  "zone"
///                ]
///              },
///              "details": {
///                "title": "Additional restart information",
///                "description": "Provides additional information for the restart. This can include details on procedures, scope or impact.",
///                "type": "string",
///                "minLength": 1
///              }
///            }
///          },
///          "url": {
///            "title": "URL to the remediation",
///            "description": "Contains the URL where to obtain the remediation.",
///            "type": "string",
///            "format": "uri"
///          }
///        }
///      },
///      "minItems": 1
///    },
///    "scores": {
///      "title": "List of scores",
///      "description": "Contains score objects for the current vulnerability.",
///      "type": "array",
///      "items": {
///        "title": "Score",
///        "description": "Specifies information about (at least one) score of the vulnerability and for which products the given value applies.",
///        "type": "object",
///        "minProperties": 2,
///        "required": [
///          "products"
///        ],
///        "properties": {
///          "cvss_v2": {
///            "type": "object"
///          },
///          "cvss_v3": {
///            "type": "object"
///          },
///          "products": {
///            "$ref": "#/$defs/products_t"
///          }
///        }
///      },
///      "minItems": 1
///    },
///    "threats": {
///      "title": "List of threats",
///      "description": "Contains information about a vulnerability that can change with time.",
///      "type": "array",
///      "items": {
///        "title": "Threat",
///        "description": "Contains the vulnerability kinetic information. This information can change as the vulnerability ages and new information becomes available.",
///        "type": "object",
///        "required": [
///          "category",
///          "details"
///        ],
///        "properties": {
///          "category": {
///            "title": "Category of the threat",
///            "description": "Categorizes the threat according to the rules of the specification.",
///            "type": "string",
///            "enum": [
///              "exploit_status",
///              "impact",
///              "target_set"
///            ]
///          },
///          "date": {
///            "title": "Date of the threat",
///            "description": "Contains the date when the assessment was done or the threat appeared.",
///            "type": "string",
///            "format": "date-time"
///          },
///          "details": {
///            "title": "Details of the threat",
///            "description": "Represents a thorough human-readable discussion of the threat.",
///            "type": "string",
///            "minLength": 1
///          },
///          "group_ids": {
///            "$ref": "#/$defs/product_groups_t"
///          },
///          "product_ids": {
///            "$ref": "#/$defs/products_t"
///          }
///        }
///      },
///      "minItems": 1
///    },
///    "title": {
///      "title": "Title",
///      "description": "Gives the document producer the ability to apply a canonical name or title to the vulnerability.",
///      "type": "string",
///      "minLength": 1
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Vulnerability {
    ///Contains a list of acknowledgment elements associated with this vulnerability item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acknowledgments: Option<AcknowledgmentsT>,
    ///Holds the MITRE standard Common Vulnerabilities and Exposures (CVE) tracking number for the vulnerability.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cve: Option<Cve>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cwe: Option<Cwe>,
    ///Holds the date and time the vulnerability was originally discovered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discovery_date: Option<chrono::DateTime<chrono::offset::Utc>>,
    ///Contains a list of machine readable flags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<Flag>>,
    ///Represents a list of unique labels or tracking IDs for the vulnerability (if such information exists).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<Id>>,
    ///Contains a list of involvements.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub involvements: Option<Vec<Involvement>>,
    ///Holds notes associated with this vulnerability item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<NotesT>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_status: Option<ProductStatus>,
    ///Holds a list of references associated with this vulnerability item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub references: Option<ReferencesT>,
    ///Holds the date and time the vulnerability was originally released into the wild.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release_date: Option<chrono::DateTime<chrono::offset::Utc>>,
    ///Contains a list of remediations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remediations: Vec<Remediation>,
    ///Contains score objects for the current vulnerability.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scores: Vec<Score>,
    ///Contains information about a vulnerability that can change with time.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub threats: Vec<Threat>,
    ///Gives the document producer the ability to apply a canonical name or title to the vulnerability.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,
}
impl From<&Vulnerability> for Vulnerability {
    fn from(value: &Vulnerability) -> Self {
        value.clone()
    }
}
impl Vulnerability {
    pub fn builder() -> builder::Vulnerability {
        Default::default()
    }
}
///Holds the ID for the weakness associated.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Weakness ID",
///  "description": "Holds the ID for the weakness associated.",
///  "examples": [
///    "CWE-22",
///    "CWE-352",
///    "CWE-79"
///  ],
///  "type": "string",
///  "pattern": "^CWE-[1-9]\\d{0,5}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WeaknessId(String);
impl ::std::ops::Deref for WeaknessId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<WeaknessId> for String {
    fn from(value: WeaknessId) -> Self {
        value.0
    }
}
impl From<&WeaknessId> for WeaknessId {
    fn from(value: &WeaknessId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for WeaknessId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^CWE-[1-9]\\d{0,5}$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^CWE-[1-9]\\d{0,5}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for WeaknessId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for WeaknessId {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for WeaknessId {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for WeaknessId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Holds the full name of the weakness as given in the CWE specification.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Weakness name",
///  "description": "Holds the full name of the weakness as given in the CWE specification.",
///  "examples": [
///    "Cross-Site Request Forgery (CSRF)",
///    "Improper Limitation of a Pathname to a Restricted Directory ('Path Traversal')",
///    "Improper Neutralization of Input During Web Page Generation ('Cross-site Scripting')"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WeaknessName(String);
impl ::std::ops::Deref for WeaknessName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<WeaknessName> for String {
    fn from(value: WeaknessName) -> Self {
        value.0
    }
}
impl From<&WeaknessName> for WeaknessName {
    fn from(value: &WeaknessName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for WeaknessName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for WeaknessName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for WeaknessName {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for WeaknessName {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for WeaknessName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Acknowledgment {
        names: Result<Vec<super::NameOfTheContributor>, String>,
        organization: Result<Option<super::ContributingOrganization>, String>,
        summary: Result<Option<super::SummaryOfTheAcknowledgment>, String>,
        urls: Result<Vec<String>, String>,
    }
    impl Default for Acknowledgment {
        fn default() -> Self {
            Self {
                names: Ok(Default::default()),
                organization: Ok(Default::default()),
                summary: Ok(Default::default()),
                urls: Ok(Default::default()),
            }
        }
    }
    impl Acknowledgment {
        pub fn names<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameOfTheContributor>>,
            T::Error: std::fmt::Display,
        {
            self.names = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for names: {}", e));
            self
        }
        pub fn organization<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ContributingOrganization>>,
            T::Error: std::fmt::Display,
        {
            self.organization = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for organization: {}", e));
            self
        }
        pub fn summary<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SummaryOfTheAcknowledgment>>,
            T::Error: std::fmt::Display,
        {
            self.summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summary: {}", e));
            self
        }
        pub fn urls<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.urls = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for urls: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Acknowledgment> for super::Acknowledgment {
        type Error = super::error::ConversionError;
        fn try_from(value: Acknowledgment) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                names: value.names?,
                organization: value.organization?,
                summary: value.summary?,
                urls: value.urls?,
            })
        }
    }
    impl From<super::Acknowledgment> for Acknowledgment {
        fn from(value: super::Acknowledgment) -> Self {
            Self {
                names: Ok(value.names),
                organization: Ok(value.organization),
                summary: Ok(value.summary),
                urls: Ok(value.urls),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AggregateSeverity {
        namespace: Result<Option<String>, String>,
        text: Result<super::TextOfAggregateSeverity, String>,
    }
    impl Default for AggregateSeverity {
        fn default() -> Self {
            Self {
                namespace: Ok(Default::default()),
                text: Err("no value supplied for text".to_string()),
            }
        }
    }
    impl AggregateSeverity {
        pub fn namespace<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.namespace = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for namespace: {}", e));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::TextOfAggregateSeverity>,
            T::Error: std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AggregateSeverity> for super::AggregateSeverity {
        type Error = super::error::ConversionError;
        fn try_from(value: AggregateSeverity) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                namespace: value.namespace?,
                text: value.text?,
            })
        }
    }
    impl From<super::AggregateSeverity> for AggregateSeverity {
        fn from(value: super::AggregateSeverity) -> Self {
            Self {
                namespace: Ok(value.namespace),
                text: Ok(value.text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Branch {
        branches: Result<Option<super::BranchesT>, String>,
        category: Result<super::CategoryOfTheBranch, String>,
        name: Result<super::NameOfTheBranch, String>,
        product: Result<Option<super::FullProductNameT>, String>,
    }
    impl Default for Branch {
        fn default() -> Self {
            Self {
                branches: Ok(Default::default()),
                category: Err("no value supplied for category".to_string()),
                name: Err("no value supplied for name".to_string()),
                product: Ok(Default::default()),
            }
        }
    }
    impl Branch {
        pub fn branches<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::BranchesT>>,
            T::Error: std::fmt::Display,
        {
            self.branches = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for branches: {}", e));
            self
        }
        pub fn category<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::CategoryOfTheBranch>,
            T::Error: std::fmt::Display,
        {
            self.category = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for category: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::NameOfTheBranch>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn product<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FullProductNameT>>,
            T::Error: std::fmt::Display,
        {
            self.product = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for product: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Branch> for super::Branch {
        type Error = super::error::ConversionError;
        fn try_from(value: Branch) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                branches: value.branches?,
                category: value.category?,
                name: value.name?,
                product: value.product?,
            })
        }
    }
    impl From<super::Branch> for Branch {
        fn from(value: super::Branch) -> Self {
            Self {
                branches: Ok(value.branches),
                category: Ok(value.category),
                name: Ok(value.name),
                product: Ok(value.product),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CommonSecurityAdvisoryFramework {
        document: Result<super::DocumentLevelMetaData, String>,
        product_tree: Result<Option<super::ProductTree>, String>,
        vulnerabilities: Result<Vec<super::Vulnerability>, String>,
    }
    impl Default for CommonSecurityAdvisoryFramework {
        fn default() -> Self {
            Self {
                document: Err("no value supplied for document".to_string()),
                product_tree: Ok(Default::default()),
                vulnerabilities: Ok(Default::default()),
            }
        }
    }
    impl CommonSecurityAdvisoryFramework {
        pub fn document<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DocumentLevelMetaData>,
            T::Error: std::fmt::Display,
        {
            self.document = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for document: {}", e));
            self
        }
        pub fn product_tree<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProductTree>>,
            T::Error: std::fmt::Display,
        {
            self.product_tree = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for product_tree: {}", e));
            self
        }
        pub fn vulnerabilities<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Vulnerability>>,
            T::Error: std::fmt::Display,
        {
            self.vulnerabilities = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vulnerabilities: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<CommonSecurityAdvisoryFramework>
        for super::CommonSecurityAdvisoryFramework
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CommonSecurityAdvisoryFramework,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                document: value.document?,
                product_tree: value.product_tree?,
                vulnerabilities: value.vulnerabilities?,
            })
        }
    }
    impl From<super::CommonSecurityAdvisoryFramework> for CommonSecurityAdvisoryFramework {
        fn from(value: super::CommonSecurityAdvisoryFramework) -> Self {
            Self {
                document: Ok(value.document),
                product_tree: Ok(value.product_tree),
                vulnerabilities: Ok(value.vulnerabilities),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CryptographicHashes {
        file_hashes: Result<Vec<super::FileHash>, String>,
        filename: Result<super::Filename, String>,
    }
    impl Default for CryptographicHashes {
        fn default() -> Self {
            Self {
                file_hashes: Err("no value supplied for file_hashes".to_string()),
                filename: Err("no value supplied for filename".to_string()),
            }
        }
    }
    impl CryptographicHashes {
        pub fn file_hashes<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::FileHash>>,
            T::Error: std::fmt::Display,
        {
            self.file_hashes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_hashes: {}", e));
            self
        }
        pub fn filename<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Filename>,
            T::Error: std::fmt::Display,
        {
            self.filename = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for filename: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<CryptographicHashes> for super::CryptographicHashes {
        type Error = super::error::ConversionError;
        fn try_from(value: CryptographicHashes) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_hashes: value.file_hashes?,
                filename: value.filename?,
            })
        }
    }
    impl From<super::CryptographicHashes> for CryptographicHashes {
        fn from(value: super::CryptographicHashes) -> Self {
            Self {
                file_hashes: Ok(value.file_hashes),
                filename: Ok(value.filename),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Cwe {
        id: Result<super::WeaknessId, String>,
        name: Result<super::WeaknessName, String>,
    }
    impl Default for Cwe {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl Cwe {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::WeaknessId>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::WeaknessName>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Cwe> for super::Cwe {
        type Error = super::error::ConversionError;
        fn try_from(value: Cwe) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                name: value.name?,
            })
        }
    }
    impl From<super::Cwe> for Cwe {
        fn from(value: super::Cwe) -> Self {
            Self {
                id: Ok(value.id),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DocumentGenerator {
        date: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        engine: Result<super::EngineOfDocumentGeneration, String>,
    }
    impl Default for DocumentGenerator {
        fn default() -> Self {
            Self {
                date: Ok(Default::default()),
                engine: Err("no value supplied for engine".to_string()),
            }
        }
    }
    impl DocumentGenerator {
        pub fn date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
            T::Error: std::fmt::Display,
        {
            self.date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date: {}", e));
            self
        }
        pub fn engine<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::EngineOfDocumentGeneration>,
            T::Error: std::fmt::Display,
        {
            self.engine = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for engine: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<DocumentGenerator> for super::DocumentGenerator {
        type Error = super::error::ConversionError;
        fn try_from(value: DocumentGenerator) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                date: value.date?,
                engine: value.engine?,
            })
        }
    }
    impl From<super::DocumentGenerator> for DocumentGenerator {
        fn from(value: super::DocumentGenerator) -> Self {
            Self {
                date: Ok(value.date),
                engine: Ok(value.engine),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DocumentLevelMetaData {
        acknowledgments: Result<Option<super::AcknowledgmentsT>, String>,
        aggregate_severity: Result<Option<super::AggregateSeverity>, String>,
        category: Result<super::DocumentCategory, String>,
        csaf_version: Result<super::CsafVersion, String>,
        distribution: Result<Option<super::RulesForSharingDocument>, String>,
        lang: Result<Option<super::LangT>, String>,
        notes: Result<Option<super::NotesT>, String>,
        publisher: Result<super::Publisher, String>,
        references: Result<Option<super::ReferencesT>, String>,
        source_lang: Result<Option<super::LangT>, String>,
        title: Result<super::TitleOfThisDocument, String>,
        tracking: Result<super::Tracking, String>,
    }
    impl Default for DocumentLevelMetaData {
        fn default() -> Self {
            Self {
                acknowledgments: Ok(Default::default()),
                aggregate_severity: Ok(Default::default()),
                category: Err("no value supplied for category".to_string()),
                csaf_version: Err("no value supplied for csaf_version".to_string()),
                distribution: Ok(Default::default()),
                lang: Ok(Default::default()),
                notes: Ok(Default::default()),
                publisher: Err("no value supplied for publisher".to_string()),
                references: Ok(Default::default()),
                source_lang: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
                tracking: Err("no value supplied for tracking".to_string()),
            }
        }
    }
    impl DocumentLevelMetaData {
        pub fn acknowledgments<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AcknowledgmentsT>>,
            T::Error: std::fmt::Display,
        {
            self.acknowledgments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for acknowledgments: {}", e));
            self
        }
        pub fn aggregate_severity<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AggregateSeverity>>,
            T::Error: std::fmt::Display,
        {
            self.aggregate_severity = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for aggregate_severity: {}",
                    e
                )
            });
            self
        }
        pub fn category<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DocumentCategory>,
            T::Error: std::fmt::Display,
        {
            self.category = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for category: {}", e));
            self
        }
        pub fn csaf_version<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::CsafVersion>,
            T::Error: std::fmt::Display,
        {
            self.csaf_version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for csaf_version: {}", e));
            self
        }
        pub fn distribution<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::RulesForSharingDocument>>,
            T::Error: std::fmt::Display,
        {
            self.distribution = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for distribution: {}", e));
            self
        }
        pub fn lang<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LangT>>,
            T::Error: std::fmt::Display,
        {
            self.lang = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lang: {}", e));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::NotesT>>,
            T::Error: std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {}", e));
            self
        }
        pub fn publisher<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Publisher>,
            T::Error: std::fmt::Display,
        {
            self.publisher = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for publisher: {}", e));
            self
        }
        pub fn references<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ReferencesT>>,
            T::Error: std::fmt::Display,
        {
            self.references = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for references: {}", e));
            self
        }
        pub fn source_lang<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LangT>>,
            T::Error: std::fmt::Display,
        {
            self.source_lang = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source_lang: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::TitleOfThisDocument>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn tracking<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Tracking>,
            T::Error: std::fmt::Display,
        {
            self.tracking = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tracking: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<DocumentLevelMetaData> for super::DocumentLevelMetaData {
        type Error = super::error::ConversionError;
        fn try_from(value: DocumentLevelMetaData) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                acknowledgments: value.acknowledgments?,
                aggregate_severity: value.aggregate_severity?,
                category: value.category?,
                csaf_version: value.csaf_version?,
                distribution: value.distribution?,
                lang: value.lang?,
                notes: value.notes?,
                publisher: value.publisher?,
                references: value.references?,
                source_lang: value.source_lang?,
                title: value.title?,
                tracking: value.tracking?,
            })
        }
    }
    impl From<super::DocumentLevelMetaData> for DocumentLevelMetaData {
        fn from(value: super::DocumentLevelMetaData) -> Self {
            Self {
                acknowledgments: Ok(value.acknowledgments),
                aggregate_severity: Ok(value.aggregate_severity),
                category: Ok(value.category),
                csaf_version: Ok(value.csaf_version),
                distribution: Ok(value.distribution),
                lang: Ok(value.lang),
                notes: Ok(value.notes),
                publisher: Ok(value.publisher),
                references: Ok(value.references),
                source_lang: Ok(value.source_lang),
                title: Ok(value.title),
                tracking: Ok(value.tracking),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EngineOfDocumentGeneration {
        name: Result<super::EngineName, String>,
        version: Result<Option<super::EngineVersion>, String>,
    }
    impl Default for EngineOfDocumentGeneration {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                version: Ok(Default::default()),
            }
        }
    }
    impl EngineOfDocumentGeneration {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::EngineName>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::EngineVersion>>,
            T::Error: std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<EngineOfDocumentGeneration> for super::EngineOfDocumentGeneration {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EngineOfDocumentGeneration,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                version: value.version?,
            })
        }
    }
    impl From<super::EngineOfDocumentGeneration> for EngineOfDocumentGeneration {
        fn from(value: super::EngineOfDocumentGeneration) -> Self {
            Self {
                name: Ok(value.name),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FileHash {
        algorithm: Result<super::AlgorithmOfTheCryptographicHash, String>,
        value: Result<super::ValueOfTheCryptographicHash, String>,
    }
    impl Default for FileHash {
        fn default() -> Self {
            Self {
                algorithm: Err("no value supplied for algorithm".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl FileHash {
        pub fn algorithm<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AlgorithmOfTheCryptographicHash>,
            T::Error: std::fmt::Display,
        {
            self.algorithm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for algorithm: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ValueOfTheCryptographicHash>,
            T::Error: std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FileHash> for super::FileHash {
        type Error = super::error::ConversionError;
        fn try_from(value: FileHash) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                algorithm: value.algorithm?,
                value: value.value?,
            })
        }
    }
    impl From<super::FileHash> for FileHash {
        fn from(value: super::FileHash) -> Self {
            Self {
                algorithm: Ok(value.algorithm),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Flag {
        date: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        group_ids: Result<Option<super::ProductGroupsT>, String>,
        label: Result<super::LabelOfTheFlag, String>,
        product_ids: Result<Option<super::ProductsT>, String>,
    }
    impl Default for Flag {
        fn default() -> Self {
            Self {
                date: Ok(Default::default()),
                group_ids: Ok(Default::default()),
                label: Err("no value supplied for label".to_string()),
                product_ids: Ok(Default::default()),
            }
        }
    }
    impl Flag {
        pub fn date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
            T::Error: std::fmt::Display,
        {
            self.date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date: {}", e));
            self
        }
        pub fn group_ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProductGroupsT>>,
            T::Error: std::fmt::Display,
        {
            self.group_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_ids: {}", e));
            self
        }
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::LabelOfTheFlag>,
            T::Error: std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {}", e));
            self
        }
        pub fn product_ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProductsT>>,
            T::Error: std::fmt::Display,
        {
            self.product_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for product_ids: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Flag> for super::Flag {
        type Error = super::error::ConversionError;
        fn try_from(value: Flag) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                date: value.date?,
                group_ids: value.group_ids?,
                label: value.label?,
                product_ids: value.product_ids?,
            })
        }
    }
    impl From<super::Flag> for Flag {
        fn from(value: super::Flag) -> Self {
            Self {
                date: Ok(value.date),
                group_ids: Ok(value.group_ids),
                label: Ok(value.label),
                product_ids: Ok(value.product_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FullProductNameT {
        name: Result<super::TextualDescriptionOfTheProduct, String>,
        product_id: Result<super::ProductIdT, String>,
        product_identification_helper: Result<Option<super::HelperToIdentifyTheProduct>, String>,
    }
    impl Default for FullProductNameT {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                product_id: Err("no value supplied for product_id".to_string()),
                product_identification_helper: Ok(Default::default()),
            }
        }
    }
    impl FullProductNameT {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::TextualDescriptionOfTheProduct>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn product_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ProductIdT>,
            T::Error: std::fmt::Display,
        {
            self.product_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for product_id: {}", e));
            self
        }
        pub fn product_identification_helper<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::HelperToIdentifyTheProduct>>,
            T::Error: std::fmt::Display,
        {
            self.product_identification_helper = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for product_identification_helper: {}",
                    e
                )
            });
            self
        }
    }
    impl std::convert::TryFrom<FullProductNameT> for super::FullProductNameT {
        type Error = super::error::ConversionError;
        fn try_from(value: FullProductNameT) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                product_id: value.product_id?,
                product_identification_helper: value.product_identification_helper?,
            })
        }
    }
    impl From<super::FullProductNameT> for FullProductNameT {
        fn from(value: super::FullProductNameT) -> Self {
            Self {
                name: Ok(value.name),
                product_id: Ok(value.product_id),
                product_identification_helper: Ok(value.product_identification_helper),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GenericUri {
        namespace: Result<String, String>,
        uri: Result<String, String>,
    }
    impl Default for GenericUri {
        fn default() -> Self {
            Self {
                namespace: Err("no value supplied for namespace".to_string()),
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl GenericUri {
        pub fn namespace<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.namespace = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for namespace: {}", e));
            self
        }
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<GenericUri> for super::GenericUri {
        type Error = super::error::ConversionError;
        fn try_from(value: GenericUri) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                namespace: value.namespace?,
                uri: value.uri?,
            })
        }
    }
    impl From<super::GenericUri> for GenericUri {
        fn from(value: super::GenericUri) -> Self {
            Self {
                namespace: Ok(value.namespace),
                uri: Ok(value.uri),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct HelperToIdentifyTheProduct {
        cpe: Result<Option<super::CommonPlatformEnumerationRepresentation>, String>,
        hashes: Result<Vec<super::CryptographicHashes>, String>,
        model_numbers: Result<Option<Vec<super::ModelNumber>>, String>,
        purl: Result<Option<String>, String>,
        sbom_urls: Result<Vec<String>, String>,
        serial_numbers: Result<Option<Vec<super::SerialNumber>>, String>,
        skus: Result<Vec<super::StockKeepingUnit>, String>,
        x_generic_uris: Result<Vec<super::GenericUri>, String>,
    }
    impl Default for HelperToIdentifyTheProduct {
        fn default() -> Self {
            Self {
                cpe: Ok(Default::default()),
                hashes: Ok(Default::default()),
                model_numbers: Ok(Default::default()),
                purl: Ok(Default::default()),
                sbom_urls: Ok(Default::default()),
                serial_numbers: Ok(Default::default()),
                skus: Ok(Default::default()),
                x_generic_uris: Ok(Default::default()),
            }
        }
    }
    impl HelperToIdentifyTheProduct {
        pub fn cpe<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CommonPlatformEnumerationRepresentation>>,
            T::Error: std::fmt::Display,
        {
            self.cpe = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cpe: {}", e));
            self
        }
        pub fn hashes<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::CryptographicHashes>>,
            T::Error: std::fmt::Display,
        {
            self.hashes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hashes: {}", e));
            self
        }
        pub fn model_numbers<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Vec<super::ModelNumber>>>,
            T::Error: std::fmt::Display,
        {
            self.model_numbers = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model_numbers: {}", e));
            self
        }
        pub fn purl<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.purl = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for purl: {}", e));
            self
        }
        pub fn sbom_urls<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.sbom_urls = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sbom_urls: {}", e));
            self
        }
        pub fn serial_numbers<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Vec<super::SerialNumber>>>,
            T::Error: std::fmt::Display,
        {
            self.serial_numbers = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for serial_numbers: {}", e));
            self
        }
        pub fn skus<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::StockKeepingUnit>>,
            T::Error: std::fmt::Display,
        {
            self.skus = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skus: {}", e));
            self
        }
        pub fn x_generic_uris<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::GenericUri>>,
            T::Error: std::fmt::Display,
        {
            self.x_generic_uris = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x_generic_uris: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<HelperToIdentifyTheProduct> for super::HelperToIdentifyTheProduct {
        type Error = super::error::ConversionError;
        fn try_from(
            value: HelperToIdentifyTheProduct,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                cpe: value.cpe?,
                hashes: value.hashes?,
                model_numbers: value.model_numbers?,
                purl: value.purl?,
                sbom_urls: value.sbom_urls?,
                serial_numbers: value.serial_numbers?,
                skus: value.skus?,
                x_generic_uris: value.x_generic_uris?,
            })
        }
    }
    impl From<super::HelperToIdentifyTheProduct> for HelperToIdentifyTheProduct {
        fn from(value: super::HelperToIdentifyTheProduct) -> Self {
            Self {
                cpe: Ok(value.cpe),
                hashes: Ok(value.hashes),
                model_numbers: Ok(value.model_numbers),
                purl: Ok(value.purl),
                sbom_urls: Ok(value.sbom_urls),
                serial_numbers: Ok(value.serial_numbers),
                skus: Ok(value.skus),
                x_generic_uris: Ok(value.x_generic_uris),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Id {
        system_name: Result<super::SystemName, String>,
        text: Result<super::Text, String>,
    }
    impl Default for Id {
        fn default() -> Self {
            Self {
                system_name: Err("no value supplied for system_name".to_string()),
                text: Err("no value supplied for text".to_string()),
            }
        }
    }
    impl Id {
        pub fn system_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::SystemName>,
            T::Error: std::fmt::Display,
        {
            self.system_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for system_name: {}", e));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Text>,
            T::Error: std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Id> for super::Id {
        type Error = super::error::ConversionError;
        fn try_from(value: Id) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_name: value.system_name?,
                text: value.text?,
            })
        }
    }
    impl From<super::Id> for Id {
        fn from(value: super::Id) -> Self {
            Self {
                system_name: Ok(value.system_name),
                text: Ok(value.text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Involvement {
        date: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        party: Result<super::PartyCategory, String>,
        status: Result<super::PartyStatus, String>,
        summary: Result<Option<super::SummaryOfTheInvolvement>, String>,
    }
    impl Default for Involvement {
        fn default() -> Self {
            Self {
                date: Ok(Default::default()),
                party: Err("no value supplied for party".to_string()),
                status: Err("no value supplied for status".to_string()),
                summary: Ok(Default::default()),
            }
        }
    }
    impl Involvement {
        pub fn date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
            T::Error: std::fmt::Display,
        {
            self.date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date: {}", e));
            self
        }
        pub fn party<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::PartyCategory>,
            T::Error: std::fmt::Display,
        {
            self.party = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for party: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::PartyStatus>,
            T::Error: std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn summary<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SummaryOfTheInvolvement>>,
            T::Error: std::fmt::Display,
        {
            self.summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summary: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Involvement> for super::Involvement {
        type Error = super::error::ConversionError;
        fn try_from(value: Involvement) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                date: value.date?,
                party: value.party?,
                status: value.status?,
                summary: value.summary?,
            })
        }
    }
    impl From<super::Involvement> for Involvement {
        fn from(value: super::Involvement) -> Self {
            Self {
                date: Ok(value.date),
                party: Ok(value.party),
                status: Ok(value.status),
                summary: Ok(value.summary),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Note {
        audience: Result<Option<super::AudienceOfNote>, String>,
        category: Result<super::NoteCategory, String>,
        text: Result<super::NoteContent, String>,
        title: Result<Option<super::TitleOfNote>, String>,
    }
    impl Default for Note {
        fn default() -> Self {
            Self {
                audience: Ok(Default::default()),
                category: Err("no value supplied for category".to_string()),
                text: Err("no value supplied for text".to_string()),
                title: Ok(Default::default()),
            }
        }
    }
    impl Note {
        pub fn audience<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AudienceOfNote>>,
            T::Error: std::fmt::Display,
        {
            self.audience = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for audience: {}", e));
            self
        }
        pub fn category<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::NoteCategory>,
            T::Error: std::fmt::Display,
        {
            self.category = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for category: {}", e));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::NoteContent>,
            T::Error: std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::TitleOfNote>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Note> for super::Note {
        type Error = super::error::ConversionError;
        fn try_from(value: Note) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                audience: value.audience?,
                category: value.category?,
                text: value.text?,
                title: value.title?,
            })
        }
    }
    impl From<super::Note> for Note {
        fn from(value: super::Note) -> Self {
            Self {
                audience: Ok(value.audience),
                category: Ok(value.category),
                text: Ok(value.text),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProductGroup {
        group_id: Result<super::ProductGroupIdT, String>,
        product_ids: Result<Vec<super::ProductIdT>, String>,
        summary: Result<Option<super::SummaryOfTheProductGroup>, String>,
    }
    impl Default for ProductGroup {
        fn default() -> Self {
            Self {
                group_id: Err("no value supplied for group_id".to_string()),
                product_ids: Err("no value supplied for product_ids".to_string()),
                summary: Ok(Default::default()),
            }
        }
    }
    impl ProductGroup {
        pub fn group_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ProductGroupIdT>,
            T::Error: std::fmt::Display,
        {
            self.group_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_id: {}", e));
            self
        }
        pub fn product_ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::ProductIdT>>,
            T::Error: std::fmt::Display,
        {
            self.product_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for product_ids: {}", e));
            self
        }
        pub fn summary<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SummaryOfTheProductGroup>>,
            T::Error: std::fmt::Display,
        {
            self.summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summary: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ProductGroup> for super::ProductGroup {
        type Error = super::error::ConversionError;
        fn try_from(value: ProductGroup) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                group_id: value.group_id?,
                product_ids: value.product_ids?,
                summary: value.summary?,
            })
        }
    }
    impl From<super::ProductGroup> for ProductGroup {
        fn from(value: super::ProductGroup) -> Self {
            Self {
                group_id: Ok(value.group_id),
                product_ids: Ok(value.product_ids),
                summary: Ok(value.summary),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProductStatus {
        first_affected: Result<Option<super::ProductsT>, String>,
        first_fixed: Result<Option<super::ProductsT>, String>,
        fixed: Result<Option<super::ProductsT>, String>,
        known_affected: Result<Option<super::ProductsT>, String>,
        known_not_affected: Result<Option<super::ProductsT>, String>,
        last_affected: Result<Option<super::ProductsT>, String>,
        recommended: Result<Option<super::ProductsT>, String>,
        under_investigation: Result<Option<super::ProductsT>, String>,
    }
    impl Default for ProductStatus {
        fn default() -> Self {
            Self {
                first_affected: Ok(Default::default()),
                first_fixed: Ok(Default::default()),
                fixed: Ok(Default::default()),
                known_affected: Ok(Default::default()),
                known_not_affected: Ok(Default::default()),
                last_affected: Ok(Default::default()),
                recommended: Ok(Default::default()),
                under_investigation: Ok(Default::default()),
            }
        }
    }
    impl ProductStatus {
        pub fn first_affected<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProductsT>>,
            T::Error: std::fmt::Display,
        {
            self.first_affected = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for first_affected: {}", e));
            self
        }
        pub fn first_fixed<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProductsT>>,
            T::Error: std::fmt::Display,
        {
            self.first_fixed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for first_fixed: {}", e));
            self
        }
        pub fn fixed<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProductsT>>,
            T::Error: std::fmt::Display,
        {
            self.fixed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fixed: {}", e));
            self
        }
        pub fn known_affected<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProductsT>>,
            T::Error: std::fmt::Display,
        {
            self.known_affected = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for known_affected: {}", e));
            self
        }
        pub fn known_not_affected<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProductsT>>,
            T::Error: std::fmt::Display,
        {
            self.known_not_affected = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for known_not_affected: {}",
                    e
                )
            });
            self
        }
        pub fn last_affected<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProductsT>>,
            T::Error: std::fmt::Display,
        {
            self.last_affected = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_affected: {}", e));
            self
        }
        pub fn recommended<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProductsT>>,
            T::Error: std::fmt::Display,
        {
            self.recommended = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recommended: {}", e));
            self
        }
        pub fn under_investigation<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProductsT>>,
            T::Error: std::fmt::Display,
        {
            self.under_investigation = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for under_investigation: {}",
                    e
                )
            });
            self
        }
    }
    impl std::convert::TryFrom<ProductStatus> for super::ProductStatus {
        type Error = super::error::ConversionError;
        fn try_from(value: ProductStatus) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                first_affected: value.first_affected?,
                first_fixed: value.first_fixed?,
                fixed: value.fixed?,
                known_affected: value.known_affected?,
                known_not_affected: value.known_not_affected?,
                last_affected: value.last_affected?,
                recommended: value.recommended?,
                under_investigation: value.under_investigation?,
            })
        }
    }
    impl From<super::ProductStatus> for ProductStatus {
        fn from(value: super::ProductStatus) -> Self {
            Self {
                first_affected: Ok(value.first_affected),
                first_fixed: Ok(value.first_fixed),
                fixed: Ok(value.fixed),
                known_affected: Ok(value.known_affected),
                known_not_affected: Ok(value.known_not_affected),
                last_affected: Ok(value.last_affected),
                recommended: Ok(value.recommended),
                under_investigation: Ok(value.under_investigation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProductTree {
        branches: Result<Option<super::BranchesT>, String>,
        full_product_names: Result<Vec<super::FullProductNameT>, String>,
        product_groups: Result<Vec<super::ProductGroup>, String>,
        relationships: Result<Vec<super::Relationship>, String>,
    }
    impl Default for ProductTree {
        fn default() -> Self {
            Self {
                branches: Ok(Default::default()),
                full_product_names: Ok(Default::default()),
                product_groups: Ok(Default::default()),
                relationships: Ok(Default::default()),
            }
        }
    }
    impl ProductTree {
        pub fn branches<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::BranchesT>>,
            T::Error: std::fmt::Display,
        {
            self.branches = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for branches: {}", e));
            self
        }
        pub fn full_product_names<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::FullProductNameT>>,
            T::Error: std::fmt::Display,
        {
            self.full_product_names = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for full_product_names: {}",
                    e
                )
            });
            self
        }
        pub fn product_groups<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::ProductGroup>>,
            T::Error: std::fmt::Display,
        {
            self.product_groups = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for product_groups: {}", e));
            self
        }
        pub fn relationships<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Relationship>>,
            T::Error: std::fmt::Display,
        {
            self.relationships = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for relationships: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ProductTree> for super::ProductTree {
        type Error = super::error::ConversionError;
        fn try_from(value: ProductTree) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                branches: value.branches?,
                full_product_names: value.full_product_names?,
                product_groups: value.product_groups?,
                relationships: value.relationships?,
            })
        }
    }
    impl From<super::ProductTree> for ProductTree {
        fn from(value: super::ProductTree) -> Self {
            Self {
                branches: Ok(value.branches),
                full_product_names: Ok(value.full_product_names),
                product_groups: Ok(value.product_groups),
                relationships: Ok(value.relationships),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Publisher {
        category: Result<super::CategoryOfPublisher, String>,
        contact_details: Result<Option<super::ContactDetails>, String>,
        issuing_authority: Result<Option<super::IssuingAuthority>, String>,
        name: Result<super::NameOfPublisher, String>,
        namespace: Result<String, String>,
    }
    impl Default for Publisher {
        fn default() -> Self {
            Self {
                category: Err("no value supplied for category".to_string()),
                contact_details: Ok(Default::default()),
                issuing_authority: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                namespace: Err("no value supplied for namespace".to_string()),
            }
        }
    }
    impl Publisher {
        pub fn category<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::CategoryOfPublisher>,
            T::Error: std::fmt::Display,
        {
            self.category = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for category: {}", e));
            self
        }
        pub fn contact_details<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ContactDetails>>,
            T::Error: std::fmt::Display,
        {
            self.contact_details = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for contact_details: {}", e));
            self
        }
        pub fn issuing_authority<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IssuingAuthority>>,
            T::Error: std::fmt::Display,
        {
            self.issuing_authority = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for issuing_authority: {}",
                    e
                )
            });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::NameOfPublisher>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn namespace<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.namespace = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for namespace: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Publisher> for super::Publisher {
        type Error = super::error::ConversionError;
        fn try_from(value: Publisher) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                category: value.category?,
                contact_details: value.contact_details?,
                issuing_authority: value.issuing_authority?,
                name: value.name?,
                namespace: value.namespace?,
            })
        }
    }
    impl From<super::Publisher> for Publisher {
        fn from(value: super::Publisher) -> Self {
            Self {
                category: Ok(value.category),
                contact_details: Ok(value.contact_details),
                issuing_authority: Ok(value.issuing_authority),
                name: Ok(value.name),
                namespace: Ok(value.namespace),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Reference {
        category: Result<super::CategoryOfReference, String>,
        summary: Result<super::SummaryOfTheReference, String>,
        url: Result<String, String>,
    }
    impl Default for Reference {
        fn default() -> Self {
            Self {
                category: Ok(super::defaults::reference_category()),
                summary: Err("no value supplied for summary".to_string()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl Reference {
        pub fn category<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::CategoryOfReference>,
            T::Error: std::fmt::Display,
        {
            self.category = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for category: {}", e));
            self
        }
        pub fn summary<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::SummaryOfTheReference>,
            T::Error: std::fmt::Display,
        {
            self.summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summary: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Reference> for super::Reference {
        type Error = super::error::ConversionError;
        fn try_from(value: Reference) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                category: value.category?,
                summary: value.summary?,
                url: value.url?,
            })
        }
    }
    impl From<super::Reference> for Reference {
        fn from(value: super::Reference) -> Self {
            Self {
                category: Ok(value.category),
                summary: Ok(value.summary),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Relationship {
        category: Result<super::RelationshipCategory, String>,
        full_product_name: Result<super::FullProductNameT, String>,
        product_reference: Result<super::ProductIdT, String>,
        relates_to_product_reference: Result<super::ProductIdT, String>,
    }
    impl Default for Relationship {
        fn default() -> Self {
            Self {
                category: Err("no value supplied for category".to_string()),
                full_product_name: Err("no value supplied for full_product_name".to_string()),
                product_reference: Err("no value supplied for product_reference".to_string()),
                relates_to_product_reference: Err(
                    "no value supplied for relates_to_product_reference".to_string(),
                ),
            }
        }
    }
    impl Relationship {
        pub fn category<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::RelationshipCategory>,
            T::Error: std::fmt::Display,
        {
            self.category = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for category: {}", e));
            self
        }
        pub fn full_product_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::FullProductNameT>,
            T::Error: std::fmt::Display,
        {
            self.full_product_name = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for full_product_name: {}",
                    e
                )
            });
            self
        }
        pub fn product_reference<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ProductIdT>,
            T::Error: std::fmt::Display,
        {
            self.product_reference = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for product_reference: {}",
                    e
                )
            });
            self
        }
        pub fn relates_to_product_reference<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ProductIdT>,
            T::Error: std::fmt::Display,
        {
            self.relates_to_product_reference = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for relates_to_product_reference: {}",
                    e
                )
            });
            self
        }
    }
    impl std::convert::TryFrom<Relationship> for super::Relationship {
        type Error = super::error::ConversionError;
        fn try_from(value: Relationship) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                category: value.category?,
                full_product_name: value.full_product_name?,
                product_reference: value.product_reference?,
                relates_to_product_reference: value.relates_to_product_reference?,
            })
        }
    }
    impl From<super::Relationship> for Relationship {
        fn from(value: super::Relationship) -> Self {
            Self {
                category: Ok(value.category),
                full_product_name: Ok(value.full_product_name),
                product_reference: Ok(value.product_reference),
                relates_to_product_reference: Ok(value.relates_to_product_reference),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Remediation {
        category: Result<super::CategoryOfTheRemediation, String>,
        date: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        details: Result<super::DetailsOfTheRemediation, String>,
        entitlements: Result<Vec<super::EntitlementOfTheRemediation>, String>,
        group_ids: Result<Option<super::ProductGroupsT>, String>,
        product_ids: Result<Option<super::ProductsT>, String>,
        restart_required: Result<Option<super::RestartRequiredByRemediation>, String>,
        url: Result<Option<String>, String>,
    }
    impl Default for Remediation {
        fn default() -> Self {
            Self {
                category: Err("no value supplied for category".to_string()),
                date: Ok(Default::default()),
                details: Err("no value supplied for details".to_string()),
                entitlements: Ok(Default::default()),
                group_ids: Ok(Default::default()),
                product_ids: Ok(Default::default()),
                restart_required: Ok(Default::default()),
                url: Ok(Default::default()),
            }
        }
    }
    impl Remediation {
        pub fn category<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::CategoryOfTheRemediation>,
            T::Error: std::fmt::Display,
        {
            self.category = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for category: {}", e));
            self
        }
        pub fn date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
            T::Error: std::fmt::Display,
        {
            self.date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date: {}", e));
            self
        }
        pub fn details<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DetailsOfTheRemediation>,
            T::Error: std::fmt::Display,
        {
            self.details = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for details: {}", e));
            self
        }
        pub fn entitlements<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::EntitlementOfTheRemediation>>,
            T::Error: std::fmt::Display,
        {
            self.entitlements = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for entitlements: {}", e));
            self
        }
        pub fn group_ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProductGroupsT>>,
            T::Error: std::fmt::Display,
        {
            self.group_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_ids: {}", e));
            self
        }
        pub fn product_ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProductsT>>,
            T::Error: std::fmt::Display,
        {
            self.product_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for product_ids: {}", e));
            self
        }
        pub fn restart_required<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::RestartRequiredByRemediation>>,
            T::Error: std::fmt::Display,
        {
            self.restart_required = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for restart_required: {}",
                    e
                )
            });
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Remediation> for super::Remediation {
        type Error = super::error::ConversionError;
        fn try_from(value: Remediation) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                category: value.category?,
                date: value.date?,
                details: value.details?,
                entitlements: value.entitlements?,
                group_ids: value.group_ids?,
                product_ids: value.product_ids?,
                restart_required: value.restart_required?,
                url: value.url?,
            })
        }
    }
    impl From<super::Remediation> for Remediation {
        fn from(value: super::Remediation) -> Self {
            Self {
                category: Ok(value.category),
                date: Ok(value.date),
                details: Ok(value.details),
                entitlements: Ok(value.entitlements),
                group_ids: Ok(value.group_ids),
                product_ids: Ok(value.product_ids),
                restart_required: Ok(value.restart_required),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RestartRequiredByRemediation {
        category: Result<super::CategoryOfRestart, String>,
        details: Result<Option<super::AdditionalRestartInformation>, String>,
    }
    impl Default for RestartRequiredByRemediation {
        fn default() -> Self {
            Self {
                category: Err("no value supplied for category".to_string()),
                details: Ok(Default::default()),
            }
        }
    }
    impl RestartRequiredByRemediation {
        pub fn category<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::CategoryOfRestart>,
            T::Error: std::fmt::Display,
        {
            self.category = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for category: {}", e));
            self
        }
        pub fn details<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AdditionalRestartInformation>>,
            T::Error: std::fmt::Display,
        {
            self.details = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for details: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<RestartRequiredByRemediation> for super::RestartRequiredByRemediation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RestartRequiredByRemediation,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                category: value.category?,
                details: value.details?,
            })
        }
    }
    impl From<super::RestartRequiredByRemediation> for RestartRequiredByRemediation {
        fn from(value: super::RestartRequiredByRemediation) -> Self {
            Self {
                category: Ok(value.category),
                details: Ok(value.details),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Revision {
        date: Result<chrono::DateTime<chrono::offset::Utc>, String>,
        legacy_version: Result<Option<super::LegacyVersionOfTheRevision>, String>,
        number: Result<super::VersionT, String>,
        summary: Result<super::SummaryOfTheRevision, String>,
    }
    impl Default for Revision {
        fn default() -> Self {
            Self {
                date: Err("no value supplied for date".to_string()),
                legacy_version: Ok(Default::default()),
                number: Err("no value supplied for number".to_string()),
                summary: Err("no value supplied for summary".to_string()),
            }
        }
    }
    impl Revision {
        pub fn date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
            T::Error: std::fmt::Display,
        {
            self.date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date: {}", e));
            self
        }
        pub fn legacy_version<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LegacyVersionOfTheRevision>>,
            T::Error: std::fmt::Display,
        {
            self.legacy_version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for legacy_version: {}", e));
            self
        }
        pub fn number<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::VersionT>,
            T::Error: std::fmt::Display,
        {
            self.number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for number: {}", e));
            self
        }
        pub fn summary<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::SummaryOfTheRevision>,
            T::Error: std::fmt::Display,
        {
            self.summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summary: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Revision> for super::Revision {
        type Error = super::error::ConversionError;
        fn try_from(value: Revision) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                date: value.date?,
                legacy_version: value.legacy_version?,
                number: value.number?,
                summary: value.summary?,
            })
        }
    }
    impl From<super::Revision> for Revision {
        fn from(value: super::Revision) -> Self {
            Self {
                date: Ok(value.date),
                legacy_version: Ok(value.legacy_version),
                number: Ok(value.number),
                summary: Ok(value.summary),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RulesForSharingDocument {
        text: Result<Option<super::TextualDescription>, String>,
        tlp: Result<Option<super::TrafficLightProtocolTlp>, String>,
    }
    impl Default for RulesForSharingDocument {
        fn default() -> Self {
            Self {
                text: Ok(Default::default()),
                tlp: Ok(Default::default()),
            }
        }
    }
    impl RulesForSharingDocument {
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::TextualDescription>>,
            T::Error: std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
        pub fn tlp<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::TrafficLightProtocolTlp>>,
            T::Error: std::fmt::Display,
        {
            self.tlp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tlp: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<RulesForSharingDocument> for super::RulesForSharingDocument {
        type Error = super::error::ConversionError;
        fn try_from(value: RulesForSharingDocument) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                text: value.text?,
                tlp: value.tlp?,
            })
        }
    }
    impl From<super::RulesForSharingDocument> for RulesForSharingDocument {
        fn from(value: super::RulesForSharingDocument) -> Self {
            Self {
                text: Ok(value.text),
                tlp: Ok(value.tlp),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Score {
        cvss_v2: Result<::serde_json::Map<String, ::serde_json::Value>, String>,
        cvss_v3: Result<::serde_json::Map<String, ::serde_json::Value>, String>,
        products: Result<super::ProductsT, String>,
    }
    impl Default for Score {
        fn default() -> Self {
            Self {
                cvss_v2: Ok(Default::default()),
                cvss_v3: Ok(Default::default()),
                products: Err("no value supplied for products".to_string()),
            }
        }
    }
    impl Score {
        pub fn cvss_v2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<::serde_json::Map<String, ::serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.cvss_v2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cvss_v2: {}", e));
            self
        }
        pub fn cvss_v3<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<::serde_json::Map<String, ::serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.cvss_v3 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cvss_v3: {}", e));
            self
        }
        pub fn products<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ProductsT>,
            T::Error: std::fmt::Display,
        {
            self.products = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for products: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Score> for super::Score {
        type Error = super::error::ConversionError;
        fn try_from(value: Score) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                cvss_v2: value.cvss_v2?,
                cvss_v3: value.cvss_v3?,
                products: value.products?,
            })
        }
    }
    impl From<super::Score> for Score {
        fn from(value: super::Score) -> Self {
            Self {
                cvss_v2: Ok(value.cvss_v2),
                cvss_v3: Ok(value.cvss_v3),
                products: Ok(value.products),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Threat {
        category: Result<super::CategoryOfTheThreat, String>,
        date: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        details: Result<super::DetailsOfTheThreat, String>,
        group_ids: Result<Option<super::ProductGroupsT>, String>,
        product_ids: Result<Option<super::ProductsT>, String>,
    }
    impl Default for Threat {
        fn default() -> Self {
            Self {
                category: Err("no value supplied for category".to_string()),
                date: Ok(Default::default()),
                details: Err("no value supplied for details".to_string()),
                group_ids: Ok(Default::default()),
                product_ids: Ok(Default::default()),
            }
        }
    }
    impl Threat {
        pub fn category<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::CategoryOfTheThreat>,
            T::Error: std::fmt::Display,
        {
            self.category = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for category: {}", e));
            self
        }
        pub fn date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
            T::Error: std::fmt::Display,
        {
            self.date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date: {}", e));
            self
        }
        pub fn details<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DetailsOfTheThreat>,
            T::Error: std::fmt::Display,
        {
            self.details = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for details: {}", e));
            self
        }
        pub fn group_ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProductGroupsT>>,
            T::Error: std::fmt::Display,
        {
            self.group_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_ids: {}", e));
            self
        }
        pub fn product_ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProductsT>>,
            T::Error: std::fmt::Display,
        {
            self.product_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for product_ids: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Threat> for super::Threat {
        type Error = super::error::ConversionError;
        fn try_from(value: Threat) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                category: value.category?,
                date: value.date?,
                details: value.details?,
                group_ids: value.group_ids?,
                product_ids: value.product_ids?,
            })
        }
    }
    impl From<super::Threat> for Threat {
        fn from(value: super::Threat) -> Self {
            Self {
                category: Ok(value.category),
                date: Ok(value.date),
                details: Ok(value.details),
                group_ids: Ok(value.group_ids),
                product_ids: Ok(value.product_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Tracking {
        aliases: Result<Option<Vec<super::AlternateName>>, String>,
        current_release_date: Result<chrono::DateTime<chrono::offset::Utc>, String>,
        generator: Result<Option<super::DocumentGenerator>, String>,
        id: Result<super::UniqueIdentifierForTheDocument, String>,
        initial_release_date: Result<chrono::DateTime<chrono::offset::Utc>, String>,
        revision_history: Result<Vec<super::Revision>, String>,
        status: Result<super::DocumentStatus, String>,
        version: Result<super::VersionT, String>,
    }
    impl Default for Tracking {
        fn default() -> Self {
            Self {
                aliases: Ok(Default::default()),
                current_release_date: Err("no value supplied for current_release_date".to_string()),
                generator: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                initial_release_date: Err("no value supplied for initial_release_date".to_string()),
                revision_history: Err("no value supplied for revision_history".to_string()),
                status: Err("no value supplied for status".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl Tracking {
        pub fn aliases<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Vec<super::AlternateName>>>,
            T::Error: std::fmt::Display,
        {
            self.aliases = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for aliases: {}", e));
            self
        }
        pub fn current_release_date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
            T::Error: std::fmt::Display,
        {
            self.current_release_date = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for current_release_date: {}",
                    e
                )
            });
            self
        }
        pub fn generator<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DocumentGenerator>>,
            T::Error: std::fmt::Display,
        {
            self.generator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for generator: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::UniqueIdentifierForTheDocument>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn initial_release_date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
            T::Error: std::fmt::Display,
        {
            self.initial_release_date = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for initial_release_date: {}",
                    e
                )
            });
            self
        }
        pub fn revision_history<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Revision>>,
            T::Error: std::fmt::Display,
        {
            self.revision_history = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for revision_history: {}",
                    e
                )
            });
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DocumentStatus>,
            T::Error: std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::VersionT>,
            T::Error: std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Tracking> for super::Tracking {
        type Error = super::error::ConversionError;
        fn try_from(value: Tracking) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                aliases: value.aliases?,
                current_release_date: value.current_release_date?,
                generator: value.generator?,
                id: value.id?,
                initial_release_date: value.initial_release_date?,
                revision_history: value.revision_history?,
                status: value.status?,
                version: value.version?,
            })
        }
    }
    impl From<super::Tracking> for Tracking {
        fn from(value: super::Tracking) -> Self {
            Self {
                aliases: Ok(value.aliases),
                current_release_date: Ok(value.current_release_date),
                generator: Ok(value.generator),
                id: Ok(value.id),
                initial_release_date: Ok(value.initial_release_date),
                revision_history: Ok(value.revision_history),
                status: Ok(value.status),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TrafficLightProtocolTlp {
        label: Result<super::LabelOfTlp, String>,
        url: Result<String, String>,
    }
    impl Default for TrafficLightProtocolTlp {
        fn default() -> Self {
            Self {
                label: Err("no value supplied for label".to_string()),
                url: Ok(super::defaults::traffic_light_protocol_tlp_url()),
            }
        }
    }
    impl TrafficLightProtocolTlp {
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::LabelOfTlp>,
            T::Error: std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<TrafficLightProtocolTlp> for super::TrafficLightProtocolTlp {
        type Error = super::error::ConversionError;
        fn try_from(value: TrafficLightProtocolTlp) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                label: value.label?,
                url: value.url?,
            })
        }
    }
    impl From<super::TrafficLightProtocolTlp> for TrafficLightProtocolTlp {
        fn from(value: super::TrafficLightProtocolTlp) -> Self {
            Self {
                label: Ok(value.label),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Vulnerability {
        acknowledgments: Result<Option<super::AcknowledgmentsT>, String>,
        cve: Result<Option<super::Cve>, String>,
        cwe: Result<Option<super::Cwe>, String>,
        discovery_date: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        flags: Result<Option<Vec<super::Flag>>, String>,
        ids: Result<Option<Vec<super::Id>>, String>,
        involvements: Result<Option<Vec<super::Involvement>>, String>,
        notes: Result<Option<super::NotesT>, String>,
        product_status: Result<Option<super::ProductStatus>, String>,
        references: Result<Option<super::ReferencesT>, String>,
        release_date: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        remediations: Result<Vec<super::Remediation>, String>,
        scores: Result<Vec<super::Score>, String>,
        threats: Result<Vec<super::Threat>, String>,
        title: Result<Option<super::Title>, String>,
    }
    impl Default for Vulnerability {
        fn default() -> Self {
            Self {
                acknowledgments: Ok(Default::default()),
                cve: Ok(Default::default()),
                cwe: Ok(Default::default()),
                discovery_date: Ok(Default::default()),
                flags: Ok(Default::default()),
                ids: Ok(Default::default()),
                involvements: Ok(Default::default()),
                notes: Ok(Default::default()),
                product_status: Ok(Default::default()),
                references: Ok(Default::default()),
                release_date: Ok(Default::default()),
                remediations: Ok(Default::default()),
                scores: Ok(Default::default()),
                threats: Ok(Default::default()),
                title: Ok(Default::default()),
            }
        }
    }
    impl Vulnerability {
        pub fn acknowledgments<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AcknowledgmentsT>>,
            T::Error: std::fmt::Display,
        {
            self.acknowledgments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for acknowledgments: {}", e));
            self
        }
        pub fn cve<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Cve>>,
            T::Error: std::fmt::Display,
        {
            self.cve = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cve: {}", e));
            self
        }
        pub fn cwe<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Cwe>>,
            T::Error: std::fmt::Display,
        {
            self.cwe = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cwe: {}", e));
            self
        }
        pub fn discovery_date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
            T::Error: std::fmt::Display,
        {
            self.discovery_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for discovery_date: {}", e));
            self
        }
        pub fn flags<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Vec<super::Flag>>>,
            T::Error: std::fmt::Display,
        {
            self.flags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for flags: {}", e));
            self
        }
        pub fn ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Vec<super::Id>>>,
            T::Error: std::fmt::Display,
        {
            self.ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ids: {}", e));
            self
        }
        pub fn involvements<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Vec<super::Involvement>>>,
            T::Error: std::fmt::Display,
        {
            self.involvements = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for involvements: {}", e));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::NotesT>>,
            T::Error: std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {}", e));
            self
        }
        pub fn product_status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProductStatus>>,
            T::Error: std::fmt::Display,
        {
            self.product_status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for product_status: {}", e));
            self
        }
        pub fn references<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ReferencesT>>,
            T::Error: std::fmt::Display,
        {
            self.references = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for references: {}", e));
            self
        }
        pub fn release_date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
            T::Error: std::fmt::Display,
        {
            self.release_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for release_date: {}", e));
            self
        }
        pub fn remediations<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Remediation>>,
            T::Error: std::fmt::Display,
        {
            self.remediations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for remediations: {}", e));
            self
        }
        pub fn scores<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Score>>,
            T::Error: std::fmt::Display,
        {
            self.scores = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scores: {}", e));
            self
        }
        pub fn threats<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Threat>>,
            T::Error: std::fmt::Display,
        {
            self.threats = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for threats: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Title>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Vulnerability> for super::Vulnerability {
        type Error = super::error::ConversionError;
        fn try_from(value: Vulnerability) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                acknowledgments: value.acknowledgments?,
                cve: value.cve?,
                cwe: value.cwe?,
                discovery_date: value.discovery_date?,
                flags: value.flags?,
                ids: value.ids?,
                involvements: value.involvements?,
                notes: value.notes?,
                product_status: value.product_status?,
                references: value.references?,
                release_date: value.release_date?,
                remediations: value.remediations?,
                scores: value.scores?,
                threats: value.threats?,
                title: value.title?,
            })
        }
    }
    impl From<super::Vulnerability> for Vulnerability {
        fn from(value: super::Vulnerability) -> Self {
            Self {
                acknowledgments: Ok(value.acknowledgments),
                cve: Ok(value.cve),
                cwe: Ok(value.cwe),
                discovery_date: Ok(value.discovery_date),
                flags: Ok(value.flags),
                ids: Ok(value.ids),
                involvements: Ok(value.involvements),
                notes: Ok(value.notes),
                product_status: Ok(value.product_status),
                references: Ok(value.references),
                release_date: Ok(value.release_date),
                remediations: Ok(value.remediations),
                scores: Ok(value.scores),
                threats: Ok(value.threats),
                title: Ok(value.title),
            }
        }
    }
}
/// Generation of default values for serde.
pub mod defaults {
    pub(super) fn reference_category() -> super::CategoryOfReference {
        super::CategoryOfReference::External
    }
    pub(super) fn traffic_light_protocol_tlp_url() -> String {
        "https://www.first.org/tlp/".to_string()
    }
}
