/// Describes each page of the book.
pub struct ComicPageInfo {
    /// Page number.
    pub image: usize,
    /// Type of the page.
    pub r#type: Option<ComicPageType>,
    /// Whether the page is a double spread.
    pub double_page: Option<bool>,
    /// Width of the image in pixels.
    pub image_width: Option<usize>,
    /// Height of the image in pixels.
    pub image_height: Option<usize>,
    /// File size of the image, supposedly in bytes.
    pub file_size: Option<usize>,
    /// ComicRack uses this field when adding a bookmark in a book.
    pub bookmark: Option<String>,
    /// Unknown.
    pub key: Option<String>,
}

/// Type of a comic book page.
pub enum ComicPageType {
    FrontCover,
    /// Sometimes found inside the book as a second cover.
    InnerCover,
    /// Summary of previous issues.
    Roundup,
    Story,
    Advertisement,
    Editorial,
    Letters,
    /// Sneak preview of the next book, or another comic.
    Preview,
    BackCover,
    Other,
    /// Indicates that the page should not be shown.
    Deleted,
}
