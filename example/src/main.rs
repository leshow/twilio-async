extern crate twilio_async;

use std::env;
use std::error::Error;
use twilio_async::twiml::{Dial, Response};
use twilio_async::{MsgResp, Twilio, TwilioRequest};

fn main() -> Result<(), Box<Error>> {
    let twilio = Twilio::new(env::var("TWILIO_SID")?, env::var("TWILIO_TOKEN")?)?;
    // try_msg(twilio)?;
    // try_call(twilio)?;
    try_conference(twilio)?;
    Ok(())
}

fn try_conference(twilio: Twilio) -> Result<(), Box<Error>> {
    let (_, _, resp) = twilio.conferences().run()?;

    let (_, _, resp) = twilio
        .conference("EH5bc4f5c62684f43d0acadb3d88a43e38")
        .run()?;

    println!("{:?}", resp);
    Ok(())
}

fn try_msg(twilio: Twilio) -> Result<(), Box<Error>> {
    let num = env::var("OUTBOUND_NUM")?;
    // sending a message
    let (_, _, resp) = twilio.send_msg("18193074013", &num, "Hello World").run()?;

    println!("{:?}", resp);
    // sending with media
    let (_, _, resp) = twilio
        .send_msg("18193074013", &num, "foo")
        .media("http://i0.kym-cdn.com/photos/images/newsfeed/000/377/946/0b9.jpg")
        .run()?;
    // get individual msg
    if let Some(json) = resp {
        let MsgResp { sid, .. } = json;
        let (_, _, resp) = twilio.msg("MMec83347e541440f389e24377dd901af7").run()?;
        println!("{:?}", resp);
    }
    // delete a message
    twilio.msg("MMec83347e541440f389e24377dd901af7").delete()?;
    // there's also redact()
    // get a msg media url
    // twilio.msg("messagesid").media()?;
    // // get all messages
    let (_, _, resp) = twilio.msgs().run()?;
    println!("{:?}", resp);
    // // get all messages between some time
    twilio.msgs().between("start date", "end date").run()?;
    // // get all messages on a specific date
    twilio.msgs().on("date").run()?;
    Ok(())
}

fn try_call(twilio: Twilio) -> Result<(), Box<Error>> {
    let (_, _, resp) = twilio
        .call(
            "18193074013",
            &env::var("OUTBOUND_NUM")?,
            "http://demo.twilio.com/docs/voice.xml",
        )
        .run()?;

    println!("{:?}", resp);
    Ok(())
}
