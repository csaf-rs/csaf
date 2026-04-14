use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::fs;
use std::path::Path;

use crate::{
    build_helper::BuildError,
    file_helper::{GENERATED_CODE_HEADER, add_ignore_clippy, add_ignore_rustfmt},
};

mod extract;
mod generate;

#[derive(Debug, Clone, Copy)]
pub enum CsafVersion {
    V2_0,
    V2_1,
}

pub(crate) enum TestGroup {
    Mandatory,
    OptionalRecommended,
    Informative,
}

/// Raw test entry extracted from JSON, before any code generation.
///
/// Contains the test id, the test group, and a vector containing the
/// valid and failing test cases.
pub(crate) struct RawTest {
    pub id: String,
    pub group: TestGroup,
    pub docs: Vec<RawTestCase>,
}

/// A raw test case entry extract from JSON. Contains the case number,
/// the name, and the base directory of its path.
pub(crate) struct RawTestCase {
    pub case_num: String,
    pub name: String,
    pub base_dir: String,
}

/// Aggregated result of the test case generation.
/// Contains collections of the mandatory, optional/recommended, informative tests,
/// the structure definitions
pub(crate) struct GeneratedTests {
    pub mandatory_tests: Vec<GeneratedTest>,
    pub optional_recommended_tests: Vec<GeneratedTest>,
    pub informative_tests: Vec<GeneratedTest>,
    pub test_struct_defs: Vec<TokenStream>,
}

/// A generated test instance with its associated struct and validator identifiers.
pub(crate) struct GeneratedTest {
    pub instance_ident: Ident,
    pub struct_ident: Ident,
    pub validator_ident: Ident,
}

/// Generates testcases module from testcases.json
pub fn generate_testcases(
    input: &str,
    supplemental_input: &str,
    output: &str,
    csaf_version: CsafVersion,
    target_path: &str,
) -> Result<(), BuildError> {
    println!("cargo:rerun-if-changed={input}");
    println!("cargo:rerun-if-changed={supplemental_input}");

    let content = fs::read_to_string(input)?;
    let supplemental_content = fs::read_to_string(supplemental_input)?;

    // Extract base directory from input path (directory containing testcases.json)
    let base_dir = Path::new(input)
        .parent()
        .expect("Failed to get parent directory of testcases.json")
        .to_str()
        .expect("Failed to convert path to string");
    let supplemental_base_dir = Path::new(supplemental_input)
        .parent()
        .expect("Failed to get parent directory of supplemental testcases.json")
        .to_str()
        .expect("Failed to convert path to string");

    // Determine CSAF document type and constant name from version parameter
    let (csaf_doc_type, tests_const_name) = match csaf_version {
        CsafVersion::V2_0 => (
            quote! { crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework },
            Ident::new("TESTS_2_0", Span::call_site()),
        ),
        CsafVersion::V2_1 => (
            quote! { crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework },
            Ident::new("TESTS_2_1", Span::call_site()),
        ),
    };

    // Parse testcases.json files as generic JSON Values
    let testcases: serde_json::Value = serde_json::from_str(&content)?;
    let supplemental_testcases: serde_json::Value = serde_json::from_str(&supplemental_content)?;

    // Extract raw test entries from JSON
    let raw_tests: Vec<RawTest> =
        extract::extract_test_entries_from_json(base_dir, supplemental_base_dir, &testcases, &supplemental_testcases);

    // Generate individual test struct definitions from the extracted entries
    let test_cases: GeneratedTests =
        generate::generate_test_cases_from_entries(csaf_doc_type, csaf_version, &raw_tests);

    // Generate group aggregations
    let group_defs = generate::generate_group_aggregation(&tests_const_name, &test_cases);

    // Write to file
    let test_struct_defs = &test_cases.test_struct_defs;
    let tokens = quote! {
        #![doc = #GENERATED_CODE_HEADER]

        #(#test_struct_defs)*

        #group_defs
    };

    let mut file: syn::File = syn::parse2(tokens)?;
    add_ignore_rustfmt(&mut file);
    add_ignore_clippy(&mut file);

    let code = prettyplease::unparse(&file);

    let out_path = Path::new(&target_path).join("src").join(output);
    fs::write(&out_path, code)?;

    Ok(())
}
