pub const RESULT_JSON_SCHEMA: &str =
    "https://raw.githubusercontent.com/oasis-tcs/csaf/master/csaf_2.1/test/validator/testresult_json_schema.json";

#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ResultJson {
    ///Contains the URL of the JSON schema for test result which the document promises to be valid for.
    #[serde(rename = "$schema")]
    pub schema: ::std::string::String,
    ///States whether the file passes all basic tests. This might differ from the result for the specific test.
    pub overall_valid: bool,
    ///Contains the expected result for this specific test. Results for any other tests may be added as secondary results.
    pub primary_result: ResultT,
    ///Contains the current version of this schema
    pub resultschema_version: ::std::string::String,
    ///Contains a list of expected result for other tests. It is not guaranteed to contain expected results for all other tests. Main purpose is to aid in understanding edge cases.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub secondary_results: ::std::option::Option<Vec<ResultT>>,
}

#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ResultT {
    ///Contains a list of errors.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub errors: ::std::option::Option<Vec<ValidationMessageT>>,
    ///Contains the section number of the test in the specification.
    pub id: String,
    ///Contains a list of information.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub infos: ::std::option::Option<Vec<ValidationMessageT>>,
    ///States whether the data passed this specific test.
    pub passed: bool,
    ///Contains a list of warnings.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub warnings: ::std::option::Option<Vec<ValidationMessageT>>,
}

#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ValidationMessageT {
    ///Contains a JSON pointer detailing the path to the instance that raised the issue.
    pub instance_path: ::std::string::String,
    ///Contains the message detailing what the issues is.
    pub message: ::std::string::String,
}
