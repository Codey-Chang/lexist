use lexist::{extractor, processor::{cleaner::Cleaner, epub::PTagTextExtractor}, source::epub::EpubSource, tokenizer::{self, SlTokenizer}};
use mdict::MDictBuilder;
use sudachi::prelude::Mode;
fn main() {

    let epub_source = EpubSource::new("resources/epub/1.epub");

    let mut extractor = extractor::Extractor::new(epub_source);

    let ptagprocessor = PTagTextExtractor::new();
    let cleaner = Cleaner::new();

    extractor.add_processor(ptagprocessor);
    extractor.add_processor(cleaner);

    let text = extractor.extract().unwrap();

    println!("{}", text);

    // let mut sftokenizer = tokenizer::SfTokenizer::new_built(Mode::A);

    // let tokens = sftokenizer.tokenize(&text);

    // let mut mdict = MDictBuilder::new("resources/mdx/Shogakukanjcv3.mdx").build().unwrap();

    // for token in tokens.iter() {
    //     let res = mdict.lookup(token.normalized_form()).unwrap();
    //     if let Some(res) = res {
    //         println!("{:?}", res.definition);
    //     }
    // }
}