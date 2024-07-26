use thiserror::Error;

#[derive(Error, Debug)]
pub enum BirdDogError {
    #[error("HTTP request error: {0}")]
    HTTPRequestError(#[from] reqwest::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Unknown error")]
    Unknown,
    #[error("Invalid response from the BirdDog device: {0}")]
    RequestError(String),
}
