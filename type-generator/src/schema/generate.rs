use std::collections::{BTreeMap, BTreeSet};

use crate::build_errors::BuildError;
use crate::utils::codegen_snippets::{
    add_generated_code_header, add_ignore_clippy, add_ignore_rustdoc, add_ignore_rustfmt, add_string_newtype_impls,
};
use crate::utils::read_write_fs::write_generated_file;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, TypePath};
use typify::{Type, TypeDetails, TypeId, TypeSpace, TypeSpaceSettings};

use super::SchemaVersion;

pub fn generate_from_schema(
    schema: schemars::schema::RootSchema,
    output: &str,
    target_folder: &str,
) -> Result<TypeSpace, BuildError> {
    let mut type_space = TypeSpace::new(
        TypeSpaceSettings::default()
            .with_struct_builder(true)
            .with_derive("PartialEq".into())
            .with_derive("Eq".into()),
    );
    type_space.add_root_schema(schema)?;

    // Convert the TypeSpace token stream into a syn::File so we can inject a file-level doc attribute
    let mut file = syn::parse2::<syn::File>(type_space.to_stream())?;

    add_string_newtype_impls(&mut file);
    add_ignore_rustdoc(&mut file);
    add_ignore_rustfmt(&mut file);
    add_ignore_clippy(&mut file);
    add_generated_code_header(&mut file);

    // Unparse the modified syn::File into Rust source code
    let content = prettyplease::unparse(&file);
    let relative_path = format!("src/schema/{output}");

    write_generated_file(target_folder, &relative_path, &content, "generated types")?;

    Ok(type_space)
}

