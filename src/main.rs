use std::{collections::HashSet, fs::read_to_string};

use genanki_rs::{Deck, Field, Model, Note, Template};
use lexist::{extractor, processor::epub::PTagTextExtractor, source::epub::EpubSource, tokenizer};
use mdict::MDictBuilder;
use sudachi::{
    prelude::Mode,
    sentence_splitter::{SentenceSplitter, SplitSentences},
};
fn main() {
    let epub_source = EpubSource::new("resources/epub/1.epub");

    let mut extractor = extractor::Extractor::new(epub_source);

    let ptagprocessor = PTagTextExtractor::new();

    extractor.add_processor(ptagprocessor);

    let text = extractor.extract().unwrap();

    let sentences = split_sentences(&text);

    let word_set = get_word_set(sentences);

    let mut mdict = MDictBuilder::new("resources/mdx/Shogakukanjcv3.mdx")
        .build()
        .unwrap();
    let mut fields = Vec::new();
    word_set.iter().take(20).for_each(|w| {
        match mdict.lookup(w).expect("Failed to lookup") {
            Some(def) => {
                println!("{}", def.definition);
                if !def.definition.starts_with("@") {
                    fields.push((w.clone(), def.definition));
                }
            }
            None => {
                println!("{} not found", w);
            }
        }
    });

    gen_anki_with_css(fields);
}

fn split_sentences(text: &str) -> Vec<&str> {
    let splitter: SentenceSplitter<'_> = SentenceSplitter::new();
    splitter.split(text).map(|(_, s)| s).collect()
}

fn get_word_set(sentences: Vec<&str>) -> HashSet<String> {
    let mut tok = tokenizer::StatefulTokenizer::new(Mode::A);
    let mut word_set = HashSet::new();
    sentences.iter().for_each(|s| {
        let tokens = tok.tokenize(s);
        tokens.iter().for_each(|t| {
            if word_set.contains(t.dictionary_form()) {
                return;
            }
            let pos = t.part_of_speech().get(0).expect("Failed to get pos");
            if pos != "空白" && pos != "補助記号" {
                word_set.insert(t.dictionary_form().to_string());
                // println!("{:?}", t.dictionary_form());
            }
        });
    });
    word_set
}

fn gen_anki_with_css(fields: Vec<(String, String)>) {
    let css = read_to_string("resources/css/Shogakukanjcv3.css").expect("Failed to read css");
    let model = Model::new(
        1607392319,
        "Jp Model",
        vec![Field::new("Front"), Field::new("Back")],
        vec![Template::new("Card 1").qfmt("{{Front}}").afmt("{{Back}}")],
    )
    .css(css);

    let mut deck = Deck::new(2059400110, "Japanese Deck", "A deck for learning Japanese");

    fields.iter().for_each(|(w, d)| {
        let note = Note::new(model.clone(), vec![w, d]).expect("Failed to create note");
        deck.add_note(note);
    });

    deck.write_to_file("output.apkg").expect("Failed to write to apkg file");
}
