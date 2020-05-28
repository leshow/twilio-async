use crate::*;
use xml::{
    writer::{EventWriter, XmlEvent},
    EmitterConfig,
};

#[derive(Debug)]
pub struct Redirect<'a> {
    method: Method,
    body: &'a str,
}

impl<'a> Default for Redirect<'a> {
    fn default() -> Self {
        Redirect {
            body: "",
            method: Method::Post,
        }
    }
}

impl<'a> Redirect<'a> {
    pub fn new(body: &'a str) -> Self {
        Redirect {
            body,
            ..Redirect::default()
        }
    }

    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }
}

impl<'a> Twiml for Redirect<'a> {
    fn write<W: Write>(&self, w: &mut EventWriter<W>) -> TwimlResult<()> {
        w.write(XmlEvent::start_element("Redirect").attr("method", self.method.to_str()))?;
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

impl<'a, T> From<T> for Redirect<'a>
where
    T: Into<&'a str>,
{
    fn from(s: T) -> Self {
        Redirect::new(s.into())
    }
}
