use sudachi::sentence_splitter::{SentenceSplitter, SplitSentences};
fn main() {
    let input = "えたいの知れない不吉な塊が私の心を始終圧えつけていた。";
    
    let splitter = SentenceSplitter::new();

    let mut iter = splitter.split(input);

    
}
