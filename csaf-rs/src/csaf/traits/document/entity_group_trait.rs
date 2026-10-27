use crate::csaf::traits::util::not_present_20::NotPresentInCsaf20;
use crate::csaf::traits::util::{impl_into_any_getter, impl_optional_str_field_getter, impl_str_field_getter};
use crate::schema::csaf2_1::schema::EntityGroup as EntityGroup21;

pub trait EntityGroupTrait {
    fn get_entity_group_id(&self) -> &str;
    fn get_entity_ids(&self) -> impl Iterator<Item = &str> + '_;
    fn get_name(&self) -> &str;
    fn get_summary(&self) -> Option<&str>;
}

impl EntityGroupTrait for NotPresentInCsaf20 {
    impl_into_any_getter!(get_entity_group_id, &str);

    fn get_entity_ids(&self) -> impl Iterator<Item = &str> + '_ {
        self.into_any_iter()
    }

    impl_into_any_getter!(get_name, &str);
    impl_into_any_getter!(get_summary, Option<&str>);
}

impl EntityGroupTrait for EntityGroup21 {
    impl_str_field_getter!(get_entity_group_id, entity_group_id);

    fn get_entity_ids(&self) -> impl Iterator<Item = &str> + '_ {
        self.entity_ids.iter().map(|s| s.as_str())
    }

    impl_str_field_getter!(get_name, name);
    impl_optional_str_field_getter!(get_summary, summary);
}
