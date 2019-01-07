# twilio-async

## **Breaking Change** 0.2.0

When updating to `hyper@0.12` I decided to change the API slightly. I never liked that twilio-async made decisions about the `Core` and event loop that the requests would run in, so `.run()` now returns a `Box<Future>`. I would have made it return an `impl Future` but this is currently impossible on stable as long as we lack existential return types from trait functions.

## Example Usage

An async and ergonomic wrapper around Twilio API & TwiML.

All types can run `run()` or a similar function. They return a value that implements `Deserialize`.

The `examples/` dir has up to date working example code.

Messages:

```rust
let twilio = Twilio::new(account_sid, token)?;
let mut core = Core::new()?;

// sending a message
core.run(twilio.send_msg("from", "to", "Hello World").run())?;
// sending a body-less message with media
core.run(twilio
    .send_msg("from", "to", "body")
    .media("http://i0.kym-cdn.com/photos/images/newsfeed/000/377/946/0b9.jpg")
    .run())?;
// get details about a message
core.run(twilio.msg("messagesid").run())?;
// redact a message
core.run(twilio.msg("messagesid").redact())?;
// get a msg media url
core.run(twilio.msg("messagesid").media())?;
// delete a msg
core.run(twilio.msg("messagesid").delete())?;
// get all messages
core.run(twilio.msgs().run())?;
// get all messages between some time
core.run(twilio.msgs().between("start date", "end date").run())?;
// get all messages on a specific date
core.run(twilio.msgs().on("date").run())?;
```

Calls:

```rust
let twilio = Twilio::new(env::var("TWILIO_SID")?, env::var("TWILIO_TOKEN")?)?;
let (headers, status, resp) = core.run(twilio
    .call("from", "to", "http://demo.twilio.com/docs/voice.xml")
    .run())?;
```

Twiml:

```rust
use twilio_async::twiml::Response;

let resp = Response::new()
    .say("Hello World") // builder pattern also supports say(Say::new("Hello World").lang("de")...)
    .play("https://api.twilio.com/Cowbell.mp3")
    .build();
let s = "<Response><Say voice=\"man\" language=\"en\" loop=\"1\">Hello World</Say><Play loop=\"1\">https://api.twilio.com/Cowbell.mp3</Play></Response>";
assert_eq!(resp.unwrap(), s.to_string());
```

## Early release

This library is a work in progress, messages and calls are tested, there is untested code for conferences/recordings.

The TwiML work is complete and has some test coverage. Webhooks will be added.

PRs and suggestions are welcome.
