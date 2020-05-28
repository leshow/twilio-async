use crate::*;
use xml::{
    writer::{EventWriter, XmlEvent},
    EmitterConfig,
};

#[derive(Debug)]
pub struct Dial<'a> {
    method: Method,
    action: Option<&'a str>,
    timeout: usize,
    number: &'a str,
    recording_callback: Option<&'a str>,
    record: Record,
}

impl<'a> Default for Dial<'a> {
    fn default() -> Self {
        Dial {
            number: "",
            method: Method::Post,
            recording_callback: None,
            record: Record::DoNotRecord,
            action: None,
            timeout: 30,
        }
    }
}

impl<'a> Dial<'a> {
    pub fn new(number: &'a str) -> Self {
        Dial {
            number,
            ..Dial::default()
        }
    }

    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    pub fn action(mut self, url: &'a str) -> Self {
        self.action = Some(url);
        self
    }

    pub fn record(mut self, status: Record) -> Self {
        self.record = status;
        self
    }

    pub fn recording_callback(mut self, url: &'a str) -> Self {
        self.recording_callback = Some(url);
        self
    }

    pub fn timeout(mut self, timeout: usize) -> Self {
        self.timeout = timeout;
        self
    }
}

#[derive(Debug)]
pub enum Record {
    DoNotRecord,
    RecordFromAnswer,
    RecordFromRinging,
}

impl Record {
    fn to_str(&self) -> &str {
        match *self {
            Record::DoNotRecord => "do-not-record",
            Record::RecordFromAnswer => "record-from-answer",
            Record::RecordFromRinging => "record-from-ringing",
        }
    }
}

impl<'a> Twiml for Dial<'a> {
    fn write<W: Write>(&self, w: &mut EventWriter<W>) -> TwimlResult<()> {
        let timeout = self.timeout.to_string();
        let el = XmlEvent::start_element("Dial")
            .attr("method", self.method.to_str())
            .attr("timeout", &timeout)
            .attr("record", self.record.to_str());

        // not sure how else to get around this particular move error
        match (self.action, self.recording_callback) {
            (None, None) => w.write(el)?,
            (Some(action), None) => w.write(el.attr("action", action))?,
            (None, Some(callback)) => w.write(el.attr("recordingStatusCallback", callback))?,
            (Some(action), Some(callback)) => w.write(
                el.attr("action", action)
                    .attr("recordingStatusCallback", callback),
            )?,
        }

        w.write(self.number)?;
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

impl<'a, T> From<T> for Dial<'a>
where
    T: Into<&'a str>,
{
    fn from(s: T) -> Self {
        Dial::new(s.into())
    }
}
