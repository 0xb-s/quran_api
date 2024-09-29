pub mod client;

pub mod error;
pub mod models;
pub mod quran_api;
pub use client::QuranApiClient;

pub use models::{
    EditionType, EditionTypesResponse, EditionsResponse, Format, FormatsResponse, Language,
    LanguagesResponse, QuranAudioResponse,
};
pub use quran_api::QuranApi;
