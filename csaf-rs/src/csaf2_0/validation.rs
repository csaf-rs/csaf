use crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework;
use crate::{
    csaf2_0::testcases::TESTS_2_0,
    validation::{Test, Validatable, ValidationPreset},
};
use std::collections::HashMap;

impl Validatable<CommonSecurityAdvisoryFramework> for CommonSecurityAdvisoryFramework {
    fn presets(&self) -> HashMap<ValidationPreset, Vec<&str>> {
        let basic_tests = Vec::from(["6.1.1", "6.1.2"]);
        // More tests may be added in extend() here later
        let extended_tests: Vec<&str> = basic_tests.clone();
        // extended_tests.extend(["foo"].iter());
        let full_tests: Vec<&str> = extended_tests.clone();
        // full_tests.extend(["bar"].iter());
        HashMap::from([
            (ValidationPreset::Basic, basic_tests),
            (ValidationPreset::Extended, extended_tests),
            (ValidationPreset::Full, full_tests),
        ])
    }

    fn tests(&self) -> HashMap<&str, Test<CommonSecurityAdvisoryFramework>> {
        HashMap::from([(
            TESTS_2_0.test_6_1_1.id(),
            (|doc| TESTS_2_0.test_6_1_1.validate(doc)) as Test<_>,
        )])
    }

    fn doc(&self) -> &CommonSecurityAdvisoryFramework {
        self
    }
}
