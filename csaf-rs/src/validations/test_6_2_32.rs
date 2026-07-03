use crate::csaf::traits::vulnerabilities::{
    file_hash_trait::FileHashTrait, hash_trait::HashTrait, product_ident_helper_trait::ProductIdentificationHelperTrait,
};
use crate::csaf_traits::{CsafTrait, ProductTrait, ProductTreeTrait};
use crate::validation::ValidationError;
use std::collections::HashMap;

fn generate_duplicate_helper_error(category: &str, value: &str, product_id: &str, base_path: &str) -> ValidationError {
    ValidationError {
        message: format!(
            "The Product Identification Helper property '{category}' contains a duplicate value '{value}' for product '{product_id}'. Helper properties must be pairwise disjoint across all distinct products."
        ),
        instance_path: format!("{base_path}/product_identification_helper/{category}"),
    }
}

/// Test 6.2.32: Use of Same Product Identification Helper for Different Products
pub fn test_6_2_32_duplicate_product_identification_helpers(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let Some(product_tree) = doc.get_product_tree() else {
        return Ok(());
    };

    let mut errors: Vec<ValidationError> = vec![];

    // Grouping tracking maps: map a unique token to a list of (product_id, base_instance_path)
    let mut purl_groups: HashMap<String, Vec<(String, String)>> = HashMap::new();
    let mut sku_groups: HashMap<String, Vec<(String, String)>> = HashMap::new();
    let mut sn_groups: HashMap<String, Vec<(String, String)>> = HashMap::new();
    let mut mn_groups: HashMap<String, Vec<(String, String)>> = HashMap::new();
    let mut hash_groups: HashMap<String, Vec<(String, String)>> = HashMap::new();

    // 1. Collect all occurrences using zero-copy / minimal lifetime allocations where possible
    product_tree.visit_all_products(&mut |product, instance_path| {
        let product_id = product.get_product_id().to_string();
        let path_str = instance_path.to_string();

        if let Some(helper) = product.get_product_identification_helper() {
            // Collect PURLs
            if let Some(purls) = helper.get_purls() {
                for purl in purls {
                    let key = format!("{purl:?}"); // format necessary due to enum debug representation
                    purl_groups
                        .entry(key)
                        .or_default()
                        .push((product_id.clone(), path_str.clone()));
                }
            }

            // Collect SKUs - using direct string allocation fallback
            for sku in helper.get_skus() {
                sku_groups
                    .entry(sku.to_string())
                    .or_default()
                    .push((product_id.clone(), path_str.clone()));
            }

            // Collect Serial Numbers - optimization: use the internal reference if available
            if let Some(serial_numbers) = helper.get_serial_numbers() {
                for sn in serial_numbers {
                    sn_groups
                        .entry(sn.to_string())
                        .or_default()
                        .push((product_id.clone(), path_str.clone()));
                }
            }

            // Collect Model Numbers
            if let Some(model_numbers) = helper.get_model_numbers() {
                for mn in model_numbers {
                    mn_groups
                        .entry(mn.to_string())
                        .or_default()
                        .push((product_id.clone(), path_str.clone()));
                }
            }

            // Collect Hashes
            for hash_obj in helper.get_hashes() {
                let filename = hash_obj.get_filename();

                // Instead of one combined string, track each inner hash independently:
                for fh in hash_obj.get_file_hashes() {
                    let specific_hash_key =
                        format!("file:{filename};alg:{:?};value:{}", fh.get_algorithm(), fh.get_hash());
                    hash_groups
                        .entry(specific_hash_key)
                        .or_default()
                        .push((product_id.clone(), path_str.clone()));
                }
            }
        }
    });

    // 2. Process groups and emit independent errors for every product variant that collides
    let mut process_violations = |groups: HashMap<String, Vec<(String, String)>>, category: &str| {
        for (value, occurrences) in groups {
            if occurrences.len() > 1 {
                for (product_id, path) in occurrences {
                    errors.push(generate_duplicate_helper_error(category, &value, &product_id, &path));
                }
            }
        }
    };

    process_violations(purl_groups, "purls");
    process_violations(sku_groups, "skus");
    process_violations(sn_groups, "serial_numbers");
    process_violations(mn_groups, "model_numbers");
    process_violations(hash_groups, "hashes");

    // Sort and deduplicate to ensure deterministic error returns
    // even if multiple hashes for the same file collide simultaneously.
    errors.sort_by(|a, b| a.instance_path.cmp(&b.instance_path).then(a.message.cmp(&b.message)));
    errors.dedup_by(|a, b| a.instance_path == b.instance_path && a.message == b.message);

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_32,
    test_6_2_32_duplicate_product_identification_helpers
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_32() {
        // Case 01: Both colliding products should flag an error independently
        let mut case_01_errors = vec![
            generate_duplicate_helper_error(
                "serial_numbers",
                "143-D-354",
                "CSAFPID-908070601",
                "/product_tree/branches/0/branches/0/branches/0/product",
            ),
            generate_duplicate_helper_error(
                "serial_numbers",
                "143-D-354",
                "CSAFPID-908070602",
                "/product_tree/branches/0/branches/0/branches/1/product",
            ),
        ];
        case_01_errors.sort_by_key(|e| e.instance_path.clone());

        // Case 02: Model number collisions cross-flagged on both variants
        let mut case_02_errors = vec![
            generate_duplicate_helper_error(
                "model_numbers",
                "143-D-354",
                "CSAFPID-908070601",
                "/product_tree/branches/0/branches/0/branches/0/product",
            ),
            generate_duplicate_helper_error(
                "model_numbers",
                "143-D-354",
                "CSAFPID-908070602",
                "/product_tree/branches/0/branches/1/branches/0/product",
            ),
        ];
        case_02_errors.sort_by_key(|e| e.instance_path.clone());

        // Case 03: Corrected structural runtime paths matching the schema generation target
        let mut case_03_errors = vec![
            generate_duplicate_helper_error(
                "model_numbers",
                "143-D-354",
                "CSAFPID-908070602",
                "/product_tree/branches/0/branches/1/branches/0/product",
            ),
            generate_duplicate_helper_error(
                "model_numbers",
                "143-D-354",
                "CSAFPID-908070603",
                "/product_tree/full_product_names/0",
            ),
            generate_duplicate_helper_error(
                "model_numbers",
                "143-D-354",
                "CSAFPID-908070605",
                "/product_tree/relationships/0/full_product_name",
            ),
        ];
        case_03_errors.sort_by_key(|e| e.instance_path.clone());

        TESTS_2_1.test_6_2_32.expect(
            Err(case_01_errors),
            Err(case_02_errors),
            Err(case_03_errors),
            Ok(()),
            Ok(()),
        );
    }
}
