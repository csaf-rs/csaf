use std::collections::{BTreeMap, BTreeSet};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, TypePath};
use typify::{Type, TypeDetails, TypeId, TypeSpace};

use json_path::{JsonPath, JsonPathComponent, JsonPathMapper};

use crate::build_errors::BuildError;
use crate::schema::SchemaVersion;
use crate::utils::codegen_snippets::{
    add_generated_code_header, add_ignore_clippy, add_ignore_rustdoc, add_ignore_rustfmt,
};
use crate::utils::read_write_fs::write_generated_file;

mod json_path;

pub fn generate(
    type_space: &TypeSpace,
    version: SchemaVersion,
    target_folder: &str,
) -> Result<JsonPathMapper, BuildError> {
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
    let mut json_paths = JsonPathMapper::default();
    json_paths.insert(0, JsonPath::root());
    generate_loose_type(
        0,
        version,
        &root,
        &mut mod_items,
        &mut todo,
        &mut type_id_to_index,
        type_space,
        &mut json_paths,
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
            &mut json_paths,
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

    Ok(json_paths)
}

pub fn generate_cast_api(
    versioned_json_paths: Vec<(SchemaVersion, JsonPathMapper)>,
    target_folder: &str,
) -> Result<(), BuildError> {
    let [(ver_a, paths_a), (ver_b, paths_b)] = versioned_json_paths.try_into().ok().unwrap();
    let paths_b = paths_b.non_trivial_paths();

    let mut mod_items = vec![];
    let mut seen_a = BTreeSet::new();
    let ver_a_ident = format_ident!("{ver_a}");
    let ver_b_ident = format_ident!("{ver_b}");
    for (path_a, indices_a) in paths_a.non_trivial_paths() {
        let index_a = indices_a.first().unwrap();
        if let Some(indices_b) = paths_b.get(&path_a) {
            let index_b = indices_b.first().unwrap();
            if seen_a.contains(index_a) {
                continue;
            }
            seen_a.extend(indices_a.iter().copied());

            let node_a = node_ident(*index_a);
            let node_b = node_ident(*index_b);

            let a_to_b = format_ident!("as_{ver_b}");
            let a_to_b_mut = format_ident!("caution_as_{ver_b}_mut");
            mod_items.push(quote!(
                impl super::#ver_a_ident::#node_a {
                    /// Views this node as if it were in the other version of the schema
                    ///
                    /// This implies that the position of this node is the same on both schemas
                    pub fn #a_to_b(&self) -> &super::#ver_b_ident::#node_b {
                        let val = bytemuck::TransparentWrapper::peel_ref(self);
                        bytemuck::TransparentWrapper::wrap_ref(val)
                    }

                    /// Mutable version of the other `as_v*` method
                    pub fn #a_to_b_mut(&mut self) -> &mut super::#ver_b_ident::#node_b {
                        let val = bytemuck::TransparentWrapper::peel_mut(self);
                        bytemuck::TransparentWrapper::wrap_mut(val)
                    }
                }
            ));

            let b_to_a = format_ident!("as_{ver_a}");
            let b_to_a_mut = format_ident!("caution_as_{ver_a}_mut");
            mod_items.push(quote!(
                impl super::#ver_b_ident::#node_b {
                    /// Views this node as if it were in the other version of the schema
                    ///
                    /// This implies that the position of this node is the same on both schemas
                    pub fn #b_to_a(&self) -> &super::#ver_a_ident::#node_a {
                        let val = bytemuck::TransparentWrapper::peel_ref(self);
                        bytemuck::TransparentWrapper::wrap_ref(val)
                    }

                    /// Mutable version of the other `as_v*` method
                    pub fn #b_to_a_mut(&mut self) -> &mut super::#ver_a_ident::#node_a {
                        let val = bytemuck::TransparentWrapper::peel_mut(self);
                        bytemuck::TransparentWrapper::wrap_mut(val)
                    }
                }
            ));
        }
    }

    let mut file = syn::parse2::<syn::File>(quote!(#(#mod_items)*))?;

    add_ignore_rustdoc(&mut file);
    add_ignore_rustfmt(&mut file);
    add_ignore_clippy(&mut file);
    add_generated_code_header(&mut file);

    let relative_path = "src/schema/loose/cast.rs";
    let content = prettyplease::unparse(&file);
    write_generated_file(target_folder, relative_path, &content, "generated types")?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn generate_loose_type(
    index: NodeIndex,
    schema_version: SchemaVersion,
    ty: &Type,
    mod_items: &mut Vec<TokenStream>,
    todo: &mut Vec<TypeId>,
    type_id_to_index: &mut TypeIdToIndex,
    type_space: &TypeSpace,
    json_paths: &mut JsonPathMapper,
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
                    #[allow(dead_code)]
                    fn default_raw_value() -> serde_json::Value {
                        serde_json::Value::Null
                    }

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

            let base_paths = json_paths.get(index).to_vec();
            let mut methods = vec![];
            for (prop_name, prop_id) in type_struct.properties() {
                let get_or_insert = format_ident!("caution_get_or_insert_{prop_name}");
                let access = format_ident!("{prop_name}");
                let access_mut = format_ident!("{prop_name}_mut");

                let child_index = type_id_to_index.get_or_insert(&prop_id);
                let child_ident = node_ident(child_index);

                for path in base_paths.clone() {
                    let mut path = path.clone();
                    path.push(JsonPathComponent::Field(prop_name.to_string()));
                    json_paths.insert(child_index, path);
                }

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
                    /// NOTE: an empty object/vec will be used as a placeholder for object/vec nodes
                    /// and `Null` for all other node types
                    /// NOTE: if `self` is not an object then it will overwritten with an initially empty object
                    pub fn #get_or_insert(&mut self) -> &mut #child_ident {
                        if !self.0.is_object() {
                            self.0 = serde_json::Value::Object(serde_json::Map::default());
                        }
                        let map = self.0.as_object_mut().unwrap_or_else(|| unreachable!());
                        bytemuck::TransparentWrapper::wrap_mut(map.entry(#prop_name).or_insert(#child_ident::default_raw_value()))

                    }
                ));

                todo.push(prop_id);
            }

            let schema_ident = new_ident(&ty.name());
            let schema_mod = version_to_mod_name(schema_version);
            mod_items.push(quote!(impl #ident {
                #(#methods)*

                #[allow(dead_code)]
                fn default_raw_value() -> serde_json::Value {
                    serde_json::Value::Object(super::Object::new())
                }

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
                        #[allow(dead_code)]
                        fn default_raw_value() -> serde_json::Value {
                            serde_json::Value::Null
                        }

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
                let dest = node_ident(index);
                let src_index = type_id_to_index.get_or_insert(&inner_id);
                let src = node_ident(src_index);
                todo.push(inner_id);
                json_paths.alias(src_index, index);

                mod_items.push(quote!(
                    pub type #dest = #src;
                ));
            }
        },

        // the API is already fallible so the accessor that returned this node already wrapped it in an `Option`
        // therefere we need to "pass through" the inner type
        TypeDetails::Option(inner_id) => {
            let dest = node_ident(index);
            let src_index = type_id_to_index.get_or_insert(&inner_id);
            let src = node_ident(src_index);
            todo.push(inner_id);
            json_paths.alias(src_index, index);

            mod_items.push(quote!(
                pub type #dest = #src;
            ));
        },

        TypeDetails::Vec(type_id) | TypeDetails::Set(type_id) => {
            let ident = node_ident(index);

            mod_items.push(quote!(
                #[derive(bytemuck::TransparentWrapper)]
                #[repr(transparent)]
                pub struct #ident(serde_json::Value);));

            let element_index = type_id_to_index.get_or_insert(&type_id);
            let element_ident = node_ident(element_index);
            todo.push(type_id);
            let base_paths = json_paths.get(index).to_vec();
            for mut path in base_paths {
                path.push(JsonPathComponent::Element);
                json_paths.insert(element_index, path);
            }
            // skip the cast on the vector/slice; we prefer to have it at the element level
            json_paths.trivial(index);

            mod_items.push(quote!(
                impl #ident {
                    #[allow(dead_code)]
                    fn default_raw_value() -> serde_json::Value {
                        serde_json::Value::Array(vec![])
                    }

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

                    /// Allows adding and removing arbitrary JSON children nodes into this node
                    pub fn caution_as_vec_mut(&mut self) -> Result<&mut Vec<serde_json::Value>, SchemaError> {
                        match bytemuck::TransparentWrapper::peel_mut(self) {
                            serde_json::Value::Array(s) => Ok(s),
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
                    #[allow(dead_code)]
                    fn default_raw_value() -> serde_json::Value {
                        serde_json::Value::Object(super::Object::new())
                    }

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
            // skip the cast API on leaf nodes that are strings
            json_paths.trivial(index);

            let ident = node_ident(index);

            mod_items.push(quote!(
                #[derive(bytemuck::TransparentWrapper)]
                #[repr(transparent)]
                pub struct #ident(serde_json::Value);));

            mod_items.push(quote!(
                impl #ident {
                    #[allow(dead_code)]
                    fn default_raw_value() -> serde_json::Value {
                        serde_json::Value::Null
                    }

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
            // skip the cast API on leaf nodes that are strings
            json_paths.trivial(index);

            let path: TypePath = syn::parse_str(path).unwrap();
            let ident = node_ident(index);

            mod_items.push(quote!(
                #[derive(bytemuck::TransparentWrapper)]
                #[repr(transparent)]
                pub struct #ident(serde_json::Value);

                impl #ident {
                    #[allow(dead_code)]
                    fn default_raw_value() -> serde_json::Value {
                        serde_json::Value::Null
                    }

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
