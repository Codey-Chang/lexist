pub mod mdict;

use thiserror::Error;

pub trait Dict {
    fn lookup(&mut self, word: &str) -> Result<Option<String>, DictError>;
}

#[derive(Debug, Error)]
pub enum DictError {
    #[error("Failed to lookup word in dictionary")]
    MdictError {
        #[from]
        source: ::mdict::Error,
    },
    #[error("other error")]
    Other,
}