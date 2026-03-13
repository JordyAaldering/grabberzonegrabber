use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug)]
#[derive(Serialize, Deserialize)]
pub enum AgeRating {
    #[serde(rename = "AdultsOnly18+")]
    AdultsOnly18Plus,
    #[serde(rename = "EarlyChildhood")]
    EarlyChildhood,
    #[serde(rename = "Everyone")]
    Everyone,
    #[serde(rename = "Everyone10+")]
    Everyone10Plus,
    #[serde(rename = "G")]
    G,
    #[serde(rename = "KidsToAdults")]
    KidsToAdults,
    #[serde(rename = "M")]
    M,
    #[serde(rename = "MA15+")]
    MA15Plus,
    #[serde(rename = "Mature17+")]
    Mature17Plus,
    #[serde(rename = "PG")]
    PG,
    #[serde(rename = "R18+")]
    R18Plus,
    #[serde(rename = "RatingPending")]
    RatingPending,
    #[serde(rename = "Teen")]
    Teen,
    #[serde(rename = "X18+")]
    X18Plus,
}

impl fmt::Display for AgeRating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use AgeRating::*;
        match self {
            AdultsOnly18Plus => write!(f, "AdultsOnly18+"),
            EarlyChildhood   => write!(f, "EarlyChildhood"),
            Everyone         => write!(f, "Everyone"),
            Everyone10Plus   => write!(f, "Everyone10+"),
            G                => write!(f, "G"),
            KidsToAdults     => write!(f, "KidsToAdults"),
            M                => write!(f, "M"),
            MA15Plus         => write!(f, "MA15+"),
            Mature17Plus     => write!(f, "Mature17+"),
            PG               => write!(f, "PG"),
            R18Plus          => write!(f, "R18+"),
            RatingPending    => write!(f, "RatingPending"),
            Teen             => write!(f, "Teen"),
            X18Plus          => write!(f, "X18+"),
        }
    }
}
