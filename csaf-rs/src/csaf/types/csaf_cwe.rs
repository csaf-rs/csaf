use crate::schema::csaf2_0::schema::Cwe as Cwe20;
use crate::schema::csaf2_1::schema::Cwe as Cwe21;

pub struct Cwe {
    ///Holds the ID for the weakness associated.
    pub id: String,
    ///Holds the full name of the weakness as given in the CWE specification.
    pub name: String,
    ///Holds the version string of the CWE specification this weakness was extracted from.
    pub version: Option<String>,
}

impl From<&Cwe21> for Cwe {
    fn from(cwe: &Cwe21) -> Self {
        Cwe {
            id: cwe.id.to_string(),
            name: cwe.name.to_string(),
            version: Some(cwe.version.to_string()),
        }
    }
}

impl From<&Cwe20> for Cwe {
    fn from(cwe: &Cwe20) -> Self {
        Cwe {
            id: cwe.id.to_string(),
            name: cwe.name.to_string(),
            version: None,
        }
    }
}
