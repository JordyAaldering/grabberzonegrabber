mod age_rating;
mod comic_info;
mod comic_page_info;
mod comic_page_info_array;
mod comic_page_type;
mod community_rating;
mod manga;
mod month;
mod yes_no;

pub use age_rating::*;
pub use comic_info::*;
pub use comic_page_info::*;
pub use comic_page_info_array::*;
pub use comic_page_type::*;
pub use community_rating::*;
pub use manga::*;
pub use month::*;
pub use yes_no::*;

pub(crate) fn merge<T>(l: &Option<T>, r: &Option<T>) -> Option<T>
where
    T: Clone + PartialEq
{
    match (l, r) {
        (l, r) if l == r => l.clone(),
        (Some(l), None) => Some(l.clone()),
        (None, Some(r)) => Some(r.clone()),
        _ => None,
    }
}
