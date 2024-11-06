use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::borrow::Cow;

#[derive(Debug)]
struct WordEntry {
    word: String,
    pinyin: String,
    meanings: Vec<Meaning>,
}

#[derive(Debug)]
struct Meaning {
    number: String,
    context: String,
    definition: String,
    examples: Vec<Example>,
}

#[derive(Debug)]
struct Example {
    japanese: String,
    chinese: String,
}

fn parse_word_xml(xml: &str) -> Result<WordEntry, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_str(xml);

    let mut word_entry = WordEntry {
        word: String::new(),
        pinyin: String::new(),
        meanings: Vec::new(),
    };

    let mut current_meaning: Option<Meaning> = None;
    let mut buf = Vec::new();
    let mut in_example = false;
    let mut current_example: Option<Example> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name().as_ref() {
                    b"h3" => {
                        // 等待提取单词
                    },
                    b"p" => {
                        let attrs = e.attributes().map(|a| a.unwrap()).collect::<Vec<_>>();
                        if let Some(attr) = attrs.iter().find(|a| a.key.as_ref() == b"data-orgtag") {
                            match attr.value.as_ref() {
                                b"meaning" => {
                                    if current_meaning.is_some() {
                                        if let Some(meaning) = current_meaning.take() {
                                            word_entry.meanings.push(meaning);
                                        }
                                    }
                                    current_meaning = Some(Meaning {
                                        number: String::new(),
                                        context: String::new(),
                                        definition: String::new(),
                                        examples: Vec::new(),
                                    });
                                },
                                b"example" => {
                                    in_example = true;
                                    current_example = Some(Example {
                                        japanese: String::new(),
                                        chinese: String::new(),
                                    });
                                },
                                _ => {}
                            }
                        }
                    },
                    _ => {}
                }
            },
            Ok(Event::Text(e)) => {
                let text = e.unescape().unwrap();
                if let Some(meaning) = &mut current_meaning {
                    if text.starts_with("〔") && text.ends_with("〕") {
                        meaning.context = text.to_string();
                    } else if text.chars().next().map_or(false, |c| c.is_numeric()) {
                        meaning.number = text.to_string();
                    }
                }
                if in_example {
                    if let Some(example) = &mut current_example {
                        if example.japanese.is_empty() {
                            example.japanese = text.to_string();
                        } else {
                            example.chinese = text.to_string();
                        }
                    }
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name().as_ref() {
                    b"p" => {
                        if in_example {
                            if let Some(example) = current_example.take() {
                                if let Some(meaning) = &mut current_meaning {
                                    meaning.examples.push(example);
                                }
                            }
                            in_example = false;
                        }
                    },
                    _ => {}
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => {},
        }
    }

    // 添加最后一个meaning
    if let Some(meaning) = current_meaning {
        word_entry.meanings.push(meaning);
    }

    Ok(word_entry)
}
