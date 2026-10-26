use crate::csaf::traits::util::resolve_entity_groups::resolve_ref_groups_to_ref_ids;
use crate::csaf::types::csaf_datetime::CsafDateTime;
use crate::csaf_traits::{ActionTrait, Csaf21InvolvementTrait, CsafTrait, DocumentTrait};
use crate::schema::csaf2_1::schema::{ActionCategory, ActionStatus, EntityRefsTItem};
use crate::validation::{TestFinding, TestFindingData};
use std::collections::{BTreeSet, HashMap};
use std::hash::{Hash, Hasher};

/// Representation of an action for duplicate detection.
///
/// Reference arrays are represented as `BTreeSet`s so their order and repeated
/// values do not affect equality.
struct ActionKey {
    date: CsafDateTime,
    category: ActionCategory,
    status: ActionStatus,
    summary: Option<String>,
    acting_entity_refs: BTreeSet<String>,
    dl_vuln_ids: BTreeSet<String>,
    group_ids: BTreeSet<String>,
    product_ids: BTreeSet<String>,
    receiving_entity_refs: BTreeSet<String>,
    referenced_action_ids: BTreeSet<String>,
}

impl Hash for ActionKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // hash valid dates as their UTC representation
        // hash invalid dates as their raw string
        match &self.date {
            CsafDateTime::Valid(date) => {
                0_u8.hash(state);
                date.get_as_utc().hash(state);
            },
            CsafDateTime::Invalid(date) => {
                1_u8.hash(state);
                date.get_raw_string().hash(state);
            },
        }
        self.category.hash(state);
        self.status.hash(state);
        self.summary.hash(state);
        self.acting_entity_refs.hash(state);
        self.dl_vuln_ids.hash(state);
        self.group_ids.hash(state);
        self.product_ids.hash(state);
        self.receiving_entity_refs.hash(state);
        self.referenced_action_ids.hash(state);
    }
}

impl Eq for ActionKey {}

impl PartialEq for ActionKey {
    fn eq(&self, other: &Self) -> bool {
        // compare raw date strings first, if they are unequal, compare the parsed dates
        (self.date.get_raw_string() == other.date.get_raw_string()
            || matches!(
                (&self.date, &other.date),
                (CsafDateTime::Valid(first), CsafDateTime::Valid(second)) if first == second
            ))
            && self.category == other.category
            && self.status == other.status
            && self.summary == other.summary
            && self.acting_entity_refs == other.acting_entity_refs
            && self.dl_vuln_ids == other.dl_vuln_ids
            && self.group_ids == other.group_ids
            && self.product_ids == other.product_ids
            && self.receiving_entity_refs == other.receiving_entity_refs
            && self.referenced_action_ids == other.referenced_action_ids
    }
}

/// Resolves entity and entity-group references to a canonical set for comparison.
///
/// Resolvable groups contribute their member entity IDs. An unresolvable group ID is retained so
/// that two actions using the same unresolvable reference still compare as equal.
fn resolve_entity_refs<'a>(
    refs: impl Iterator<Item = &'a EntityRefsTItem>,
    all_document_entity_groups: &HashMap<String, BTreeSet<String>>,
) -> BTreeSet<String> {
    let resolved = resolve_ref_groups_to_ref_ids(refs, all_document_entity_groups);
    let mut entity_ids = resolved.resolved_entity_ids.unwrap_or_default();
    entity_ids.extend(resolved.unresolvable_group_ids.unwrap_or_default());
    entity_ids
}

