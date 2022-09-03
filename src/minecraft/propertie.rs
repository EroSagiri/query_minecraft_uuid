use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug)]
pub struct Propertie{
    pub name: String,
    pub value: String
}

impl Display for Propertie {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{name} {value}", name = self.name, value = self.value)
    }
}