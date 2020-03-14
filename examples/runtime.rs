#![allow(dead_code)]
use std::{env, error::Error};
use twilio_async::{MsgResp, Twilio, TwilioJson, TwilioRequest};

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

#[tokio::main]
async fn main() -> Result<()> {
    let twilio = Twilio::new(env::var("TWILIO_SID")?, env::var("TWILIO_TOKEN")?)?;
    try_msg(twilio).await?;
    // try_call(twilio).await?;
    // try_conference(twilio).await?;
    Ok(())
}

async fn try_conference(twilio: Twilio) -> Result<()> {
    let _resp = twilio.conferences().run().await?;

    let resp = twilio
        .conference("EH5bc4f5c62684f43d0acadb3d88a43e38")
        .run()
        .await?;

    println!("{:?}", resp);
    Ok(())
}

async fn try_msg(twilio: Twilio) -> Result<()> {
    let num = env::var("OUTBOUND_NUM")?;
    // sending a message
    let resp = twilio.send_msg(&num, &num, "Hello World").run().await?;

    println!("{:?}", resp);
    // sending with media
    let resp = twilio
        .send_msg("18193074013", &num, "foo")
        .media("http://i0.kym-cdn.com/photos/images/newsfeed/000/377/946/0b9.jpg")
        .run()
        .await?;
    // get individual msg
    if let Some(TwilioJson::Success(MsgResp { sid, .. })) = resp {
        let resp = twilio.msg(&sid).run().await?;
        println!("{:?}", resp);
    }
    // delete a message
    twilio
        .msg("SM5585720d3f244b1cb054862040b7b858")
        .delete()
        .await?;
    // there's also redact()
    // get a msg media url
    // twilio.msg("messagesid").media()?;
    // // get all messages
    let resp = twilio.msgs().run().await?;
    println!("{:?}", resp);
    // // get all messages between some time
    let resp = twilio
        .msgs()
        .between("2010-01-01", "2020-01-01")
        .run()
        .await?;
    println!("{:?}", resp);
    // // get all messages on a specific date
    let resp = twilio.msgs().on("2020-01-01").run().await?;
    println!("{:?}", resp);
    Ok(())
}

async fn try_call(twilio: Twilio) -> Result<()> {
    let resp = twilio
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
