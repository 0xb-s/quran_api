// src/models.rs
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub enum Format {
    Text,
    Audio,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Format::Text => write!(f, "text"),
            Format::Audio => write!(f, "audio"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Language {
    En, // English
    Fr, // French
    Ar, // Arabic
        //TODO:  Add other lang
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::En => write!(f, "en"),
            Language::Fr => write!(f, "fr"),
            Language::Ar => write!(f, "ar"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EditionType {
    VerseByVerse,
    Translation,
    Tafsir,
    Quran,
    Transliteration,
    Other(String),
}

impl fmt::Display for EditionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EditionType::VerseByVerse => write!(f, "versebyverse"),
            EditionType::Translation => write!(f, "translation"),
            EditionType::Tafsir => write!(f, "tafsir"),
            EditionType::Quran => write!(f, "quran"),
            EditionType::Transliteration => write!(f, "transliteration"),
            EditionType::Other(typ) => write!(f, "{}", typ),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Edition {
    pub identifier: String,
    pub language: String,
    pub name: String,
    #[serde(rename = "englishName")]
    pub english_name: String,
    pub format: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub direction: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditionsResponse {
    pub code: u32,
    pub status: String,
    pub data: Vec<Edition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguagesResponse {
    pub code: u32,
    pub status: String,
    pub data: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditionTypesResponse {
    pub code: u32,
    pub status: String,
    pub data: Vec<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct FormatsResponse {
    pub code: u32,
    pub status: String,
    pub data: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ayah {
    pub number: u32,
    pub text: String,
    pub numberInSurah: u32,
    pub juz: u32,
    pub manzil: u32,
    pub page: u32,
    pub ruku: u32,
    pub hizbQuarter: u32,

    pub sajda: SajdaType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SajdaType {
    Bool(bool),
    Object(SajdaDetail),
}

#[derive(Debug, Serialize, Deserialize)]

pub struct SajdaDetail {
    pub recommended: bool,
    pub obligatory: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Surah {
    pub number: u32,
    pub name: String,
    #[serde(rename = "englishName")]
    pub english_name: String,
    #[serde(rename = "englishNameTranslation")]
    pub english_name_translation: String,
    #[serde(rename = "revelationType")]
    pub revelation_type: String,
    pub ayahs: Vec<Ayah>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuranResponse {
    pub code: u32,
    pub status: String,
    pub data: QuranData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuranData {
    pub surahs: Vec<Surah>,
    pub edition: QuranEdition,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QuranEdition {
    pub identifier: String,
    pub language: String,
    pub name: String,
    #[serde(rename = "englishName")]
    pub english_name: String,
    pub format: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuranTextData {
    pub surahs: Vec<Surah>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioAyah {
    pub number: u32,
    pub text: String,
    pub numberInSurah: u32,
    pub juz: u32,
    pub manzil: u32,
    pub page: u32,
    pub ruku: u32,
    pub hizbQuarter: u32,

  
    pub audio: String,
    pub audioSecondary: Vec<String>,

    
    pub sajda: SajdaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioSurah {
    pub number: u32,
    pub name: String,
    #[serde(rename = "englishName")]
    pub english_name: String,
    #[serde(rename = "englishNameTranslation")]
    pub english_name_translation: String,
    pub revelationType: String,
    pub ayahs: Vec<AudioAyah>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QuranAudioResponse {
    pub code: u32,
    pub status: String,
    pub data: QuranAudioData,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QuranAudioData {
    pub surahs: Vec<AudioSurah>,
    pub edition: QuranEdition,
}
