
use ::epub::doc::DocError;
use thiserror::Error;

pub mod epub;
pub trait Source {
    fn fetch(&self) -> Result<String, SourceError>;
}

#[derive(Debug, Error)]
pub enum SourceError {
    #[error("Failed to fetch text from epub source")]
    EpubError{
        #[from]
        source: DocError
    },
}