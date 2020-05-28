use crate::*;
use xml::{
    writer::{EventWriter, XmlEvent},
    EmitterConfig,
};

#[derive(Debug)]
pub struct Gather<'a> {
    method: Method,
    action: Option<&'a str>,
    key: char,
    timeout: usize,
    body: GatherBody<'a>,
}

#[derive(Debug)]
enum GatherBody<'a> {
    Nil,
    Say(Say<'a>),
    Play(Play<'a>),
    Redirect(Redirect<'a>),
}

impl<'a> Default for Gather<'a> {
    fn default() -> Self {
        Gather {
            body: GatherBody::Nil,
            method: Method::Post,
            key: '#',
            action: None,
            timeout: 5,
        }
    }
}

impl<'a> Gather<'a> {
    pub fn say<S: Into<Say<'a>>>(mut self, say: S) -> Self {
        self.body = GatherBody::Say(say.into());
        self
    }

    pub fn play<P: Into<Play<'a>>>(mut self, play: P) -> Self {
        self.body = GatherBody::Play(play.into());
        self
    }

    pub fn redirect<R: Into<Redirect<'a>>>(mut self, redirect: R) -> Self {
        self.body = GatherBody::Redirect(redirect.into());
        self
    }

    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    pub fn finish_on_key(mut self, key: char) -> Self {
        self.key = key;
        self
    }

    pub fn timeout(mut self, timeout: usize) -> Self {
        self.timeout = timeout;
        self
    }
}

impl<'a> Twiml for Gather<'a> {
    fn write<W: Write>(&self, w: &mut EventWriter<W>) -> TwimlResult<()> {
        let timeout = self.timeout.to_string();
        let key = self.key.to_string();
        let el = XmlEvent::start_element("Gather")
            .attr("method", self.method.to_str())
            .attr("timeout", &timeout)
            .attr("finishOnKey", &key);
        if let Some(action) = self.action {
            w.write(el.attr("action", action))?;
        } else {
            w.write(el)?;
        }
        match self.body {
            GatherBody::Nil => {}
            GatherBody::Play(ref val) => val.write(w)?,
            GatherBody::Say(ref val) => val.write(w)?,
            GatherBody::Redirect(ref val) => val.write(w)?,
        }

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
