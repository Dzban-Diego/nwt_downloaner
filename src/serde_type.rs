use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub status: i64,
    pub current_locale: String,
    pub vernacular_section_headings: VernacularSectionHeadings,
    pub ranges: Ranges,
    pub edition_data: EditionData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VernacularSectionHeadings {
    #[serde(rename = "HdgMedia")]
    pub hdg_media: String,
    #[serde(rename = "HdgFootnotes")]
    pub hdg_footnotes: String,
    #[serde(rename = "HdgMarginalReferences")]
    pub hdg_marginal_references: String,
    #[serde(rename = "MarginalReferenceChars")]
    pub marginal_reference_chars: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ranges {
    #[serde(rename = "42001024-42009015")]
    pub n42001024_42009015: N4200102442009015,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct N4200102442009015 {
    pub citation: String,
    pub link: String,
    pub valid_range: String,
    pub citation_verse_range: String,
    pub html: String,
    pub footnotes: Vec<Footnote>,
    pub superscriptions: Vec<Value>,
    pub multimedia: Vec<Multimedum>,
    pub commentaries: Vec<Commentary>,
    pub cross_references: Vec<CrossReference>,
    pub verses: Vec<Verse>,
    pub num_translations: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Footnote {
    pub id: i64,
    pub content: String,
    pub source: String,
    pub anchor: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Multimedum {
    pub id: i64,
    #[serde(rename = "docID")]
    pub doc_id: String,
    pub source: String,
    pub label: String,
    pub caption: String,
    pub picture_credit: Option<String>,
    pub resource: Resource,
    #[serde(rename = "type")]
    pub type_field: String,
    pub thumbnail: Thumbnail,
    pub keyframe: Option<Keyframe>,
    pub source_standard_citations: Vec<SourceStandardCitation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub src: Value,
    pub sizes: Option<Sizes>,
    pub zoom: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub xl: String,
    pub lg: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub src: String,
    pub sizes: Sizes2,
    pub zoom: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes2 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keyframe {
    pub src: String,
    pub sizes: Sizes3,
    pub zoom: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes3 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceStandardCitation {
    pub vs: String,
    pub standard_citation: String,
    pub abbreviated_citation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Commentary {
    pub id: i64,
    pub content: Option<String>,
    pub source: Option<String>,
    pub label: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrossReference {
    pub id: i64,
    pub source: String,
    pub targets: Vec<Target>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    pub vs: String,
    pub standard_citation: String,
    pub abbreviated_citation: String,
    pub category: Category,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: String,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Verse {
    #[serde(rename = "vsID")]
    pub vs_id: String,
    pub book_number: i64,
    pub chapter_number: i64,
    pub verse_number: i64,
    pub standard_citation: String,
    pub abbreviated_citation: String,
    pub content: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditionData {
    pub locale: String,
    pub book_count: String,
    pub vernacular_full_name: String,
    pub vernacular_short_name: Value,
    pub vernacular_abbreviation: String,
    pub url: String,
    pub title_format: String,
    #[serde(rename = "pageCSSClassNames")]
    pub page_cssclass_names: String,
    #[serde(rename = "articleCSSClassNames")]
    pub article_cssclass_names: String,
    pub books: Books,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Books {
    #[serde(rename = "1")]
    pub n1: Book,
    #[serde(rename = "2")]
    pub n2: Book,
    #[serde(rename = "3")]
    pub n3: Book,
    #[serde(rename = "4")]
    pub n4: Book,
    #[serde(rename = "5")]
    pub n5: Book,
    #[serde(rename = "6")]
    pub n6: Book,
    #[serde(rename = "7")]
    pub n7: Book,
    #[serde(rename = "8")]
    pub n8: Book,
    #[serde(rename = "9")]
    pub n9: Book,
    #[serde(rename = "10")]
    pub n10: Book,
    #[serde(rename = "11")]
    pub n11: Book,
    #[serde(rename = "12")]
    pub n12: Book,
    #[serde(rename = "13")]
    pub n13: Book,
    #[serde(rename = "14")]
    pub n14: Book,
    #[serde(rename = "15")]
    pub n15: Book,
    #[serde(rename = "16")]
    pub n16: Book,
    #[serde(rename = "17")]
    pub n17: Book,
    #[serde(rename = "18")]
    pub n18: Book,
    #[serde(rename = "19")]
    pub n19: Book,
    #[serde(rename = "20")]
    pub n20: Book,
    #[serde(rename = "21")]
    pub n21: Book,
    #[serde(rename = "22")]
    pub n22: Book,
    #[serde(rename = "23")]
    pub n23: Book,
    #[serde(rename = "24")]
    pub n24: Book,
    #[serde(rename = "25")]
    pub n25: Book,
    #[serde(rename = "26")]
    pub n26: Book,
    #[serde(rename = "27")]
    pub n27: Book,
    #[serde(rename = "28")]
    pub n28: Book,
    #[serde(rename = "29")]
    pub n29: Book,
    #[serde(rename = "30")]
    pub n30: Book,
    #[serde(rename = "31")]
    pub n31: Book,
    #[serde(rename = "32")]
    pub n32: Book,
    #[serde(rename = "33")]
    pub n33: Book,
    #[serde(rename = "34")]
    pub n34: Book,
    #[serde(rename = "35")]
    pub n35: Book,
    #[serde(rename = "36")]
    pub n36: Book,
    #[serde(rename = "37")]
    pub n37: Book,
    #[serde(rename = "38")]
    pub n38: Book,
    #[serde(rename = "39")]
    pub n39: Book,
    #[serde(rename = "40")]
    pub n40: Book,
    #[serde(rename = "41")]
    pub n41: Book,
    #[serde(rename = "42")]
    pub n42: Book,
    #[serde(rename = "43")]
    pub n43: Book,
    #[serde(rename = "44")]
    pub n44: Book,
    #[serde(rename = "45")]
    pub n45: Book,
    #[serde(rename = "46")]
    pub n46: Book,
    #[serde(rename = "47")]
    pub n47: Book,
    #[serde(rename = "48")]
    pub n48: Book,
    #[serde(rename = "49")]
    pub n49: Book,
    #[serde(rename = "50")]
    pub n50: Book,
    #[serde(rename = "51")]
    pub n51: Book,
    #[serde(rename = "52")]
    pub n52: Book,
    #[serde(rename = "53")]
    pub n53: Book,
    #[serde(rename = "54")]
    pub n54: Book,
    #[serde(rename = "55")]
    pub n55: Book,
    #[serde(rename = "56")]
    pub n56: Book,
    #[serde(rename = "57")]
    pub n57: Book,
    #[serde(rename = "58")]
    pub n58: Book,
    #[serde(rename = "59")]
    pub n59: Book,
    #[serde(rename = "60")]
    pub n60: Book,
    #[serde(rename = "61")]
    pub n61: Book,
    #[serde(rename = "62")]
    pub n62: Book,
    #[serde(rename = "63")]
    pub n63: Book,
    #[serde(rename = "64")]
    pub n64: Book,
    #[serde(rename = "65")]
    pub n65: Book,
    #[serde(rename = "66")]
    pub n66: Book,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    pub chapter_count: String,
    pub standard_name: String,
    pub standard_abbreviation: String,
    pub official_abbreviation: String,
    pub standard_singular_book_name: String,
    pub standard_singular_abbreviation: String,
    pub official_singular_abbreviation: String,
    pub standard_plural_book_name: String,
    pub standard_plural_abbreviation: String,
    pub official_plural_abbreviation: String,
    pub book_display_title: String,
    pub chapter_display_title: String,
    pub url_segment: String,
    pub url: String,
    pub has_audio: bool,
    pub has_multimedia: bool,
    pub has_study_notes: bool,
    pub additional_pages: Vec<AdditionalPage>,
    pub images: Vec<Image>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage {
    pub title: String,
    pub meps_title: String,
    pub url: String,
    #[serde(rename = "pageID")]
    pub page_id: String,
    #[serde(rename = "pageCSSClassNames")]
    pub page_cssclass_names: String,
    #[serde(rename = "articleCSSClassNames")]
    pub article_cssclass_names: String,
    pub abbreviated_title: String,
    pub doc_class: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes4,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes4 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}
