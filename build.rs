use std::fs;
use std::path::Path;
use typify::{TypeSpace, TypeSpaceSettings};

fn main() {
    let content = fs::read_to_string("./csaf_json_schema.json").unwrap();
    let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content).unwrap();

    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    type_space.add_root_schema(schema).unwrap();

    let content = prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap());

    let mut out_file = Path::new("src").to_path_buf();
    out_file.push("codegen.rs");
    fs::write(out_file, content).unwrap();
}
