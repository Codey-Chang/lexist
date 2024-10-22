use std::{
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

use lazy_static::lazy_static;
use sudachi::{
    analysis::{
        stateful_tokenizer::StatefulTokenizer, stateless_tokenizer::StatelessTokenizer, Tokenize,
    },
    config::Config,
    dic::{
        dictionary::JapaneseDictionary, grammar::Grammar, header::Header, lexicon::Lexicon,
        subset::InfoSubset,
    },
    error::SudachiResult,
    prelude::{Mode, MorphemeList},
};

pub fn dictionary_bytes_from_path<P: AsRef<Path>>(dictionary_path: P) -> SudachiResult<Vec<u8>> {
    let dictionary_path = dictionary_path.as_ref();
    let dictionary_stat = fs::metadata(&dictionary_path)?;
    let mut dictionary_file = File::open(dictionary_path)?;
    let mut dictionary_bytes = Vec::with_capacity(dictionary_stat.len() as usize);
    dictionary_file.read_to_end(&mut dictionary_bytes)?;

    Ok(dictionary_bytes)
}

lazy_static! {
    pub static ref CONFIG: Config = {
        let config_path = "resources/sudachi.json";
        let conf = Config::new(Some(PathBuf::from(config_path)), None, None)
            .expect("Failed to read config file");
        conf
    };
    static ref DICTIONARY_BYTES: Vec<u8> = {
        let dictionary_path = CONFIG.resolved_system_dict().expect("system dict failure");

        let dictionary_bytes = dictionary_bytes_from_path(dictionary_path)
            .expect("Failed to read dictionary from path");
        dictionary_bytes
    };
    pub static ref HEADER: Header =
        Header::parse(&DICTIONARY_BYTES).expect("Failed to create Header for tests");
    pub static ref GRAMMAR: Grammar<'static> =
        Grammar::parse(&DICTIONARY_BYTES, Header::STORAGE_SIZE)
            .expect("Failed to read grammar for tests");
    pub static ref LEXICON: Lexicon<'static> = {
        let offset = Header::STORAGE_SIZE + GRAMMAR.storage_size;
        let mut lex = Lexicon::parse(&DICTIONARY_BYTES, offset, HEADER.has_synonym_group_ids())
            .expect("Failed to read lexicon for tests");
        lex.set_dic_id(0);
        lex
    };
}

pub struct SlTokenizer {
    tok: StatelessTokenizer<Arc<JapaneseDictionary>>,
}

impl SlTokenizer {
    pub fn new() -> SlTokenizer {
        let dict = JapaneseDictionary::from_cfg(&CONFIG).expect("Failed to create dictionary");
        let tok = StatelessTokenizer::new(Arc::new(dict));
        SlTokenizer { tok }
    }

    pub fn tokenize<'a>(
        &'a self,
        data: &'a str,
        mode: Mode,
    ) -> MorphemeList<Arc<JapaneseDictionary>> {
        let ret = self.tok.tokenize(&data, mode, false);
        ret.expect("tokenization failed")
    }

    pub fn dict(&self) -> &JapaneseDictionary {
        self.tok.as_dict()
    }
}

pub struct SfTokenizer {
    tok: StatefulTokenizer<Rc<JapaneseDictionary>>,
    result: MorphemeList<Rc<JapaneseDictionary>>,
}

impl SfTokenizer {
    pub fn new_built(mode: Mode) -> SfTokenizer {
        let dic = Rc::new(JapaneseDictionary::from_cfg(&CONFIG).expect("works"));
        Self {
            tok: StatefulTokenizer::new(dic.clone(), mode),
            result: MorphemeList::empty(dic),
        }
    }

    pub fn tokenize(&mut self, data: &str) -> &MorphemeList<Rc<JapaneseDictionary>> {
        self.tok.reset().push_str(data);
        self.tok.do_tokenize().expect("tokenization failed");
        self.result
            .collect_results(&mut self.tok)
            .expect("collection failed");
        &self.result
    }

    pub fn dict(&self) -> &JapaneseDictionary {
        self.tok.dict()
    }

    pub fn set_mode(&mut self, mode: Mode) -> Mode {
        self.tok.set_mode(mode)
    }

    pub fn entries(&mut self, query: impl AsRef<str>) -> &MorphemeList<Rc<JapaneseDictionary>> {
        self.result.clear();
        self.result
            .lookup(query.as_ref(), InfoSubset::all())
            .expect("should not fail");
        &self.result
    }
}
