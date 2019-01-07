mod dial;
pub mod error;
mod gather;
mod hangup;
mod msg;
mod play;
mod redirect;
mod response;
mod say;

pub use crate::{
    dial::*, error::*, gather::*, hangup::*, msg::*, play::*, redirect::*, response::*, say::*,
};

use std::io::Write;
use xml::writer::EventWriter;

pub trait Twiml {
    fn write<W: Write>(&self, w: &mut EventWriter<W>) -> TwimlResult<()>;
    fn build(&self) -> TwimlResult<String>;
}

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
}

impl Method {
    fn to_str(&self) -> &str {
        match *self {
            Method::Get => "GET",
            Method::Post => "POST",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twiml_response() {
        let resp = Response::new()
            .say("Hello World")
            .play("https://api.twilio.com/Cowbell.mp3")
            .build();
        let s = "<Response><Say voice=\"man\" language=\"en\" loop=\"1\">Hello World</Say><Play loop=\"1\">https://api.twilio.com/Cowbell.mp3</Play></Response>";
        assert_eq!(resp.unwrap(), s.to_string());
    }

    #[test]
    fn twiml_resp_build() {
        let resp = Response::new()
            .say(Say::new("Hello World").lang("de").voice(Voice::alice))
            .play("https://api.twilio.com/Cowbell.mp3")
            .build();
        let s = "<Response><Say voice=\"alice\" language=\"de\" loop=\"1\">Hello World</Say><Play loop=\"1\">https://api.twilio.com/Cowbell.mp3</Play></Response>";
        assert_eq!(resp.unwrap(), s.to_string());
    }

    #[test]
    fn twiml_say() {
        let say = Say::new("Hello World")
            .lang("de")
            .voice(Voice::alice)
            .build();
        let s = "<Say voice=\"alice\" language=\"de\" loop=\"1\">Hello World</Say>";
        assert_eq!(say.unwrap(), s.to_string());
    }

    #[test]
    fn twiml_play() {
        let play = Play::new("https://api.twilio.com/Cowbell.mp3")
            .count(3)
            .build();
        let s = "<Play loop=\"3\">https://api.twilio.com/Cowbell.mp3</Play>";
        assert_eq!(play.unwrap(), s.to_string());
    }

    #[test]
    fn twiml_response_dial() {
        let resp = Response::new().dial("415-123-4567").build();
        let s = "<Response><Dial method=\"POST\" timeout=\"30\" record=\"do-not-record\">415-123-4567</Dial></Response>";
        assert_eq!(resp.unwrap(), s.to_string());
    }

    #[test]
    fn twiml_response_hangup() {
        let resp = Response::new().hangup().build();
        let s = "<Response><Hangup /></Response>";
        assert_eq!(resp.unwrap(), s.to_string());
    }
}
