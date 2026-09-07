mod string_newtype;
mod test_gen;

pub(crate) use string_newtype::impl_string_newtype_ergonomics;
pub(crate) use test_gen::define_csaf_test;
pub(crate) use test_gen::define_test_cases_aggregate;
