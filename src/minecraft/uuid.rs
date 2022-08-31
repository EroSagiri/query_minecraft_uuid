use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Uuid {
    pub name : String,
    pub id : String
}

impl Uuid {

}

impl Display for Uuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.name, self.id)
    }
}