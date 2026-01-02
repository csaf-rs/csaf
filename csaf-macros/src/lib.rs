use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Attribute macro for document category checks.
///
/// This macro automatically inserts a category check at the beginning of a
/// validation function. If the document's category doesn't match the specified
/// categories for the given CSAF version, the function returns `Ok(())` early.
///
/// # Arguments
///
/// The macro accepts the following arguments:
///
/// - `csaf20 = [...]` - Categories that apply for CSAF version 2.0
/// - `csaf21 = [...]` - Categories that apply for CSAF version 2.1
/// - `all = [...]` - Categories that apply for all CSAF versions (shorthand)
///
/// When using `all`, the same categories are used for both csaf20 and csaf21.
///
/// # Examples
///
/// ```rust,ignore
/// // Different categories per version
/// #[profile_test_applies_to_category(
///     csaf20 = [CsafSecurityAdvisory, CsafVex],
///     csaf21 = [CsafSecurityAdvisory, CsafVex, CsafDeprecatedSecurityAdvisory]
/// )]
/// pub fn test_6_1_27_11_vulnerabilities(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
///     // ...
/// }
///
/// // Same categories for all versions
/// #[profile_test_applies_to_category(all = [CsafVex])]
/// pub fn test_6_1_27_07_vex_product_status(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
///     // ...
/// }
/// ```
///
/// # Generated Code
///
/// The macro generates code equivalent to:
///
/// ```rust,ignore
/// let doc_category = doc.get_document().get_category();
/// let __csaf_version = doc.get_document().get_csaf_version();
///
/// match *__csaf_version {
///     CsafVersion::X20 => {
///         if !matches!(doc_category, DocumentCategory::CsafSecurityAdvisory | DocumentCategory::CsafVex) {
///             return Ok(());
///         }
///     }
///     CsafVersion::X21 => {
///         if !matches!(doc_category, DocumentCategory::CsafSecurityAdvisory | DocumentCategory::CsafVex | DocumentCategory::CsafDeprecatedSecurityAdvisory) {
///             return Ok(());
///         }
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn profile_test_applies_to_category(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let attrs = parse_macro_input!(attr as CategoryAttrs);

    let fn_name = &input.sig.ident;
    let fn_inputs = &input.sig.inputs;
    let fn_output = &input.sig.output;
    let fn_generics = &input.sig.generics;
    let fn_body = &input.block;
    let fn_vis = &input.vis;
    let fn_attrs = &input.attrs;

    // Extract the name of the doc parameter (first parameter)
    let doc_param = extract_doc_param(&input.sig.inputs);

    let category_check = generate_category_check(&doc_param, &attrs);

    // Extract the statements from the original block (without the outer braces)
    let stmts = &fn_body.stmts;

    let expanded = quote! {
        #(#fn_attrs)*
        #fn_vis fn #fn_name #fn_generics (#fn_inputs) #fn_output {
            #category_check

            #(#stmts)*
        }
    };

    TokenStream::from(expanded)
}

/// Parsed category attributes
struct CategoryAttrs {
    v20_categories: Vec<syn::Ident>,
    v21_categories: Vec<syn::Ident>,
}

impl syn::parse::Parse for CategoryAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut v20_categories = Vec::<syn::Ident>::new();
        let mut v21_categories =  Vec::<syn::Ident>::new();

        while !input.is_empty() {
            let key: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;

            let content;
            syn::bracketed!(content in input);
            let categories: syn::punctuated::Punctuated<syn::Ident, syn::Token![,]> =
                content.parse_terminated(syn::Ident::parse, syn::Token![,])?;

            match key.to_string().as_str() {
                "csaf20" => v20_categories.extend(categories.into_iter().collect::<Vec<syn::Ident>>()),
                "csaf21" => v21_categories.extend(categories.into_iter().collect::<Vec<syn::Ident>>()),
                "all" => {
                    let cats: Vec<syn::Ident> = categories.into_iter().collect();
                    v20_categories.extend(cats.clone());
                    v21_categories.extend(cats);
                }
                _ => {
                    return Err(syn::Error::new(
                        key.span(),
                        format!(
                            "Unknown attribute key '{}'. Expected 'csaf20', 'csaf21', or 'all'",
                            key
                        ),
                    ));
                }
            }

            // Optional comma between attributes
            let _ = input.parse::<syn::Token![,]>();
        }

        // Validate that we have categories in either of the versions
        // TODO: When v21 implementation is done, this check should validate that at v21 always present.
        // At the same time, v20 will be excluded for v21 specific tests.
        if v20_categories.is_empty() && v21_categories.is_empty() {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "Must specify categories for either CSAF 2.0 or 2.1. Use 'csaf20 = [...]', 'csaf21 = [...]' or 'all = [...]'",
            ));
        }

        let v20_pre_dedup_count = v20_categories.len();
        v20_categories.sort_unstable();
        v20_categories.dedup();

        if v20_pre_dedup_count != v20_categories.len() {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "Duplicate definition of a profile test CSAF 2.0 category."
            ))
        }

        let v21_pre_dedup_count = v21_categories.len();
        v21_categories.sort_unstable();
        v21_categories.dedup();

        if v21_pre_dedup_count != v21_categories.len() {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "Duplicate definition of a profile test CSAF 2.1 category."
            ))
        }

        Ok(CategoryAttrs {
            v20_categories,
            v21_categories,
        })
    }
}

