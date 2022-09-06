use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug)]
pub struct Name {
    pub name: String,
    #[serde(rename(serialize = "changedToAt", deserialize = "changedToAt"))]
    pub changed_to_at: Option<i64>,
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name).unwrap();
        match self.changed_to_at {
            Some(changed_to_at) => {
                let date = DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp(changed_to_at / 1000, 0),
                    Utc,
                );
                write!(f, " {}", date.format("%Y-%m-%d"))
            }
            None => {
                write!(f, " First name")
            }
        }
    }
}
