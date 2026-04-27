use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::resolve_product_groups;
use crate::csaf_traits::{CsafTrait, DocumentTrait, ProductStatusTrait, VulnerabilityTrait};
use crate::document_category_test_helper::DocumentCategoryTestConfig;
use crate::helpers::resolve_product_groups;
use crate::validation::ValidationError;
use crate::validations::utils::document_category_test_config::DocumentCategoryTestConfig;
use std::collections::{HashMap, HashSet};

/// 6.1.27.10 Action Statement
///
/// This test only applies to documents with `/document/category` with value `csaf_vex`.
///
/// Each item in `/vulnerabilities[]/product_status/known_affected` must have a corresponding
/// action statement in `/vulnerabilities[]/remediations`
///
pub fn test_6_1_27_10_action_statement(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let doc_category = doc.get_document().get_category();
    let vulnerabilities = doc.get_vulnerabilities();

    // Only execute this test for documents with category 'csaf_vex'
    // and if there are any vulnerabilities present
    if !PROFILE_TEST_CONFIG.matches_category(&doc_category) || vulnerabilities.is_empty() {
        return Ok(()); // ToDo generate skipped https://github.com/csaf-rs/csaf/issues/409
    }

    let mut errors: Option<Vec<ValidationError>> = None;
    // for each vulnerability
    for (v_i, vulnerability) in vulnerabilities.iter().enumerate() {
        // generate hashmap of all known_affected product or group ids with value of known_not_affected path index
        let mut known_affected_product_or_group_ids: HashMap<String, usize> = HashMap::new();
        if let Some(product_status) = vulnerability.get_product_status()
            && let Some(known_affected) = product_status.get_known_affected()
        {
            for (kna_i, known_affected_entry) in known_affected.into_iter().enumerate() {
                known_affected_product_or_group_ids.insert(known_affected_entry.to_owned(), kna_i);
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
        for known_affected_product_or_group_id in known_affected_product_or_group_ids.iter() {
            errors.get_or_insert_default().push(test_6_1_27_10_err_generator(
                known_affected_product_or_group_id.0.to_string(),
                v_i,
                *known_affected_product_or_group_id.1,
            ));
        }
    }

    errors.map_or(Ok(()), Err)
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig =
    DocumentCategoryTestConfig::new().shared(&[CsafDocumentCategory::CsafVex]);

fn test_6_1_27_10_err_generator(
    product_or_group_id: String,
    vuln_path_index: usize,
    known_affected_path_index: usize,
) -> ValidationError {
    ValidationError {
        message: format!(
            "No action statement found for 'known_affected' product status entry '{product_or_group_id}'."
        ),
        instance_path: format!(
            "/vulnerabilities/{vuln_path_index}/product_status/known_affected/{known_affected_path_index}"
        ),
    }
}

crate::test_validation::impl_validator!(ValidatorForTest6_1_27_10, test_6_1_27_10_action_statement);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_27_10() {
        let case_one_product_missing_from_group =
            Err(vec![test_6_1_27_10_err_generator("CSAFPID-9080702".to_string(), 0, 2)]);
        let case_one_product_missing_from_products =
            Err(vec![test_6_1_27_10_err_generator("CSAFPID-9080702".to_string(), 0, 2)]);
        let case_missing_remediation = Err(vec![
            test_6_1_27_10_err_generator("CSAFPID-9080700".to_string(), 0, 0),
            test_6_1_27_10_err_generator("CSAFPID-9080701".to_string(), 0, 1),
            test_6_1_27_10_err_generator("CSAFPID-9080702".to_string(), 0, 2),
        ]);

        TESTS_2_0.test_6_1_27_10.expect(
            case_one_product_missing_from_group.clone(),
            case_missing_remediation.clone(),
            case_one_product_missing_from_products.clone(),
            Ok(()),
            Ok(()),
        );
        TESTS_2_1.test_6_1_27_10.expect(
            case_one_product_missing_from_group,
            case_missing_remediation,
            case_one_product_missing_from_products,
            Ok(()),
            Ok(()),
        );
    }
}
