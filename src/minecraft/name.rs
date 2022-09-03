use std::{fmt::{Display, Formatter}};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize,Debug)]
pub struct  Name {
    pub name: String,
    #[serde(rename(serialize = "changedToAt", deserialize = "changedToAt"))]
    pub changed_to_at: Option<i64>
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name).unwrap();
        match self.changed_to_at {
            Some(changed_to_at) => {
                write!(f, " {}", changed_to_at)
            },
            None => {
                write!(f, " First name")
            },
        }
    }
}