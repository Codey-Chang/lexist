use lexist::{extractor, processor::epub::PTagTextExtractor, source::epub::EpubSource, tokenizer::{self, SlTokenizer}};
use mdict::MDictBuilder;
use sudachi::prelude::Mode;
fn main() {

    let epub_source = EpubSource::new("resources/epub/1.epub");

    let mut extractor = extractor::Extractor::new(epub_source);

    extractor.add_processor(PTagTextExtractor::new());

    let text = extractor.extract().unwrap();

    println!("{}", text);

    let mut sltokenizer = SlTokenizer::new();

    let tokens = sltokenizer.tokenize(&text, Mode::A);

    for token in tokens.iter() {
        println!("{}", token.normalized_form());
    }

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