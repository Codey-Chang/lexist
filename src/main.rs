use std::{path::{Path, PathBuf}, rc::Rc};

use lexist::{extractor, processor::epub::PTagTextExtractor, source::epub::EpubSource};
use sudachi::{analysis::{stateful_tokenizer::StatefulTokenizer, stateless_tokenizer::StatelessTokenizer}, config::{Config, ConfigBuilder}, dic::dictionary::JapaneseDictionary, prelude::{Mode, MorphemeList}, sentence_splitter::{SentenceSplitter, SplitSentences}};
fn main() {

    let epub_source = EpubSource::new("resources/epub/1.epub");

    let mut extractor = extractor::Extractor::new(epub_source);

    let ptagprocessor = PTagTextExtractor::new();

    extractor.add_processor(ptagprocessor);

    let text = extractor.extract().unwrap();

    let splitter = SentenceSplitter::new();
    let sentences: Vec<&str> = splitter.split(&text).map(|(_, s)| {
        s
    }).collect();

    sentences.iter().for_each(|s| {
        println!("{}", s);
    });

    let config_path = Path::new("resources/sudachi.json");

    let sudachi_config = ConfigBuilder::from_file(config_path).expect("Failed to load config file").build();

    let rc_jp_dict = Rc::new(JapaneseDictionary::from_cfg(&sudachi_config).expect("Failed to load dictionary"));

    let mut sf_tok =StatefulTokenizer::new(rc_jp_dict.clone(), Mode::A);

    let mut tokens = MorphemeList::empty(rc_jp_dict.clone());

    sentences.iter().for_each(|s| {
        sf_tok.reset().push_str(s);
        sf_tok.do_tokenize().expect("tokenize failed");
        tokens.collect_results(&mut sf_tok).expect("Failed to collect results");
        
        tokens.iter().for_each(|t| {
            println!("{:?}", t.part_of_speech());
        });
    });

    // let mut mdict = MDictBuilder::new("resources/mdx/Shogakukanjcv3.mdx").build().unwrap();

    // for token in tokens.iter() {
    //     let res = mdict.lookup(token.normalized_form()).unwrap();
    //     if let Some(res) = res {
    //         println!("{:?}", res.definition);
    //     }
    // }
}