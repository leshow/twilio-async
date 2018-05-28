use hyper::{self, Method};
use serde;
use {encode_pairs, Execute, Twilio, TwilioErr, TwilioRequest};

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
            Some(m_url) => encode_pairs(&[
                ("To", self.to),
                ("From", self.from),
                ("Body", self.body),
                ("MediaUrl", m_url),
            ]).unwrap(),
            None => {
                encode_pairs(&[("To", self.to), ("From", self.from), ("Body", self.body)]).unwrap()
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
    pub media_url: String,
    pub price: String,
    pub uri: String,
    pub date_created: String,
    pub date_sent: String,
    pub date_updated: String,
}

// for outbound sms
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

impl<'a> TwilioRequest for SendMsg<'a> {
    type Resp = MsgResp;
    fn send(self) -> Result<(hyper::Headers, hyper::StatusCode, Option<MsgResp>), TwilioErr> {
        let msg = self.msg.to_string();
        self.execute(Method::Post, "Messages.json", Some(msg))
    }
}

// to get
pub struct GetMessage<'a> {
    pub message_sid: &'a str,
    pub client: &'a Twilio,
}

execute!(GetMessage);

impl<'a> TwilioRequest for GetMessage<'a> {
    type Resp = MsgResp;
    fn send(self) -> Result<(hyper::Headers, hyper::StatusCode, Option<MsgResp>), TwilioErr> {
        let msg_sid = format!("{}.json", self.message_sid);
        self.execute(Method::Get, msg_sid, None)
    }
}

impl<'a> GetMessage<'a> {
    pub fn redact(self) -> Result<(hyper::Headers, hyper::StatusCode, Option<MsgResp>), TwilioErr> {
        let msg_sid = format!("{}.json", self.message_sid);
        self.execute(Method::Post, msg_sid, Some(format!("Body=")))
    }
}

pub struct Messages<'a> {
    pub client: &'a Twilio,
}

execute!(Messages);

impl<'a> TwilioRequest for Messages<'a> {
    type Resp = MsgResp;
    fn send(self) -> Result<(hyper::Headers, hyper::StatusCode, Option<MsgResp>), TwilioErr> {
        self.execute(Method::Get, "Messages.json", None)
    }
}

pub struct ListAllMsgs {
    pub msgs: Vec<MsgResp>,
    pub num_pages: usize,
    pub page: usize,
    pub page_size: usize,
    pub total: usize,
    pub uri: String,
}
