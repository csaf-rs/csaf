use crate::csaf::types::purl::csaf_purl::CsafPurl::{Invalid, Valid};
use crate::csaf_traits::{CsafTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait};
use crate::validation::ValidationError;
use std::collections::HashMap;

fn create_purl_consistency_error(path: &str, index: usize) -> ValidationError {
    ValidationError {
        message: String::from("PURLs within the same product_identification_helper must only differ in qualifiers"),
        instance_path: format!("{path}/product_identification_helper/purls/{index}"),
    }
}

/// 6.1.42 PURL Consistency
/// Checks the consistency of PURLs within the same product_identification_helper. PURLs must only differ in qualifiers.
pub fn test_6_1_42_purl_consistency(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    if let Some(product_tree) = doc.get_product_tree() {
        product_tree.visit_all_products(&mut |product, path| {
            if let Some(helper) = product.get_product_identification_helper()
                && let Some(purls) = helper.get_purls()
            {
                // break early if there are 0 or 1 PURLs, as consistency is not an issue
                if purls.len() <= 1 {
                    return;
                }

                let mut bases_map: Option<HashMap<String, Vec<usize>>> = None;

                for (i, purl) in purls.into_iter().enumerate() {
                    // check purl validation result
                    match purl {
                        Valid(mut p) => {
                            bases_map
                                // create hashmap if it does not exist
                                .get_or_insert_default()
                                // create entry for base if it does not exist
                                .entry(p.clear_qualifiers().to_string())
                                // create vec if it does not exist
                                .or_default()
                                // push path index into vec
                                .push(i);
                        },
                        Invalid(_) => {
                            // ToDo #409 create percondition failed warning
                            continue;
                        },
                    };
                }

                // if there were any valid purls
                if let Some(bases) = bases_map {
                    // Collect values and sort by length descending
                    let mut sorted_values: Vec<Vec<usize>> = bases.into_values().collect();
                    // Sort by group size descending, then by first index ascending for determinism
                    sorted_values.sort_by(|a, b| b.len().cmp(&a.len()).then_with(|| a[0].cmp(&b[0])));

                    // If there is more than one group, the PURLs differ in more than qualifiers.
                    // Skip the first (largest) group and report errors for all indices in the remaining groups.
                    for group in sorted_values.iter().skip(1) {
                        for &i in group {
                            errors
                                .get_or_insert_default()
                                .push(create_purl_consistency_error(path, i));
                        }
                    }
                }
            }
        });
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_42, test_6_1_42_purl_consistency);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_42() {
        TESTS_2_1.test_6_1_42.expect(
            Err(vec![create_purl_consistency_error(
                "/product_tree/full_product_names/0",
                1,
            )]),
            Err(vec![create_purl_consistency_error(
                "/product_tree/branches/0/branches/0/branches/0/product",
                2,
            )]),
            Err(vec![create_purl_consistency_error(
                "/product_tree/full_product_names/0",
                1,
            )]),
            Ok(()),
            Ok(()),
        );
    }
}
