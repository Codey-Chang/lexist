use crate::{processor::Processor, source::Source};


struct Extractor {
    source: Box<dyn Source>,
    processor: Vec<Box<dyn Processor>>,
}
pub mod error {

    use crate::source::SourceError;
    use crate::processor::ProcessError;
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum ExtractorError {
        #[error("Failed to extract text from source")]
        SourceError(#[from]SourceError),
        #[error("Failed to process text")]
        ProcessError(#[from]ProcessError),
    }
}