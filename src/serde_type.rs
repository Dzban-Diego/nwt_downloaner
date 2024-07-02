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
    #[serde(flatten)]
    pub n42001024_42009015: n4200102442009015,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n4200102442009015 {
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
    pub n1: n1,
    #[serde(rename = "2")]
    pub n2: n2,
    #[serde(rename = "3")]
    pub n3: n3,
    #[serde(rename = "4")]
    pub n4: n4,
    #[serde(rename = "5")]
    pub n5: n5,
    #[serde(rename = "6")]
    pub n6: n6,
    #[serde(rename = "7")]
    pub n7: n7,
    #[serde(rename = "8")]
    pub n8: n8,
    #[serde(rename = "9")]
    pub n9: n9,
    #[serde(rename = "10")]
    pub n10: n10,
    #[serde(rename = "11")]
    pub n11: n11,
    #[serde(rename = "12")]
    pub n12: n12,
    #[serde(rename = "13")]
    pub n13: n13,
    #[serde(rename = "14")]
    pub n14: n14,
    #[serde(rename = "15")]
    pub n15: n15,
    #[serde(rename = "16")]
    pub n16: n16,
    #[serde(rename = "17")]
    pub n17: n17,
    #[serde(rename = "18")]
    pub n18: n18,
    #[serde(rename = "19")]
    pub n19: n19,
    #[serde(rename = "20")]
    pub n20: n20,
    #[serde(rename = "21")]
    pub n21: n21,
    #[serde(rename = "22")]
    pub n22: n22,
    #[serde(rename = "23")]
    pub n23: n23,
    #[serde(rename = "24")]
    pub n24: n24,
    #[serde(rename = "25")]
    pub n25: n25,
    #[serde(rename = "26")]
    pub n26: n26,
    #[serde(rename = "27")]
    pub n27: n27,
    #[serde(rename = "28")]
    pub n28: n28,
    #[serde(rename = "29")]
    pub n29: n29,
    #[serde(rename = "30")]
    pub n30: n30,
    #[serde(rename = "31")]
    pub n31: n31,
    #[serde(rename = "32")]
    pub n32: n32,
    #[serde(rename = "33")]
    pub n33: n33,
    #[serde(rename = "34")]
    pub n34: n34,
    #[serde(rename = "35")]
    pub n35: n35,
    #[serde(rename = "36")]
    pub n36: n36,
    #[serde(rename = "37")]
    pub n37: n37,
    #[serde(rename = "38")]
    pub n38: n38,
    #[serde(rename = "39")]
    pub n39: n39,
    #[serde(rename = "40")]
    pub n40: n40,
    #[serde(rename = "41")]
    pub n41: n41,
    #[serde(rename = "42")]
    pub n42: n42,
    #[serde(rename = "43")]
    pub n43: n43,
    #[serde(rename = "44")]
    pub n44: n44,
    #[serde(rename = "45")]
    pub n45: n45,
    #[serde(rename = "46")]
    pub n46: n46,
    #[serde(rename = "47")]
    pub n47: n47,
    #[serde(rename = "48")]
    pub n48: n48,
    #[serde(rename = "49")]
    pub n49: n49,
    #[serde(rename = "50")]
    pub n50: n50,
    #[serde(rename = "51")]
    pub n51: n51,
    #[serde(rename = "52")]
    pub n52: n52,
    #[serde(rename = "53")]
    pub n53: n53,
    #[serde(rename = "54")]
    pub n54: n54,
    #[serde(rename = "55")]
    pub n55: n55,
    #[serde(rename = "56")]
    pub n56: n56,
    #[serde(rename = "57")]
    pub n57: n57,
    #[serde(rename = "58")]
    pub n58: n58,
    #[serde(rename = "59")]
    pub n59: n59,
    #[serde(rename = "60")]
    pub n60: n60,
    #[serde(rename = "61")]
    pub n61: n61,
    #[serde(rename = "62")]
    pub n62: n62,
    #[serde(rename = "63")]
    pub n63: n63,
    #[serde(rename = "64")]
    pub n64: n64,
    #[serde(rename = "65")]
    pub n65: n65,
    #[serde(rename = "66")]
    pub n66: n66,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1 {
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n2 {
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
    pub additional_pages: Vec<AdditionalPage2>,
    pub images: Vec<Image2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage2 {
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
pub struct Image2 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes5,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes5 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n3 {
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
    pub additional_pages: Vec<AdditionalPage3>,
    pub images: Vec<Image3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage3 {
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
pub struct Image3 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes6,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes6 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n4 {
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
    pub additional_pages: Vec<AdditionalPage4>,
    pub images: Vec<Image4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage4 {
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
pub struct Image4 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes7,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes7 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n5 {
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
    pub additional_pages: Vec<AdditionalPage5>,
    pub images: Vec<Image5>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage5 {
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
pub struct Image5 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes8,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes8 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6 {
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
    pub additional_pages: Vec<AdditionalPage6>,
    pub images: Vec<Image6>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage6 {
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
pub struct Image6 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes9,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes9 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7 {
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
    pub additional_pages: Vec<AdditionalPage7>,
    pub images: Vec<Image7>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage7 {
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
pub struct Image7 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes10,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes10 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n8 {
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
    pub additional_pages: Vec<AdditionalPage8>,
    pub images: Vec<Image8>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage8 {
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
pub struct Image8 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes11,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes11 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9 {
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
    pub additional_pages: Vec<AdditionalPage9>,
    pub images: Vec<Image9>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage9 {
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
pub struct Image9 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes12,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes12 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n10 {
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
    pub additional_pages: Vec<AdditionalPage10>,
    pub images: Vec<Image10>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage10 {
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
pub struct Image10 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes13,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes13 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n11 {
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
    pub additional_pages: Vec<AdditionalPage11>,
    pub images: Vec<Image11>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage11 {
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
pub struct Image11 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes14,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes14 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n12 {
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
    pub additional_pages: Vec<AdditionalPage12>,
    pub images: Vec<Image12>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage12 {
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
pub struct Image12 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes15,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes15 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n13 {
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
    pub additional_pages: Vec<AdditionalPage13>,
    pub images: Vec<Image13>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage13 {
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
pub struct Image13 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes16,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes16 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n14 {
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
    pub additional_pages: Vec<AdditionalPage14>,
    pub images: Vec<Image14>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage14 {
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
pub struct Image14 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes17,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes17 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n15 {
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
    pub additional_pages: Vec<AdditionalPage15>,
    pub images: Vec<Image15>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage15 {
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
pub struct Image15 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes18,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes18 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n16 {
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
    pub additional_pages: Vec<AdditionalPage16>,
    pub images: Vec<Image16>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage16 {
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
pub struct Image16 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes19,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes19 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n17 {
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
    pub additional_pages: Vec<AdditionalPage17>,
    pub images: Vec<Image17>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage17 {
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
pub struct Image17 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes20,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes20 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n18 {
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
    pub additional_pages: Vec<AdditionalPage18>,
    pub images: Vec<Image18>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage18 {
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
pub struct Image18 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes21,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes21 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n19 {
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
    pub additional_pages: Vec<AdditionalPage19>,
    pub images: Vec<Image19>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage19 {
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
pub struct Image19 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes22,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes22 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n20 {
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
    pub additional_pages: Vec<AdditionalPage20>,
    pub images: Vec<Image20>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage20 {
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
pub struct Image20 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes23,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes23 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n21 {
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
    pub additional_pages: Vec<AdditionalPage21>,
    pub images: Vec<Image21>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage21 {
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
pub struct Image21 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes24,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes24 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n22 {
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
    pub additional_pages: Vec<AdditionalPage22>,
    pub images: Vec<Image22>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage22 {
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
pub struct Image22 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes25,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes25 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n23 {
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
    pub additional_pages: Vec<AdditionalPage23>,
    pub images: Vec<Image23>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage23 {
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
pub struct Image23 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes26,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes26 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n24 {
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
    pub additional_pages: Vec<AdditionalPage24>,
    pub images: Vec<Image24>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage24 {
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
pub struct Image24 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes27,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes27 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n25 {
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
    pub additional_pages: Vec<AdditionalPage25>,
    pub images: Vec<Image25>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage25 {
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
pub struct Image25 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes28,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes28 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n26 {
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
    pub additional_pages: Vec<AdditionalPage26>,
    pub images: Vec<Image26>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage26 {
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
pub struct Image26 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes29,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes29 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n27 {
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
    pub additional_pages: Vec<AdditionalPage27>,
    pub images: Vec<Image27>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage27 {
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
pub struct Image27 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes30,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes30 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n28 {
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
    pub additional_pages: Vec<AdditionalPage28>,
    pub images: Vec<Image28>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage28 {
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
pub struct Image28 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes31,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes31 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n29 {
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
    pub additional_pages: Vec<AdditionalPage29>,
    pub images: Vec<Image29>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage29 {
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
pub struct Image29 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes32,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes32 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n30 {
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
    pub additional_pages: Vec<AdditionalPage30>,
    pub images: Vec<Image30>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage30 {
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
pub struct Image30 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes33,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes33 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n31 {
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
    pub additional_pages: Vec<AdditionalPage31>,
    pub images: Vec<Image31>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage31 {
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
pub struct Image31 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes34,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes34 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n32 {
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
    pub additional_pages: Vec<AdditionalPage32>,
    pub images: Vec<Image32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage32 {
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
pub struct Image32 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes35,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes35 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n33 {
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
    pub additional_pages: Vec<AdditionalPage33>,
    pub images: Vec<Image33>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage33 {
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
pub struct Image33 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes36,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes36 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n34 {
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
    pub additional_pages: Vec<AdditionalPage34>,
    pub images: Vec<Image34>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage34 {
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
pub struct Image34 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes37,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes37 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n35 {
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
    pub additional_pages: Vec<AdditionalPage35>,
    pub images: Vec<Image35>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage35 {
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
pub struct Image35 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes38,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes38 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n36 {
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
    pub additional_pages: Vec<AdditionalPage36>,
    pub images: Vec<Image36>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage36 {
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
pub struct Image36 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes39,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes39 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n37 {
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
    pub additional_pages: Vec<AdditionalPage37>,
    pub images: Vec<Image37>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage37 {
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
pub struct Image37 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes40,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes40 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n38 {
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
    pub additional_pages: Vec<AdditionalPage38>,
    pub images: Vec<Image38>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage38 {
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
pub struct Image38 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes41,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes41 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n39 {
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
    pub additional_pages: Vec<AdditionalPage39>,
    pub images: Vec<Image39>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage39 {
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
pub struct Image39 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes42,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes42 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n40 {
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
    pub additional_pages: Vec<AdditionalPage40>,
    pub images: Vec<Image40>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage40 {
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
pub struct Image40 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes43,
    pub alt_text: String,
    pub caption: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes43 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n41 {
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
    pub additional_pages: Vec<AdditionalPage41>,
    pub images: Vec<Image41>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage41 {
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
pub struct Image41 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes44,
    pub alt_text: String,
    pub caption: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes44 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n42 {
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
    pub additional_pages: Vec<AdditionalPage42>,
    pub images: Vec<Image42>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage42 {
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
pub struct Image42 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes45,
    pub alt_text: String,
    pub caption: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes45 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n43 {
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
    pub additional_pages: Vec<AdditionalPage43>,
    pub images: Vec<Image43>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage43 {
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
pub struct Image43 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes46,
    pub alt_text: String,
    pub caption: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes46 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n44 {
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
    pub additional_pages: Vec<AdditionalPage44>,
    pub images: Vec<Image44>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage44 {
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
pub struct Image44 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes47,
    pub alt_text: String,
    pub caption: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes47 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n45 {
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
    pub additional_pages: Vec<AdditionalPage45>,
    pub images: Vec<Image45>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage45 {
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
pub struct Image45 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes48,
    pub alt_text: String,
    pub caption: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes48 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n46 {
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
    pub additional_pages: Vec<AdditionalPage46>,
    pub images: Vec<Image46>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage46 {
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
pub struct Image46 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes49,
    pub alt_text: String,
    pub caption: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes49 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n47 {
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
    pub additional_pages: Vec<AdditionalPage47>,
    pub images: Vec<Image47>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage47 {
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
pub struct Image47 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes50,
    pub alt_text: String,
    pub caption: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes50 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n48 {
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
    pub additional_pages: Vec<AdditionalPage48>,
    pub images: Vec<Image48>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage48 {
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
pub struct Image48 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes51,
    pub alt_text: String,
    pub caption: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes51 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n49 {
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
    pub additional_pages: Vec<AdditionalPage49>,
    pub images: Vec<Image49>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage49 {
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
pub struct Image49 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes52,
    pub alt_text: String,
    pub caption: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes52 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n50 {
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
    pub additional_pages: Vec<AdditionalPage50>,
    pub images: Vec<Image50>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage50 {
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
pub struct Image50 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes53,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes53 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n51 {
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
    pub additional_pages: Vec<AdditionalPage51>,
    pub images: Vec<Image51>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage51 {
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
pub struct Image51 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes54,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes54 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n52 {
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
    pub additional_pages: Vec<AdditionalPage52>,
    pub images: Vec<Image52>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage52 {
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
pub struct Image52 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes55,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes55 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n53 {
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
    pub additional_pages: Vec<AdditionalPage53>,
    pub images: Vec<Image53>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage53 {
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
pub struct Image53 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes56,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes56 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n54 {
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
    pub additional_pages: Vec<AdditionalPage54>,
    pub images: Vec<Image54>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage54 {
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
pub struct Image54 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes57,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes57 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n55 {
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
    pub additional_pages: Vec<AdditionalPage55>,
    pub images: Vec<Image55>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage55 {
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
pub struct Image55 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes58,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes58 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n56 {
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
    pub additional_pages: Vec<AdditionalPage56>,
    pub images: Vec<Image56>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage56 {
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
pub struct Image56 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes59,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes59 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n57 {
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
    pub additional_pages: Vec<AdditionalPage57>,
    pub images: Vec<Image57>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage57 {
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
pub struct Image57 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes60,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes60 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n58 {
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
    pub additional_pages: Vec<AdditionalPage58>,
    pub images: Vec<Image58>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage58 {
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
pub struct Image58 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes61,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes61 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n59 {
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
    pub additional_pages: Vec<AdditionalPage59>,
    pub images: Vec<Image59>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage59 {
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
pub struct Image59 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes62,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes62 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n60 {
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
    pub additional_pages: Vec<AdditionalPage60>,
    pub images: Vec<Image60>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage60 {
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
pub struct Image60 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes63,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes63 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n61 {
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
    pub additional_pages: Vec<AdditionalPage61>,
    pub images: Vec<Image61>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage61 {
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
pub struct Image61 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes64,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes64 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n62 {
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
    pub additional_pages: Vec<AdditionalPage62>,
    pub images: Vec<Image62>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage62 {
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
pub struct Image62 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes65,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes65 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n63 {
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
    pub additional_pages: Vec<AdditionalPage63>,
    pub images: Vec<Image63>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage63 {
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
pub struct Image63 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes66,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes66 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n64 {
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
    pub additional_pages: Vec<AdditionalPage64>,
    pub images: Vec<Image64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage64 {
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
pub struct Image64 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes67,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes67 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n65 {
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
    pub additional_pages: Vec<AdditionalPage65>,
    pub images: Vec<Image65>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage65 {
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
pub struct Image65 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes68,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes68 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n66 {
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
    pub additional_pages: Vec<AdditionalPage66>,
    pub images: Vec<Image66>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPage66 {
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
pub struct Image66 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: Sizes69,
    pub alt_text: String,
    pub caption: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes69 {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}
