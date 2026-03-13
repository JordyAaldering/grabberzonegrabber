use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum YesNo {
    No,
    Yes,
    Unknown,
}

impl fmt::Display for YesNo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use YesNo::*;
        match self {
            No => write!(f, "No"),
            Yes => write!(f, "Yes"),
            Unknown => write!(f, "Unknown"),
        }
    }
}
