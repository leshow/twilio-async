use super::{encode_pairs, Execute, Twilio, TwilioErr, TwilioRequest, TwilioResp};
use hyper::{self, Method};
use serde;

#[derive(Debug, Default)]
pub struct Conference<'a> {
    sid: &'a str,
    status: Option<&'a str>,
}

const COMPLETED: &str = "completed";

impl<'a> Conference<'a> {
    pub fn new(sid: &'a str) -> Conference<'a> {
        Conference { sid, status: None }
    }
}

// GET ONE CONFERENCE
pub struct GetConference<'a> {
    pub conference: Conference<'a>,
    pub client: &'a Twilio,
}

impl<'a> GetConference<'a> {
    pub fn end(mut self) -> GetConference<'a> {
        self.conference.status = Some(COMPLETED);
        self
    }
}

execute!(GetConference);

impl<'a> TwilioRequest for GetConference<'a> {
    type Resp = ConferenceResp;

    fn run(self) -> TwilioResp<Self::Resp> {
        let url = format!("Conferences/{}.json", self.conference.sid);
        match self.conference.status {
            Some(status) => self.execute(
                Method::Post,
                url,
                Some(encode_pairs(&[("Status", status)]).unwrap()),
            ),
            None => self.execute(Method::Get, url, None),
        }
    }
}

// GET ALL CONFERENCES
pub struct Conferences<'a> {
    pub client: &'a Twilio,
}

execute!(Conferences);

impl<'a> TwilioRequest for Conferences<'a> {
    type Resp = ListConferencesResp;

    fn run(self) -> TwilioResp<Self::Resp> {
        self.execute(Method::Get, "Conferences.json", None)
    }
}

#[derive(Deserialize, Debug)]
pub struct ListConferencesResp {
    pub conferences: Vec<ConferenceResp>,
    pub end: usize,
    pub next_page_uri: Option<String>,
    pub previous_page_uri: Option<String>,
    pub uri: String,
    pub start: usize,
    pub page: usize,
    pub page_size: usize,
}

#[derive(Deserialize, Debug)]
pub struct ConferenceResp {
    pub account_sid: String,
    pub date_created: Option<String>,
    pub date_updated: String,
    pub friendly_name: String,
    pub region: String,
    pub sid: String,
    pub status: String,
    pub uri: String,
}
