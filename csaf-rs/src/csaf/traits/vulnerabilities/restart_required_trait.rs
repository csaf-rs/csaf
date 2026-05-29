use crate::csaf::traits::util::impl_optional_str_field_getter;
use crate::schema::csaf2_0::schema::{
    CategoryOfRestart, RestartRequiredByRemediation as RestartRequiredByRemediation20,
};
use crate::schema::csaf2_1::schema::CategoryOfRestart as CategoryOfRestart21;
use crate::schema::csaf2_1::schema::RestartRequiredByRemediation as RestartRequiredByRemediation21;

pub trait RestartRequiredTrait {
    fn get_category(&self) -> CategoryOfRestart21;
    fn get_details(&self) -> Option<&str>;
}

impl RestartRequiredTrait for RestartRequiredByRemediation20 {
    fn get_category(&self) -> CategoryOfRestart21 {
        match &self.category {
            CategoryOfRestart::Connected => CategoryOfRestart21::Connected,
            CategoryOfRestart::Dependencies => CategoryOfRestart21::Dependencies,
            CategoryOfRestart::Machine => CategoryOfRestart21::Machine,
            CategoryOfRestart::None => CategoryOfRestart21::None,
            CategoryOfRestart::Parent => CategoryOfRestart21::Parent,
            CategoryOfRestart::Service => CategoryOfRestart21::Service,
            CategoryOfRestart::System => CategoryOfRestart21::System,
            CategoryOfRestart::VulnerableComponent => CategoryOfRestart21::VulnerableComponent,
            CategoryOfRestart::Zone => CategoryOfRestart21::Zone,
        }
    }

    impl_optional_str_field_getter!(get_details, details);
}

impl RestartRequiredTrait for RestartRequiredByRemediation21 {
    fn get_category(&self) -> CategoryOfRestart21 {
        self.category
    }

    impl_optional_str_field_getter!(get_details, details);
}
