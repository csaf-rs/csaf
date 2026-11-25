use csaf_validator::validate;

fn main() {
    let input = include_str!("../benches/data/csaf-rs_csaf-csaf_2_1-bad.json");
    let test_suite = String::from("full");
    let _result = validate(input, "2.1", &[test_suite]).unwrap();
    #[cfg(feature = "cli")]
    csaf_validator::cli::print_validation_result(&_result, true);
}
