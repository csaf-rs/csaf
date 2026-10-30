use cpe::{cpe::Cpe, fsb::Fsb, uri::Uri};
use purl::GenericPurl;
use vers_rs::VersionRange as _;

use crate::{
    csaf::{
        enums::version_range::VersionRange, traits::vulnerabilities::cpe_trait::CpeTrait,
        types::purl::csaf_purl::CsafPurl,
    },
    csaf_traits::{
        BranchTrait, CsafTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait,
        build_leaf_instance_path,
    },
    validation::{TestFinding, TestFindingData},
};

use std::str::FromStr;

fn create_inconsistent_product_identification_helper_warning(
    pih_type: &str,
    category: &str,
    pih_value: &str,
    categorized_string: &str,
    path: &str,
) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: format!(
            "Product's {pih_type} contains inconsistent information for `{category}: {categorized_string}` while {pih_type}'s value is {pih_value}"
        ),
        instance_path: path.to_string(),
    })
}

fn create_missing_product_identification_helper_field_warning(
    pih_type: &str,
    category: &str,
    categorized_string: &str,
    pih_value: &str,
    path: &str,
) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: format!(
            "Product's {pih_type} and categorized_strings contain information not provided by the other. {pih_type} is {pih_value} while `{category}` is `{categorized_string}`"
        ),
        instance_path: path.to_string(),
    })
}

fn create_invalid_product_identification_helper_warning(pih_type: &str, path: &str) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: format!("Product's product identification helper contains an invalid {pih_type}"),
        instance_path: path.to_string(),
    })
}

#[derive(Debug, Default)]
struct CategorizedStrings {
    product_name: Option<String>,
    product_version: Option<String>,
    product_version_range: Option<String>,
    language: Option<String>,
    vendor: Option<String>,
    service_pack: Option<String>,
    architecture: Option<String>,
}

fn value_approximately_matches(pih_value: &str, cat_str_value: &str) -> bool {
    cat_str_value
        .to_lowercase()
        .replace('_', " ")
        .contains(&pih_value.to_lowercase().replace('_', " "))
}

impl CategorizedStrings {
    fn add_value(&mut self, category: &str, value: &str) {
        match category {
            "product_name" => self.product_name = Some(value.to_string()),
            "product_version" => self.product_version = Some(value.to_string()),
            "product_version_range" => self.product_version_range = Some(value.to_string()),
            "language" => self.language = Some(value.to_string()),
            "vendor" => self.vendor = Some(value.to_string()),
            "service_pack" => self.service_pack = Some(value.to_string()),
            "architecture" => self.architecture = Some(value.to_string()),
            // Ignore invalid or unchecked categories
            _ => (),
        }
    }

    /// Checks whether:
    /// * A set categorized string contains the same information as the CPE, if the CPE's attribute
    ///   is not ANY
    /// * A set CPE attribute contains the same information as the corresponding categorized_string
    /// * If either CPE or categorized string are set and the corresponding value is missing
    fn check_cpe(&self, cpe: &impl Cpe, path: &str) -> Vec<TestFinding> {
        let mut test_findings = Vec::new();

        macro_rules! check_attribute {
            ($category:ident, $cpe_attr:ident, $cat_str:expr, $any:expr) => {
                if let Some(val) = self.$category.as_ref()
                    && !value_approximately_matches(&format!("{:#}", cpe.$cpe_attr()), val)
                {
                    test_findings.push(create_inconsistent_product_identification_helper_warning(
                        "CPE",
                        $cat_str,
                        &cpe.$cpe_attr().to_string(),
                        val,
                        &format!("{path}/product_identification_helper/cpe"),
                    ));
                } else if self.$category.is_none() && cpe.$cpe_attr() != $any {
                    test_findings.push(create_missing_product_identification_helper_field_warning(
                        "CPE",
                        $cat_str,
                        "Missing",
                        &cpe.$cpe_attr().to_string(),
                        &format!("{path}/product_identification_helper/cpe"),
                    ));
                }
            };
        }

        check_attribute!(product_name, product, "product_name", cpe::component::Component::Any);
        if let Some(product_version) = self.product_version.as_ref()
            && self.product_version_range.is_none()
            && product_version != &cpe.version().to_string()
        {
            test_findings.push(create_inconsistent_product_identification_helper_warning(
                "CPE",
                "product_version",
                &cpe.version().to_string(),
                product_version,
                &format!("{path}/product_identification_helper/cpe"),
            ));
        } else if cpe.version() != cpe::component::Component::Any
            && let Some(version_range) = self.product_version_range.as_ref()
        // TODO: Switch to VLS/VERS once PR #1150 is merged, see
        // https://github.com/oasis-tcs/csaf/blob/master/csaf_2.1/prose/share/csaf-v2.1-draft.md#31332-branches-type---name-under-product-version-range
        // and #1337
        {
            let version_range_parsed = VersionRange::parse(version_range);
            let version_range_constraints = match version_range_parsed {
                VersionRange::DynamicVers(dynamic_version_range) => dynamic_version_range
                    .constraints()
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join("|"),
                VersionRange::Vls(vls) => vls.to_string(),
                VersionRange::Invalid(content) => content.to_string(),
            };
            if version_range_constraints != cpe.version().to_string() {
                test_findings.push(create_inconsistent_product_identification_helper_warning(
                    "CPE",
                    "product_version_range",
                    &cpe.version().to_string(),
                    version_range,
                    &format!("{path}/product_identification_helper/cpe"),
                ));
            }
        } else if (self.product_version.is_none() && self.product_version_range.is_none())
            && cpe.version() != cpe::component::Component::Any
        {
            test_findings.push(create_missing_product_identification_helper_field_warning(
                "CPE",
                "product_version or product_version_range",
                "Missing",
                &cpe.version().to_string(),
                &format!("{path}/product_identification_helper/cpe"),
            ));
        }

        check_attribute!(language, language, "language", &cpe::cpe::Language::Any);
        check_attribute!(vendor, vendor, "vendor", cpe::component::Component::Any);
        check_attribute!(service_pack, update, "service_pack", cpe::component::Component::Any);
        check_attribute!(architecture, target_hw, "architecture", cpe::component::Component::Any);

        test_findings
    }

