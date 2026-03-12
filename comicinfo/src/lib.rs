mod comic_info;
mod comic_page_info;

use serde::Deserialize;

pub use crate::comic_info::*;
pub use crate::comic_page_info::*;

pub(crate) fn serialize_yes_no<S>(value: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
{
    match value {
        Some(true) => serializer.serialize_str("Yes"),
        Some(false) => serializer.serialize_str("No"),
        None => serializer.serialize_none(),
    }
}

pub(crate) fn deserialize_yes_no<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
    where
        D: serde::Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_ref() {
        "Yes" => Ok(Some(true)),
        "No" => Ok(Some(false)),
        other => Err(serde::de::Error::custom(format!("Expected 'Yes' or 'No', got '{}'", other))),
    }
}

pub(crate) fn serialize_vec_csv<S>(value: &Option<Vec<String>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(v) => serializer.serialize_str(&v.join(",")),
        None => serializer.serialize_none(),
    }
}

pub(crate) fn deserialize_vec_csv<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let v = s.split(',').map(|s| s.trim().to_string()).collect();
    Ok(Some(v))
}
