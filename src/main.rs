
use lexist::common::SlTokenizer;
use sudachi::prelude::Mode;
fn main() {

    let data = "太郎は次郎が持っている本を花子に渡した。\n";
    
    let tokenizer = SlTokenizer::new();
    
    tokenizer.tokenize(data, Mode::C).iter().for_each(|m| {
        println!("{:?}", m);
    });

}
