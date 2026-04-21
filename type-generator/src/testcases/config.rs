use super::{CsafVersion, TestcaseConfig};

pub fn get_testcase_configs() -> Vec<TestcaseConfig> {
    vec![
        TestcaseConfig {
            input: "../csaf/csaf_2.0/test/validator/data/testcases.json",
            supplemental_input: "assets/tests/csaf_2.0/testcases.json",
            output: "csaf2_0/testcases.generated.rs",
            csaf_version: CsafVersion::V2_0,
        },
        TestcaseConfig {
            input: "../csaf/csaf_2.1/test/validator/data/testcases.json",
            supplemental_input: "assets/tests/csaf_2.1/testcases.json",
            output: "csaf2_1/testcases.generated.rs",
            csaf_version: CsafVersion::V2_1,
        },
    ]
}
