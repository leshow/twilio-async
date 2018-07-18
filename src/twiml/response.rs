use twiml::*;
use xml::{writer::XmlEvent, EmitterConfig};

#[derive(Debug, Default)]
pub struct Response<'a> {
    say: Option<Say<'a>>,
    play: Option<Play<'a>>,
    msg: Option<Msg<'a>>,
    redirect: Option<Redirect<'a>>,
    gather: Option<Gather<'a>>,
    dial: Option<Dial<'a>>,
    hangup: Option<Hangup>,
}

impl<'a> Response<'a> {
    pub fn new() -> Self {
        Response {
            say: None,
            play: None,
            msg: None,
            redirect: None,
            gather: None,
            dial: None,
            hangup: None,
        }
    }
    pub fn say<S: Into<Say<'a>>>(mut self, say: S) -> Self {
        self.say = Some(say.into());
        self
    }
    pub fn play<P: Into<Play<'a>>>(mut self, play: P) -> Self {
        self.play = Some(play.into());
        self
    }
    pub fn redirect<R: Into<Redirect<'a>>>(mut self, redirect: R) -> Self {
        self.redirect = Some(redirect.into());
        self
    }
    pub fn msg<M: Into<Msg<'a>>>(mut self, msg: M) -> Self {
        self.msg = Some(msg.into());
        self
    }
    pub fn gather(mut self, gather: Gather<'a>) -> Self {
        self.gather = Some(gather);
        self
    }
    pub fn dial<D: Into<Dial<'a>>>(mut self, dial: D) -> Self {
        self.dial = Some(dial.into());
        self
    }
    pub fn hangup(mut self) -> Self {
        self.hangup = Some(Hangup::new());
        self
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
        if let Some(ref val) = self.redirect {
            val.write(w)?;
        }
        if let Some(ref val) = self.msg {
            val.write(w)?;
        }
        if let Some(ref val) = self.gather {
            val.write(w)?;
        }
        if let Some(ref val) = self.dial {
            val.write(w)?;
        }
        if let Some(ref val) = self.hangup {
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
