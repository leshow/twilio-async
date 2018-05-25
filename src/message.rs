use hyper::{self, Method};
use serde;
use {Execute, Twilio, TwilioErr, TwilioReq};

pub struct Msg<'a> {
    pub from: &'a str,
    pub to: &'a str,
    pub body: &'a str,
    pub media_url: Option<&'a str>,
}

impl<'a> Msg<'a> {
    pub fn new(from: &'a str, to: &'a str, body: &'a str) -> Msg<'a> {
        Msg {
            from,
            to,
            body,
            media_url: None,
        }
    }
}

impl<'a> ToString for Msg<'a> {
    fn to_string(&self) -> String {
        match self.media_url {
            Some(m_url) => super::encode_pairs(&[
                ("To", self.to),
                ("From", self.from),
                ("Body", self.body),
                ("MediaUrl", m_url),
            ]).unwrap(),
            None => {
                super::encode_pairs(&[("To", self.to), ("From", self.from), ("Body", self.body)])
                    .unwrap()
            }
        }
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

impl<'a> SendMsg<'a> {
    fn set_media_url(&mut self, media_url: &'a str) {
        self.msg.media_url = Some(media_url);
    }
}

execute!(SendMsg);

impl<'a> TwilioReq for SendMsg<'a> {
    fn send<D>(self) -> Result<(hyper::Headers, hyper::StatusCode, Option<D>), TwilioErr>
    where
        D: for<'de> serde::Deserialize<'de>,
    {
        let msg = self.msg.to_string();
        self.execute(Method::Post, "Messages.json", msg)
    }
}
