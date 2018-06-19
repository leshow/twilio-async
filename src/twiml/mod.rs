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
            .say(Say::new("Hello World"))
            .play(Play::new("https://api.twilio.com/Cowbell.mp3"))
            .build();
        let s = "<Response><Say voice=\"man\" language=\"en\" loop=\"1\">Hello World</Say><Play loop=\"1\">https://api.twilio.com/Cowbell.mp3</Play></Response>";
        assert_eq!(resp.unwrap(), s.to_string());
    }

}
