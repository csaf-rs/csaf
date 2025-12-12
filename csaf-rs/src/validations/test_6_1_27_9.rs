use crate::csaf_traits::{
    CsafTrait, DocumentCategory, DocumentTrait, ProductStatusTrait, ThreatTrait, VulnerabilityTrait,
    WithOptionalGroupIds, WithOptionalProductIds,
};
use crate::csaf2_1::schema::CategoryOfTheThreat;
use crate::helpers::resolve_product_groups;
use crate::validation::ValidationError;
use std::collections::{HashMap, HashSet};

/// 6.1.27.9 Impact Statement
///
/// This test only applies to documents with `/document/category` with value `csaf_vex`.
///
/// Each item in `/vulnerabilities[]/product_status/known_not_affected` must have a corresponding
/// impact statement in `/vulnerabilities[]/flags` or `/vulnerabilities[]/threats`. For impact statements under
/// `threats`, the category must be `impact`.
pub fn test_6_1_27_9_impact_statement(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let doc_category = doc.get_document().get_category();
    let vulnerabilities = doc.get_vulnerabilities();

    // Only execute this test for documents with category 'csaf_vex'
    // and if there are any vulnerabilities present
    if doc_category != DocumentCategory::CsafVex || vulnerabilities.is_empty() {
        return Ok(());
    }

    let mut errors: Option<Vec<ValidationError>> = None;
    // for each vulnerability
    for (v_i, vulnerability) in vulnerabilities.iter().enumerate() {
        // generate hashmap of all known_not_affected product or group ids with value of known_not_affected path index
        let mut known_not_affected_product_or_group_ids: HashMap<String, usize> = HashMap::new();
        if let Some(product_status) = vulnerability.get_product_status() {
            if let Some(known_not_affected) = product_status.get_known_not_affected() {
                for (kna_i, known_not_affected_entry) in known_not_affected.into_iter().enumerate() {
                    known_not_affected_product_or_group_ids.insert(known_not_affected_entry.to_owned(), kna_i);
                }
            }
        }

        // generate all unique product ids and group ids found in flags and threats with category impact
        let mut found_product_ids: HashSet<String> = HashSet::new();
        let mut found_group_ids: HashSet<String> = HashSet::new();

        // gather from flags
        found_group_ids.extend(
            vulnerability
                .get_flags_group_references()
                .iter()
                .map(|(group_id, _)| group_id.to_owned())
                .collect::<Vec<String>>(),
        );
        found_product_ids.extend(
            vulnerability
                .get_flags_product_references()
                .iter()
                .map(|(product_id, _)| product_id.to_owned())
                .collect::<Vec<String>>(),
        );

        // gather from threats with category impact
        for threat in vulnerability.get_threats().iter() {
            if threat.get_category() == CategoryOfTheThreat::Impact {
                if let Some(group_ids) = threat.get_group_ids() {
                    found_group_ids.extend(group_ids.map(|group_id| group_id.to_owned()).collect::<Vec<String>>());
                }
                if let Some(product_ids) = threat.get_product_ids() {
                    found_product_ids.extend(
                        product_ids
                            .map(|product_id| product_id.to_owned())
                            .collect::<Vec<String>>(),
                    );
                }
            }
        }

        // merge the resolved product ids from group ids into the directly found product ids
        if let Some(resolved_product_ids) =
            resolve_product_groups(doc, &found_group_ids.into_iter().collect::<Vec<_>>())
        {
            found_product_ids.extend(resolved_product_ids.iter().map(|product_id| product_id.to_owned()));
        }

        // remove all found product ids from known_not_affected_product_or_group_ids
        for product_id in found_product_ids.iter() {
            known_not_affected_product_or_group_ids.remove(product_id);
        }

        // generate errors for all remaining known_not_affected product or group ids
        for known_not_affected_group_id in known_not_affected_product_or_group_ids.iter() {
            errors.get_or_insert_with(Vec::new).push(test_6_1_27_9_err_generator(
                &doc_category,
                known_not_affected_group_id.0.to_string(),
                v_i,
                *known_not_affected_group_id.1,
            ));
        }
    }

    errors.map_or(Ok(()), Err)
}

fn test_6_1_27_9_err_generator(
    document_category: &DocumentCategory,
    product_or_group_id: String,
    vuln_path_index: usize,
    known_not_affected_path_index: usize,
) -> ValidationError {
    ValidationError {
        message: format!(
            "In documents with category '{}', vulnerability product status 'known_not_affected' entries \
            must have a corresponding impact statement in 'flags' or 'threats' with category 'impact'. \
            Found 'known_not_affected' product status entry '{}' without impact statement.",
            document_category, product_or_group_id
        ),
        instance_path: format!(
            "/vulnerabilities/{}/product_status/known_not_affected/{}",
            vuln_path_index, known_not_affected_path_index
        ),
    }
}

#[cfg(test)]
mod tests {
    use crate::csaf_traits::DocumentCategory;
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::validations::test_6_1_27_9::{test_6_1_27_9_err_generator, test_6_1_27_9_impact_statement};
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_27_9() {
        let errors = HashMap::from([
            (
                "01",
                vec![test_6_1_27_9_err_generator(
                    &DocumentCategory::CsafVex,
                    "CSAFPID-9080702".to_string(),
                    0,
                    2,
                )],
            ),
            (
                "02",
                vec![test_6_1_27_9_err_generator(
                    &DocumentCategory::CsafVex,
                    "CSAFPID-9080702".to_string(),
                    0,
                    2,
                )],
            ),
            (
                "03",
                vec![test_6_1_27_9_err_generator(
                    &DocumentCategory::CsafVex,
                    "CSAFPID-9080700".to_string(),
                    0,
                    0,
                )],
            ),
            (
                "04",
                vec![test_6_1_27_9_err_generator(
                    &DocumentCategory::CsafVex,
                    "CSAFPID-9080700".to_string(),
                    0,
                    0,
                )],
            ),
            (
                "05",
                vec![test_6_1_27_9_err_generator(
                    &DocumentCategory::CsafVex,
                    "CSAFPID-9080700".to_string(),
                    0,
                    0,
                )],
            ),
            (
                "06",
                vec![test_6_1_27_9_err_generator(
                    &DocumentCategory::CsafVex,
                    "CSAFPID-9080701".to_string(),
                    1,
                    1,
                )],
            ),
        ]);
        run_csaf20_tests("27-09", test_6_1_27_9_impact_statement, errors.clone());
        run_csaf21_tests("27-09", test_6_1_27_9_impact_statement, errors);
    }
}
