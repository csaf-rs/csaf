use crate::csaf_traits::{CsafTrait, DocumentCategory, ProductStatusTrait, VulnerabilityTrait};
use crate::helpers::resolve_product_groups;
use crate::validation::ValidationError;
use csaf_macros::profile_test_applies_to_category;
use std::collections::{HashMap, HashSet};

/// 6.1.27.10 Action Statement
///
/// This test only applies to documents with `/document/category` with value `csaf_vex`.
///
/// Each item in `/vulnerabilities[]/product_status/known_affected` must have a corresponding
/// action statement in `/vulnerabilities[]/remediations`
#[profile_test_applies_to_category(
    all = [CsafVex],
)]
pub fn test_6_1_27_10_action_statement(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let vulnerabilities = doc.get_vulnerabilities();

    // Only execute this test if there are any vulnerabilities present
    if vulnerabilities.is_empty() {
        return Ok(());
    }

    let mut errors: Option<Vec<ValidationError>> = None;
    // for each vulnerability
    for (v_i, vulnerability) in vulnerabilities.iter().enumerate() {
        // generate hashmap of all known_affected product or group ids with value of known_not_affected path index
        let mut known_affected_product_or_group_ids: HashMap<String, usize> = HashMap::new();
        if let Some(product_status) = vulnerability.get_product_status() {
            if let Some(known_affected) = product_status.get_known_affected() {
                for (kna_i, known_affected_entry) in known_affected.into_iter().enumerate() {
                    known_affected_product_or_group_ids.insert(known_affected_entry.to_owned(), kna_i);
                }
            }
        }

        // generate all unique product ids and group ids found in remediations
        let mut found_product_ids: HashSet<String> = HashSet::new();
        let mut found_group_ids: HashSet<String> = HashSet::new();
        found_product_ids.extend(
            vulnerability
                .get_remediations_product_references()
                .iter()
                .map(|(product_id, _)| product_id.to_owned())
                .collect::<Vec<String>>(),
        );
        found_group_ids.extend(
            vulnerability
                .get_remediations_group_references()
                .iter()
                .map(|(group_id, _)| group_id.to_owned())
                .collect::<Vec<String>>(),
        );

        // merge the resolved product ids from group ids into the directly found product ids
        if let Some(resolved_product_ids) =
            resolve_product_groups(doc, &found_group_ids.into_iter().collect::<Vec<_>>())
        {
            found_product_ids.extend(resolved_product_ids.iter().map(|product_id| product_id.to_owned()));
        }

        // remove all found product ids from hashmap
        for product_id in found_product_ids.iter() {
            known_affected_product_or_group_ids.remove(product_id);
        }

        // generate errors for all remaining known_affected product or group ids
        for known_not_affected_product_or_group_id in known_affected_product_or_group_ids.iter() {
            errors.get_or_insert_with(Vec::new).push(test_6_1_27_10_err_generator(
                &doc.get_document().get_category(),
                known_not_affected_product_or_group_id.0.to_string(),
                v_i,
                *known_not_affected_product_or_group_id.1,
            ));
        }
    }

    errors.map_or(Ok(()), Err)
}

fn test_6_1_27_10_err_generator(
    document_category: &DocumentCategory,
    product_or_group_id: String,
    vuln_path_index: usize,
    known_affected_path_index: usize,
) -> ValidationError {
    ValidationError {
        message: format!(
            "In documents with category '{}', vulnerability product status 'known_affected' entries \
            must have a corresponding action statement in 'remediations'. \
            Found 'known_affected' product status entry '{}' without action statement.",
            document_category, product_or_group_id
        ),
        instance_path: format!(
            "/vulnerabilities/{}/product_status/known_not_affected/{}",
            vuln_path_index, known_affected_path_index
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_27_9() {
        let errors = HashMap::from([(
            "01",
            vec![test_6_1_27_10_err_generator(
                &DocumentCategory::CsafVex,
                "CSAFPID-9080702".to_string(),
                0,
                2,
            )],
        )]);
        run_csaf20_tests("27-10", test_6_1_27_10_action_statement, errors.clone());
        run_csaf21_tests("27-10", test_6_1_27_10_action_statement, errors);
    }
}
