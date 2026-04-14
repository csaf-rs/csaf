/// Macro to generate the boilerplate for a single CSAF test case.
///
/// Generates:
/// - A generic `struct TestX_Y_Z<V>` with `ID`, `new()`, `id()`
/// - An impl block bounded on `TestValidator<RawDocument<...>>` with `validate()` and `expect()`
/// - A default validator marker struct `ValidatorForTestX_Y_Z`
///
/// This is exclusively created by the type-generator to be used in the
/// `csaf_(20/21)/testcases.generated.rs` files.
///
/// # Usage
/// ```ignore
/// define_csaf_test!(
///     Test6_1_1, ValidatorForTest6_1_1,
///     id: "6.1.1",
///     doc_type: crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
///     version: "V2_0",
///     cases: [
///         (case_01, "01", "../path/to/test-01.json", "mandatory/test-01.json"),
///         (case_11, "11", "../path/to/test-11.json", "mandatory/test-11.json"),
///     ]
/// );
/// ```
macro_rules! define_csaf_test {
    (
        $struct_name:ident, $validator_name:ident,
        id: $id:literal,
        doc_type: $doc_type:ty,
        version: $version:literal,
        cases: [$(($case_name:ident, $case_num:literal, $path:literal, $display:literal)),* $(,)?]
    ) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $struct_name<V>(std::marker::PhantomData<V>);

        impl<V> $struct_name<V> {
            /// Test ID
            pub const ID: &'static str = $id;

            /// Create a new test instance
            pub const fn new() -> Self {
                Self(std::marker::PhantomData)
            }

            /// Get the test ID
            pub fn id(&self) -> &'static str {
                Self::ID
            }
        }

        impl<
            V: crate::test_validation::TestValidator<
                    crate::csaf::raw::RawDocument<$doc_type>,
                > + Default,
        > $struct_name<V> {
            /// Validate a CSAF document using this test's validator.
            ///
            /// # Arguments
            /// * `doc` - The CSAF document to validate
            ///
            /// # Returns
            /// * `Ok(())` if validation passes
            /// * `Err(Vec<ValidationError>)` if validation fails
            pub fn validate(
                &self,
                doc: &crate::csaf::raw::RawDocument<$doc_type>,
            ) -> Result<(), Vec<crate::validation::ValidationError>> {
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
            pub fn expect(
                &self,
                $($case_name: Result<(), Vec<crate::validation::ValidationError>>),*
            ) {
                let test_cases = vec![
                    $({
                        let content = std::fs::read_to_string($path)
                            .unwrap_or_else(|e| panic!(
                                "Failed to load {} (case {}): {}",
                                $display, $case_num, e
                            ));
                        let json: serde_json::Value = serde_json::from_str(&content)
                            .unwrap_or_else(|e| panic!(
                                "Failed to parse {} (case {}): {}",
                                $display, $case_num, e
                            ));
                        let doc: crate::csaf::raw::RawDocument<$doc_type> =
                            crate::csaf::raw::RawDocument::new(json);
                        ($case_num, doc, $case_name)
                    }),*
                ];
                let validator = V::default();
                for (case_num, doc, expected) in test_cases {
                    let actual = validator.validate(&doc);
                    crate::test_result_comparison::compare_test_results(
                        &actual,
                        &expected,
                        $version,
                        Self::ID,
                        case_num,
                    )
                    .unwrap_or_else(|e| panic!("{}", e));
                }
            }
        }

        /// Validator for this test case.
        ///
        /// Implement `TestValidator` on this struct to provide validation logic.
        #[derive(Debug, Clone, Copy, Default)]
        pub struct $validator_name;
    };
}

pub(crate) use define_csaf_test;

/// Macro to generate the `TestCases` aggregate struct with category-based accessors.
///
/// Generates:
/// - `struct TestCases` with a field per test
/// - `impl TestCases` with `new()`, `basic()`, `extended()`, `full()`, `mandatory()`,
///   `recommended()`, `informative()`
/// - A global `const` instance
/// - Free functions `mandatory_tests()`, `recommended_tests()`, `informative_tests()`
///
/// # Usage
/// ```ignore
/// define_test_cases_aggregate!(
///     const_name: TESTS_2_0,
///     mandatory: [(test_6_1_1, Test6_1_1, ValidatorForTest6_1_1), ...],
///     recommended: [(test_6_2_1, Test6_2_1, ValidatorForTest6_2_1), ...],
///     informative: [(test_6_3_1, Test6_3_1, ValidatorForTest6_3_1), ...]
/// );
/// ```
macro_rules! define_test_cases_aggregate {
    (
        const_name: $const_name:ident,
        mandatory: [$(($m_inst:ident, $m_struct:ident, $m_validator:ident)),* $(,)?],
        recommended: [$(($r_inst:ident, $r_struct:ident, $r_validator:ident)),* $(,)?],
        informative: [$(($i_inst:ident, $i_struct:ident, $i_validator:ident)),* $(,)?]
    ) => {
        /// Collection of all available test cases
        #[derive(Debug, Clone, Copy)]
        pub struct TestCases {
            $(pub $m_inst: $m_struct<$m_validator>,)*
            $(pub $r_inst: $r_struct<$r_validator>,)*
            $(pub $i_inst: $i_struct<$i_validator>,)*
        }

        impl TestCases {
            /// Create a new TestCases instance with all tests
            pub const fn new() -> Self {
                Self {
                    $($m_inst: $m_struct::new(),)*
                    $($r_inst: $r_struct::new(),)*
                    $($i_inst: $i_struct::new(),)*
                }
            }

            /// Get all mandatory test IDs (basic preset)
            pub fn basic(&self) -> Vec<&'static str> {
                self.mandatory()
            }

            /// Get mandatory + recommended test IDs (extended preset)
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
                vec![$(self.$m_inst.id()),*]
            }

            /// Get recommended test IDs
            pub fn recommended(&self) -> Vec<&'static str> {
                vec![$(self.$r_inst.id()),*]
            }

            /// Get informative test IDs
            pub fn informative(&self) -> Vec<&'static str> {
                vec![$(self.$i_inst.id()),*]
            }
        }

        /// Global constant instance of all test cases
        pub const $const_name: TestCases = TestCases::new();

        /// Get all mandatory tests as IDs
        pub fn mandatory_tests() -> Vec<&'static str> {
            vec![$($const_name.$m_inst.id()),*]
        }

        /// Get all optional tests as IDs
        pub fn recommended_tests() -> Vec<&'static str> {
            vec![$($const_name.$r_inst.id()),*]
        }

        /// Get all informative tests as IDs
        pub fn informative_tests() -> Vec<&'static str> {
            vec![$($const_name.$i_inst.id()),*]
        }
    };
}

pub(crate) use define_test_cases_aggregate;
