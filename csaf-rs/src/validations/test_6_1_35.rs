use crate::csaf_traits::{CsafTrait, RemediationTrait, VulnerabilityTrait};
use crate::schema::csaf2_1::schema::CategoryOfTheRemediation;
use crate::validation::ValidationError;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

enum ExclusivityKind {
    Exclusive,
    MutuallyExclusive,
}

impl Display for ExclusivityKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExclusivityKind::Exclusive => write!(f, "exclusive"),
            ExclusivityKind::MutuallyExclusive => write!(f, "mutually exclusive"),
        }
    }
}

fn generate_category_contradiction_error(
    product_id: &str,
    exclusivity_kind: ExclusivityKind,
    exclusive_category: CategoryOfTheRemediation,
    contradiction_categories: String,
    vulnerability_index: usize,
    remediation_index: usize,
) -> ValidationError {
    ValidationError {
        message: format!(
            "Product {product_id} has {exclusivity_kind} remediation category '{exclusive_category}', but also '{contradiction_categories}'",
        ),
        instance_path: format!("/vulnerabilities/{vulnerability_index}/remediations/{remediation_index}"),
    }
}

/// Aggregation mapping:
/// resolved remediation product IDs -> remediation category -> vector of remediation indices of occurrences
/// Also provides utility functions to check for exclusive / mutually exclusive category contradictions
struct ProductIdRemediationCategoriesMap(BTreeMap<String, BTreeMap<CategoryOfTheRemediation, Vec<usize>>>);

impl ProductIdRemediationCategoriesMap {
    /// Totally exclusive categories that cannot be combined with any other category
    const EX_STATES: &[CategoryOfTheRemediation] = &[
        CategoryOfTheRemediation::NoneAvailable,
        CategoryOfTheRemediation::OptionalPatch,
    ];

    /// Mutually exclusive states that cannot apply at the same time
    const MUT_EX_STATES: &[CategoryOfTheRemediation] = &[
        CategoryOfTheRemediation::NoFixPlanned,
        CategoryOfTheRemediation::FixPlanned,
        CategoryOfTheRemediation::VendorFix,
    ];

    pub fn aggregate(doc: &impl CsafTrait, vulnerability: &impl VulnerabilityTrait) -> Self {
        let mut map: BTreeMap<String, BTreeMap<CategoryOfTheRemediation, Vec<usize>>> = BTreeMap::new();
        for (remediation_index, remediation) in vulnerability.get_remediations().iter().enumerate() {
            // get the associated product ids, if there are none, continue
            let product_ids = match remediation.get_all_product_ids(doc) {
                Some(ids) => ids,
                None => continue,
            };

            // fill the map
            for product_id in product_ids.into_iter() {
                map.entry(product_id)
                    .or_default()
                    .entry(remediation.get_category())
                    .or_default()
                    .push(remediation_index);
            }
        }
        Self(map)
    }

    fn format_contradiction_categories<'a>(
        categories: impl Iterator<Item = &'a CategoryOfTheRemediation>,
        exclude: &CategoryOfTheRemediation,
    ) -> String {
        categories
            .filter(|cat| *cat != exclude)
            .map(|cat| cat.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }

    /// Checks a [ProductIdRemediationCategoriesMap] for contradicting exclusive categories.
    ///
    /// For each exclusive category, if there are any other categories, for each remediation with
    /// this category an error is added to `errors`.
    fn check_exclusive_categories_contradiction(&self, vuln_index: usize, errors: &mut Option<Vec<ValidationError>>) {
        // for each product and the associated categories map
        for (product_id, category_map) in &self.0 {
            // check if the map contains an exclusive category and any other category
            for ex_state in Self::EX_STATES {
                if let Some(remediation_indices) = category_map.get(ex_state)
                    && category_map.len() > 1
                {
                    // report error for all offending categories
                    let contradiction_categories = Self::format_contradiction_categories(category_map.keys(), ex_state);
                    // for each remediation with that category
                    for remediation_index in remediation_indices {
                        errors
                            .get_or_insert_default()
                            .push(generate_category_contradiction_error(
                                product_id,
                                ExclusivityKind::Exclusive,
                                *ex_state,
                                contradiction_categories.clone(),
                                vuln_index,
                                *remediation_index,
                            ));
                    }
                }
            }
        }
    }

    /// Checks a [ProductIdRemediationCategoriesMap] for contradicting mutually exclusive categories.
    ///
    /// For each mutually exclusive category, if there are any other mutually exclusive categories,
    /// for each remediation with this category an error is added to `errors`.
    fn check_mutually_exclusive_category_contradiction(
        &self,
        vuln_index: usize,
        errors: &mut Option<Vec<ValidationError>>,
    ) {
        // for each product and the associated categories map
        for (product_id, category_map) in &self.0 {
            // extract the mutually exclusive categories
            let mut_ex = category_map
                .iter()
                .filter(|entry| Self::MUT_EX_STATES.contains(entry.0))
                .collect::<Vec<_>>();
            // if there is more than one mutually exclusive category, there is a contradiction
            if mut_ex.len() > 1 {
                // generate an error for each of the contradicting categories
                for (mut_ex_category, remediation_indices) in &mut_ex {
                    let contradiction_categories =
                        Self::format_contradiction_categories(mut_ex.iter().map(|(cat, _)| *cat), mut_ex_category);
                    // for each remediation with that category
                    for remediation_index in *remediation_indices {
                        errors
                            .get_or_insert_default()
                            .push(generate_category_contradiction_error(
                                product_id,
                                ExclusivityKind::MutuallyExclusive,
                                **mut_ex_category,
                                contradiction_categories.clone(),
                                vuln_index,
                                *remediation_index,
                            ));
                    }
                }
            }
        }
    }

    /// Calls [`Self::check_mutually_exclusive_category_contradiction`] and [`Self::check_exclusive_categories_contradiction].
    pub fn check_category_contradiction(&self, vuln_index: usize, errors: &mut Option<Vec<ValidationError>>) {
        self.check_exclusive_categories_contradiction(vuln_index, errors);
        self.check_mutually_exclusive_category_contradiction(vuln_index, errors);
    }
}

