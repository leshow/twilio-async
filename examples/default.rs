extern crate tokio_core;
extern crate twilio_async;

use std::{env, error::Error};
use tokio_core::reactor::Core;
use twilio_async::{
    twiml::{Dial, Twiml},
    MsgResp, Twilio, TwilioRequest,
};

fn main() -> Result<(), Box<dyn Error>> {
    let twilio = Twilio::new(env::var("TWILIO_SID")?, env::var("TWILIO_TOKEN")?)?;
    let mut core = Core::new()?;
    try_msg(&mut core, twilio)?;
    // try_call(&mut core, twilio)?;
    // try_conference(&mut core, twilio)?;

    println!("{:?}", Dial::new("5555555").build()?);
    Ok(())
}

fn _try_conference(core: &mut Core, twilio: Twilio) -> Result<(), Box<dyn Error>> {
    let (_, resp) = core.run(twilio.conferences().run())?;

    println!("{:?}", resp);

    let (_, resp) = core.run(
        twilio
            .conference("EH5bc4f5c62684f43d0acadb3d88a43e38")
            .run(),
    )?;

    println!("{:?}", resp);
    Ok(())
}

fn try_msg(core: &mut Core, twilio: Twilio) -> Result<(), Box<dyn Error>> {
    let num = env::var("OUTBOUND_NUM")?;
    // sending a message
    let (_, resp) = core.run(twilio.send_msg("18193074013", &num, "Hello World").run())?;

    println!("{:?}", resp);
    // sending with media
    let (_, resp) = core.run(
        twilio
            .send_msg("18193074013", &num, "foo")
            .media("http://i0.kym-cdn.com/photos/images/newsfeed/000/377/946/0b9.jpg")
            .run(),
    )?;
    // get individual msg
    if let Some(json) = resp {
        let MsgResp { sid, .. } = json;
        println!("msg sid {:?}", sid);
        let (_, resp) = core.run(twilio.msg("MMec83347e541440f389e24377dd901af7").run())?;
        println!("{:?}", resp);
    }
    // delete a message
    core.run(twilio.msg("MMec83347e541440f389e24377dd901af7").delete())?;
    // there's also redact()
    // get a msg media url
    // twilio.msg("messagesid").media()?;
    // // get all messages
    let (_, resp) = core.run(twilio.msgs().run())?;
    println!("{:?}", resp);
    // // get all messages between some time
    core.run(twilio.msgs().between("start date", "end date").run())?;
    // // get all messages on a specific date
    core.run(twilio.msgs().on("date").run())?;
    Ok(())
}

fn _try_call(core: &mut Core, twilio: Twilio) -> Result<(), Box<dyn Error>> {
    let (_, resp) = core.run(
        twilio
            .call(
                "18193074013",
                &env::var("OUTBOUND_NUM")?,
                "http://demo.twilio.com/docs/voice.xml",
            )
            .run(),
    )?;

    println!("{:?}", resp);
    Ok(())
}
