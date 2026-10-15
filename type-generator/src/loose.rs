use std::collections::{BTreeMap, BTreeSet};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, TypePath};
use typify::{Type, TypeDetails, TypeId, TypeSpace};

use crate::build_errors::BuildError;
use crate::schema::SchemaVersion;
use crate::utils::codegen_snippets::{
    add_generated_code_header, add_ignore_clippy, add_ignore_rustdoc, add_ignore_rustfmt,
};
use crate::utils::read_write_fs::write_generated_file;

pub fn generate(type_space: &TypeSpace, version: SchemaVersion, target_folder: &str) -> Result<(), BuildError> {
    const ROOT: &str = "CommonSecurityAdvisoryFramework";

    let mut mod_items = vec![quote!(
        use super::{Object, SchemaError};
    )];
    let Some(root) = type_space.iter_types().find(|ty| ty.name() == ROOT) else {
        panic!("type {ROOT} was not found")
    };

    let mut done = BTreeSet::new();
    let mut todo = vec![];
    let mut type_id_to_index = TypeIdToIndex::default();
    generate_loose_type(
        0,
        version,
        &root,
        &mut mod_items,
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
        generate_loose_type(
            index,
            version,
            &ty,
            &mut mod_items,
            &mut todo,
            &mut type_id_to_index,
            type_space,
        );
        done.insert(type_id);
    }

    let mut file = syn::parse2::<syn::File>(quote!(#(#mod_items)*))?;

    add_ignore_rustdoc(&mut file);
    add_ignore_rustfmt(&mut file);
    add_ignore_clippy(&mut file);
    add_generated_code_header(&mut file);

    let relative_path = format!("src/schema/loose/{version}.rs");
    let content = prettyplease::unparse(&file);
    write_generated_file(target_folder, &relative_path, &content, "generated types")?;

    Ok(())
}

fn generate_loose_type(
    index: NodeIndex,
    schema_version: SchemaVersion,
    ty: &Type,
    mod_items: &mut Vec<TokenStream>,
    todo: &mut Vec<TypeId>,
    type_id_to_index: &mut TypeIdToIndex,
    type_space: &TypeSpace,
) {
    match ty.details() {
        TypeDetails::Enum(_type_enum) => {
            let ident = node_ident(index);

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
                    pub fn get_enum(&self) -> Result<crate::schema::#schema_mod::schema::#schema_ident, serde_json::Error> {
                        serde_json::from_value(bytemuck::TransparentWrapper::peel_ref(self).clone())
                    }

                    /// Serializes `value` into JSON and writes into this node
                    pub fn set_enum(&mut self, value: crate::schema::#schema_mod::schema::#schema_ident) {
                        *bytemuck::TransparentWrapper::peel_mut(self) = serde_json::to_value(&value).unwrap();
                    }
                }
            ));
        },

        TypeDetails::Struct(type_struct) => {
            let ident = node_ident(index);
            mod_items.push(quote!(
                #[derive(bytemuck::TransparentWrapper)]
                #[repr(transparent)]
                pub struct #ident(serde_json::Value);));

            let mut methods = vec![];
            for (prop_name, prop_id) in type_struct.properties() {
                let get_or_insert = format_ident!("caution_get_or_insert_{prop_name}");
                let access = format_ident!("{prop_name}");
                let access_mut = format_ident!("{prop_name}_mut");

                let child_ident = node_ident(type_id_to_index.get_or_insert(&prop_id));

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
                pub fn get_struct(&self) -> Result<crate::schema::#schema_mod::schema::#schema_ident, serde_json::Error> {
                    serde_json::from_value(bytemuck::TransparentWrapper::peel_ref(self).clone())
                }

                /// Serializes `value` into JSON and writes into this node
                pub fn set_struct(&mut self, value: crate::schema::#schema_mod::schema::#schema_ident) {
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

        TypeDetails::Newtype(type_newtype) => {
            let inner_id = type_newtype.inner();
            let inner_ty = type_space.get_type(&inner_id).unwrap();
            if let TypeDetails::String = inner_ty.details() {
                let ident = node_ident(index);
                let schema_ident = new_ident(&ty.name());
                let schema_mod = version_to_mod_name(schema_version);
                // if this is a newtype over a string it likely performs string validation so
                // we prefer using the schema type in the setter; for the getter we cannot assume
                // that the input is valid so we return the raw string
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

                        pub fn set_str(&mut self, s: crate::schema::#schema_mod::schema::#schema_ident) {
                            *bytemuck::TransparentWrapper::peel_mut(self) =
                                serde_json::Value::String(s.into());
                        }
                    }
                ))
            } else {
                // we "peel off" the newtype using a type alias between the outer node and the inner node
                let src = node_ident(index);
                let dest = node_ident(type_id_to_index.get_or_insert(&inner_id));
                todo.push(inner_id);

                mod_items.push(quote!(
                    pub type #src = #dest;
                ));
            }
        },

        // the API is already fallible so the accessor that returned this node already wrapped it in an `Option`
        // therefere we need to "pass through" the inner type
        TypeDetails::Option(inner_id) => {
            let src = node_ident(index);
            let dest = node_ident(type_id_to_index.get_or_insert(&inner_id));
            todo.push(inner_id);

            mod_items.push(quote!(
                pub type #src = #dest;
            ));
        },

        TypeDetails::Vec(type_id) | TypeDetails::Set(type_id) => {
            let ident = node_ident(index);

            mod_items.push(quote!(
                #[derive(bytemuck::TransparentWrapper)]
                #[repr(transparent)]
                pub struct #ident(serde_json::Value);));

            let element_ident = node_ident(type_id_to_index.get_or_insert(&type_id));
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

            let ident = node_ident(index);

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
            let ident = node_ident(index);

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
            let ident = node_ident(index);

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
    inner: BTreeMap<TypeId, NodeIndex>,
}

impl TypeIdToIndex {
    fn get_or_insert(&mut self, type_id: &TypeId) -> NodeIndex {
        if let Some(index) = self.inner.get(type_id) {
            return *index;
        }

        let index = self.inner.len() + 1;
        self.inner.insert(type_id.clone(), index);
        index
    }
}

fn new_ident(name: &str) -> Ident {
    format_ident!("{name}")
}

fn node_ident(index: NodeIndex) -> Ident {
    format_ident!("Node{index}")
}

fn version_to_mod_name(version: SchemaVersion) -> Ident {
    let name = match version {
        SchemaVersion::V20 => "csaf2_0",
        SchemaVersion::V21 => "csaf2_1",
    };
    new_ident(name)
}

type NodeIndex = usize;
