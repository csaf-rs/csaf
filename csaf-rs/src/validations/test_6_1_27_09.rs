use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{
    CsafTrait, DocumentTrait, ProductStatusTrait, ThreatTrait, VulnerabilityTrait, WithOptionalGroupIds,
    WithOptionalProductIds,
};
use crate::document_category_test_helper::DocumentCategoryTestConfig;
use crate::helpers::resolve_product_groups;
use crate::schema::csaf2_1::schema::CategoryOfTheThreat;
use crate::validation::ValidationError;
use std::collections::{HashMap, HashSet};

/// 6.1.27.9 Impact Statement
///
/// This test only applies to documents with `/document/category` with value `csaf_vex`.
///
/// Each item in `/vulnerabilities[]/product_status/known_not_affected` must have a corresponding
/// impact statement in `/vulnerabilities[]/flags` or `/vulnerabilities[]/threats`. For impact statements under
/// `threats`, the category must be `impact`.
pub fn test_6_1_27_09_impact_statement(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let doc_category = doc.get_document().get_category();
    let vulnerabilities = doc.get_vulnerabilities();

    // Only execute this test for documents with category 'csaf_vex'
    // and if there are any vulnerabilities present
    if !PROFILE_TEST_CONFIG.matches_category(&doc_category) || vulnerabilities.is_empty() {
        return Ok(());
    }

    let mut errors: Option<Vec<ValidationError>> = None;
    // for each vulnerability
    for (v_i, vulnerability) in vulnerabilities.iter().enumerate() {
        // generate hashmap of all known_not_affected product or group ids with value of known_not_affected path index
        let mut known_not_affected_product_or_group_ids: HashMap<String, usize> = HashMap::new();
        if let Some(product_status) = vulnerability.get_product_status()
            && let Some(known_not_affected) = product_status.get_known_not_affected()
        {
            for (kna_i, known_not_affected_entry) in known_not_affected.into_iter().enumerate() {
                known_not_affected_product_or_group_ids.insert(known_not_affected_entry.to_owned(), kna_i);
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
            errors.get_or_insert_with(Vec::new).push(test_6_1_27_09_err_generator(
                known_not_affected_group_id.0.to_string(),
                v_i,
                *known_not_affected_group_id.1,
            ));
        }
    }

    errors.map_or(Ok(()), Err)
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig =
    DocumentCategoryTestConfig::new().shared(&[CsafDocumentCategory::CsafVex]);

fn test_6_1_27_09_err_generator(
    product_or_group_id: String,
    vuln_path_index: usize,
    known_not_affected_path_index: usize,
) -> ValidationError {
    ValidationError {
        message: format!(
            "No impact statement found for 'known_not_affected' product status entry '{product_or_group_id}'."
        ),
        instance_path: format!(
            "/vulnerabilities/{vuln_path_index}/product_status/known_not_affected/{known_not_affected_path_index}"
        ),
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_27_9
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_27_09_impact_statement(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_27_9
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_27_09_impact_statement(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_27_09() {
        let case_group_covered_by_threats =
            Err(vec![test_6_1_27_09_err_generator("CSAFPID-9080702".to_string(), 0, 2)]);
        let case_group_covered_by_flag = Err(vec![test_6_1_27_09_err_generator("CSAFPID-9080702".to_string(), 0, 2)]);
        let case_products_covered_by_threats =
            Err(vec![test_6_1_27_09_err_generator("CSAFPID-9080700".to_string(), 0, 0)]);
        let case_products_covered_by_flags =
            Err(vec![test_6_1_27_09_err_generator("CSAFPID-9080700".to_string(), 0, 0)]);
        let case_products_covered_by_flags_or_threats =
            Err(vec![test_6_1_27_09_err_generator("CSAFPID-9080700".to_string(), 0, 0)]);
        let case_one_not_covered_with_multiple_vulnerabilities =
            Err(vec![test_6_1_27_09_err_generator("CSAFPID-9080701".to_string(), 1, 1)]);
        let case_one_not_covered_by_threat_with_wrong_category =
            Err(vec![test_6_1_27_09_err_generator("CSAFPID-9080702".to_string(), 0, 2)]);

        TESTS_2_0.test_6_1_27_9.expect(
            case_group_covered_by_threats.clone(),
            case_group_covered_by_flag.clone(),
            case_products_covered_by_threats.clone(),
            case_products_covered_by_flags.clone(),
            case_products_covered_by_flags_or_threats.clone(),
            case_one_not_covered_with_multiple_vulnerabilities.clone(),
            case_one_not_covered_by_threat_with_wrong_category.clone(),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
        TESTS_2_1.test_6_1_27_9.expect(
            case_group_covered_by_threats,
            case_group_covered_by_flag,
            case_products_covered_by_threats,
            case_products_covered_by_flags,
            case_products_covered_by_flags_or_threats,
            case_one_not_covered_with_multiple_vulnerabilities,
            case_one_not_covered_by_threat_with_wrong_category,
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
