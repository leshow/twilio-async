extern crate twilio;

use std::env;
use std::error::Error;
use twilio::{MsgResp, Twilio, TwilioRequest};

fn main() -> Result<(), Box<Error>> {
    // try_msg()?;
    try_call()?;
    Ok(())
}

fn try_msg() -> Result<(), Box<Error>> {
    let twilio = Twilio::new(env::var("TWILIO_SID")?, env::var("TWILIO_TOKEN")?)?;
    let num = env::var("OUTBOUND_NUM")?;
    // sending a message
    let (_, _, resp) = twilio.send_msg("18193074013", &num, "Hello World").send()?;

    println!("{:?}", resp);
    // sending with media
    let (_, _, resp) = twilio
        .send_msg("18193074013", &num, "foo")
        .media("http://i0.kym-cdn.com/photos/images/newsfeed/000/377/946/0b9.jpg")
        .send()?;
    // get individual msg
    if let Some(json) = resp {
        let MsgResp { sid, .. } = json;
        let (_, _, resp) = twilio.msg("MMec83347e541440f389e24377dd901af7").send()?;
        println!("{:?}", resp);
    }
    // delete a message
    twilio.msg("MMec83347e541440f389e24377dd901af7").delete()?;
    // there's also redact()
    // get a msg media url
    // twilio.msg("messagesid").media()?;
    // // get all messages
    let (_, _, resp) = twilio.msgs().send()?;
    println!("{:?}", resp);
    // // get all messages between some time
    twilio.msgs().between("start date", "end date").send()?;
    // // get all messages on a specific date
    twilio.msgs().on("date").send()?;
    Ok(())
}

fn try_call() -> Result<(), Box<Error>> {
    let twilio = Twilio::new(env::var("TWILIO_SID")?, env::var("TWILIO_TOKEN")?)?;
    let (_, _, resp) = twilio
        .call(
            "18193074013",
            &env::var("OUTBOUND_NUM")?,
            "http://demo.twilio.com/docs/voice.xml",
        )
        .send()?;

    println!("{:?}", resp);
    Ok(())
}
