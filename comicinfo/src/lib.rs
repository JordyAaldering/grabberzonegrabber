mod comic_info;
mod comic_page_info;

pub use comic_info::*;
pub use comic_page_info::*;

pub(crate) fn serialize_vec_csv<S>(value: &Option<Vec<String>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(v) => serializer.serialize_str(&v.join(",")),
        None => serializer.serialize_none(),
    }
}

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