/// 6.1.35 Contradicting Remediations
///
/// For each item in /vulnerabilities[]/remediations it MUST be tested that a product
/// is not member of contradicting remediation categories.
/// This takes indirect relations through product groups into account.
///
/// For more details on how the checks work, see [`ProductIdRemediationCategoriesMap].
pub fn test_6_1_35_contradicting_remediations(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let vulnerabilities = doc.get_vulnerabilities();
    if vulnerabilities.is_empty() {
        return Ok(()); // TODO #409 this will be noData after refactor
    }
    let mut errors: Option<Vec<ValidationError>> = None;
    for (v_i, v) in vulnerabilities.iter().enumerate() {
        ProductIdRemediationCategoriesMap::aggregate(doc, v).check_category_contradiction(v_i, &mut errors);
    }
    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_35, test_6_1_35_contradicting_remediations);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    fn join_categories(categories: Vec<CategoryOfTheRemediation>) -> String {
        categories
            .into_iter()
            .map(|cat| cat.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }

    #[test]
    fn test_test_6_1_35() {
        let case_01_mutually_exclusive_via_product = Err(vec![
            generate_category_contradiction_error(
                "CSAFPID-9080700",
                ExclusivityKind::MutuallyExclusive,
                CategoryOfTheRemediation::NoFixPlanned,
                join_categories(vec![CategoryOfTheRemediation::VendorFix]),
                0,
                0,
            ),
            generate_category_contradiction_error(
                "CSAFPID-9080700",
                ExclusivityKind::MutuallyExclusive,
                CategoryOfTheRemediation::VendorFix,
                join_categories(vec![CategoryOfTheRemediation::NoFixPlanned]),
                0,
                1,
            ),
        ]);

        let case_02_exclusive_none_available_via_group = Err(vec![generate_category_contradiction_error(
            "CSAFPID-9080700",
            ExclusivityKind::Exclusive,
            CategoryOfTheRemediation::NoneAvailable,
            join_categories(vec![CategoryOfTheRemediation::Mitigation]),
            0,
            0,
        )]);

        let case_03_exclusive_optional_patch_via_group = Err(vec![generate_category_contradiction_error(
            "CSAFPID-9080702",
            ExclusivityKind::Exclusive,
            CategoryOfTheRemediation::OptionalPatch,
            join_categories(vec![
                CategoryOfTheRemediation::FixPlanned,
                CategoryOfTheRemediation::Workaround,
            ]),
            0,
            2,
        )]);

        let case_04_exclusive_optional_patch_via_groups_multiple_products = Err(vec![
            generate_category_contradiction_error(
                "CSAFPID-9080701",
                ExclusivityKind::Exclusive,
                CategoryOfTheRemediation::OptionalPatch,
                join_categories(vec![
                    CategoryOfTheRemediation::FixPlanned,
                    CategoryOfTheRemediation::Mitigation,
                ]),
                0,
                2,
            ),
            generate_category_contradiction_error(
                "CSAFPID-9080702",
                ExclusivityKind::Exclusive,
                CategoryOfTheRemediation::OptionalPatch,
                join_categories(vec![
                    CategoryOfTheRemediation::FixPlanned,
                    CategoryOfTheRemediation::Mitigation,
                ]),
                0,
                2,
            ),
        ]);

        // Case 11: One product, one remediation
        // Case 12: One product, one group, exclusive optional patch only on the product
        // Case 13: One product, one group, exclusive optional patch only on the group
        // Case 14: Two groups, exclusive optional patch applies only to one group
        // Case s11: Duplicate optional_patch (same exclusive category, no contradiction)
        // Case s12: Duplicate vendor_fix (same mut_ex category, no contradiction)

        TESTS_2_1.test_6_1_35.expect(
            case_01_mutually_exclusive_via_product,
            case_02_exclusive_none_available_via_group,
            case_03_exclusive_optional_patch_via_group,
            case_04_exclusive_optional_patch_via_groups_multiple_products,
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
