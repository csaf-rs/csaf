use crate::{
    csaf::traits::vulnerabilities::cpe_trait::CpeTrait,
    csaf_traits::{BranchTrait, CsafTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait},
    validation::{TestFinding, TestFindingData},
};

fn create_inconsistend_product_identification_helper_error(content_json_path: &str) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: format!(""),
        instance_path: format!("{content_json_path}/epss/timestamp"),
    })
}

/// 6.2.42 Inconsistent Product Identification Helper
///
/// For each product identification helper which resides in branches, it SHALL be tested that the
/// product identification helper contain at least the same information as the categorized strings.
/// Information that cannot be represented in the specific product identification helper SHALL be
/// omitted from the comparison.
///
/// # PIH - Category Mapping
///
/// CPE 2.3 Schema: cpe:<cpe_version>:<part>:<vendor>:<product>:<version>:<update>:<edition>:<language>:<sw_edition>:<target_sw>:<target_hw>:<other>
/// PURL 1.0 Schema: scheme:type/namespace/name@version?qualifiers#subpath
///
///
/// | CSAF 2.1              | CPE 2.3   | PURL 1.0             |
/// | --------------------- | --------- | -------------------- |
/// | architecture          | target_hw | arch[^2]             |
/// | host_name             | N/A       | N/A                  |
/// | language              | language  | N/A                  |
/// | patch_level           |           | version or patch[^3] |
/// | platform              | target_hw | platform[^2]         |
/// | product_family        | N/A       | N/A                  |
/// | product_name          | product   | name                 |
/// | product_version       | version   | version              |
/// | product_version_range | version   | version              |
/// | service_pack          | update    | build or patch[^4]   |
/// | specification         |           |                      |
/// | vendor                | vendor    | namespace[^1]        |
///
/// [^1] typically names the vendor though this can also follow a different convention, e.g. for
/// type git, the hoster of the git forge.
/// [^2] not used by all purl `types`
/// [^3] `patch` is only available for the swid type, otherwise patch_level is often included in the
/// version
/// [^4] Depends on the type whether these options are used or available
pub fn test_6_2_42_inconsistent_product_identification_helper(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let Some(product_tree) = doc.get_product_tree() else {
        return Ok(());
    };
    let leaf_paths = product_tree
        .collect_leaf_paths()
        .iter()
        .inspect(|(branches, idxs)| {
            println!("{:?}", idxs);
            let _ = branches
                .iter()
                .inspect(|branch| {
                    print!(
                        "{:?} - {:?} - ",
                        branch.get_category(),
                        branch.get_name()
                    );
                    if let Some(pih) = branch.get_product().and_then(|product| product.get_product_identification_helper()) {
                        if let Some(cpe) = pih.get_cpe() {
                            println!("{:?}", cpe::uri::Wfn::parse(cpe.as_str()));
                        } else if let Some(purls) = pih.get_purls() {
                            println!("{:?}", purls);
                        } else {
                            println!("None");
                        }
                    }
                })
                .count();
        })
        .collect::<Vec<_>>();
    Ok(())
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_42,
    test_6_2_42_inconsistent_product_identification_helper
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_42 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_42() {
        TESTS_2_1.test_6_2_42.expect(ExpectedResults {
            // 1. Language is set in CPE but not provided for product
            // 2. Update value is not provided through service_pack
            case_01: Ok(()),
            // architecture is not provided for any of the entries
            case_02: Ok(()),
            // versions are given but different ranges than in the cpes
            case_03: Ok(()),
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
        });
    }
}