pub fn generate_ir(type_spaces: &[(SchemaVersion, TypeSpace)], target_folder: &str) -> Result<(), BuildError> {
    let mut mod_items = vec![];

    let v20_root = node_ident(0, SchemaVersion::V20);
    let v21_root = node_ident(0, SchemaVersion::V21);
    mod_items.push(quote!(
        /// A potentially schema-invalid CSAF document
        pub struct Document {
            inner: serde_json::Value,
        }

        impl Document {
            /// Treats the given `json_value` as a `csaf2x::Document`
            ///
            /// This constructor does not perform any parsing or validation
            pub fn new(json_value: serde_json::Value) -> Self {
                Self {
                    inner: json_value,
                }
            }

            /// Immutably views the document as if it were a 2.0 document
            ///
            /// This can be thought of as a zero-cost cast
            pub fn v20_view(&self) -> &#v20_root {
                bytemuck::TransparentWrapper::wrap_ref(&self.inner)
            }

            /// Mutably views the document as if it were a 2.0 document
            ///
            /// This can be thought of as a zero-cost cast
            pub fn v20_view_mut(&mut self) -> &mut #v20_root {
                bytemuck::TransparentWrapper::wrap_mut(&mut self.inner)
            }

            /// Immutably views the document as if it were a 2.1 document
            ///
            /// This can be thought of as a zero-cost cast
            pub fn v21_view(&self) -> &#v21_root {
                bytemuck::TransparentWrapper::wrap_ref(&self.inner)
            }

            /// Mutably views the document as if it were a 2.1 document
            ///
            /// This can be thought of as a zero-cost cast
            pub fn v21_view_mut(&mut self) -> &mut #v21_root {
                bytemuck::TransparentWrapper::wrap_mut(&mut self.inner)
            }

            pub fn into_raw(self) -> serde_json::Value {
                self.inner
            }
        }

        #[derive(Debug, PartialEq)]
        pub struct SchemaError;

        type Object = serde_json::Map<String, serde_json::Value>;
    ));

    for (version, type_space) in type_spaces {
        generate_ir_for_schema(type_space, *version, &mut mod_items);
    }

    let mut file = syn::parse2::<syn::File>(quote!(#(#mod_items)*))?;

    add_ignore_rustdoc(&mut file);
    add_ignore_rustfmt(&mut file);
    add_ignore_clippy(&mut file);
    add_generated_code_header(&mut file);

    let content = prettyplease::unparse(&file);
    write_generated_file(target_folder, "src/schema/csaf2x.rs", &content, "generated types")?;

    Ok(())
}

pub fn generate_ir_for_schema(type_space: &TypeSpace, version: SchemaVersion, mod_items: &mut Vec<TokenStream>) {
    const ROOT: &str = "CommonSecurityAdvisoryFramework";

    let Some(root) = type_space.iter_types().find(|ty| ty.name() == ROOT) else {
        panic!("type {ROOT} was not found")
    };

    let mut done = BTreeSet::new();
    let mut todo = vec![];
    let mut type_id_to_index = TypeIdToIndex::default();
    generate_ir_ty(
        0,
        version,
        &root,
        mod_items,
        &mut todo,
        &mut type_id_to_index,
        type_space,
    );
    while let Some(type_id) = todo.pop() {
        if done.contains(&type_id) {
            continue;
        }

        let ty = type_space.get_type(&type_id).unwrap();
        let index = type_id_to_index.get_or_insert(&type_id);
        generate_ir_ty(
            index,
            version,
            &ty,
            mod_items,
            &mut todo,
            &mut type_id_to_index,
            type_space,
        );
        done.insert(type_id);
    }
}

fn generate_ir_ty(
    index: usize,
    schema_version: SchemaVersion,
    ty: &Type,
    mod_items: &mut Vec<TokenStream>,
    todo: &mut Vec<TypeId>,
    type_id_to_index: &mut TypeIdToIndex,
    type_space: &TypeSpace,
) {
    match ty.details() {
        TypeDetails::Enum(_type_enum) => {
            let ident = node_ident(index, schema_version);

            mod_items.push(quote!(
                #[derive(bytemuck::TransparentWrapper)]
                #[repr(transparent)]
                pub struct #ident(serde_json::Value);
            ));

            let schema_ident = new_ident(&ty.name());
            let schema_mod = version_to_mod_name(schema_version);
            mod_items.push(quote!(
                impl #ident {
                    /// Deserializes the node's content into an enum according to the schema
                    pub fn get_enum(&self) -> Result<super::#schema_mod::schema::#schema_ident, serde_json::Error> {
                        serde_json::from_value(bytemuck::TransparentWrapper::peel_ref(self).clone())
                    }

                    /// Serializes `value` into JSON and writes into this node
                    pub fn set_enum(&mut self, value: super::#schema_mod::schema::#schema_ident) {
                        *bytemuck::TransparentWrapper::peel_mut(self) = serde_json::to_value(&value).unwrap();
                    }
                }
            ));
        },

        TypeDetails::Struct(type_struct) => {
            let ident = node_ident(index, schema_version);
            mod_items.push(quote!(
                #[derive(bytemuck::TransparentWrapper)]
                #[repr(transparent)]
                pub struct #ident(serde_json::Value);));

            let mut methods = vec![];
            for (prop_name, prop_id) in type_struct.properties() {
                let get_or_insert = format_ident!("caution_get_or_insert_{prop_name}");
                let access = format_ident!("{prop_name}");
                let access_mut = format_ident!("{prop_name}_mut");

                let child_ident = node_ident(type_id_to_index.get_or_insert(&prop_id), schema_version);

                methods.push(quote!(
                    /// Grants shared access to the stated node
                    pub fn #access(&self) -> Option<&#child_ident> {
                        self.0.get(#prop_name).map(bytemuck::TransparentWrapper::wrap_ref)
                    }
                    /// Grants mutable access to the stated node
                    pub fn #access_mut(&mut self) -> Option<&mut #child_ident> {
                        self.0.get_mut(#prop_name).map(bytemuck::TransparentWrapper::wrap_mut)
                    }
                    /// Accesses the stated node creating it if it does not exist
                    ///
                    /// NOTE: `Null` will be used as a placeholder for the new node
                    /// NOTE: if `self` is not an object then it will overwritten with an initially empty object
                    pub fn #get_or_insert(&mut self) -> &mut #child_ident {
                        if !self.0.is_object() {
                            self.0 = serde_json::Value::Object(serde_json::Map::default());
                        }
                        let map = self.0.as_object_mut().unwrap_or_else(|| unreachable!());
                        bytemuck::TransparentWrapper::wrap_mut(map.entry(#prop_name).or_insert(serde_json::Value::Null))

                    }
                ));

                todo.push(prop_id);
            }

            let schema_ident = new_ident(&ty.name());
            let schema_mod = version_to_mod_name(schema_version);
            mod_items.push(quote!(impl #ident {
                #(#methods)*

                /// Deserializes the node's content into a struct according to the schema
                pub fn get_struct(&self) -> Result<super::#schema_mod::schema::#schema_ident, serde_json::Error> {
                    serde_json::from_value(bytemuck::TransparentWrapper::peel_ref(self).clone())
                }

                /// Serializes `value` into JSON and writes into this node
                pub fn set_struct(&mut self, value: super::#schema_mod::schema::#schema_ident) {
                    *bytemuck::TransparentWrapper::peel_mut(self) = serde_json::to_value(&value).unwrap();
                }

                /// Views the node as mutable JSON object
                ///
                /// Caution: this can invalidate the schema despite being in a versioned view
                pub fn caution_as_object_mut(&mut self) -> Result<&mut Object, SchemaError> {
                    match bytemuck::TransparentWrapper::peel_mut(self) {
                        serde_json::Value::Object(o) => {
                            Ok(o)
                        },
                        _ => Err(SchemaError),
                    }
                }
            }));
        },

        // we "peel off" the newtype using a type alias between the outer node and the inner node
        TypeDetails::Newtype(type_newtype) => {
            let inner_id = type_newtype.inner();
            let src = node_ident(index, schema_version);
            let dest = node_ident(type_id_to_index.get_or_insert(&inner_id), schema_version);
            todo.push(inner_id);

            mod_items.push(quote!(
                pub type #src = #dest;
            ));
        },

        // the API is already fallible so the accessor that returned this node already wrapped it in an `Option`
        // therefere we need to "pass through" the inner type
        TypeDetails::Option(inner_id) => {
            let src = node_ident(index, schema_version);
            let dest = node_ident(type_id_to_index.get_or_insert(&inner_id), schema_version);
            todo.push(inner_id);

            mod_items.push(quote!(
                pub type #src = #dest;
            ));
        },

        TypeDetails::Vec(type_id) | TypeDetails::Set(type_id) => {
            let ident = node_ident(index, schema_version);

            mod_items.push(quote!(
                #[derive(bytemuck::TransparentWrapper)]
                #[repr(transparent)]
                pub struct #ident(serde_json::Value);));

            let element_ident = node_ident(type_id_to_index.get_or_insert(&type_id), schema_version);
            todo.push(type_id);

            mod_items.push(quote!(
                impl #ident {
                    pub fn as_slice(&self) -> Result<&[#element_ident], SchemaError> {
                        match bytemuck::TransparentWrapper::peel_ref(self) {
                            serde_json::Value::Array(s) => {
                                Ok(bytemuck::TransparentWrapper::wrap_slice(s))
                            },
                            _ => Err(SchemaError)
                        }
                    }

                    pub fn as_slice_mut(&mut self) -> Result<&mut [#element_ident], SchemaError> {
                        match bytemuck::TransparentWrapper::peel_mut(self) {
                            serde_json::Value::Array(s) => {
                                Ok(bytemuck::TransparentWrapper::wrap_slice_mut(s))
                            },
                            _ => Err(SchemaError)
                        }
                    }
                }

            ));
        },

        TypeDetails::Map(type_id, type_id1) => {
            let key = type_space.get_type(&type_id).unwrap();
            assert_eq!(
                "::std::string::String",
                key.name().replace(" ", ""),
                "expected a JSON object"
            );
            let value = type_space.get_type(&type_id1).unwrap();
            assert_eq!(
                "::serde_json::Value",
                value.name().replace(" ", ""),
                "expected a JSON object"
            );

            let ident = node_ident(index, schema_version);

            mod_items.push(quote!(
                #[derive(bytemuck::TransparentWrapper)]
                #[repr(transparent)]
                pub struct #ident(serde_json::Value);

                impl #ident {
                    pub fn as_object(&self) -> Result<&Object, SchemaError> {
                        match bytemuck::TransparentWrapper::peel_ref(self) {
                            serde_json::Value::Object(m) => Ok(m),
                            _ => Err(SchemaError)
                        }
                    }

                    pub fn as_object_mut(&mut self) -> Result<&mut Object, SchemaError> {
                        match bytemuck::TransparentWrapper::peel_mut(self) {
                            serde_json::Value::Object(m) => Ok(m),
                            _ => Err(SchemaError)
                        }
                    }
                }

            ));
        },

        TypeDetails::String => {
            let ident = node_ident(index, schema_version);

            mod_items.push(quote!(
                #[derive(bytemuck::TransparentWrapper)]
                #[repr(transparent)]
                pub struct #ident(serde_json::Value);));

            mod_items.push(quote!(
                impl #ident {
                    pub fn get_str(&self) -> Result<&str, SchemaError> {
                        match bytemuck::TransparentWrapper::peel_ref(self) {
                            serde_json::Value::String(s) => Ok(s),
                            _ => Err(SchemaError)
                        }
                    }

                    pub fn set_str(&mut self, s: String) {
                        *bytemuck::TransparentWrapper::peel_mut(self) = s.into();
                    }
                }
            ))
        },

        TypeDetails::Builtin(path) => {
            let path: TypePath = syn::parse_str(path).unwrap();
            let ident = node_ident(index, schema_version);

            mod_items.push(quote!(
                #[derive(bytemuck::TransparentWrapper)]
                #[repr(transparent)]
                pub struct #ident(serde_json::Value);

                impl #ident {
                    pub fn get(&self) -> Result<#path, serde_json::Error> {
                        serde_json::from_value(bytemuck::TransparentWrapper::peel_ref(self).clone())
                    }

                    pub fn set(&mut self, value: #path) {
                        *bytemuck::TransparentWrapper::peel_mut(self) = serde_json::to_value(&value).unwrap();
                    }
                }
            ));
        },

        // unexpected branches given the CSAF schemas 2.0 and 2.1
        TypeDetails::Box(_type_id) => unreachable!(),
        TypeDetails::Tuple(_iterator) => unreachable!(),
        TypeDetails::Array(_type_id, _) => unreachable!(),
        TypeDetails::Unit => unreachable!(),
    }
}

#[derive(Default)]
struct TypeIdToIndex {
    inner: BTreeMap<TypeId, usize>,
}

impl TypeIdToIndex {
    fn get_or_insert(&mut self, type_id: &TypeId) -> usize {
        if let Some(index) = self.inner.get(type_id) {
            return *index;
        }

        let index = self.inner.len() + 1;
        self.inner.insert(type_id.clone(), index);
        index
    }
}

fn new_ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
}

fn node_ident(index: usize, version: SchemaVersion) -> Ident {
    format_ident!("{version:?}Node{index}")
}

fn version_to_mod_name(version: SchemaVersion) -> Ident {
    let name = match version {
        SchemaVersion::V20 => "csaf2_0",
        SchemaVersion::V21 => "csaf2_1",
    };
    new_ident(name)
}
