use std::fs;

use lexist::tokenizer::SfTokenizer;
use sudachi::prelude::Mode;

use epub::doc::EpubDoc;

fn main() {

    let doc= EpubDoc::new("resources/epub/epub3-spec.epub").unwrap();

    doc.get_num_pages();

    let mut tokenizer = SfTokenizer::new_built(Mode::C);

}
