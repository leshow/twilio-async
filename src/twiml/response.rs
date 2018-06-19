use twiml::*;
use xml::{writer::XmlEvent, EmitterConfig};

#[derive(Debug)]
pub struct Response<'a> {
    say: Option<Say<'a>>,
    play: Option<Play<'a>>,
}

impl<'a> Response<'a> {
    pub fn new() -> Self {
        Response {
            say: None,
            play: None,
        }
    }
    pub fn say(self, say: Say<'a>) -> Response<'a> {
        Response {
            say: Some(say),
            ..self
        }
    }
    pub fn play(self, play: Play<'a>) -> Response<'a> {
        Response {
            play: Some(play),
            ..self
        }
    }
}

impl<'a> Twiml for Response<'a> {
    fn write<W: Write>(&self, w: &mut EventWriter<W>) -> TwilioResult<()> {
        w.write(XmlEvent::start_element("Response"))?;
        if let Some(ref val) = self.say {
            val.write(w)?;
        }
        if let Some(ref val) = self.play {
            val.write(w)?;
        }
        w.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn build(&self) -> TwilioResult<String> {
        // Create a buffer and serialize our nodes into it
        let mut writer = Vec::with_capacity(128);
        {
            let mut w = EmitterConfig::new()
                .write_document_declaration(false)
                .create_writer(&mut writer);
            self.write(&mut w)?;
        }
        Ok(String::from_utf8(writer)?)
    }
}
