use lexist::{extractor, processor::epub::PTagTextExtractor, source::epub::EpubSource};
use sudachi::{analysis::stateful_tokenizer::StatefulTokenizer, sentence_splitter::{SentenceSplitter, SplitSentences}};
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

    // let mut mdict = MDictBuilder::new("resources/mdx/Shogakukanjcv3.mdx").build().unwrap();

    // for token in tokens.iter() {
    //     let res = mdict.lookup(token.normalized_form()).unwrap();
    //     if let Some(res) = res {
    //         println!("{:?}", res.definition);
    //     }
    // }
}