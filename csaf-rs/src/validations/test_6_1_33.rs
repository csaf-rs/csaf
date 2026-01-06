use crate::csaf_traits::{CsafTrait, FlagTrait, VulnerabilityTrait, WithOptionalGroupIds, WithOptionalProductIds};
use crate::csaf2_1::schema::LabelOfTheFlag;
use crate::helpers::resolve_product_groups;
use crate::validation::ValidationError;
use std::collections::HashMap;

/// 6.1.33 Multiple Flags with VEX Justification Codes per Product
pub fn test_6_1_33_multiple_flags_with_vex_codes_per_product(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    // Check each flag in each vulnerability
    for (vuln_i, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        // Generate a hashmap of product IDs to flag labels, flag indices, and group ids
        let mut product_id_to_flags_map: HashMap<String, Vec<(LabelOfTheFlag, usize, Option<&String>)>> = HashMap::new();
        // if there are flags for the vuln, iterate over them
        if let Some(flags) = vulnerability.get_flags() {
            for (flag_i, flag) in flags.iter().enumerate() {
                let label = flag.get_label();

                // add all product ids to the product_id_to_flags map (no group ids for direct product ids)
                if let Some(product_ids) = flag.get_product_ids() {
                    for product_id in product_ids {
                        product_id_to_flags_map
                            .entry(product_id.to_string())
                            .or_default()
                            .push((label, flag_i, None));
                    }
                }

                // iterate over all group ids, resolve each group id separately
                if let Some(group_ids) = flag.get_group_ids() {
                    for group_id in group_ids {
                        if let Some(resolved_product_ids) = resolve_product_groups(doc, [group_id].into_iter()) {
                            // add the resolved product ids to the product_id_to_flags map with group id
                            for product_id in resolved_product_ids {
                                product_id_to_flags_map
                                    .entry(product_id)
                                    .or_default()
                                    .push((label, flag_i, Some(group_id)));
                            }
                        }
                    }
                }
            }
        }

        // Collect errors for products that appear in multiple flag labels
        for (product_id, flag_flag_i_group_ids_arr) in product_id_to_flags_map {
            // Extract unique labels from the (label, flag_i, group_ids) tuples
            if flag_flag_i_group_ids_arr.len() > 1 {
                // extract all labels
                let labels: Vec<LabelOfTheFlag> = flag_flag_i_group_ids_arr.iter().map(|(label, _, _)| *label).collect();
                // generate error
                for (label, flag_i, group_id) in flag_flag_i_group_ids_arr {
                    errors.get_or_insert_with(Vec::new).push(test_6_1_33_err_generator(
                        &product_id,
                        &labels,
                        label,
                        group_id,
                        vuln_i,
                        flag_i
                    ));
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

fn test_6_1_33_err_generator(
    product_id: &str,
    labels: &[LabelOfTheFlag],
    label: LabelOfTheFlag,
    group_id: Option<&String>,
    vuln_i: usize,
    flag_i: usize
) -> ValidationError {
    let labels_joined = {
        let mut labels_str: Vec<_> = labels.iter().map(|l| l.to_string()).collect();
        labels_str.sort();
        labels_str.join(", ")
    };
    let group_id_str = {
        if let Some(group_id) = group_id {
            format!("(via group: {})", group_id)
        } else {
            "".to_string()
        }
    };
    ValidationError {
        message: format!(
            "Product '{}' is associated with multiple flag labels: [{}] {}, it has flag label {} on this path",
            product_id, labels_joined, group_id_str, label
        ),
        instance_path: format!("/vulnerabilities/{}/flags/{}", vuln_i, flag_i),
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
                LabelOfTheFlag::ComponentNotPresent,
                Option::from(&"CSAFGID-0001".to_string()),
                0,
                0
            ),
                 test_6_1_33_err_generator(
                     "CSAFPID-9080700",
                     &[
                         LabelOfTheFlag::ComponentNotPresent,
                         LabelOfTheFlag::VulnerableCodeCannotBeControlledByAdversary,
                     ],
                     LabelOfTheFlag::VulnerableCodeCannotBeControlledByAdversary,
                     None,
                     0,
                     1
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
