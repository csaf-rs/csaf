use crate::schema::csaf2_0::schema::AggregateSeverity as AggregateSeverity20;
use crate::schema::csaf2_1::schema::AggregateSeverity as AggregateSeverity21;

pub trait AggregateSeverityTrait {
    fn get_namespace(&self) -> Option<&str>;
    fn get_text(&self) -> &str;
}

impl AggregateSeverityTrait for AggregateSeverity20 {
    fn get_namespace(&self) -> Option<&str> {
        self.namespace.as_deref()
    }

    fn get_text(&self) -> &str {
        self.text.as_str()
    }
}

impl AggregateSeverityTrait for AggregateSeverity21 {
    fn get_namespace(&self) -> Option<&str> {
        self.namespace.as_deref()
    }
    fn get_text(&self) -> &str {
        self.text.as_str()
    }
}
