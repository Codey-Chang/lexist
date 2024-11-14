use quick_xml::{events::Event, Reader};

use super::Processor;

pub struct PTagTextExtractor;

impl Processor for PTagTextExtractor {
    fn process(&self, text: &str) -> Result<String, super::ProcessError> {
        let mut ret = "".to_string();
        let mut rdr = Reader::from_str(text);

        let mut inside_p_tag = false;
        let mut inside_rt_tag = false;

        loop {
            match rdr.read_event() {
                Err(e) => {
                    return Err(e.into());
                }
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => match e.name().as_ref() {
                    b"p" => {
                        inside_p_tag = true;
                    }
                    b"rt" => {
                        inside_rt_tag = true;
                    }
                    _ => (),
                },
                Ok(Event::Text(e)) if inside_p_tag => {
                    if !inside_rt_tag {
                        let s = e.unescape()?;
                        ret.push_str(s.as_ref());
                    }
                }
                Ok(Event::End(e)) => match e.name().as_ref() {
                    b"p" => {
                        inside_p_tag = false;
                    }
                    b"rt" => {
                        inside_rt_tag = false;
                    }
                    _ => (),
                },
                _ => (),
            }
        }
        Ok(ret)
    }
}

impl PTagTextExtractor {
    pub fn new() -> Self {
        PTagTextExtractor
    }
}
