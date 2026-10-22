use std::collections::{BTreeSet, HashMap, HashSet};

use crate::csaf::traits::document::entity_group_trait::EntityGroupTrait;
use crate::csaf::traits::util::extract_references::define_reference_accessors;
use crate::csaf::traits::util::not_present_20::NotPresentInCsaf20;
use crate::csaf_traits::{ActionTrait, EntityTrait};
use crate::schema::csaf2_1::schema::{
    Action as Action21, Entity as Entity21, EntityGroup as EntityGroup21, Involvement as Involvement21,
};

pub trait Csaf21InvolvementTrait {
    type ActionType: ActionTrait;

    type EntityType: EntityTrait;

    type EntityGroupType: EntityGroupTrait;

    fn get_actions(&self) -> impl Iterator<Item = &Self::ActionType> + '_;

    fn get_entities(&self) -> impl Iterator<Item = &Self::EntityType> + '_;

    fn get_entity_groups(&self) -> Option<impl Iterator<Item = &Self::EntityGroupType> + '_>;

    /// Returns the entity group IDs and their corresponding entity IDs as a map.
    ///
    /// If [`get_entity_groups`] returns `None`, this method will also return `None`.
    /// Otherwise, it will return a `HashMap` where the keys are entity group IDs and
    /// the values are `BTreeSet`s of entity IDs.
    fn get_entity_groups_map(&self) -> Option<HashMap<String, BTreeSet<String>>> {
        self.get_entity_groups().map(|groups| {
            groups
                .map(|group| {
                    (
                        group.get_entity_group_id().to_owned(),
                        group.get_entity_ids().map(str::to_owned).collect(),
                    )
                })
                .collect()
        })
    }

    /// Returns all defined entity group IDs as a `HashSet<String>`.
    ///
    /// If [`get_entity_groups`] returns `None`, this method will also return `None`.
    fn get_entity_group_ids(&self) -> Option<HashSet<String>> {
        self.get_entity_groups()
            .map(|groups| groups.map(|group| group.get_entity_group_id().to_owned()).collect())
    }

    define_reference_accessors! {
        iter: [
            (
                get_actions_group_references,
                get_actions_product_references,
                get_actions,
                "/document/involvement/actions"
            ),
        ],
    }
}

impl Csaf21InvolvementTrait for NotPresentInCsaf20 {
    type ActionType = NotPresentInCsaf20;
    type EntityType = NotPresentInCsaf20;
    type EntityGroupType = NotPresentInCsaf20;

    fn get_actions(&self) -> impl Iterator<Item = &Self::ActionType> + '_ {
        self.into_any_iter()
    }

    fn get_entities(&self) -> impl Iterator<Item = &Self::EntityType> + '_ {
        self.into_any_iter()
    }

    fn get_entity_groups(&self) -> Option<impl Iterator<Item = &Self::EntityGroupType> + '_> {
        self.into_any_optional_iter()
    }
}

impl Csaf21InvolvementTrait for Involvement21 {
    type ActionType = Action21;
    type EntityType = Entity21;
    type EntityGroupType = EntityGroup21;

    fn get_actions(&self) -> impl Iterator<Item = &Self::ActionType> + '_ {
        self.actions.iter()
    }

    fn get_entities(&self) -> impl Iterator<Item = &Self::EntityType> + '_ {
        self.entities.iter()
    }

    fn get_entity_groups(&self) -> Option<impl Iterator<Item = &Self::EntityGroupType> + '_> {
        self.entity_groups.as_ref().map(|v| v.iter())
    }
}