/// Creates the comparison key for an action, excluding its `action_id`.
///
/// Optional reference arrays that are absent are treated as empty sets. Acting and receiving
/// entity references are expanded through the document's entity-group definitions before being
/// compared.
fn action_key(
    action: &impl ActionTrait,
    all_document_local_vuln_ids: &BTreeSet<String>,
    all_document_entity_groups: &HashMap<String, BTreeSet<String>>,
) -> ActionKey {
    // put the `dl_vuln_ids` into a BTreeSet
    let mut dl_vuln_ids: BTreeSet<String> = action
        .get_document_local_vuln_ids()
        .into_iter()
        .flatten()
        .map(ToString::to_string)
        .collect();
    // if `dl_vuln_ids` contains all vulnerabilities in the document, treat it as if it were absent
    // the BTreeSet ensures this ignores order and the values are deduplicated
    if dl_vuln_ids == *all_document_local_vuln_ids {
        dl_vuln_ids.clear();
    }

    ActionKey {
        date: action.get_date(),
        category: action.get_category(),
        status: action.get_status(),
        summary: action.get_summary().map(str::to_owned),
        acting_entity_refs: resolve_entity_refs(action.get_acting_entity_refs(), all_document_entity_groups),
        dl_vuln_ids,
        group_ids: action
            .get_group_ids()
            .into_iter()
            .flatten()
            .map(ToString::to_string)
            .collect(),
        product_ids: action
            .get_product_ids()
            .into_iter()
            .flatten()
            .map(ToString::to_string)
            .collect(),
        receiving_entity_refs: resolve_entity_refs(
            action.get_receiving_entity_refs().into_iter().flatten(),
            all_document_entity_groups,
        ),
        referenced_action_ids: action
            .get_referenced_action_ids()
            .into_iter()
            .flatten()
            .map(ToString::to_string)
            .collect(),
    }
}

fn duplicate_action_error(action_ids: &[&str], action_i: usize) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "The list of actions contains items with the same content when ignoring their action_id: {}",
            action_ids.join(", ")
        ),
        instance_path: format!("/document/involvement/actions/{action_i}"),
    })
}

/// Test 6.1.24: Multiple Definition in Actions
///
/// It SHALL be tested that items of the list of actions do differ in more values than just the `action_id`.
//
/// Ignoring the `action_id`, two items in the list of actions are the same, if:
///
/// 1. all of the following fields contain the same value: `category`, `date`, `status`, `summary`
/// 2. and all of the following sets contain the same values: `acting_entity_refs`, `dl_vuln_ids`, `group_ids`, `product_ids`, `receiving_entity_refs`, `referenced_action_ids`
///
/// A `dl_vuln_ids` mentioning all vulnerabilities in the document is equivalent to the element missing.
///
/// All groups have to be expanded to their definitions for the test.
///
/// As the timestamps might use different timezones, the comparison SHALL take timezones into account.
pub fn test_6_1_24_multiple_definition_in_actions(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // check if there is an involvement section
    let Some(involvement) = doc.get_document().get_involvement() else {
        return Ok(()); // TODO #409 wasSkipped
    };

    // get all document defined local vuln ids
    let all_document_local_vuln_ids = doc.get_all_document_local_vuln_ids();
    // get all document defined entity groups
    let all_document_entity_groups = involvement.get_entity_groups_map().unwrap_or_default();

    // create a reverse lookup of action content to the action_ids with that content and their index in the actions array.
    let mut actions = HashMap::<ActionKey, Vec<(&str, usize)>>::new();
    for (action_i, action) in involvement.get_actions().enumerate() {
        actions
            .entry(action_key(
                action,
                &all_document_local_vuln_ids,
                &all_document_entity_groups,
            ))
            .or_default()
            .push((action.get_action_id(), action_i));
    }

    // generate an error for each action that has the same content as another action
    let errors = actions
        .values()
        .filter(|actions| actions.len() > 1)
        .flat_map(|actions| {
            // collect the action_ids of the duplicate actions for the error message
            let action_ids = actions.iter().map(|(action_id, _)| *action_id).collect::<Vec<_>>();
            actions
                .iter()
                .map(|(_, action_i)| duplicate_action_error(&action_ids, *action_i))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_24,
    test_6_1_24_multiple_definition_in_actions
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::{ExpectedResults_6_1_24, TESTS_2_1};

    #[test]
    fn test_test_6_1_24() {
        let action_ids = ["ACN-1", "ACN-2"];
        let duplicate_actions = Err(vec![
            duplicate_action_error(&action_ids, 0),
            duplicate_action_error(&action_ids, 1),
        ]);

        TESTS_2_1.test_6_1_24.expect(ExpectedResults_6_1_24 {
            case_01: duplicate_actions,
            case_11: Ok(()),
        });
    }
}
