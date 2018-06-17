use TwilioResult;
use {serde::ser::Serialize, serde_xml_rs::Serializer, std::io::Write};

#[derive(Debug, Serialize)]
pub struct Response<'a> {
    #[serde(rename = "Say")]
    pub say: Option<Say<'a>>,
    #[serde(rename = "Play")]
    pub play: Option<Play<'a>>,
}

#[derive(Debug, Serialize)]
pub struct Say<'a> {
    pub voice: Voice,
    // #[serde(rename = "loop")]
    // pub count: i32,
    // pub language: &'a str,
    #[serde(rename = "$value")]
    pub body: &'a str,
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum Voice {
    man,
    woman,
    alice,
}

#[derive(Debug, Serialize)]
pub struct Play<'a> {
    // #[serde(rename = "loop")]
    pub count: i32,
    #[serde(rename = "$value")]
    pub body: &'a str,
}

impl<'a> Response<'a> {
    pub fn new() -> Self {
        Response {
            say: None,
            play: None,
        }
    }
    pub fn say(self, body: &'a str) -> Response<'a> {
        Response {
            say: Some(Say::new(body)),
            ..self
        }
    }
    pub fn play(self, body: &'a str) -> Response<'a> {
        Response {
            play: Some(Play::new(body)),
            ..self
        }
    }
    pub fn build(self) -> TwilioResult<String> {
        // Create a buffer and serialize our nodes into it
        let mut writer = Vec::with_capacity(128);
        to_writer(&mut writer, &self)?;

        // We then check that the serialized string is the same as what we expect
        let string = String::from_utf8(writer)?;
        Ok(string)
    }
}

pub fn to_writer<W: Write, S: Serialize>(writer: W, value: &S) -> TwilioResult<()> {
    let mut ser = Serializer::new(writer);
    Ok(value.serialize(&mut ser)?)
}

impl<'a> Say<'a> {
    pub fn new(body: &'a str) -> Self {
        Say {
            body,
            voice: Voice::man,
            // count: 1,
            // language: "en",
        }
    }
}

impl<'a> Play<'a> {
    pub fn new(body: &'a str) -> Self {
        Play { body, count: 1 }
    }
}
