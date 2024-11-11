use crate::{processor::Processor, source::Source};

// TODO: Implement the enum dipatcher for Source and Processor for improving the performance
pub struct Extractor<'a> {
    source: Box<dyn Source + 'a>,
    processors: Vec<Box<dyn Processor + 'a>>,
}

impl<'a> Extractor<'a> {
    pub fn new(source: impl Source + 'a) -> Self {
        Extractor {
            source: Box::new(source),
            processors: Vec::new(),
        }
    }

    pub fn extract(&self) -> Result<String, error::ExtractorError> {
        let text = self.source.fetch()?;
        let mut processed_text = text;
        for processor in &self.processors {
            processed_text = processor.process(&processed_text)?;
        }
        Ok(processed_text)
    }

    pub fn add_processor(&mut self, processor: impl Processor + 'a) {
        self.processors.push(Box::new(processor));
    }
    
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