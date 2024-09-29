
use crate::error::QuranApiError;
use crate::models::{
    EditionType, EditionTypesResponse, EditionsResponse, Format, FormatsResponse, Language,
    LanguagesResponse, QuranAudioResponse, QuranResponse,
};

pub trait QuranApi {
    ///get all available editions with optional filtering.
    async fn get_editions(
        &self,
        format: Option<Format>,
        language: Option<Language>,
        edition_type: Option<EditionType>,
    ) -> Result<EditionsResponse, QuranApiError>;

    /// get all available languages.
    async fn get_languages(&self) -> Result<LanguagesResponse, QuranApiError>;

    /// Function to get editions for a specific language.
    async fn get_editions_by_language(
        &self,
        language: Language,
    ) -> Result<EditionsResponse, QuranApiError>;

    /// get all available types of editions.
    async fn get_edition_types(&self) -> Result<EditionTypesResponse, QuranApiError>;

    /// get editions for a specific type.
    async fn get_editions_by_type(
        &self,
        edition_type: EditionType,
    ) -> Result<EditionsResponse, QuranApiError>;

    ///get all available formats.
    async fn get_formats(&self) -> Result<FormatsResponse, QuranApiError>;

    ///  get editions for a specific format.
    async fn get_editions_by_format(
        &self,
        format: Format,
    ) -> Result<EditionsResponse, QuranApiError>;

    ///  get a complete Quran edition (text).
    async fn get_quran_text(&self, edition: &str) -> Result<QuranResponse, QuranApiError>;

    /// get a complete Quran audio edition.
    async fn get_quran_audio(&self, edition: &str) -> Result<QuranAudioResponse, QuranApiError>;
}
