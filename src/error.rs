
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use std::fmt;


#[derive(Debug)]
pub enum QuranApiError {
    Http(ReqwestError),          
    Deserialization(SerdeError), 
    Other(String),               
}

impl fmt::Display for QuranApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuranApiError::Http(err) => write!(f, "HTTP error: {}", err),
            QuranApiError::Deserialization(err) => write!(f, "Deserialization error: {}", err),
            QuranApiError::Other(err) => write!(f, "Other error: {}", err),
        }
    }
}

impl std::error::Error for QuranApiError {}

impl From<ReqwestError> for QuranApiError {
    fn from(err: ReqwestError) -> Self {
        QuranApiError::Http(err)
    }
}

impl From<SerdeError> for QuranApiError {
    fn from(err: SerdeError) -> Self {
        QuranApiError::Deserialization(err)
    }
}
