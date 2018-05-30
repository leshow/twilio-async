use hyper::{self, Method};
use serde;
use {encode_pairs, url_encode, Execute, Twilio, TwilioErr, TwilioRequest, TwilioResp};

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
    fn send(self) -> TwilioResp<Self::Resp> {
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
    fn send(self) -> TwilioResp<Self::Resp> {
        let msg_sid = format!("{}.json", self.message_sid);
        self.execute(Method::Get, msg_sid, None)
    }
}

impl<'a> GetMessage<'a> {
    pub fn redact(self) -> TwilioResp<MsgResp> {
        let msg_sid = format!("{}.json", self.message_sid);
        self.execute(Method::Post, msg_sid, Some("Body=".into()))
    }
    pub fn get_media(self) -> TwilioResp<MediaResp> {
        let msg_sid = format!("Messages/{}/Media.json", self.message_sid);
        self.execute(Method::Get, msg_sid, None)
    }
}

#[derive(Debug, Deserialize)]
pub struct MediaResp {
    pub media_list: Vec<MediaItem>,
    pub num_pages: i32,
    pub page: i32,
    pub page_size: i32,
    pub start: i32,
    pub total: i32,
    pub uri: String,
    pub account_sid: String,
    pub message_sid: String,
}

#[derive(Debug, Deserialize)]
pub struct MediaItem {
    pub account_sid: String,
    pub content_type: String,
    pub sid: String,
    pub uri: String,
    pub message_sid: String,
    pub date_created: String,
    pub date_update: String,
}

pub struct Messages<'a> {
    pub client: &'a Twilio,
}

impl<'a> Messages<'a> {
    pub fn between(self, from: &'a str, to: &'a str) -> MessagesDetails<'a> {
        MessagesDetails {
            client: &self.client,
            from: Some(from),
            to: Some(to),
            date_sent: None,
        }
    }
    pub fn on(self, date_sent: &'a str) -> MessagesDetails<'a> {
        MessagesDetails {
            client: &self.client,
            from: None,
            to: None,
            date_sent: Some(date_sent),
        }
    }
}

execute!(Messages);

impl<'a> TwilioRequest for Messages<'a> {
    type Resp = ListAllMsgs;
    fn send(self) -> TwilioResp<Self::Resp> {
        self.execute(Method::Get, "Messages.json", None)
    }
}

pub struct MessagesDetails<'a> {
    pub client: &'a Twilio,
    pub from: Option<&'a str>,
    pub to: Option<&'a str>,
    pub date_sent: Option<&'a str>,
}

impl<'a> ToString for MessagesDetails<'a> {
    fn to_string(&self) -> String {
        let mut pairs = Vec::new();
        if let Some(from) = self.from {
            pairs.push(("From", from));
        }
        if let Some(to) = self.to {
            pairs.push(("To", to));
        }
        if let Some(date_sent) = self.date_sent {
            pairs.push(("DateSent", date_sent));
        }
        // does this have to be different? will the encode_pairs work here?
        url_encode(pairs)
    }
}

execute!(MessagesDetails);

impl<'a> TwilioRequest for MessagesDetails<'a> {
    type Resp = ListAllMsgs;
    fn send(self) -> TwilioResp<Self::Resp> {
        let url = format!("Messages.json?{}", self.to_string());
        self.execute(Method::Get, url, None)
    }
}

#[derive(Debug, Deserialize)]
pub struct ListAllMsgs {
    pub msgs: Vec<MsgResp>,
    pub num_pages: usize,
    pub page: usize,
    pub page_size: usize,
    pub total: usize,
    pub uri: String,
}
