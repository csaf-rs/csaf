use crate::csaf_traits::{CsafTrait, FlagTrait, VulnerabilityTrait, WithOptionalGroupIds, WithOptionalProductIds};
use crate::csaf2_1::schema::LabelOfTheFlag;
use crate::helpers::resolve_product_groups;
use crate::validation::ValidationError;
use std::collections::{HashMap, HashSet};

/// 6.1.33 Multiple Flags with VEX Justification Codes per Product
pub fn test_6_1_33_multiple_flags_with_vex_codes_per_product(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    // Check each flag in each vulnerability
    for (vuln_i, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        // Generate a hashmap of unique flag labels to product IDs in a hashset, which guarantees uniqueness of the product ids
        let mut flag_to_product_ids_map: HashMap<LabelOfTheFlag, HashSet<String>> = HashMap::new();
        // Generate a hashmap of product ids back to group ids and the flag labels for back-tracing in the error message
        let mut product_id_to_group_id_map: HashMap<String, HashSet<(String, LabelOfTheFlag)>> = HashMap::new();
        // if there are flags for the vuln, iterate over them
        if let Some(flags) = vulnerability.get_flags() {
            for flag in flags.iter() {
                // get entry or put empty entry for the flag in the flag -> product id map
                let entry = flag_to_product_ids_map.entry(flag.get_label()).or_default();

                // add all product ids to the product id hashset
                if let Some(product_ids) = flag.get_product_ids() {
                    entry.extend(product_ids.map(|id| id.to_string()));
                }

                // iterate over all group ids, resolve each group id seperatly
                if let Some(group_ids) = flag.get_group_ids() {
                    for group_id in group_ids {
                        if let Some(resolved_product_ids) = resolve_product_groups(doc, [group_id].into_iter()) {
                            // add the resolved group id to the product id hashset
                            entry.extend(resolved_product_ids.clone().into_iter());
                            // add product id -> group id + flag label mapping
                            for product_id in resolved_product_ids {
                                let product_to_group_id_entry =
                                    product_id_to_group_id_map.entry(product_id).or_default();
                                product_to_group_id_entry.insert((group_id.to_string(), flag.get_label()));
                            }
                        }
                    }
                }
            }
        }

        // Invert the flag -> product id hashmap to map unique product IDs to flag labels
        let mut product_id_to_flags: HashMap<String, Vec<LabelOfTheFlag>> = HashMap::new();
        for (label, product_ids) in flag_to_product_ids_map {
            for product_id in product_ids.iter() {
                product_id_to_flags.entry(product_id.clone()).or_default().push(label);
            }
        }

        // Collect errors for products that appear in multiple flag labels
        for (product_id, labels) in product_id_to_flags {
            if labels.len() > 1 {
                // from the back-tracing mapping, get the entries for this product id, filter on the multiple flags found
                let mut group_ids: Option<Vec<String>> = None;
                if let Some(group_ids_with_flags) = product_id_to_group_id_map.get(&product_id.clone()) {
                    group_ids = Some(
                        group_ids_with_flags
                            .iter()
                            .filter(|x| labels.contains(&x.1))
                            .map(|x| x.0.clone())
                            .collect(),
                    );
                }
                // generate error
                errors.get_or_insert_with(Vec::new).push(test_6_1_33_err_generator(
                    &product_id,
                    &labels,
                    group_ids,
                    vuln_i,
                ));
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

fn test_6_1_33_err_generator(
    product_id: &str,
    labels: &[LabelOfTheFlag],
    group_ids: Option<Vec<String>>,
    vuln_i: usize,
) -> ValidationError {
    let mut labels_str_vec = labels.iter().map(|l| l.to_string()).collect::<Vec<_>>();
    labels_str_vec.sort();
    let labels_str = labels_str_vec.join(", ");
    let group_ids_str: String;
    if let Some(group_ids) = group_ids {
        group_ids_str = format!(", via groups: {}", group_ids.join(", "));
    } else {
        group_ids_str = "".to_string();
    }
    ValidationError {
        message: format!(
            "Product '{}' is associated with multiple flag labels: [{}], {}",
            product_id, labels_str, group_ids_str
        ),
        instance_path: format!("/vulnerabilities/{}/flags", vuln_i),
    }
}

#[cfg(test)]
mod tests {
    use crate::csaf2_1::schema::LabelOfTheFlag;
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::validations::test_6_1_33::{
        test_6_1_33_err_generator, test_6_1_33_multiple_flags_with_vex_codes_per_product,
    };
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_33() {
        let errors = HashMap::from([(
            "01",
            vec![test_6_1_33_err_generator(
                "CSAFPID-9080700",
                &[
                    LabelOfTheFlag::ComponentNotPresent,
                    LabelOfTheFlag::VulnerableCodeCannotBeControlledByAdversary,
                ],
                Option::from(vec!["CSAFGID-0001".to_string()]),
                0,
            )],
        )]);
        run_csaf20_tests(
            "33",
            test_6_1_33_multiple_flags_with_vex_codes_per_product,
            errors.clone(),
        );
        run_csaf21_tests("33", test_6_1_33_multiple_flags_with_vex_codes_per_product, errors);
    }
}
