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

// Private helper function instead of a capturing closure
fn process_violations(
    groups: HashMap<String, Vec<(String, String)>>,
    category: &str,
    errors: &mut Vec<ValidationError>,
) {
    for (value, occurrences) in groups {
        if occurrences.len() > 1 {
            for (product_id, path) in occurrences {
                errors.push(generate_duplicate_helper_error(category, &value, &product_id, &path));
            }
        }
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
                    // Safe string conversion without using the Debug {:?} trait token format
                    let alg_str = format!("{}", fh.get_algorithm()).to_lowercase();
                    let hash_val = fh.get_hash().to_lowercase();

                    let specific_hash_key = format!("file:{filename};alg:{alg_str};value:{hash_val}");
                    hash_groups
                        .entry(specific_hash_key)
                        .or_default()
                        .push((product_id.clone(), path_str.clone()));
                }
            }
        }
    });

    process_violations(purl_groups, "purls", &mut errors);
    process_violations(sku_groups, "skus", &mut errors);
    process_violations(sn_groups, "serial_numbers", &mut errors);
    process_violations(mn_groups, "model_numbers", &mut errors);
    process_violations(hash_groups, "hashes", &mut errors);

    // Simplified deduplication leveraging implemented PartialEq trait
    errors.dedup();

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

    #[test]
    fn test_test_6_2_32_edge_cases() {
        use crate::schema::csaf2_1::schema::{
            AlgorithmOfTheCryptographicHash, CommonSecurityAdvisoryFramework, CryptographicHashes,
            DocumentLevelMetaData, FileHash, Filename, FullProductNameT, HelperToIdentifyTheProduct, JsonSchema,
            PackageUrlRepresentation, ProductIdT, ProductTree, TextualDescriptionOfTheProduct,
            ValueOfTheCryptographicHash,
        };

        // Case 01: Hashes collision across independent full product helpers
        let case_01_doc = {
            let duplicate_hash = FileHash {
                algorithm: "sha256".parse::<AlgorithmOfTheCryptographicHash>().unwrap(),
                value: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                    .parse::<ValueOfTheCryptographicHash>()
                    .unwrap(),
            };
            let prod_a = FullProductNameT {
                product_id: "CSAFPID-HASH-FAIL-1".parse::<ProductIdT>().unwrap(),
                name: "Product A".parse::<TextualDescriptionOfTheProduct>().unwrap(),
                product_identification_helper: Some(HelperToIdentifyTheProduct {
                    hashes: vec![CryptographicHashes {
                        filename: "test_file.bin".parse::<Filename>().unwrap(),
                        file_hashes: vec![duplicate_hash.clone()],
                    }],
                    ..Default::default()
                }),
                x_extensions: None,
            };
            let prod_b = FullProductNameT {
                product_id: "CSAFPID-HASH-FAIL-2".parse::<ProductIdT>().unwrap(),
                name: "Product B".parse::<TextualDescriptionOfTheProduct>().unwrap(),
                product_identification_helper: Some(HelperToIdentifyTheProduct {
                    hashes: vec![CryptographicHashes {
                        filename: "test_file.bin".parse::<Filename>().unwrap(),
                        file_hashes: vec![duplicate_hash],
                    }],
                    ..Default::default()
                }),
                x_extensions: None,
            };
            CommonSecurityAdvisoryFramework {
                schema: JsonSchema::HttpsDocsOasisOpenOrgCsafCsafV21SchemaCsafJson,
                product_tree: Some(ProductTree {
                    full_product_names: vec![prod_a, prod_b],
                    ..Default::default()
                }),
                document: unsafe { std::mem::transmute([0u8; std::mem::size_of::<DocumentLevelMetaData>()]) },
                vulnerabilities: vec![],
                x_extensions: None,
            }
        };

        // Case 02: PURL metadata collisions cross-flagged on product variants
        let case_02_doc = {
            let duplicate_purl = "pkg:npm/csaf-validator@0.5.1"
                .parse::<PackageUrlRepresentation>()
                .unwrap();
            let prod_a = FullProductNameT {
                product_id: "CSAFPID-PURL-FAIL-1".parse::<ProductIdT>().unwrap(),
                name: "Product A".parse::<TextualDescriptionOfTheProduct>().unwrap(),
                product_identification_helper: Some(HelperToIdentifyTheProduct {
                    purls: Some(vec![duplicate_purl.clone()]),
                    ..Default::default()
                }),
                x_extensions: None,
            };
            let prod_b = FullProductNameT {
                product_id: "CSAFPID-PURL-FAIL-2".parse::<ProductIdT>().unwrap(),
                name: "Product B".parse::<TextualDescriptionOfTheProduct>().unwrap(),
                product_identification_helper: Some(HelperToIdentifyTheProduct {
                    purls: Some(vec![duplicate_purl]),
                    ..Default::default()
                }),
                x_extensions: None,
            };
            CommonSecurityAdvisoryFramework {
                schema: JsonSchema::HttpsDocsOasisOpenOrgCsafCsafV21SchemaCsafJson,
                product_tree: Some(ProductTree {
                    full_product_names: vec![prod_a, prod_b],
                    ..Default::default()
                }),
                document: unsafe { std::mem::transmute([0u8; std::mem::size_of::<DocumentLevelMetaData>()]) },
                vulnerabilities: vec![],
                x_extensions: None,
            }
        };

        // Case 03: Distinct disjoint identifiers matching clean validation target
        let case_03_doc = {
            let hash_a = FileHash {
                algorithm: "sha256".parse::<AlgorithmOfTheCryptographicHash>().unwrap(),
                value: "1111144298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                    .parse::<ValueOfTheCryptographicHash>()
                    .unwrap(),
            };
            let hash_b = FileHash {
                algorithm: "sha256".parse::<AlgorithmOfTheCryptographicHash>().unwrap(),
                value: "2222244298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                    .parse::<ValueOfTheCryptographicHash>()
                    .unwrap(),
            };
            let prod_a = FullProductNameT {
                product_id: "CSAFPID-OK-1".parse::<ProductIdT>().unwrap(),
                name: "Product A".parse::<TextualDescriptionOfTheProduct>().unwrap(),
                product_identification_helper: Some(HelperToIdentifyTheProduct {
                    hashes: vec![CryptographicHashes {
                        filename: "a.bin".parse::<Filename>().unwrap(),
                        file_hashes: vec![hash_a],
                    }],
                    purls: Some(vec![
                        "pkg:npm/csaf-core@1.0.0".parse::<PackageUrlRepresentation>().unwrap(),
                    ]),
                    ..Default::default()
                }),
                x_extensions: None,
            };
            let prod_b = FullProductNameT {
                product_id: "CSAFPID-OK-2".parse::<ProductIdT>().unwrap(),
                name: "Product B".parse::<TextualDescriptionOfTheProduct>().unwrap(),
                product_identification_helper: Some(HelperToIdentifyTheProduct {
                    hashes: vec![CryptographicHashes {
                        filename: "b.bin".parse::<Filename>().unwrap(),
                        file_hashes: vec![hash_b],
                    }],
                    purls: Some(vec![
                        "pkg:npm/csaf-utils@2.0.0".parse::<PackageUrlRepresentation>().unwrap(),
                    ]),
                    ..Default::default()
                }),
                x_extensions: None,
            };
            CommonSecurityAdvisoryFramework {
                schema: JsonSchema::HttpsDocsOasisOpenOrgCsafCsafV21SchemaCsafJson,
                product_tree: Some(ProductTree {
                    full_product_names: vec![prod_a, prod_b],
                    ..Default::default()
                }),
                document: unsafe { std::mem::transmute([0u8; std::mem::size_of::<DocumentLevelMetaData>()]) },
                vulnerabilities: vec![],
                x_extensions: None,
            }
        };

        test_6_2_32_duplicate_product_identification_helpers(&case_01_doc)
            .err()
            .expect("Case 01 should flag a duplicate hash validation error");

        test_6_2_32_duplicate_product_identification_helpers(&case_02_doc)
            .err()
            .expect("Case 02 should flag a duplicate purl validation error");

        test_6_2_32_duplicate_product_identification_helpers(&case_03_doc)
            .expect("Case 03 should pass validation for pairwise disjoint helpers");
    }
}
