use crate::csaf::traits::util::resolve_entity_groups::resolve_entity_refs;
use crate::csaf::traits::util::resolve_product_groups::resolve_product_refs;
use crate::csaf::types::csaf_datetime::CsafDateTime;
use crate::csaf_traits::{ActionTrait, Csaf21InvolvementTrait, CsafTrait, DocumentTrait};
use crate::schema::csaf2_1::schema::{ActionCategory, ActionStatus};
use crate::validation::{TestFinding, TestFindingData};
use std::collections::{BTreeSet, HashMap};
use std::hash::{Hash, Hasher};

struct ContradictingActionKey {
    date: CsafDateTime,
    category: ActionCategory,
    acting_entity_refs: BTreeSet<String>,
    dl_vuln_ids: BTreeSet<String>,
    unresolvable_product_group_ids: BTreeSet<String>,
    product_ids_including_resolved_product_group_ids: BTreeSet<String>,
    receiving_entity_refs: BTreeSet<String>,
    referenced_action_ids: BTreeSet<String>,
}

impl Hash for ContradictingActionKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
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
        self.acting_entity_refs.hash(state);
        self.dl_vuln_ids.hash(state);
        self.unresolvable_product_group_ids.hash(state);
        self.product_ids_including_resolved_product_group_ids.hash(state);
        self.receiving_entity_refs.hash(state);
        self.referenced_action_ids.hash(state);
    }
}

impl Eq for ContradictingActionKey {}

impl PartialEq for ContradictingActionKey {
    fn eq(&self, other: &Self) -> bool {
        (self.date.get_raw_string() == other.date.get_raw_string()
            || matches!(
                (&self.date, &other.date),
                (CsafDateTime::Valid(first), CsafDateTime::Valid(second)) if first == second
            ))
            && self.category == other.category
            && self.acting_entity_refs == other.acting_entity_refs
            && self.dl_vuln_ids == other.dl_vuln_ids
            && self.unresolvable_product_group_ids == other.unresolvable_product_group_ids
            && self.product_ids_including_resolved_product_group_ids
                == other.product_ids_including_resolved_product_group_ids
            && self.receiving_entity_refs == other.receiving_entity_refs
            && self.referenced_action_ids == other.referenced_action_ids
    }
}

fn action_key(
    action: &impl ActionTrait,
    doc: &impl CsafTrait,
    all_document_local_vuln_ids: Option<&BTreeSet<String>>,
    all_document_entity_groups: &HashMap<String, BTreeSet<String>>,
) -> (ContradictingActionKey, ActionStatus) {
    let mut dl_vuln_ids: BTreeSet<String> = action
        .get_document_local_vuln_ids()
        .into_iter()
        .flatten()
        .map(ToString::to_string)
        .collect();

    if let Some(all_document_local_vuln_ids) = all_document_local_vuln_ids
        && dl_vuln_ids == *all_document_local_vuln_ids
    {
        dl_vuln_ids.clear();
    }

    let (product_ids_including_resolved_product_group_ids, unresolvable_product_group_ids) =
        resolve_product_refs(doc, action);

    (
        ContradictingActionKey {
            date: action.get_date(),
            category: action.get_category(),
            acting_entity_refs: resolve_entity_refs(action.get_acting_entity_refs(), all_document_entity_groups),
            dl_vuln_ids,
            unresolvable_product_group_ids,
            product_ids_including_resolved_product_group_ids,
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
        },
        action.get_status(),
    )
}

fn contradicting_action_error(action_ids_with_status: &[(&str, ActionStatus)], action_i: usize) -> TestFinding {
    let action_ids = action_ids_with_status
        .iter()
        .map(|(action_id, status)| format!("{action_id} (status: `{status:?}`)"))
        .collect::<Vec<_>>()
        .join(", ");
    TestFinding::Error(TestFindingData {
        message: format!("The list of actions contains items with contradicting statuses: {action_ids}",),
        instance_path: format!("/document/involvement/actions/{action_i}"),
    })
}

/// Test 6.1.64.2: Contradicting Definition in Actions.
pub fn test_6_1_64_2_contradicting_definition_in_actions(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // check if involvement is present, if not, skip the test
    let Some(involvement) = doc.get_document().get_involvement() else {
        return Ok(());
    };

    // get all document defined local vuln ids, if all vulnerabilities have a document local vuln id
    let all_document_local_vuln_ids = doc
        .all_vulnerabilities_have_document_local_vuln_ids()
        .then(|| doc.get_all_document_local_vuln_ids());

    // get all document defined entity groups
    let all_document_entity_groups = involvement.get_entity_groups_map().unwrap_or_default();

    // create a reverse lookup of action content to the action_ids with that content without the status, their status and their index in the actions array
    let mut actions = HashMap::<ContradictingActionKey, Vec<(&str, ActionStatus, usize)>>::new();
    for (action_i, action) in involvement.get_actions().enumerate() {
        let (key, status) = action_key(
            action,
            doc,
            all_document_local_vuln_ids.as_ref(),
            &all_document_entity_groups,
        );
        actions
            .entry(key)
            .or_default()
            .push((action.get_action_id(), status, action_i));
    }

    let errors = actions
        .values()
        .filter(|actions| {
            actions
                .iter()
                .map(|(_, status, _)| status)
                .collect::<BTreeSet<_>>()
                .len()
                > 1
        })
        .flat_map(|actions| {
            let action_ids_with_status = actions
                .iter()
                .map(|(action_id, status, _)| (*action_id, *status))
                .collect::<Vec<_>>();
            actions
                .iter()
                .map(|(_, _, action_i)| contradicting_action_error(&action_ids_with_status, *action_i))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_64_2,
    test_6_1_64_2_contradicting_definition_in_actions
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::{ExpectedResults_6_1_64_2 as ExpectedResults, TESTS_2_1};

    #[test]
    fn test_test_6_1_64_2() {
        let discontinued_completed = [
            ("ACN-1", ActionStatus::Discontinued),
            ("ACN-2", ActionStatus::Completed),
        ];
        let attempted_completed = [("ACN-1", ActionStatus::Attempted), ("ACN-2", ActionStatus::Completed)];

        TESTS_2_1.test_6_1_64_2.expect(ExpectedResults {
            case_01: Err(vec![
                contradicting_action_error(&discontinued_completed, 0),
                contradicting_action_error(&discontinued_completed, 1),
            ]),
            case_02: Err(vec![
                contradicting_action_error(&attempted_completed, 0),
                contradicting_action_error(&attempted_completed, 1),
            ]),
            case_11: Ok(()),
            case_12: Ok(()),
        });
    }
}
