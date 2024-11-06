
use ::epub::doc::DocError;
use thiserror::Error;

mod epub;
trait DataSource {
    fn fetech_text(&self) -> Result<String, DataSourceError>;
}

#[derive(Debug, Error)]
pub enum DataSourceError {
    #[error("Failed to fetch text from epub source")]
    EpubError{
        #[from]
        source: DocError
    },
}