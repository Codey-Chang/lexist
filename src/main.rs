use std::fs;

use lexist::common::SfTokenizer;
use sudachi::prelude::Mode;

fn main() {

    let data = fs::read_to_string("resources/input.txt").unwrap();

    let mut tokenizer = SfTokenizer::new_built(Mode::C);

    tokenizer.tokenize(&data).iter().for_each(|m| {
        println!("{:?}", m);
    });
}
