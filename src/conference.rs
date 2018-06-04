use hyper::{self, Method};
use serde;
use {encode_pairs, Execute, Twilio, TwilioErr, TwilioRequest, TwilioResp};

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
    pub fn end(&'a mut self) {
        self.conference.status = Some(COMPLETED);
    }
}

execute!(GetConference);

impl<'a> TwilioRequest for GetConference<'a> {
    type Resp = ConferenceResp;
    fn send(self) -> TwilioResp<Self::Resp> {
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
    fn send(self) -> TwilioResp<Self::Resp> {
        self.execute(Method::Get, "Conferences.json", None)
    }
}

#[derive(Deserialize, Debug)]
pub struct ListConferencesResp {
    conferences: Vec<ConferenceResp>,
    end: usize,
    account_sid: String,
    start: usize,
    page: usize,
    page_size: usize,
}

#[derive(Deserialize, Debug)]
pub struct ConferenceResp {
    account_sid: String,
    date_created: String,
    date_update: String,
    friendly_name: String,
    region: String,
    sid: String,
    status: String,
    uri: String,
}
