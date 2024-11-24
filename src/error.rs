#![allow(unused)]
use std::{error::Error as StdError, fmt};



pub type Result<T> = std::result::Result<T, Error>;
pub (crate) type BoxError = Box<dyn StdError + Send + Sync>;

pub (crate) struct Error {
    inner: Box<Inner>,
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner.source.as_ref().map(|e| &**e as _)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.inner.kind {
            Kind::Source => f.write_str("source error"),
            Kind::Process => f.write_str("process error"),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = f.debug_struct("lexist::Error");
        builder.field("kind", &self.inner.kind);
        if let Some(source) = &self.inner.source {
            builder.field("source", source);
        }

        builder.finish()
    }
}

struct Inner {
    kind: Kind,
    source: Option<BoxError>,    
}

#[derive(Debug)]
enum Kind {
    Source,
    Process,
}