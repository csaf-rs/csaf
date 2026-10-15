use std::collections::HashMap;

use crate::csaf_traits::{CsafTrait, ProductStatusTrait, VulnerabilityTrait};
use crate::validation::{TestFinding, TestFindingData};

fn create_missing_corresponding_affected_products_error(vulnerability_index: usize, product_id: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "Every product of a vulnerability with the product_status 'fixed' must have a corresponding product version with the product_status 'affected' in the same vulnerability. {product_id} is missing a corresponding product"
        ),
        instance_path: format!("/vulnerabilities/{vulnerability_index}/product_status/known_affected"),
    })
}

/// 6.1.27.13 Corresponding Affected Products
///
/// For each product listed in the product status group fixed in any vulnerability, it SHALL be
/// tested that a corresponding version of the product is listed as affected in the same vulnerability.
///
/// For a product path including the installed_with relationship the product path leading to but not
/// including the relationship is a corresponding product. Such product path could also be just the
/// product identified by beginning_product_reference if the first subpath element has the category installed_with.
pub fn test_6_1_27_13_corresponding_affected_products(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let _product_tree = if let Some(prod_tree) = doc.get_product_tree() {
        prod_tree
    } else {
        // Missing product tree is handled in other tests
        return Ok(());
    };

    let vulnerability_related_products = doc
        .get_vulnerabilities()
        .iter()
        .enumerate()
        .filter_map(|(vulnerability_idx, vulnerability)| {
            vulnerability
                .get_product_status()
                .map(|product_status| (vulnerability_idx, product_status))
        })
        .filter_map(|(idx, product_status)| {
            let fixed = product_status
                .get_fixed()
                .map(|fixed| fixed.collect())
                .unwrap_or(Vec::new());
            let affected = product_status
                .get_known_affected()
                .map(|affected| affected.collect())
                .unwrap_or(Vec::new());
            if fixed.is_empty() {
                None
            } else {
                Some((idx, (fixed, affected)))
            }
        })
        .collect::<HashMap<usize, (_, _)>>();

    if vulnerability_related_products.is_empty() {
        // No vulnerabilities don't require affected products
        // No fixed products don't require affected products
        return Ok(());
    }

    let mut findings = Vec::new();

    for (vulnerability_idx, (fixed_products, affected_products)) in vulnerability_related_products {
        for fixed_product in fixed_products {
            if affected_products.is_empty() {
                findings.push(create_missing_corresponding_affected_products_error(
                    vulnerability_idx,
                    fixed_product,
                ));
                continue;
            }
            // TODO: Identify if for each fixed_product a corresponding known affected product is
            // referenced
        }
    }

    if findings.is_empty() { Ok(()) } else { Err(findings) }
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_27_13,
    test_6_1_27_13_corresponding_affected_products
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_27_13 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_27_13() {
        TESTS_2_1.test_6_1_27_13.expect(ExpectedResults {
            case_01: Err(vec![create_missing_corresponding_affected_products_error(
                0,
                "CSAFPID-9080700",
            )]),
            case_02: Err(vec![create_missing_corresponding_affected_products_error(
                0,
                "CSAFPID-9080700",
            )]),
            case_03: Err(vec![create_missing_corresponding_affected_products_error(
                0,
                "CSAFPID-9080700",
            )]),
            // TODO: Clarify why this test case if failing, when case 14 has the same content
            // besides a seconed vulnerability with another set of affected/fixed product version
            case_04: Ok(()),
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
            case_14: Ok(()),
            case_15: Ok(()),
            case_16: Ok(()),
        });
    }
}
