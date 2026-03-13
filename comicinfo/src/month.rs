use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug)]
#[derive(Serialize)]
#[serde(into = "u8")]
pub enum Month {
    Jan = 1,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

impl From<Month> for u8 {
    fn from(m: Month) -> Self {
        m as u8
    }
}

impl<'de> Deserialize<'de> for Month {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.to_lowercase().as_str() {
            "1" | "01" | "jan" | "january"   => Ok(Month::Jan),
            "2" | "02" | "feb" | "february"  => Ok(Month::Feb),
            "3" | "03" | "mar" | "march"     => Ok(Month::Mar),
            "4" | "04" | "apr" | "april"     => Ok(Month::Apr),
            "5" | "05" | "may"               => Ok(Month::May),
            "6" | "06" | "jun" | "june"      => Ok(Month::Jun),
            "7" | "07" | "jul" | "july"      => Ok(Month::Jul),
            "8" | "08" | "aug" | "august"    => Ok(Month::Aug),
            "9" | "09" | "sep" | "september" => Ok(Month::Sep),
                  "10" | "oct" | "october"   => Ok(Month::Oct),
                  "11" | "nov" | "november"  => Ok(Month::Nov),
                  "12" | "dec" | "december"  => Ok(Month::Dec),
            other => Err(serde::de::Error::custom(format!("Invalid month: {}", other))),
        }
    }
}
