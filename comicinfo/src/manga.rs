use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Manga {
    No,
    Yes,
    YesAndRightToLeft,
    Unknown,
}

impl fmt::Display for Manga {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Manga::*;
        match self {
            No => write!(f, "No"),
            Yes => write!(f, "Yes"),
            YesAndRightToLeft => write!(f, "YesAndRightToLeft"),
            Unknown => write!(f, "Unknown"),
        }
    }
}
