use super::{ProcessError, Processor};
pub struct Cleaner;

impl Cleaner {
    pub fn new() -> Self {
        Cleaner
    }
}

impl Processor for Cleaner {
    fn process(&self, text: &str) -> Result<String, ProcessError> {
        let cleaned_text = text.chars().filter(|c| !c.is_whitespace()).collect();
        Ok(cleaned_text)
    }
}