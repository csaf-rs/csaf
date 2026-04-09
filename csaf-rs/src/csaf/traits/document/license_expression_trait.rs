use std::ops::Deref;

use crate::csaf::traits::util::not_present_20::NotPresentInCsaf20;
use crate::schema::csaf2_1::schema::LicenseExpression as LicenseExpression21;

pub trait LicenseExpressionTrait: Deref<Target = String> {
}

impl LicenseExpressionTrait for NotPresentInCsaf20 {
}

impl LicenseExpressionTrait for LicenseExpression21 {
}