use serde::{Deserialize, Serialize};

use crate::YesNo;

/// Wrapper to holds all pages of the book.
#[derive(Clone, Default, Debug)]
#[derive(Serialize, Deserialize)]
#[serde(default, rename = "ArrayOfComicPageInfo")]
pub struct ArrayOfComicPageInfo {
    #[serde(rename = "Page")]
    pub pages: Vec<Page>,
}

/// Describes each page of the book.
#[derive(Clone, Default, Debug)]
#[derive(Serialize, Deserialize)]
#[serde(default, rename = "Page")]
pub struct Page {
    /// Page number.
    #[serde(rename = "@Image")]
    pub image: usize,

    /// Type of the page.
    #[serde(rename = "@Type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ComicPageType>,

    /// Whether the page is a double spread.
    #[serde(rename = "@DoublePage", skip_serializing_if = "Option::is_none")]
    pub double_page: Option<YesNo>,

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
#[derive(Copy, Clone, Debug)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
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
