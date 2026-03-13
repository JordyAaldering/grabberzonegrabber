use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
#[serde_as]
pub struct CommunityRating(
    #[serde_as(as = "DisplayFromStr")]
    pub f32
);

impl FromStr for CommunityRating {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: f32 = s.parse().map_err(|_| "invalid rating")?;
        if (v < 0.0) | (v > 5.0) {
            Err("rating out of range")
        } else {
            Ok(Self(v))
        }
    }
}

impl fmt::Display for CommunityRating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
