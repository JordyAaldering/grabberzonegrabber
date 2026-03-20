use std::cmp;

use serde::{Deserialize, Serialize};

use crate::{ComicPageType, YesNo, merge};

/// Describes each page of the book.
#[derive(Clone, Default, Debug, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
#[serde_with::skip_serializing_none]
pub struct Page {
    /// Page number.
    #[serde(rename = "@Image")]
    pub image: usize,

    /// Type of the page.
    #[serde(rename = "@Type")]
    pub r#type: Option<ComicPageType>,

    /// Whether the page is a double spread.
    #[serde(rename = "@DoublePage")]
    pub double_page: Option<YesNo>,

    /// Width of the image in pixels.
    #[serde(rename = "@ImageWidth")]
    pub image_width: Option<usize>,

    /// Height of the image in pixels.
    #[serde(rename = "@ImageHeight")]
    pub image_height: Option<usize>,

    /// File size of the image, supposedly in bytes.
    #[serde(rename = "@FileSize")]
    pub file_size: Option<usize>,

    /// ComicRack uses this field when adding a bookmark in a book.
    #[serde(rename = "@Bookmark")]
    pub bookmark: Option<String>,

    /// Unknown.
    #[serde(rename = "@Key")]
    pub key: Option<String>,
}

impl Page {
    pub fn merge(&self, other: &Self) -> Self {
        assert_eq!(self.image, other.image);
        Self {
            image: self.image,
            r#type: merge(&self.r#type, &other.r#type),
            double_page: merge(&self.double_page, &other.double_page),
            image_width: merge(&self.image_width, &other.image_width),
            image_height: merge(&self.image_height, &other.image_height),
            file_size: merge(&self.file_size, &other.file_size),
            bookmark: merge(&self.bookmark, &other.bookmark),
            key: merge(&self.key, &other.key),
        }
    }
}

impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Page {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.image.cmp(&other.image)
    }
}
