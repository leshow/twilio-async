# Twilio (WIP)

A (presumably) fast, async, twilio API wrapper. Based on hyper & tokio.

All types can run `send()` or a similar function. They return a value that implements `Deserialize`.

The `examples/` dir has up to date working example code.

Messages:

```rs
    let twilio = Twilio::new(account_sid, token)?;
    // sending a message
    twilio.send_msg("from", "to", "Hello World").send()?;
    // sending a body-less message with media
    twilio
        .send_msg("from", "to", "body")
        .media("http://i0.kym-cdn.com/photos/images/newsfeed/000/377/946/0b9.jpg")
        .send()?;
    // get details about a message
    twilio.msg("messagesid").send()?;
    // redact a message
    twilio.msg("messagesid").redact()?;
    // get a msg media url
    twilio.msg("messagesid").media()?;
    // delete a msg
    twilio.msg("messagesid").delete()?;
    // get all messages
    twilio.msgs().send()?;
    // get all messages between some time
    twilio.msgs().between("start date", "end date").send()?;
    // get all messages on a specific date
    twilio.msgs().on("date").send()?;
```

Calls:

```rs
    let twilio = Twilio::new(env::var("TWILIO_SID")?, env::var("TWILIO_TOKEN")?)?;
    let (headers, status, resp) = twilio
        .call("from", "to", "http://demo.twilio.com/docs/voice.xml")
        .send()?;
```

## Beta

This library is a work in progress, messages and calls should work, there is untested code for conferences/recordings also, and I'm working on support twiml.