/// Extracts the name of the first function parameter (expected to be `doc`)
fn extract_doc_param(
    inputs: &syn::punctuated::Punctuated<syn::FnArg, syn::Token![,]>,
) -> syn::Ident {
    if let Some(syn::FnArg::Typed(pat_type)) = inputs.first() {
        if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
            return pat_ident.ident.clone();
        }
    }
    // Fallback to 'doc' if we can't extract the parameter name
    syn::Ident::new("doc", proc_macro2::Span::call_site())
}

/// Generates the category check code
fn generate_category_check(
    doc_param: &syn::Ident,
    attrs: &CategoryAttrs,
) -> proc_macro2::TokenStream {
    let v20_cats = &attrs.v20_categories;
    let v21_cats = &attrs.v21_categories;

    quote! {
        use crate::csaf_traits::{CsafVersion, DocumentCategory, DocumentTrait, CsafTrait};

        let doc_category = #doc_param.get_document().get_category();
        let __csaf_version = #doc_param.get_document().get_csaf_version();

        match *__csaf_version {
            CsafVersion::X20 => {
                if !matches!(doc_category, #(DocumentCategory::#v20_cats)|*) {
                    return Ok(());
                }
            }
            CsafVersion::X21 => {
                if !matches!(doc_category, #(DocumentCategory::#v21_cats)|*) {
                    return Ok(());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_all_categories() {
        let tokens: proc_macro2::TokenStream = "all = [CsafVex]".parse().unwrap();
        let attrs: CategoryAttrs = syn::parse2(tokens).unwrap();

        assert_eq!(attrs.v20_categories.len(), 1);
        assert_eq!(attrs.v21_categories.len(), 1);
        assert_eq!(attrs.v20_categories[0].to_string(), "CsafVex");
        assert_eq!(attrs.v21_categories[0].to_string(), "CsafVex");
    }

    #[test]
    fn test_parse_separate_versions() {
        let tokens: proc_macro2::TokenStream =
            "csaf20 = [CsafSecurityAdvisory, CsafVex], csaf21 = [CsafSecurityAdvisory, CsafVex, CsafDeprecatedSecurityAdvisory]"
                .parse()
                .unwrap();
        let attrs: CategoryAttrs = syn::parse2(tokens).unwrap();

        assert_eq!(attrs.v20_categories.len(), 2);
        assert_eq!(attrs.v21_categories.len(), 3);
    }

    #[test]
    fn test_parse_all_with_separate_versions() {
        let tokens: proc_macro2::TokenStream =
            "all = [CsafVex], csaf20 = [CsafSecurityAdvisory], csaf21 = [CsafSecurityAdvisory, CsafDeprecatedSecurityAdvisory]"
                .parse()
                .unwrap();
        let attrs: CategoryAttrs = syn::parse2(tokens).unwrap();

        assert_eq!(attrs.v20_categories.len(), 2);
        assert_eq!(attrs.v21_categories.len(), 3);
    }

    #[test]
    fn test_parse_error_on_unknown_key() {
        let tokens: proc_macro2::TokenStream = "unknown = [CsafVex]".parse().unwrap();
        let result: syn::Result<CategoryAttrs> = syn::parse2(tokens);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_empty_string() {
        let tokens: proc_macro2::TokenStream = "".parse().unwrap();
        let result: syn::Result<CategoryAttrs> = syn::parse2(tokens);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_empty_list() {
        let tokens: proc_macro2::TokenStream = "csaf20 = []".parse().unwrap();
        let result: syn::Result<CategoryAttrs> = syn::parse2(tokens);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_duplicate_category_v20() {
        let tokens: proc_macro2::TokenStream = "csaf20 = [CsafVex], all = [CsafVex]"
            .parse().unwrap();
        let result: syn::Result<CategoryAttrs> = syn::parse2(tokens);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_duplicate_category_v21() {
        let tokens: proc_macro2::TokenStream = "csaf21 = [CsafVex], all = [CsafVex]"
            .parse().unwrap();
        let result: syn::Result<CategoryAttrs> = syn::parse2(tokens);

        assert!(result.is_err());
    }
}

