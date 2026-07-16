use crate::csaf::traits::util::impl_str_field_getter;
use crate::schema::csaf2_0::schema::GenericUri as GenericUri20;
use crate::schema::csaf2_1::schema::GenericUri as GenericUri21;

pub trait GenericUriTrait {
    fn get_namespace(&self) -> &str;
    fn get_uri(&self) -> &str;
}

impl GenericUriTrait for GenericUri20 {
    impl_str_field_getter!(get_namespace, namespace);
    impl_str_field_getter!(get_uri, uri);
}

impl GenericUriTrait for GenericUri21 {
    impl_str_field_getter!(get_namespace, namespace);
    impl_str_field_getter!(get_uri, uri);
}