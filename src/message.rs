use hyper::{self, Method};
use serde;
use {encode_pairs, url_encode, Execute, Twilio, TwilioErr, TwilioRequest, TwilioResp};

#[derive(Default, Debug)]
pub struct Msg<'a> {
    from: &'a str,
    to: &'a str,
    body: Option<&'a str>,
    media_url: Option<&'a str>,
}

impl<'a> Msg<'a> {
    pub fn new(from: &'a str, to: &'a str) -> Msg<'a> {
        Msg {
            from,
            to,
            ..Msg::default()
        }
    }
}

impl<'a> ToString for Msg<'a> {
    fn to_string(&self) -> String {
        let mut pairs = vec![("To", self.to), ("From", self.from)];
        pair!(self, media_url, "MediaUrl", pairs);
        pair!(self, body, "Body", pairs);
        encode_pairs(pairs).unwrap()
        // match self.media_url {
        //     Some(m_url) => encode_pairs(&[
        //         ("To", self.to),
        //         ("From", self.from),
        //         ("Body", self.body),
        //         ("MediaUrl", m_url),
        //     ]).unwrap(),
        //     None => {
        //         encode_pairs(&[("To", self.to), ("From", self.from), ("Body", self.body)]).unwrap()
        //     }
        // }
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
    from: String,
    to: String,
    body: Option<String>,
    sid: String,
    status: Option<MsgStatus>,
    media_url: String,
    price: String,
    uri: String,
    date_created: String,
    date_sent: String,
    date_updated: String,
}

// for outbound sms
#[derive(Debug)]
pub struct SendMsg<'a> {
    pub msg: Msg<'a>,
    pub client: &'a Twilio,
}

impl<'a> SendMsg<'a> {
    pub fn media(&mut self, media_url: &'a str) {
        self.msg.media_url = Some(media_url);
    }

    pub fn body(&mut self, body: &'a str) {
        self.msg.body = Some(body);
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
        let msg_sid = format!("{}.json", self.message_sid);
        self.execute(Method::Get, msg_sid, None)
    }
}

impl<'a> GetMessage<'a> {
    pub fn redact(self) -> TwilioResp<MsgResp> {
        let msg_sid = format!("{}.json", self.message_sid);
        self.execute(Method::Post, msg_sid, Some("Body=".into()))
    }
    pub fn media(self) -> TwilioResp<MediaResp> {
        let msg_sid = format!("Messages/{}/Media.json", self.message_sid);
        self.execute(Method::Get, msg_sid, None)
    }
}

#[derive(Debug, Deserialize)]
pub struct MediaResp {
    media_list: Vec<MediaItem>,
    num_pages: i32,
    page: i32,
    page_size: i32,
    start: i32,
    total: i32,
    uri: String,
    account_sid: String,
    message_sid: String,
}

#[derive(Debug, Deserialize)]
pub struct MediaItem {
    account_sid: String,
    content_type: String,
    sid: String,
    uri: String,
    message_sid: String,
    date_created: String,
    date_update: String,
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
    msgs: Vec<MsgResp>,
    num_pages: usize,
    page: usize,
    page_size: usize,
    total: usize,
    uri: String,
}
