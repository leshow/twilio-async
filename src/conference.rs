use super::{encode_pairs, Execute, Twilio, TwilioErr, TwilioJson, TwilioRequest, TwilioResp};
use async_trait::async_trait;
use hyper::{self, Method};
use serde::Deserialize;

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

#[async_trait]
impl<'a> TwilioRequest for GetConference<'a> {
    type Resp = ConferenceResp;

    async fn run(&self) -> TwilioResp<TwilioJson<Self::Resp>> {
        let url = format!("Conferences/{}.json", self.conference.sid);
        match self.conference.status {
            Some(status) => {
                self.execute(
                    Method::POST,
                    url,
                    Some(encode_pairs(&[("Status", status)]).unwrap()),
                )
                .await
            }
            None => self.execute(Method::GET, url, None).await,
        }
    }
}

// GET ALL CONFERENCES
pub struct Conferences<'a> {
    pub client: &'a Twilio,
}

execute!(Conferences);

#[async_trait]
impl<'a> TwilioRequest for Conferences<'a> {
    type Resp = ListConferencesResp;

    async fn run(&self) -> TwilioResp<TwilioJson<Self::Resp>> {
        self.execute(Method::GET, "Conferences.json", None).await
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
