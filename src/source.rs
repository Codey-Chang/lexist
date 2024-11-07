
use ::epub::doc::DocError;
use thiserror::Error;

mod epub;
pub trait Source {
    fn fetech_text(&self) -> Result<String, SourceError>;
}

#[derive(Debug, Error)]
pub enum SourceError {
    #[error("Failed to fetch text from epub source")]
    EpubError{
        #[from]
        source: DocError
    },
}