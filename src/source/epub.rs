use std::path::{Path, PathBuf};

use epub::doc::EpubDoc;

use super::DataSource;

struct EpubSource {
    path: PathBuf,
}

impl EpubSource {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        EpubSource {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl DataSource for EpubSource {
    fn fetech_text(&self) -> Result<String, super::DataSourceError> {
        let mut doc = EpubDoc::new(&self.path)?;
        let spine_ids = doc.spine.clone();
        let mut ret = "".to_string();
        
        spine_ids.iter().for_each(|id| {
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
        let text = epub_source.fetech_text().unwrap();
        println!("{}", text);
    }
}