    /// Checks whether:
    /// * A set categorized string contains the same information as the PURL
    /// * If the categorized string is set and the corresponding PURL value is missing
    fn check_purl(&self, purl: &GenericPurl<String>, path: &str, idx: usize) -> Vec<TestFinding> {
        let mut test_findings = Vec::new();

        if let Some(product_name) = self.product_name.as_ref()
            && !value_approximately_matches(purl.name(), product_name)
        {
            test_findings.push(create_inconsistent_product_identification_helper_warning(
                "PURL",
                "product_name",
                purl.name(),
                product_name,
                &format!("{path}/product_identification_helper/purls/{idx}"),
            ));
        }
        if let Some(product_version) = self.product_version.as_ref()
            && self.product_version_range.is_none()
            && let Some(version) = purl.version()
            && product_version != version
        {
            test_findings.push(create_inconsistent_product_identification_helper_warning(
                "PURL",
                "product_version",
                version,
                product_version,
                &format!("{path}/product_identification_helper/purls/{idx}"),
            ));
        } else if let Some(product_version) = self.product_version.as_ref()
            && purl.version().is_none()
        {
            test_findings.push(create_missing_product_identification_helper_field_warning(
                "PURL",
                "product_version",
                product_version,
                "Missing",
                &format!("{path}/product_identification_helper/purls/{idx}"),
            ));
        }
        if let Some(product_version_range) = self.product_version_range.as_ref()
            && let Some(version) = purl.version()
            // TODO: Switch to VLS/VERS once #1150 is merged, see
            // https://github.com/oasis-tcs/csaf/blob/master/csaf_2.1/prose/share/csaf-v2.1-draft.md#31332-branches-type---name-under-product-version-range
            && product_version_range != version
        {
            test_findings.push(create_inconsistent_product_identification_helper_warning(
                "PURL",
                "product_version_range",
                version,
                product_version_range,
                &format!("{path}/product_identification_helper/purls/{idx}"),
            ));
        } else if let Some(product_version_range) = self.product_version_range.as_ref()
            && purl.version().is_none()
        {
            test_findings.push(create_missing_product_identification_helper_field_warning(
                "PURL",
                "product_version_range",
                product_version_range,
                purl.version().unwrap_or("Missing"),
                &format!("{path}/product_identification_helper/purls/{idx}"),
            ));
        }
        test_findings
    }
}

/// This functions normalizes a URI CPE to only use lower case schema and part. This function
/// assumes it is handed a CSAF 2.1 valid CPE
fn normalize_uri_cpe(uri: &str) -> String {
    let prefix_length = uri.len().min(6);
    format!("{}{}", uri[..prefix_length].to_ascii_lowercase(), &uri[prefix_length..])
}

