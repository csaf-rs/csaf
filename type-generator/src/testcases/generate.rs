use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use super::{CsafVersion, GeneratedTest, GeneratedTests, RawTest, TestGroup};

/// Takes [RawTest]s and produces the `define_csaf_test!` macro invocations for each test, as well
/// as the grouping needed for [generate_group_aggregation].
pub(crate) fn generate_test_cases_from_entries(
    csaf_doc_type: TokenStream,
    csaf_version: CsafVersion,
    tests: &[RawTest],
) -> GeneratedTests {
    let mut test_struct_defs = Vec::new();
    let mut mandatory_tests = Vec::new();
    let mut optional_recommended_tests = Vec::new();
    let mut informative_tests = Vec::new();

    for test in tests {
        let case_entries: Vec<TokenStream> = test
            .docs
            .iter()
            .map(|c| {
                let param_name = Ident::new(&format!("case_{}", c.case_num), Span::call_site());
                let case_num = &c.case_num;
                let full_path = format!("{}/{}", c.base_dir, c.name);
                let display = &c.name;
                quote! { (#param_name, #case_num, #full_path, #display) }
            })
            .collect();

        // Generate idents
        let id_formatted = test.id.replace('.', "_");
        let struct_ident = Ident::new(&format!("Test{id_formatted}"), Span::call_site());
        let validator_ident = Ident::new(&format!("ValidatorForTest{id_formatted}"), Span::call_site());
        let instance_ident = Ident::new(&format!("test_{id_formatted}"), Span::call_site());
        let test_id = &test.id;

        let csaf_version = match csaf_version {
            CsafVersion::V2_0 => "V2_0",
            CsafVersion::V2_1 => "V2_1",
        };

        // push the struct def with the test cases
        test_struct_defs.push(quote! {
            crate::macros::define_csaf_test!(
                #struct_ident,
                #validator_ident,
                id: #test_id,
                doc_type: #csaf_doc_type,
                version: #csaf_version,
                cases: [#(#case_entries),*]
            );
        });

        let generated = GeneratedTest {
            instance_ident,
            struct_ident,
            validator_ident,
        };

        match test.group {
            TestGroup::Mandatory => mandatory_tests.push(generated),
            TestGroup::OptionalRecommended => optional_recommended_tests.push(generated),
            TestGroup::Informative => informative_tests.push(generated),
        }
    }

    GeneratedTests {
        mandatory_tests,
        optional_recommended_tests,
        informative_tests,
        test_struct_defs,
    }
}

/// Takes [GeneratedTests] and produces the `define_test_cases_aggregate!` macro invocation from
/// the grouped test instances.
pub(crate) fn generate_group_aggregation(tests_const_name: &Ident, test_cases: &GeneratedTests) -> TokenStream {
    let to_entries = |tests: &[GeneratedTest]| -> Vec<TokenStream> {
        tests
            .iter()
            .map(|t| {
                let instance = &t.instance_ident;
                let struct_ident = &t.struct_ident;
                let validator = &t.validator_ident;
                quote! { (#instance, #struct_ident, #validator) }
            })
            .collect()
    };

    let mandatory = to_entries(&test_cases.mandatory_tests);
    let recommended = to_entries(&test_cases.optional_recommended_tests);
    let informative = to_entries(&test_cases.informative_tests);

    quote! {
        crate::macros::define_test_cases_aggregate!(
            const_name: #tests_const_name,
            mandatory: [#(#mandatory),*],
            recommended: [#(#recommended),*],
            informative: [#(#informative),*]
        );
    }
}
