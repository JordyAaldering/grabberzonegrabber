use serde::{Deserialize, Serialize, Serializer};
use serde_with::{StringWithSeparator, formats::CommaSeparator, serde_as, skip_serializing_none};

use crate::ArrayOfComicPageInfo;

#[allow(unused)]
type CsvVec = Option<StringWithSeparator<CommaSeparator, String>>;

/// The `ComicInfo.xml` file originates from the ComicRack application, which is not developed anymore.
/// The `ComicInfo.xml` however is used by a variety of applications.
#[derive(Clone, Default, Debug)]
#[derive(Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
#[skip_serializing_none]
#[serde_as]
pub struct ComicInfo {
    /// Title of the book.
    pub title: Option<String>,

    /// A group or collection the series belongs to.
    #[serde_as(as = "CsvVec")]
    pub series_group: Option<Vec<String>>,

    /// Title of the series the book is part of.
    pub series: Option<String>,

    /// Number of the book in the series.
    pub number: Option<usize>,

    /// Total number of books in the series.
    pub count: Option<usize>,

    /// Volume containing the book.
    ///
    /// Volume is a notion that is specific to US comics, where the same series can have multiple volumes.
    /// Volumes can be referenced by number (1, 2, 3, …) or by year (2018, 2020, …).
    pub volume: Option<usize>,

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

    /// Alternate title of the series the book is part of.
    /// Quite specific to US comics, some books can be part of cross-over story arcs.
    pub alternate_series: Option<String>,

    /// Alternate number of the book in the series..
    /// Quite specific to US comics, some books can be part of cross-over story arcs.
    pub alternate_number: Option<String>,

    /// Alternate total number of books in the series.
    /// Quite specific to US comics, some books can be part of cross-over story arcs.
    pub alternate_count: Option<String>,

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
    #[serde_as(as = "CsvVec")]
    pub writer: Option<Vec<String>>,

    /// Person or organization responsible for drawing the art.
    #[serde_as(as = "CsvVec")]
    pub penciller: Option<Vec<String>>,

    /// Person or organization responsible for inking the pencil art.
    #[serde_as(as = "CsvVec")]
    pub inker: Option<Vec<String>>,

    /// Person or organization responsible for applying color to drawings.
    #[serde_as(as = "CsvVec")]
    pub colorist: Option<Vec<String>>,

    /// Person or organization responsible for drawing text and speech bubbles.
    #[serde_as(as = "CsvVec")]
    pub letterer: Option<Vec<String>>,

    /// Person or organization responsible for drawing the cover art.
    #[serde_as(as = "CsvVec")]
    pub cover_artist: Option<Vec<String>>,

    /// A person or organization contributing to a resource by revising or elucidating the content, e.g., adding an introduction, notes, or other critical matter.
    /// An editor may also prepare a resource for production, publication, or distribution.
    #[serde_as(as = "CsvVec")]
    pub editor: Option<Vec<String>>,

    /// A person or organization who renders a text from one language into another, or from an older form of a language into the modern form.
    #[serde_as(as = "CsvVec")]
    pub translator: Option<Vec<String>>,

    /// A person or organization responsible for publishing, releasing, or issuing a resource.
    #[serde_as(as = "CsvVec")]
    pub publisher: Option<Vec<String>>,

    /// An imprint is a group of publications under the umbrella of a larger imprint or a Publisher.
    ///
    /// For example, Vertigo is an Imprint of DC Comics.
    pub imprint: Option<String>,

    /// Genre of the book or series. For example, Science-Fiction or Shonen.
    #[serde_as(as = "CsvVec")]
    pub genre: Option<Vec<String>>,

    /// Tags of the book or series. For example, ninja or school life.
    #[serde_as(as = "CsvVec")]
    pub tags: Option<Vec<String>>,

    /// A URL pointing to a reference website for the book.
    ///
    /// If a space is a part of the url it must be [percent encoded](https://datatracker.ietf.org/doc/html/rfc2396#section-2.4.1).
    #[serde_as(as = "CsvVec")]
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
    #[serde(rename = "LanguageISO")]
    pub language_iso: Option<String>,

    /// The original publication's binding format for scanned physical books or presentation format for digital sources.
    ///
    /// `TBP`, `HC`, `Web`, `Digital` are common designators.
    pub format: Option<String>,

    /// Whether the book is in black and white.
    pub black_and_white: Option<YesNo>,

    /// Whether the book is a manga.
    pub manga: Option<Manga>,

    /// Characters present in the book.
    #[serde_as(as = "CsvVec")]
    pub characters: Option<Vec<String>>,

    /// Teams present in the book.
    #[serde_as(as = "CsvVec")]
    pub teams: Option<Vec<String>>,

    /// Locations mentioned in the book.
    #[serde_as(as = "CsvVec")]
    pub locations: Option<Vec<String>>,

    /// Main character or team mentioned in the book.
    pub main_character_or_team: Option<String>,

    /// Describes each page of the book.
    pub pages: Option<ArrayOfComicPageInfo>,
}

#[derive(Copy, Clone, Debug)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum YesNo {
    No,
    Yes,
    Unknown,
}

#[derive(Copy, Clone, Debug)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Manga {
    No,
    Yes,
    YesAndRightToLeft,
    Unknown,
}

#[derive(Copy, Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub enum AgeRating {
    #[serde(rename = "AdultsOnly18+")]
    AdultsOnly18Plus,
    #[serde(rename = "EarlyChildhood")]
    EarlyChildhood,
    #[serde(rename = "Everyone")]
    Everyone,
    #[serde(rename = "Everyone10+")]
    Everyone10Plus,
    #[serde(rename = "G")]
    G,
    #[serde(rename = "KidsToAdults")]
    KidsToAdults,
    #[serde(rename = "M")]
    M,
    #[serde(rename = "MA15+")]
    MA15Plus,
    #[serde(rename = "Mature17+")]
    Mature17Plus,
    #[serde(rename = "PG")]
    PG,
    #[serde(rename = "R18+")]
    R18Plus,
    #[serde(rename = "RatingPending")]
    RatingPending,
    #[serde(rename = "Teen")]
    Teen,
    #[serde(rename = "X18+")]
    X18Plus,
}

pub struct CommunityRating(pub f32);

impl Serialize for CommunityRating {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{:.1}", self.0))
    }
}

impl<'de> Deserialize<'de> for CommunityRating {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse::<f32>()
            .map(CommunityRating)
            .map_err(|e| serde::de::Error::custom(format!("Failed to parse community rating: {}", e)))
    }
}

#[derive(Copy, Clone, Debug)]
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
