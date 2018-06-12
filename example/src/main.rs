extern crate twilio;

use std::env;
use std::error::Error;
use twilio::{MsgResp, Twilio, TwilioRequest};

fn main() -> Result<(), Box<Error>> {
    let twilio = Twilio::new(env::var("TWILIO_SID")?, env::var("TWILIO_TOKEN")?)?;
    // sending a message
    let (_, _, resp) = twilio.send_msg("+xxxxxx", "+xxxxxx", "Hello World").send()?;

    println!("{:?}", resp);
    // sending with media
    let (_, _, resp) = twilio
        .send_msg("+xxxxx3", "+161xxxxxx5", "foo")
        .media("http://www.example.com/cheeseburger.png")
        .send()?;
    // get individual msg
    if let Some(json) = resp {
        let MsgResp { sid } = json;
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
