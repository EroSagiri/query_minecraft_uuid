use std::{fmt::{Display, Formatter}};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize,Debug)]
pub struct  Name {
    name: String,
    #[serde(rename(serialize = "changedToAt", deserialize = "changedToAt"))]
    changed_to_at: u32
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.name, self.changed_to_at)
    }
}