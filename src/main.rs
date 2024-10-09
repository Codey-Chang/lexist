use std::{fs::{self, File}, io::Read, path::{Path, PathBuf}, rc::Rc, sync::Arc};

use lazy_static::lazy_static;
use sudachi::{analysis::{stateful_tokenizer::StatefulTokenizer, stateless_tokenizer::StatelessTokenizer, Tokenize}, config::{Config, ConfigBuilder}, dic::{build::DictBuilder, dictionary::JapaneseDictionary, grammar::Grammar, header::Header, lexicon::Lexicon, storage::{Storage, SudachiDicData}, DictionaryLoader}, error::SudachiResult, prelude::{Mode, MorphemeList}};

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
        println!("{:?}", conf);
        conf
    };

    static ref DICTIONARY_BYTES: Vec<u8> = {
        let dictionary_path = CONFIG
            .resolved_system_dict()
            .expect("system dict failure");

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
    tok: StatelessTokenizer<Arc<JapaneseDictionary>>
}

impl SlTokenizer{
    pub fn new() -> SlTokenizer {
        let dict = JapaneseDictionary::from_cfg(&CONFIG).expect("Failed to create dictionary");
        let tok = StatelessTokenizer::new(Arc::new(dict));
        SlTokenizer { tok }
    }
    
    pub fn tokenize<'a>(&'a self, data: &'a str, mode: Mode) -> MorphemeList<Arc<JapaneseDictionary>> {
        let ret = self.tok.tokenize(&data, mode, false);
        ret.expect("tokenization failed")
    }

    pub fn dict(&self) -> &JapaneseDictionary {
        self.tok.as_dict()
    }
}

pub struct TokenizerBuilder<'a> {
    pub conn: Option<&'a [u8]>,
    pub system: &'a [u8],
    pub user: Vec<&'a [u8]>,
    pub mode: Mode,
    pub debug: bool,
    pub config: Option<&'a [u8]>,
}

impl<'a> TokenizerBuilder<'a> {
    pub fn user(mut self, data: &'a [u8]) -> Self {
        self.user.push(data);
        self
    }

    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    pub fn config(mut self, data: &'static [u8]) -> Self {
        self.config = Some(data);
        self
    }

    pub fn build(self) -> SfTokenizer {
        let mut sys = DictBuilder::new_system();
        sys.read_conn(
            self.conn
                .unwrap_or(include_bytes!("../resources/matrix_10x10.def")),
        )
        .unwrap();
        sys.read_lexicon(self.system).unwrap();
        sys.resolve().unwrap();
        let mut sys_bytes = Vec::new();
        sys.compile(&mut sys_bytes).unwrap();

        let mut data = SudachiDicData::new(Storage::Owned(sys_bytes));

        if !self.user.is_empty() {
            let dic =
                DictionaryLoader::read_system_dictionary(unsafe { data.system_static_slice() })
                    .unwrap()
                    .to_loaded()
                    .unwrap();

            for u in self.user {
                let mut ubld = DictBuilder::new_user(&dic);
                ubld.read_lexicon(u).unwrap();
                ubld.resolve().unwrap();
                let mut user_bytes = Vec::new();
                ubld.compile(&mut user_bytes).unwrap();
                data.add_user(Storage::Owned(user_bytes));
            }
        }

        let config = match self.config {
            None => CONFIG.clone(),
            Some(data) => ConfigBuilder::from_bytes(data).unwrap().build(),
        };

        let dic = JapaneseDictionary::from_cfg_storage(&config, data).unwrap();
        let rcdic = Rc::new(dic);

        SfTokenizer {
            tok: StatefulTokenizer::create(rcdic.clone(), self.debug, self.mode),
            result: MorphemeList::empty(rcdic),
        }
    }
}

pub struct SfTokenizer {
    tok: StatefulTokenizer<Rc<JapaneseDictionary>>,
    result: MorphemeList<Rc<JapaneseDictionary>>,
}

fn main() {

    let data = "今日はいい天気ですね。";
    
    let tokenizer = SlTokenizer::new();
    
    tokenizer.tokenize(data, Mode::C).iter().for_each(|m| {
        println!("{:?}", m);
    });

}
