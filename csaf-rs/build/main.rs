mod cwe_embed;
mod ssvc_validation;

fn main() {
    println!("cargo:rerun-if-changed=build/main.rs");
    println!("cargo:rerun-if-changed=build/ssvc_validation.rs");
    println!("cargo:rerun-if-changed=build/cwe_embed.rs");

    ssvc_validation::validate_ssvc_lib_json();
    cwe_embed::generate_cwe_file_lookup();
}
