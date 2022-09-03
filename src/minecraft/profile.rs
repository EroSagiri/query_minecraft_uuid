use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

use super::propertie::Propertie;

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub properties: Vec<Propertie>
}

impl Display for Profile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {name} id: {id}", name = self.name, id = self.id)
    }
}