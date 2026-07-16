use std::ops::Deref;
use crate::schema::csaf2_0::schema::CommonPlatformEnumerationRepresentation as CommonPlatformEnumerationRepresentation20;
use crate::schema::csaf2_1::schema::CommonPlatformEnumerationRepresentation as CommonPlatformEnumerationRepresentation21;

pub trait CpeTrait {
    fn as_str(&self) -> &str;
}

impl CpeTrait for CommonPlatformEnumerationRepresentation20 {
    fn as_str(&self) -> &str { &self.deref() }
}

impl CpeTrait for CommonPlatformEnumerationRepresentation21 {
    fn as_str(&self) -> &str { &self.deref() }
}