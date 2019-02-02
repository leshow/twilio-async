# TwiML

```rust
   #[test]
    fn twiml_response() {
        let resp = Response::new()
            .say("Hello World")
            .play("https://api.twilio.com/Cowbell.mp3")
            .build();
        let s = "<Response><Say voice=\"man\" language=\"en\" loop=\"1\">Hello World</Say><Play loop=\"1\">https://api.twilio.com/Cowbell.mp3</Play></Response>";
        assert_eq!(resp.unwrap(), s.to_string());
    }

    #[test]
    fn twiml_resp_build() {
        let resp = Response::new()
            .say(Say::new("Hello World").lang("de").voice(Voice::alice))
            .play("https://api.twilio.com/Cowbell.mp3")
            .build();
        let s = "<Response><Say voice=\"alice\" language=\"de\" loop=\"1\">Hello World</Say><Play loop=\"1\">https://api.twilio.com/Cowbell.mp3</Play></Response>";
        assert_eq!(resp.unwrap(), s.to_string());
    }

    #[test]
    fn twiml_say() {
        let say = Say::new("Hello World")
            .lang("de")
            .voice(Voice::alice)
            .build();
        let s = "<Say voice=\"alice\" language=\"de\" loop=\"1\">Hello World</Say>";
        assert_eq!(say.unwrap(), s.to_string());
    }

    #[test]
    fn twiml_play() {
        let play = Play::new("https://api.twilio.com/Cowbell.mp3")
            .count(3)
            .build();
        let s = "<Play loop=\"3\">https://api.twilio.com/Cowbell.mp3</Play>";
        assert_eq!(play.unwrap(), s.to_string());
    }

    #[test]
    fn twiml_response_dial() {
        let resp = Response::new().dial("415-123-4567").build();
        let s = "<Response><Dial method=\"POST\" timeout=\"30\" record=\"do-not-record\">415-123-4567</Dial></Response>";
        assert_eq!(resp.unwrap(), s.to_string());
    }

    #[test]
    fn twiml_response_hangup() {
        let resp = Response::new().hangup().build();
        let s = "<Response><Hangup /></Response>";
        assert_eq!(resp.unwrap(), s.to_string());
    }
```
