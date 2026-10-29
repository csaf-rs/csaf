use crate::csaf::traits::util::not_present_20::NotPresentInCsaf20;
use crate::csaf::traits::util::{
    impl_into_any_getter, impl_into_any_iter_getter, impl_into_any_optional_iter_getter,
    impl_optional_str_field_getter, impl_str_field_getter,
};
use crate::csaf_traits::{WithDate, WithOptionalGroupIds, WithOptionalProductIds};
use crate::schema::csaf2_1::schema::{Action as Action21, ActionCategory, ActionStatus, EntityRefsTItem};

pub trait ActionTrait: WithDate + WithOptionalGroupIds + WithOptionalProductIds {
    fn get_acting_entity_refs(&self) -> impl Iterator<Item = &EntityRefsTItem> + '_;
    fn get_action_id(&self) -> &str;
    fn get_category(&self) -> ActionCategory;
    fn get_document_local_vuln_ids(&self) -> Option<impl Iterator<Item = &str> + '_>;
    fn get_receiving_entity_refs(&self) -> Option<impl Iterator<Item = &EntityRefsTItem> + '_>;
    fn get_referenced_action_ids(&self) -> Option<impl Iterator<Item = &str> + '_>;
    fn get_summary(&self) -> Option<&str>;
    fn get_status(&self) -> ActionStatus;
}

impl ActionTrait for NotPresentInCsaf20 {
    impl_into_any_iter_getter!(get_acting_entity_refs, impl Iterator<Item = &EntityRefsTItem> + '_);
    impl_into_any_getter!(get_action_id, &str);
    impl_into_any_getter!(get_category, ActionCategory);
    impl_into_any_optional_iter_getter!(get_document_local_vuln_ids, Option<impl Iterator<Item = &str> + '_>);
    impl_into_any_optional_iter_getter!(
        get_receiving_entity_refs,
        Option<impl Iterator<Item = &EntityRefsTItem> + '_>
    );
    impl_into_any_optional_iter_getter!(get_referenced_action_ids, Option<impl Iterator<Item = &str> + '_>);
    impl_into_any_getter!(get_summary, Option<&str>);
    impl_into_any_getter!(get_status, ActionStatus);
}

impl ActionTrait for Action21 {
    fn get_acting_entity_refs(&self) -> impl Iterator<Item = &EntityRefsTItem> + '_ {
        self.acting_entity_refs.iter()
    }
    impl_str_field_getter!(get_action_id, action_id);

    fn get_category(&self) -> ActionCategory {
        self.category
    }

    fn get_document_local_vuln_ids(&self) -> Option<impl Iterator<Item = &str> + '_> {
        self.dl_vuln_ids.as_ref().map(|ids| ids.iter().map(|id| id.as_str()))
    }

    fn get_receiving_entity_refs(&self) -> Option<impl Iterator<Item = &EntityRefsTItem> + '_> {
        self.receiving_entity_refs.as_ref().map(|refs| refs.iter())
    }

    fn get_referenced_action_ids(&self) -> Option<impl Iterator<Item = &str> + '_> {
        self.referenced_action_ids
            .as_ref()
            .map(|ids| ids.iter().map(|id| id.as_str()))
    }

    fn get_status(&self) -> ActionStatus {
        self.status
    }

    impl_optional_str_field_getter!(get_summary, summary);
}
crate::csaf::traits::impl_with_date!(Action21);
crate::csaf::traits::impl_optional_ids!(Action21, WithOptionalGroupIds, ReturnsValues);
crate::csaf::traits::impl_optional_ids!(Action21, WithOptionalProductIds, ReturnsValues);