/// 6.2.42 Inconsistent Product Identification Helper
///
/// For each product identification helper which resides in branches, it SHALL be tested that the
/// product identification helper contain at least the same information as the categorized strings.
/// Information that cannot be represented in the specific product identification helper SHALL be
/// omitted from the comparison.
///
/// This check currently matches product_name, product_version and product_version_range for
/// PURLs. In addition vendor, service_pack, architecture and language are checked for CPEs.
pub fn test_6_2_42_inconsistent_product_identification_helper(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let Some(product_tree) = doc.get_product_tree() else {
        return Ok(());
    };
    let findings = product_tree
        .collect_leaf_paths()
        .iter()
        .flat_map(|(branches, idxs)| {
            let Some(pih) = branches.last().and_then(|branch| {
                branch
                    .get_product()
                    .and_then(|product| product.get_product_identification_helper())
            }) else {
                // No PIHs available, nothing to test
                return Vec::new();
            };
            let mut categorized_strings = CategorizedStrings::default();
            for branch in branches {
                categorized_strings.add_value(branch.get_category().as_ref(), branch.get_name());
            }
            let leaf_path = build_leaf_instance_path(idxs);
            let mut inner_test_findings = Vec::new();
            if let Some(cpe) = pih.get_cpe() {
                // CPE 2.3 WFN is not supported by CSAF
                if cpe.as_str().starts_with("cpe:2.3:") {
                    if let Ok(fsb) = Fsb::parse(cpe.as_str()) {
                        inner_test_findings.extend(categorized_strings.check_cpe(&fsb, &leaf_path));
                    } else {
                        inner_test_findings
                            .push(create_invalid_product_identification_helper_warning("CPE", &leaf_path));
                    };
                } else {
                    let cpe = normalize_uri_cpe(cpe.as_str());
                    if let Ok(uri) = Uri::parse(&cpe) {
                        inner_test_findings.extend(categorized_strings.check_cpe(&uri, &leaf_path));
                    } else {
                        inner_test_findings
                            .push(create_invalid_product_identification_helper_warning("CPE", &leaf_path));
                    };
                }
            }
            if let Some(purls) = pih.get_purls() {
                purls.iter().enumerate().for_each(|(idx, purl)| {
                    if let CsafPurl::Valid(purl) = purl
                        && let Ok(purl) = purl::GenericPurl::<String>::from_str(purl.original_purl())
                    {
                        inner_test_findings.extend(categorized_strings.check_purl(&purl, &leaf_path, idx))
                    } else {
                        inner_test_findings
                            .push(create_invalid_product_identification_helper_warning("PURL", &leaf_path));
                    }
                });
            }
            inner_test_findings
        })
        .collect::<Vec<_>>();
    if findings.is_empty() { Ok(()) } else { Err(findings) }
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
        let cpe_contains_language_and_service_pack_but_categorized_strings_dont = Err(vec![
            create_missing_product_identification_helper_field_warning(
                "CPE",
                "language",
                "Missing",
                "de",
                "/product_tree/branches/0/branches/0/branches/0/product/product_identification_helper/cpe",
            ),
            create_missing_product_identification_helper_field_warning(
                "CPE",
                "service_pack",
                "Missing",
                "PL17",
                "/product_tree/branches/0/branches/0/branches/1/product/product_identification_helper/cpe",
            ),
        ]);

        let purl_contains_architecture_and_version_but_categorized_strings_dont = Err(vec![
            create_inconsistent_product_identification_helper_warning(
                "PURL",
                "product_name",
                "debian",
                "curl",
                "/product_tree/branches/0/branches/0/branches/1/product/product_identification_helper/purls/1",
            ),
            create_missing_product_identification_helper_field_warning(
                "PURL",
                "product_version",
                "7.88.1-10+deb12u12",
                "Missing",
                "/product_tree/branches/0/branches/0/branches/0/product/product_identification_helper/purls/1",
            ),
            create_inconsistent_product_identification_helper_warning(
                "PURL",
                "product_version",
                "8.13.0-5~bpo12+1+deb12u12",
                "8.13.0-5~bpo12+1",
                "/product_tree/branches/0/branches/0/branches/1/product/product_identification_helper/purls/1",
            ),
            create_inconsistent_product_identification_helper_warning(
                "PURL",
                "product_version",
                "8.13.0-5~bpo12+1+deb12u12",
                "8.13.0-5~bpo12+1",
                "/product_tree/branches/0/branches/0/branches/1/product/product_identification_helper/purls/2",
            ),
            create_inconsistent_product_identification_helper_warning(
                "PURL",
                "product_version",
                "7.1.0-5",
                "8.13.0-5~bpo12+1",
                "/product_tree/branches/0/branches/0/branches/1/product/product_identification_helper/purls/3",
            ),
        ]);

        let cpe_lists_version_but_categorized_string_is_version_range_and_cpe_contains_missmatching_version_number =
            Err(vec![
                create_inconsistent_product_identification_helper_warning(
                    "CPE",
                    "product_version_range",
                    "2.2.0",
                    "vers:intdot/<=2.2.0",
                    "/product_tree/branches/0/branches/0/branches/0/product/product_identification_helper/cpe",
                ),
                create_inconsistent_product_identification_helper_warning(
                    "CPE",
                    "product_version_range",
                    "<3,16",
                    ">3,16",
                    "/product_tree/branches/0/branches/1/branches/0/product/product_identification_helper/cpe",
                ),
            ]);

        TESTS_2_1.test_6_2_42.expect(ExpectedResults {
            case_01: cpe_contains_language_and_service_pack_but_categorized_strings_dont,
            case_02: purl_contains_architecture_and_version_but_categorized_strings_dont,
            case_03:
                cpe_lists_version_but_categorized_string_is_version_range_and_cpe_contains_missmatching_version_number,
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
        });
    }
}
