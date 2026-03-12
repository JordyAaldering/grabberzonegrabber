use serde::{Serialize, Serializer};

use crate::serialize_yes_no;

/// Wrapper to holds all pages of the book.
#[derive(Clone, Default, Serialize)]
pub struct ArrayOfComicPageInfo {
    #[serde(rename = "Page")]
    pub pages: Vec<ComicPageInfo>,
}

/// Describes each page of the book.
#[derive(Clone, Default, Serialize)]
#[serde(rename = "Page")]
pub struct ComicPageInfo {
    /// Page number.
    #[serde(rename = "@Image")]
    pub image: usize,

    /// Type of the page.
    #[serde(rename = "@Type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ComicPageType>,

    /// Whether the page is a double spread.
    #[serde(rename = "@DoublePage", skip_serializing_if = "Option::is_none", serialize_with = "serialize_yes_no")]
    pub double_page: Option<bool>,

    /// Width of the image in pixels.
    #[serde(rename = "@ImageWidth", skip_serializing_if = "Option::is_none")]
    pub image_width: Option<usize>,

    /// Height of the image in pixels.
    #[serde(rename = "@ImageHeight", skip_serializing_if = "Option::is_none")]
    pub image_height: Option<usize>,

    /// File size of the image, supposedly in bytes.
    #[serde(rename = "@FileSize", skip_serializing_if = "Option::is_none")]
    pub file_size: Option<usize>,

    /// ComicRack uses this field when adding a bookmark in a book.
    #[serde(rename = "@Bookmark", skip_serializing_if = "Option::is_none")]
    pub bookmark: Option<String>,

    /// Unknown.
    #[serde(rename = "@Key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

/// Type of a comic book page.
#[derive(Copy, Clone)]
pub enum ComicPageType {
    /// The front cover of the book.
    FrontCover,
    /// Sometimes found inside the book as a second cover.
    InnerCover,
    /// Summary of previous issues.
    Roundup,
    /// The main content of the book.
    Story,
    /// An advertisement page.
    Advertisement,
    /// Editorial content, such as a letter from the editor.
    Editorial,
    /// Letters from readers.
    Letters,
    /// Sneak preview of the next book, or another comic.
    Preview,
    /// The back cover of the book.
    BackCover,
    /// Anything not covered above
    Other,
    /// Indicates that the page should not be shown.
    Deleted,
}

impl Serialize for ComicPageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use ComicPageType::*;
        match *self {
            FrontCover    => serializer.serialize_str("FrontCover"),
            InnerCover    => serializer.serialize_str("InnerCover"),
            Roundup       => serializer.serialize_str("Roundup"),
            Story         => serializer.serialize_str("Story"),
            Advertisement => serializer.serialize_str("Advertisement"),
            Editorial     => serializer.serialize_str("Editorial"),
            Letters       => serializer.serialize_str("Letters"),
            Preview       => serializer.serialize_str("Preview"),
            BackCover     => serializer.serialize_str("BackCover"),
            Other         => serializer.serialize_str("Other"),
            Deleted       => serializer.serialize_str("Deleted"),
        }
    }
}
