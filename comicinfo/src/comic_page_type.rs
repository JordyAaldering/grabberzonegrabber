use serde::{Deserialize, Serialize};

/// Type of a comic book page.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
