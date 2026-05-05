mod ssvc_validation;

fn main() {
    println!("cargo:rerun-if-changed=build/main.rs");
    println!("cargo:rerun-if-changed=build/ssvc_validation.rs");

    ssvc_validation::validate_ssvc_lib_json();
}
