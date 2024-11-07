
pub enum ValidationError {

}

pub trait Validate {

    /// Validates this object.
    fn validate(&self);
}
