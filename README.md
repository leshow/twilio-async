# Twilio

A fast (presumably), async, twilio API wrapper

`send` returns a value that implements `Deserialize`.

```rs
let twilio = Twilio::new(account_sid, token)?;
// sending a message
twilio.send_msg("from", "to").body("textbody").send()?;
// sending a body-less message with media
twilio.send_msg("from", "to").media("www.example.com/cheeseburger.png").send()?;
// get details about a message
twilio.msg("messagesid").send()?;
// redact a message
twilio.msg("messagesid").redact()?;
// get a msg media url
twilio.msg("messagesid").media()?;
// get all messages
twilio.msgs().send()?;
// get all messages between some time
twilio.msgs().between("start date", "end date").send()?;
// get all messages on a specific date
twilio.msgs().on("date").send()?;
```
