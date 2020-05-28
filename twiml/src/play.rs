use crate::*;
use xml::{
    writer::{EventWriter, XmlEvent},
    EmitterConfig,
};

#[derive(Debug)]
pub struct Play<'a> {
    count: usize,
    body: &'a str,
}

impl<'a> Play<'a> {
    pub fn new(body: &'a str) -> Self {
        Play { body, count: 1 }
    }

    pub fn count(mut self, count: usize) -> Play<'a> {
        self.count = count;
        self
    }
}

impl<'a> Twiml for Play<'a> {
    fn write<W: Write>(&self, w: &mut EventWriter<W>) -> TwimlResult<()> {
        w.write(XmlEvent::start_element("Play").attr("loop", &self.count.to_string()))?;
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

impl<'a, T> From<T> for Play<'a>
where
    T: Into<&'a str>,
{
    fn from(s: T) -> Self {
        Play::new(s.into())
    }
}
