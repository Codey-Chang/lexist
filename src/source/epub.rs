use std::path::{Path, PathBuf};

use epub::doc::EpubDoc;

use super::Source;

pub struct EpubSource {
    path: PathBuf,
}

impl EpubSource {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        EpubSource {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl Source for EpubSource {
    fn fetch(&self) -> Result<String, super::SourceError> {
        let mut doc = EpubDoc::new(&self.path)?;
        let spine_ids = doc.spine.clone();
        let mut ret = "".to_string();


        let rex = regex::Regex::new(r"^x_p-[0-9]+").unwrap();
        spine_ids.iter()
        .filter(|id| {
            rex.is_match(id)
        })
        .for_each(|id| {
            if let Some(s) = doc.get_resource_str(id) {
                ret.push_str(&s.0);
            }
        });
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epub_source() {
        let epub_source = EpubSource::new("resources/epub/1.epub");
        let text = epub_source.fetch().unwrap();
        println!("{}", text);
    }

    #[test]
    fn test_epub_crate() {
        let mut doc = EpubDoc::new("resources/epub/1.epub").unwrap();
        let rex = regex::Regex::new(r"^x_p-[0-9]+").unwrap();
        doc.resources.iter()
        .filter(|(id, r)| {
            rex.is_match(id)
        })
        .for_each(|(id, r)| {
            if r.0.to_str().unwrap().contains("Text") {
                println!("id: {}, path: {:?}, minetype: {}", id, r.0, r.1);
            }
        });
    }
}