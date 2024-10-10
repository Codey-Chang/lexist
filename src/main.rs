
use lexist::common::SlTokenizer;
use sudachi::prelude::Mode;
fn main() {

    let data = "今日はいい天気ですね。";
    
    let tokenizer = SlTokenizer::new();
    
    tokenizer.tokenize(data, Mode::C).iter().for_each(|m| {
        println!("{:?}", m);
    });

}
