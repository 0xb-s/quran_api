// src/client.rs
use crate::error::QuranApiError;
use crate::models::{
    EditionType, EditionTypesResponse, EditionsResponse, Format, FormatsResponse, Language,
    LanguagesResponse, QuranAudioResponse, QuranResponse,
};
use crate::quran_api::QuranApi;

use reqwest::Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct QuranApiClient {
    pub base_url: String,
    pub client: Arc<Client>,
}

impl QuranApiClient {
    /// Creates a new instance of `QuranApiClient`.
    pub fn new(base_url: &str) -> Self {
        QuranApiClient {
            base_url: base_url.to_string(),
            client: Arc::new(Client::new()),
        }
    }

    /// Builds the complete API URL.
    pub fn build_url(&self, endpoint: &str) -> String {
        format!("{}/{}", self.base_url, endpoint)
    }
}

impl QuranApi for QuranApiClient {
    async fn get_editions(
        &self,
        format: Option<Format>,
        language: Option<Language>,
        edition_type: Option<EditionType>,
    ) -> Result<EditionsResponse, QuranApiError> {
        let mut endpoint = String::from("edition");
        let mut params = vec![];

        if let Some(fmt) = format {
            params.push(format!("format={}", fmt.to_string().to_lowercase()));
        }

        if let Some(lang) = language {
            params.push(format!("language={}", lang.to_string().to_lowercase()));
        }

        if let Some(edition_type) = edition_type {
            params.push(format!("type={}", edition_type.to_string().to_lowercase()));
        }

        if !params.is_empty() {
            endpoint = format!("{}?{}", endpoint, params.join("&"));
        }

        // Send the request
        let url = self.build_url(&endpoint);
        let response = self.client.get(&url).send().await?.error_for_status()?;

        // Deserialize the response
        let editions_response = response.json::<EditionsResponse>().await?;

        Ok(editions_response)
    }

    async fn get_languages(&self) -> Result<LanguagesResponse, QuranApiError> {
        let endpoint = "edition/language";
        let url = self.build_url(endpoint);
        let response = self.client.get(&url).send().await?.error_for_status()?;

        let languages_response = response.json::<LanguagesResponse>().await?;

        Ok(languages_response)
    }

    async fn get_editions_by_language(
        &self,
        language: Language,
    ) -> Result<EditionsResponse, QuranApiError> {
        let endpoint = format!("edition/language/{}", language.to_string().to_lowercase());
        let url = self.build_url(&endpoint);
        let response = self.client.get(&url).send().await?.error_for_status()?;

        let editions_response = response.json::<EditionsResponse>().await?;

        Ok(editions_response)
    }

    async fn get_edition_types(&self) -> Result<EditionTypesResponse, QuranApiError> {
        let endpoint = "edition/type";
        let url = self.build_url(endpoint);
        let response = self.client.get(&url).send().await?.error_for_status()?;

        let edition_types_response = response.json::<EditionTypesResponse>().await?;

        Ok(edition_types_response)
    }

    async fn get_editions_by_type(
        &self,
        edition_type: EditionType,
    ) -> Result<EditionsResponse, QuranApiError> {
        let endpoint = format!("edition/type/{}", edition_type.to_string().to_lowercase());
        let url = self.build_url(&endpoint);
        let response = self.client.get(&url).send().await?.error_for_status()?;

        let editions_response = response.json::<EditionsResponse>().await?;

        Ok(editions_response)
    }

    async fn get_formats(&self) -> Result<FormatsResponse, QuranApiError> {
        let endpoint = "edition/format";
        let url = self.build_url(endpoint);
        let response = self.client.get(&url).send().await?.error_for_status()?;

        let formats_response = response.json::<FormatsResponse>().await?;

        Ok(formats_response)
    }

    async fn get_editions_by_format(
        &self,
        format: Format,
    ) -> Result<EditionsResponse, QuranApiError> {
        let endpoint = format!("edition/format/{}", format.to_string().to_lowercase());
        let url = self.build_url(&endpoint);
        let response = self.client.get(&url).send().await?.error_for_status()?;

        let editions_response = response.json::<EditionsResponse>().await?;

        Ok(editions_response)
    }

    async fn get_quran_text(&self, edition: &str) -> Result<QuranResponse, QuranApiError> {
        let endpoint = format!("quran/{}", edition);
        let url = self.build_url(&endpoint);
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(QuranApiError::Http)?;

        let quran_response = response.json::<QuranResponse>().await.unwrap();

        Ok(quran_response)
    }

    async fn get_quran_audio(&self, edition: &str) -> Result<QuranAudioResponse, QuranApiError> {
        let endpoint = format!("quran/{}", edition);
        let url = self.build_url(&endpoint);

        // Handle the HTTP request error first
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(QuranApiError::Http)?;

        let quran_audio_response = response.json::<QuranAudioResponse>().await.unwrap();

        Ok(quran_audio_response)
    }
}
