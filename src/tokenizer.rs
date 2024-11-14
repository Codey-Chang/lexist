use std::{path::Path, sync::Arc};

use lazy_static::lazy_static;
use sudachi::analysis::stateful_tokenizer::StatefulTokenizer as SfTok;
use sudachi::analysis::stateless_tokenizer::StatelessTokenizer as SlTok;
use sudachi::{
    analysis::Tokenize,
    config::{Config, ConfigBuilder},
    dic::dictionary::JapaneseDictionary,
    prelude::{Mode, MorphemeList},
};

lazy_static! {
    static ref CONFIG: Config = {
        let config_path = Path::new("resources/sudachi.json");
        ConfigBuilder::from_file(config_path)
            .expect("Failed to load config file")
            .build()
    };
    static ref JpDict: Arc<JapaneseDictionary> =
        Arc::new(JapaneseDictionary::from_cfg(&CONFIG).expect("Failed to load dictionary"));
}

pub struct StatefulTokenizer {
    tok: SfTok<Arc<JapaneseDictionary>>,
    result: MorphemeList<Arc<JapaneseDictionary>>,
}

impl StatefulTokenizer {
    pub fn new(mode: Mode) -> Self {
        StatefulTokenizer {
            tok: SfTok::new(JpDict.clone(), mode),
            result: MorphemeList::empty(JpDict.clone()),
        }
    }

    pub fn tokenize(&mut self, text: &str) -> &MorphemeList<Arc<JapaneseDictionary>> {
        self.tok.reset().push_str(text);
        self.tok.do_tokenize().expect("Failed to tokenize");
        self.result
            .collect_results(&mut self.tok)
            .expect("Failed to collect results");
        &self.result
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.tok.set_mode(mode);
    }
}

pub struct StatelessTokenizer {
    tok: SlTok<Arc<JapaneseDictionary>>,
    result: MorphemeList<Arc<JapaneseDictionary>>,
    mode: Mode,
}

impl StatelessTokenizer {
    pub fn new(mode: Mode) -> Self {
        StatelessTokenizer {
            tok: SlTok::new(JpDict.clone()),
            result: MorphemeList::empty(JpDict.clone()),
            mode: mode,
        }
    }

    pub fn tokenize(&mut self, text: &str) -> &MorphemeList<Arc<JapaneseDictionary>> {
        self.result = self
            .tok
            .tokenize(text, self.mode, false)
            .expect("Failed to tokenize");
        &self.result
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }
}
