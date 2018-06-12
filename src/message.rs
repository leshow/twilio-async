use hyper::{self, Method};
use serde;
use {encode_pairs, url_encode, Execute, Twilio, TwilioErr, TwilioRequest, TwilioResp};

#[derive(Default, Debug)]
pub struct Msg<'a> {
    from: &'a str,
    to: &'a str,
    body: &'a str,
    media_url: Option<&'a str>,
}

impl<'a> Msg<'a> {
    pub fn new(from: &'a str, to: &'a str, body: &'a str) -> Msg<'a> {
        Msg {
            from,
            to,
            body,
            ..Msg::default()
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
    pub body: String,
    pub sid: String,
    pub status: MsgStatus,
    pub media_url: Option<String>,
    pub price: Option<String>,
    pub price_unit: String,
    pub uri: String,
    pub date_created: String,
    pub date_sent: Option<String>,
    pub date_updated: String,
}

// for outbound sms
#[derive(Debug)]
pub struct SendMsg<'a> {
    pub msg: Msg<'a>,
    pub client: &'a Twilio,
}

impl<'a> SendMsg<'a> {
    pub fn media(self, media_url: &'a str) -> SendMsg<'a> {
        SendMsg {
            msg: Msg {
                media_url: Some(media_url),
                ..self.msg
            },
            ..self
        }
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

#[derive(Debug)]
pub struct GetMessage<'a> {
    pub message_sid: &'a str,
    pub client: &'a Twilio,
}

execute!(GetMessage);

impl<'a> TwilioRequest for GetMessage<'a> {
    type Resp = MsgResp;
    fn send(self) -> TwilioResp<Self::Resp> {
        let msg_sid = format!("Messages/{}.json", self.message_sid);
        self.execute(Method::Get, msg_sid, None)
    }
}

impl<'a> GetMessage<'a> {
    pub fn redact(self) -> TwilioResp<MsgResp> {
        let msg_sid = format!("Messages/{}.json", self.message_sid);
        self.execute(Method::Post, msg_sid, Some("Body=".into()))
    }
    pub fn delete(self) -> TwilioResp<MsgResp> {
        let msg_sid = format!("Messages/{}.json", self.message_sid);
        self.execute(Method::Delete, msg_sid, None)
    }
    pub fn media(self) -> TwilioResp<MediaResp> {
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
        pair!(self, from, "From", pairs);
        pair!(self, to, "To", pairs);
        pair!(self, date_sent, "DateSent", pairs);
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
    pub messages: Vec<MsgResp>,
    pub page: usize,
    pub page_size: usize,
    pub uri: String,
    pub next_page_uri: Option<String>,
    pub previous_page_uri: Option<String>,
}
