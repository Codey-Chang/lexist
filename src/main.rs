use std::{collections::HashSet, str::from_utf8};

use lexist::tokenizer::SfTokenizer;
use quick_xml::{events::Event, NsReader, Reader};
use sudachi::prelude::Mode;

use epub::doc::EpubDoc;

fn main() {
    let mut tokenizer = SfTokenizer::new_built(Mode::C);

    let mut doc = EpubDoc::new("resources/epub/epub3-spec.epub").unwrap();
    let spines = doc.spine.clone();

    spines.iter().for_each(|id| {
        let res = doc.get_resource_str(id).unwrap();
        let content = res.0;
        xml_inside_tags(&content, "p");
        let mut rdr = Reader::from_str(&content);
        rdr.config_mut().trim_text(true);

        let mut inside_p_content = String::new();
        let mut txt = Vec::new();

        let mut inside_p_tag = false;
        let mut inside_ruby_tag = false;
        let mut inside_rt_tag = false;

        loop {
            match rdr.read_event() {
                Err(e) => panic!("Error at position {}: {:?}", rdr.error_position(), e),

                Ok(Event::Eof) => break,

                Ok(Event::Start(e)) => match e.name().as_ref() {
                    b"p" => {
                        inside_p_tag = true;
                    }
                    b"ruby" => {
                        inside_ruby_tag = true;
                    }
                    b"rt" => {
                        inside_rt_tag = true;
                    }
                    _ => (),
                },

                Ok(Event::Text(e)) if inside_p_tag => {
                    if inside_p_tag || inside_ruby_tag {
                        inside_p_content.push_str(e.unescape().unwrap().as_ref());
                    } else if inside_rt_tag {
                        inside_p_content
                            .push_str(format!("（{}）", e.unescape().unwrap()).as_str());
                    }
                }

                Ok(Event::End(e)) => match e.name().as_ref() {
                    b"p" => {
                        inside_p_tag = false;
                        txt.push(inside_p_content.clone());
                        inside_p_content.clear();
                    }
                    b"ruby" => {
                        inside_ruby_tag = false;
                    }
                    b"rt" => {
                        inside_rt_tag = false;
                    }
                    _ => (),
                },

                _ => (),
            }
        }

        // txt.iter().for_each(|t| println!("{}", t));
    });
}

fn xml_inside_tags(content: &str, tag: &str) -> HashSet<String> {
    let mut inside_tag = false;

    let mut rdr = Reader::from_str(content);

    let mut ret = HashSet::new();

    loop {
        match rdr.read_event() {
            Err(e) => panic!("Error at position {}: {:?}", rdr.error_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => match e.name().as_ref() {
                _ if tag.as_bytes() == e.name().as_ref() => {
                    inside_tag = true;
                }
                other => {
                    if inside_tag {
                        ret.insert(from_utf8(other).unwrap().to_string());
                    }
                }
            },
            Ok(Event::End(e)) => {
                if e.name().as_ref() == tag.as_bytes() {
                    inside_tag = false;
                }
            }
            _ => (),
        }
    }
    ret
}
