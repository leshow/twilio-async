use hyper::{self, Method};
use serde;
use {Execute, Twilio, TwilioErr, TwilioReq};

pub struct Msg<'a> {
    pub from: &'a str,
    pub to: &'a str,
    pub body: &'a str,
}

impl<'a> Msg<'a> {
    pub fn new(from: &'a str, to: &'a str, body: &'a str) -> Msg<'a> {
        Msg { from, to, body }
    }
}

impl<'a> ToString for Msg<'a> {
    fn to_string(&self) -> String {
        super::encode_pairs([("To", self.to), ("From", self.from), ("Body", self.body)].iter())
            .unwrap()
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum MsgStatus {
    queued,
    sending,
    sent,
    failed,
    delivered,
    undelivered,
    receiving,
    received,
}

#[derive(Debug, Deserialize)]
pub struct MsgResp {
    pub from: String,
    pub to: String,
    pub body: Option<String>,
    pub sid: String,
    pub status: Option<MsgStatus>,
}

pub struct SendMsg<'a> {
    pub msg: Msg<'a>,
    pub client: &'a Twilio,
}

execute!(SendMsg);

impl<'a> TwilioReq for SendMsg<'a> {
    fn send<D>(self) -> Result<(hyper::Headers, hyper::StatusCode, Option<D>), TwilioErr>
    where
        D: serde::de::DeserializeOwned,
    {
        let msg = self.msg.to_string();
        self.execute(Method::Post, "Messages", msg)
    }
}
