use crate::schema::csaf2_1::schema::EntityRefsTItem;
use std::collections::{BTreeSet, HashMap};

pub struct ResolvedEntityGroupIds {
    pub resolved_entity_ids: Option<BTreeSet<String>>,
    pub unresolvable_group_ids: Option<BTreeSet<String>>,
}

/// TODO: Refactor to be more ergonomic and align this with the resolve_product_groups function (#1384)
pub fn resolve_ref_groups_to_ref_ids<'a>(
    group_references: impl Iterator<Item = &'a EntityRefsTItem>,
    entity_groups: &HashMap<String, BTreeSet<String>>,
) -> ResolvedEntityGroupIds {
    let mut resolved_entity_ids: Option<BTreeSet<String>> = None;
    let mut unresolvable_group_ids: Option<BTreeSet<String>> = None;

    for group_ref in group_references {
        match group_ref {
            EntityRefsTItem::GroupIdT(group_id) => {
                let group_id_str = group_id.to_string();
                // if its a group ID, we need to resolve it to its corresponding entity IDs
                if let Some(entity_ids) = entity_groups.get(&group_id_str) {
                    // if the group ID is resolvable, we can add its entity IDs to the resolved entity IDs
                    resolved_entity_ids.get_or_insert_default().extend(entity_ids.clone());
                } else {
                    // if the group ID is not resolvable, we need to add it to the unresolvable group IDs
                    unresolvable_group_ids.get_or_insert_default().insert(group_id_str);
                }
            },
            // if its an entity ID, we can directly add it to the resolved entity IDs
            EntityRefsTItem::IdT(id) => {
                resolved_entity_ids.get_or_insert_default().insert(id.to_string());
            },
        }
    }

    ResolvedEntityGroupIds {
        resolved_entity_ids,
        unresolvable_group_ids,
    }
}
