use crate::*;
use xml::{
    writer::{EventWriter, XmlEvent},
    EmitterConfig,
};

#[derive(Debug)]
pub struct Msg<'a> {
    media: Option<&'a str>,
    body: &'a str,
}

impl<'a> Msg<'a> {
    pub fn new(body: &'a str) -> Self {
        Msg { body, media: None }
    }

    pub fn media(mut self, media: &'a str) -> Self {
        self.media = Some(media);
        self
    }
}

impl<'a> Twiml for Msg<'a> {
    fn write<W: Write>(&self, w: &mut EventWriter<W>) -> TwimlResult<()> {
        w.write(XmlEvent::start_element("Message"))?; // .attr("loop", &self.count.to_string())
        w.write(XmlEvent::start_element("Body"))?;
        w.write(self.body)?;
        w.write(XmlEvent::end_element())?;
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

impl<'a, T> From<T> for Msg<'a>
where
    T: Into<&'a str>,
{
    fn from(s: T) -> Self {
        Msg::new(s.into())
    }
}
