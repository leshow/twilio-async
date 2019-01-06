use crate::twiml::*;
use xml::{
    writer::{EventWriter, XmlEvent},
    EmitterConfig,
};

#[derive(Debug)]
pub struct Hangup;

impl Hangup {
    pub fn new() -> Self {
        Hangup
    }
}

impl Twiml for Hangup {
    fn write<W: Write>(&self, w: &mut EventWriter<W>) -> TwilioResult<()> {
        w.write(XmlEvent::start_element("Hangup"))?;
        w.write(XmlEvent::end_element())?;
        Ok(())
    }

    fn build(&self) -> TwilioResult<String> {
        // Create a buffer and serialize our nodes into it
        let mut writer = Vec::with_capacity(30);
        {
            let mut w = EmitterConfig::new()
                .write_document_declaration(false)
                .create_writer(&mut writer);

            self.write(&mut w)?;
        }
        Ok(String::from_utf8(writer)?)
    }
}
