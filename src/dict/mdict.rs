

use std::{borrow::Cow, path::{Path, PathBuf}};

use mdict::KeyMaker;

use super::{Dict, DictError};

struct KeyMakerImpl;

impl KeyMakerImpl {
    fn new() -> Self {
        KeyMakerImpl
    }
    
}

impl KeyMaker for KeyMakerImpl {
    fn make(&self, key: &Cow<str>, _resource: bool) -> String {
        key.to_ascii_lowercase()
    }
    
}

pub struct MDict {
    dict: mdict::MDict<KeyMakerImpl>,
}

impl MDict {
    pub fn new(path: impl AsRef<Path>) -> Self {
        MDict {
            dict: mdict::MDictBuilder::new(path.as_ref()).build_with_key_maker(KeyMakerImpl::new()).expect("Failed to build MDict"),
        }
    }
    
}

impl Dict for MDict {
    fn lookup(&mut self, word: &str) -> Result<Option<String>, DictError> {
        Ok(self.dict.lookup(word)?.map(|def| def.definition))
    }
}