use quick_xml::Error;
use thiserror::Error;
pub mod cleaner;
pub mod epub;

pub trait Processor {
    fn process(&self, text: &str) -> Result<String, ProcessError>;
}

#[derive(Debug, Error)]
pub enum ProcessError {
    #[error("Failed to process epub text")]
    EpubError {
        #[from]
        source: Error,
    },
}
