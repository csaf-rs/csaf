mod string_newtype;

pub(crate) use string_newtype::impl_string_newtype_ergonomics;

#[cfg(feature = "validation")]
mod test_gen;

#[cfg(feature = "validation")]
pub(crate) use test_gen::define_csaf_test;
#[cfg(feature = "validation")]
pub(crate) use test_gen::define_test_cases_aggregate;
