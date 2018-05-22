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
