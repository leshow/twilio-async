extern crate twilio_async;

use std::{env, error::Error};
use tokio::prelude::*;
use twilio_async::{
    twiml::{Dial, Response},
    MsgResp, Twilio, TwilioRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let twilio = Twilio::new(env::var("TWILIO_SID")?, env::var("TWILIO_TOKEN")?)?;
    // try_msg(twilio)?;
    // try_call(twilio)?;
    try_conference(twilio).await?;
    Ok(())
}

async fn try_conference(twilio: Twilio) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let (_, resp) = twilio.conferences().run().await?;

    let (_, resp) = twilio
        .conference("EH5bc4f5c62684f43d0acadb3d88a43e38")
        .run()
        .await?;

    println!("{:?}", resp);
    Ok(())
}

async fn try_msg(twilio: Twilio) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let num = env::var("OUTBOUND_NUM")?;
    // sending a message
    let (_, resp) = twilio
        .send_msg("18193074013", &num, "Hello World")
        .run()
        .await?;

    println!("{:?}", resp);
    // sending with media
    let (_, resp) = twilio
        .send_msg("18193074013", &num, "foo")
        .media("http://i0.kym-cdn.com/photos/images/newsfeed/000/377/946/0b9.jpg")
        .run()
        .await?;
    // get individual msg
    if let Some(json) = resp {
        let MsgResp { sid, .. } = json;
        let (_, resp) = twilio
            .msg("MMec83347e541440f389e24377dd901af7")
            .run()
            .await?;
        println!("{:?}", resp);
    }
    // delete a message
    twilio
        .msg("MMec83347e541440f389e24377dd901af7")
        .delete()
        .await?;
    // there's also redact()
    // get a msg media url
    // twilio.msg("messagesid").media()?;
    // // get all messages
    let (_, resp) = twilio.msgs().run().await?;
    println!("{:?}", resp);
    // // get all messages between some time
    twilio
        .msgs()
        .between("start date", "end date")
        .run()
        .await?;
    // // get all messages on a specific date
    twilio.msgs().on("date").run().await?;
    Ok(())
}

async fn try_call(twilio: Twilio) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let (_, resp) = twilio
        .call(
            "18193074013",
            &env::var("OUTBOUND_NUM")?,
            "http://demo.twilio.com/docs/voice.xml",
        )
        .run()
        .await?;

    println!("{:?}", resp);
    Ok(())
}
