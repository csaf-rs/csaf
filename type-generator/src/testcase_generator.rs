use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::fs;
use std::path::Path;
use std::string::ToString;

use crate::{
    build_helper::BuildError,
    file_helper::{GENERATED_CODE_HEADER, add_ignore_clippy, add_ignore_rustfmt},
};

#[derive(Debug, Clone, Copy)]
pub enum CsafVersion {
    V2_0,
    V2_1,
}

type TestCaseResult = (
    Vec<String>,
    Vec<String>,
    Vec<String>,
    Vec<TokenStream>,
    Vec<(Ident, Ident, Ident)>,
);

fn generate_test_cases_from_json(
    csaf_doc_type: TokenStream,
    base_dir: &str,
    testcases: &serde_json::Value,
) -> TestCaseResult {
    // Generate individual test struct definitions
    let mut test_struct_defs = Vec::new();
    let mut test_instances = Vec::new();
    let mut mandatory_tests = Vec::new();
    let mut optional_tests = Vec::new();
    let mut informative_tests = Vec::new();

    let tests = testcases["tests"]
        .as_array()
        .expect("testcases.json should have a 'tests' array");

    for test in tests {
        let id = test["id"].as_str().expect("test should have 'id' string");
        let group = test["group"].as_str().expect("test should have 'group' string");

        // Convert "6.1.1" to "Test6_1_1" (camel case to avoid warnings)
        let struct_name = format!("Test{}", id.replace('.', "_"));
        let instance_name = format!("test_{}", id.replace('.', "_"));
        let test_id = id.to_string();

        // Collect failure and valid test documents with their paths
        let mut failure_docs: Vec<(String, String)> = Vec::new(); // (case_num, path)
        let mut valid_docs: Vec<(String, String)> = Vec::new();

        if let Some(failures) = test["failures"].as_array() {
            for failure in failures {
                if let Some(name) = failure["name"].as_str() {
                    if let Some(case_num) = extract_test_case_number(name) {
                        failure_docs.push((case_num, name.to_string()));
                    }
                }
            }
        }

        if let Some(valid_cases) = test.get("valid").and_then(|v| v.as_array()) {
            for valid in valid_cases {
                if let Some(name) = valid["name"].as_str() {
                    if let Some(case_num) = extract_test_case_number(name) {
                        valid_docs.push((case_num, name.to_string()));
                    }
                }
            }
        }

        // Generate struct definition for this specific test
        let struct_ident = Ident::new(&struct_name, Span::call_site());
        let instance_ident = Ident::new(&instance_name, Span::call_site());

        // Generate parameter names and types for the run method
        let mut param_names = Vec::new();
        let mut param_types = Vec::new();

        // Combine all docs with their case numbers
        let all_docs: Vec<_> = failure_docs.iter().chain(valid_docs.iter()).collect();

        for (case_num, _) in &all_docs {
            param_names.push(Ident::new(&format!("case_{}", case_num), Span::call_site()));
            param_types.push(quote! { Result<(), Vec<crate::validation::ValidationError>> });
        }

        // Check if this test requires raw string content (not parsed JSON)
        let uses_raw_string = id == "6.2.13";

        // Generate tuples of (case_num, doc_expr, expected_param) - with or without raw string
        let test_cases: Vec<_> = all_docs
            .iter()
            .enumerate()
            .map(|(idx, (case_num, path))| {
                let full_path = format!("{}/{}", base_dir, path);
                let param_name = &param_names[idx];
                if uses_raw_string {
                    quote! {
                        (
                            #case_num,
                            {
                                let path = #full_path;
                                std::fs::read_to_string(path)
                                    .unwrap_or_else(|e| panic!("Failed to load {} (case {}): {}", #path, #case_num, e))
                            },
                            #param_name
                        )
                    }
                } else {
                    quote! {
                        (
                            #case_num,
                            {
                                let path = #full_path;
                                let content = std::fs::read_to_string(path)
                                    .unwrap_or_else(|e| panic!("Failed to load {} (case {}): {}", #path, #case_num, e));
                                serde_json::from_str::<#csaf_doc_type>(&content)
                                    .unwrap_or_else(|e| panic!("Failed to parse {} (case {}): {}", #path, #case_num, e))
                            },
                            #param_name
                        )
                    }
                }
            })
            .collect();

        // Generate validator struct name
        let validator_name = format!("ValidatorFor{}", struct_name);
        let validator_ident = Ident::new(&validator_name, Span::call_site());

        // Generate the struct definition using the shared TestValidator trait
        // For test 6.2.13, use TestValidatorWithRawString; for all others, use TestValidator
        let struct_def = if uses_raw_string {
            quote! {
                #[derive(Debug, Clone, Copy)]
                pub struct #struct_ident<V>(std::marker::PhantomData<V>);

                impl<V> #struct_ident<V> {
                    /// Test ID
                    pub const ID: &'static str = #test_id;

                    /// Create a new test instance
                    pub const fn new() -> Self {
                        Self(std::marker::PhantomData)
                    }

                    /// Get the test ID
                    pub fn id(&self) -> &'static str {
                        Self::ID
                    }
                }

                impl<V: crate::test_validation::TestValidatorWithRawString + Default> #struct_ident<V> {
                    /// Validate a CSAF document using this test's validator.
                    ///
                    /// # Arguments
                    /// * `raw` - The raw string content of the document
                    ///
                    /// # Returns
                    /// * `Ok(())` if validation passes
                    /// * `Err(Vec<ValidationError>)` if validation fails
                    pub fn validate(&self, raw: &str) -> Result<(), Vec<crate::validation::ValidationError>> {
                        let validator = V::default();
                        validator.validate(raw)
                    }

                    /// Run the test with expected results for each test case.
                    ///
                    /// The method automatically loads test documents from the file system and runs
                    /// the validation function on each one, comparing actual vs expected results.
                    ///
                    /// # Arguments
                    /// * One parameter per test case document with the expected result
                    ///
                    /// # Panics
                    /// Panics if any test case fails to load, parse, or doesn't match the expected result
                    ///
                    pub fn expect(
                        &self,
                        #(#param_names: #param_types),*
                    ) {
                        // Create test cases as tuples of (case_num, raw_content, expected)
                        let test_cases = vec![#(#test_cases),*];

                        // Create validator instance
                        let validator = V::default();

                        // Run test on each document and compare with expected result
                        for (case_num, raw, expected) in test_cases {
                            let actual = validator.validate(&raw);

                            // Use the extracted comparison function, panic on error
                            crate::test_result_comparison::compare_test_results(
                                &actual,
                                &expected,
                                Self::ID,
                                case_num
                            ).unwrap_or_else(|e| panic!("{}", e));
                        }
                    }
                }

                /// Validator for test case #test_id
                ///
                /// Implement `TestValidatorWithRawString` on this struct to provide validation logic.
                #[derive(Debug, Clone, Copy, Default)]
                pub struct #validator_ident;
            }
        } else {
            quote! {
                #[derive(Debug, Clone, Copy)]
                pub struct #struct_ident<V>(std::marker::PhantomData<V>);

                impl<V> #struct_ident<V> {
                    /// Test ID
                    pub const ID: &'static str = #test_id;

                    /// Create a new test instance
                    pub const fn new() -> Self {
                        Self(std::marker::PhantomData)
                    }

                    /// Get the test ID
                    pub fn id(&self) -> &'static str {
                        Self::ID
                    }
                }

                impl<V: crate::test_validation::TestValidator<#csaf_doc_type> + Default> #struct_ident<V> {
                    /// Validate a CSAF document using this test's validator.
                    ///
                    /// # Arguments
                    /// * `doc` - The CSAF document to validate
                    ///
                    /// # Returns
                    /// * `Ok(())` if validation passes
                    /// * `Err(Vec<ValidationError>)` if validation fails
                    pub fn validate(&self, doc: &#csaf_doc_type) -> Result<(), Vec<crate::validation::ValidationError>> {
                        let validator = V::default();
                        validator.validate(doc)
                    }

                    /// Run the test with expected results for each test case.
                    ///
                    /// The method automatically loads test documents from the file system and runs
                    /// the validation function on each one, comparing actual vs expected results.
                    ///
                    /// # Arguments
                    /// * One parameter per test case document with the expected result
                    ///
                    /// # Panics
                    /// Panics if any test case fails to load, parse, or doesn't match the expected result
                    ///
                    pub fn expect(
                        &self,
                        #(#param_names: #param_types),*
                    ) {
                        // Create test cases as tuples of (case_num, doc, expected)
                        let test_cases = vec![#(#test_cases),*];

                        // Create validator instance
                        let validator = V::default();

                        // Run test on each document and compare with expected result
                        for (case_num, doc, expected) in test_cases {
                            let actual = validator.validate(&doc);

                            // Use the extracted comparison function, panic on error
                            crate::test_result_comparison::compare_test_results(
                                &actual,
                                &expected,
                                Self::ID,
                                case_num
                            ).unwrap_or_else(|e| panic!("{}", e));
                        }
                    }
                }

                /// Validator for test case #test_id
                ///
                /// Implement `TestValidator<#csaf_doc_type>` on this struct to provide validation logic.
                #[derive(Debug, Clone, Copy, Default)]
                pub struct #validator_ident;
            }
        };

        test_struct_defs.push(struct_def);
        test_instances.push((instance_ident, struct_ident, validator_ident));

        match group {
            "mandatory" => mandatory_tests.push(instance_name.clone()),
            "optional" | "recommended" => optional_tests.push(instance_name.clone()),
            "informative" => informative_tests.push(instance_name.clone()),
            _ => {},
        }
    }

    (
        mandatory_tests,
        optional_tests,
        informative_tests,
        test_struct_defs,
        test_instances,
    )
}

/// Generates testcases module from testcases.json
pub fn generate_testcases(
    input: &str,
    output: &str,
    csaf_version: CsafVersion,
    target_path: &str,
) -> Result<(), BuildError> {
    println!("cargo:rerun-if-changed={}", input);

    let content = fs::read_to_string(input)?;

    // Extract base directory from input path (directory containing testcases.json)
    let base_dir = std::path::Path::new(input)
        .parent()
        .expect("Failed to get parent directory of testcases.json")
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

    // Parse testcases.json as generic JSON Value
    let testcases: serde_json::Value = serde_json::from_str(&content)?;

    // Generate individual test struct definitions
    let (mandatory_tests, recommended_tests, informative_tests, test_struct_defs, test_instances) =
        generate_test_cases_from_json(csaf_doc_type, base_dir, &testcases);

    // Generate field definitions for TESTS constant (each test as a field with typed validator)
    let field_defs = test_instances
        .iter()
        .map(|(instance_name, struct_name, validator_name)| {
            quote! {
                pub #instance_name: #struct_name<#validator_name>
            }
        });

    // Generate field initializers for TESTS constant (with typed instances)
    let field_inits = test_instances
        .iter()
        .map(|(instance_name, struct_name, _validator_name)| {
            quote! {
                #instance_name: #struct_name::new()
            }
        });

    // Generate preset methods - collect the vecs for reuse
    let mandatory_refs: Vec<_> = mandatory_tests
        .iter()
        .map(|name| {
            let name_ident = Ident::new(name, Span::call_site());
            quote! { &#tests_const_name.#name_ident }
        })
        .collect();

    let recommended_refs: Vec<_> = recommended_tests
        .iter()
        .map(|name| {
            let name_ident = Ident::new(name, Span::call_site());
            quote! { &#tests_const_name.#name_ident }
        })
        .collect();

    let informative_refs: Vec<_> = informative_tests
        .iter()
        .map(|name| {
            let name_ident = Ident::new(name, Span::call_site());
            quote! { &#tests_const_name.#name_ident }
        })
        .collect();

    let tokens = quote! {
        #![doc = #GENERATED_CODE_HEADER]

        // Generate individual test structs
        #(#test_struct_defs)*

        /// Collection of all available test cases
        #[derive(Debug, Clone, Copy)]
        pub struct TestCases {
            #(#field_defs),*
        }

        impl TestCases {
            /// Create a new TestCases instance with all tests
            pub const fn new() -> Self {
                Self {
                    #(#field_inits),*
                }
            }

            /// Get all mandatory test IDs (basic preset)
            pub fn basic(&self) -> Vec<&'static str> {
                vec![#(#mandatory_refs.id()),*]
            }

            /// Get mandatory + optional test IDs (extended preset)
            pub fn extended(&self) -> Vec<&'static str> {
                let mut tests = self.basic();
                tests.extend(self.recommended());
                tests
            }

            /// Get all test IDs (full preset)
            pub fn full(&self) -> Vec<&'static str> {
                let mut tests = self.extended();
                tests.extend(self.informative());
                tests
            }

            /// Get mandatory test IDs
            pub fn mandatory(&self) -> Vec<&'static str> {
                vec![#(#mandatory_refs.id()),*]
            }

            /// Get recommended test IDs
            pub fn recommended(&self) -> Vec<&'static str> {
                vec![#(#recommended_refs.id()),*]
            }

            /// Get informative test IDs
            pub fn informative(&self) -> Vec<&'static str> {
                vec![#(#informative_refs.id()),*]
            }
        }

        /// Global constant instance of all test cases
        pub const #tests_const_name: TestCases = TestCases::new();

        /// Get all mandatory tests as IDs
        pub fn mandatory_tests() -> Vec<&'static str> {
            vec![#(#mandatory_refs.id()),*]
        }

        /// Get all optional tests as IDs
        pub fn recommended_tests() -> Vec<&'static str> {
            vec![#(#recommended_refs.id()),*]
        }

        /// Get all informative tests as IDs
        pub fn informative_tests() -> Vec<&'static str> {
            vec![#(#informative_refs.id()),*]
        }
    };

    let mut file: syn::File = syn::parse2(tokens)?;
    add_ignore_rustfmt(&mut file);
    add_ignore_clippy(&mut file);

    let code = prettyplease::unparse(&file);

    let out_path = Path::new(&target_path).join("src").join(output);
    fs::write(&out_path, code)?;

    Ok(())
}

/// Extract test case number from filename like "oasis_csaf_tc-csaf_2_0-2021-6-1-08-01.json" -> "01"
fn extract_test_case_number(filename: &str) -> Option<String> {
    // Remove path prefix and .json suffix
    let name = filename.split('/').next_back().unwrap_or(filename);
    let name = name.strip_suffix(".json").unwrap_or(name);

    // Get the last component after the last dash
    name.split('-').next_back().map(|s| s.to_string())
}
