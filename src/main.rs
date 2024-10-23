use lexist::tokenizer::SfTokenizer;
use quick_xml::{events::Event, Reader};
use sudachi::prelude::Mode;

use epub::doc::EpubDoc;

fn main() {
    let mut tokenizer = SfTokenizer::new_built(Mode::C);

    let mut doc = EpubDoc::new("resources/epub/epub3-spec.epub").unwrap();
    let spines = doc.spine.clone();

    spines.iter().for_each(|id| {
        let res = doc.get_resource_str(id).unwrap();
        let content = res.0;

        let mut rdr = Reader::from_str(&content);
        rdr.config_mut().trim_text(true);

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
                    txt.push(e.unescape().unwrap().into_owned());
                }

                Ok(Event::End(e)) => match e.name().as_ref() {
                    b"p" => {
                        inside_p_tag = false;
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

        txt.iter().for_each(|t| println!("{}", t));
    });
}
