pub mod play;
pub mod response;
pub mod say;

use twiml::play::*;
use twiml::response::*;
use twiml::say::*;

use std::io::Write;
use xml::writer::EventWriter;
use TwilioResult;

pub trait Twiml {
    fn write<W: Write>(&self, w: &mut EventWriter<W>) -> TwilioResult<()>;
    fn build(&self) -> TwilioResult<String>;
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
}
