use crate::*;
use xml::{
    writer::{EventWriter, XmlEvent},
    EmitterConfig,
};

#[derive(Debug)]
pub struct Say<'a> {
    voice: Voice,
    count: usize,
    language: &'a str,
    body: &'a str,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Voice {
    man,
    woman,
    alice,
}

impl<'a> Say<'a> {
    pub fn new(body: &'a str) -> Self {
        Say {
            body,
            voice: Voice::man,
            count: 1,
            language: "en",
        }
    }

    pub fn lang(mut self, language: &'a str) -> Say<'a> {
        self.language = language;
        self
    }

    pub fn voice(mut self, voice: Voice) -> Say<'a> {
        self.voice = voice;
        self
    }

    pub fn say_count(mut self, count: usize) -> Say<'a> {
        self.count = count;
        self
    }
}

impl<'a> Twiml for Say<'a> {
    fn write<W: Write>(&self, w: &mut EventWriter<W>) -> TwimlResult<()> {
        // Create a buffer and serialize our nodes into it
        w.write(
            XmlEvent::start_element("Say")
                .attr("voice", self.voice.to_str())
                .attr("language", self.language)
                .attr("loop", &self.count.to_string()),
        )?;
        w.write(self.body)?;
        w.write(XmlEvent::end_element())?;
        Ok(())
    }

    fn build(&self) -> TwimlResult<String> {
        // Create a buffer and serialize our nodes into it
        let mut writer = Vec::new();
        {
            let mut w = EmitterConfig::new()
                .write_document_declaration(false)
                .create_writer(&mut writer);

            self.write(&mut w)?;
        }
        Ok(String::from_utf8(writer)?)
    }
}

impl Voice {
    pub fn to_str(&self) -> &str {
        match *self {
            Voice::man => "man",
            Voice::woman => "woman",
            Voice::alice => "alice",
        }
    }
}

impl<'a, T> From<T> for Say<'a>
where
    T: Into<&'a str>,
{
    fn from(s: T) -> Self {
        Say::new(s.into())
    }
}
