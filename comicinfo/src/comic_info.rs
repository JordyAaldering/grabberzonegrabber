use crate::ComicPageInfo;

/// The `ComicInfo.xml` file originates from the ComicRack application, which is not developed anymore.
/// The `ComicInfo.xml` however is used by a variety of applications.
pub struct ComicInfo {
    /// Title of the book.
    pub title: Option<String>,
    /// A group or collection the series belongs to.
    pub series_group: Option<Vec<String>>,
    /// Title of the series the book is part of.
    pub series: Option<String>,
    /// Number of the book in the series.
    pub number: Option<usize>,
    /// The total number of books in the series.
    pub count: Option<usize>,
    /// Volume containing the book.
    ///
    /// Volume is a notion that is specific to US comics, where the same series can have multiple volumes.
    /// Volumes can be referenced by number (1, 2, 3, …) or by year (2018, 2020, …).
    pub volume: Option<Volume>,
    /// The story arc that books belong to.
    pub story_arc: Option<String>,
    /// While StoryArc was originally designed to store the arc within a series, it was often used to
    /// indicate that a book was part of a reading order, composed of books from multiple series.
    /// Mylar for instance was using the field as such.
    ///
    /// Since StoryArc itself wasn't able to carry the information about ordering of books within a reading order, StoryArcNumber was added.
    ///
    /// StoryArc and StoryArcNumber can work in combination, to indicate in which position the book is located at for a specific reading order.
    ///
    /// It is accepted that multiple values can be specified for both StoryArc and StoryArcNumber. Multiple values are comma separated.
    pub story_arc_number: Option<usize>,
    /// The number of pages in the book.
    pub page_count: Option<usize>,

    // /// Quite specific to US comics, some books can be part of cross-over story arcs.
    // ///
    // /// Those fields can be used to specify an alternate series, its number, and count of books.
    // ///
    // /// Excluded for now.
    // pub alternate_series: Option<String>,
    // pub alternate_number: Option<String>,
    // pub alternate_count: Option<String>,

    /// Usually contains the release year of the book.
    pub year: Option<u16>,
    /// Usually contains the release month of the book.
    pub month: Option<Month>,
    /// Usually contains the release day of the book.
    pub day: Option<u8>,

    /// Age rating of the book.
    pub age_rating: Option<AgeRating>,
    /// Community rating of the book, from 0.0 to 5.0.
    pub community_rating: Option<f32>,
    /// A description or summary of the book.
    pub description: Option<String>,
    /// A free text field, usually used to store information about the application that created the `ComicInfo.xml`` file.
    pub notes: Option<String>,
    /// A free text field, usually used to store information about who scanned the book.
    pub scan_information: Option<String>,
    /// Review of the book.
    pub reviews: Option<String>,

    /// Person or organization responsible for creating the scenario.
    ///
    /// It is accepted that multiple values are comma separated.
    pub writer: Option<String>,
    /// Person or organization responsible for drawing the art.
    ///
    /// It is accepted that multiple values are comma separated.
    pub penciller: Option<String>,
    /// Person or organization responsible for inking the pencil art.
    ///
    /// It is accepted that multiple values are comma separated.
    pub inker: Option<String>,
    /// Person or organization responsible for applying color to drawings.
    ///
    /// It is accepted that multiple values are comma separated.
    pub colorist: Option<String>,
    /// Person or organization responsible for drawing text and speech bubbles.
    ///
    /// It is accepted that multiple values are comma separated.
    pub letterer: Option<String>,
    /// Person or organization responsible for drawing the cover art.
    ///
    /// It is accepted that multiple values are comma separated.
    pub cover_artist: Option<String>,
    /// A person or organization contributing to a resource by revising or elucidating the content, e.g., adding an introduction, notes, or other critical matter.
    /// An editor may also prepare a resource for production, publication, or distribution.
    ///
    /// It is accepted that multiple values are comma separated.
    pub editor: Option<String>,
    /// A person or organization who renders a text from one language into another, or from an older form of a language into the modern form.
    ///
    /// It is accepted that multiple values are comma separated.
    pub translator: Option<String>,
    /// A person or organization responsible for publishing, releasing, or issuing a resource.
    ///
    /// It is accepted that multiple values are comma separated.
    pub publisher: Option<String>,
    /// An imprint is a group of publications under the umbrella of a larger imprint or a Publisher.
    ///
    /// For example, Vertigo is an Imprint of DC Comics.
    pub imprint: Option<String>,

    /// Genre of the book or series. For example, Science-Fiction or Shonen.
    ///
    /// It is accepted that multiple values are comma separated.
    pub genre: Option<Vec<String>>,
    /// Tags of the book or series. For example, ninja or school life.
    ///
    /// It is accepted that multiple values are comma separated.
    pub tags: Option<Vec<String>>,
    /// A URL pointing to a reference website for the book.
    ///
    /// It is accepted that multiple values are space separated.
    /// If a space is a part of the url it must be [percent encoded](https://datatracker.ietf.org/doc/html/rfc2396#section-2.4.1).
    pub url: Option<Vec<String>>,

    /// A [Global Trade Item Number](https://en.wikipedia.org/wiki/Global_Trade_Item_Number) identifying the book.
    ///
    /// GTIN incorporates other standards like ISBN, ISSN, EAN, or JAN.
    pub gtin: Option<String>,

    /// A language code describing the language of the book.
    ///
    /// Without any information on what kind of code this element is supposed to contain, it is recommended to use the [IETF BCP 47 language tag](https://en.wikipedia.org/wiki/IETF_language_tag), which can describe the language but also the script used.
    /// This helps to differentiate languages with multiple scripts, like Traditional and Simplified Chinese.
    ///
    /// See also:
    ///  * [Choosing a language tag - W3C](https://www.w3.org/International/questions/qa-choosing-language-tags)
    ///  * [Language subtag lookup app](https://r12a.github.io/app-subtags/)
    pub language_iso: Option<String>,
    /// The original publication's binding format for scanned physical books or presentation format for digital sources.
    ///
    /// `TBP`, `HC`, `Web`, `Digital` are common designators.
    pub format: Option<String>,

    /// Whether the book is in black and white.
    pub black_and_white: Option<bool>,
    /// Whether the book is a manga.
    pub manga: Option<Manga>,

    /// Characters present in the book.
    pub characters: Option<Vec<String>>,
    /// Teams present in the book.
    pub teams: Option<Vec<String>>,
    /// Locations mentioned in the book.
    pub locations: Option<Vec<String>>,
    /// Main character or team mentioned in the book.
    pub main_character_or_team: Option<String>,

    pub pages: Option<Vec<ComicPageInfo>>,
}

pub enum Volume {
    Number(usize),
    Year(u16),
}

pub enum Manga {
    No,
    Yes,
    YesAndRightToLeft,
}

pub enum AgeRating {
    Unknown,
    AdultsOnly18Plus,
    EarlyChildhood,
    Everyone,
    Everyone10Plus,
    G,
    KidsToAdults,
    M,
    MA15Plus,
    Mature17Plus,
    PG,
    R18Plus,
    RatingPending,
    Teen,
    X18Plus,
}

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
