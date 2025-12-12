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
        // Generate a hashmap of unique flag labels to product IDs in a hashset, which guarantees uniqueness
        let mut flag_to_product_ids_map: HashMap<LabelOfTheFlag, HashSet<String>> = HashMap::new();
        if let Some(flags) = vulnerability.get_flags() {
            for flag in flags.iter() {
                let entry = flag_to_product_ids_map.entry(flag.get_label()).or_default();

                if let Some(product_ids) = flag.get_product_ids() {
                    entry.extend(product_ids.map(|id| id.to_string()));
                }

                if let Some(group_ids) = flag.get_group_ids() {
                    if let Some(resolved_product_ids) = resolve_product_groups(doc, group_ids.into_iter()) {
                        entry.extend(resolved_product_ids.into_iter());
                    }
                }
            }
        }

        // Invert the hashmap to map unique product IDs to flag labels
        let mut product_id_to_flags: HashMap<String, Vec<LabelOfTheFlag>> = HashMap::new();

        for (label, product_ids) in flag_to_product_ids_map {
            for product_id in product_ids.iter() {
                product_id_to_flags.entry(product_id.clone()).or_default().push(label);
            }
        }

        // Collect errors for products that appear in multiple flag labels
        for (product_id, labels) in product_id_to_flags {
            if labels.len() > 1 {
                errors
                    .get_or_insert_with(Vec::new)
                    .push(test_6_1_33_err_generator(&product_id, &labels, vuln_i));
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

fn test_6_1_33_err_generator(product_id: &str, labels: &[LabelOfTheFlag], vuln_i: usize) -> ValidationError {
    let mut labels_str_vec = labels.iter().map(|l| l.to_string()).collect::<Vec<_>>();
    labels_str_vec.sort();
    let labels_str = labels_str_vec.join(", ");
    ValidationError {
        message: format!(
            "Product '{}' is associated with multiple flag labels: [{}], potentially via groups",
            product_id, labels_str
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
