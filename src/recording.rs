use super::{Execute, Twilio, TwilioErr, TwilioRequest, TwilioResp};
use async_trait::async_trait;
use hyper::{self, Method};
use serde::Deserialize;

#[derive(Debug, Default)]
pub struct Recording<'a> {
    sid: &'a str,
}

impl<'a> Recording<'a> {
    pub fn new(sid: &'a str) -> Recording<'a> {
        Recording { sid }
    }
}

// GET ONE Recording
pub struct GetRecording<'a> {
    pub recording: Recording<'a>,
    pub client: &'a Twilio,
}

execute!(GetRecording);

#[async_trait]
impl<'a> TwilioRequest for GetRecording<'a> {
    type Resp = RecordingResp;

    async fn run(&self) -> TwilioResp<Self::Resp> {
        let url = format!("Recordings/{}.json", self.recording.sid);
        self.execute(Method::GET, url, None).await
    }
}

impl<'a> GetRecording<'a> {
    pub async fn delete(&self) -> TwilioResp<Option<bool>> {
        let url = format!("Recordings/{}.json", self.recording.sid);
        self.execute(Method::DELETE, url, None).await
    }
}

// GET ALL RECORDINGS
pub struct Recordings<'a> {
    pub client: &'a Twilio,
}

execute!(Recordings);

#[async_trait]
impl<'a> TwilioRequest for Recordings<'a> {
    type Resp = ListRecordingResp;

    async fn run(&self) -> TwilioResp<Self::Resp> {
        self.execute(Method::GET, "Recordings.json", None).await
    }
}

impl<'a> Recordings<'a> {
    pub async fn for_call(&self, call_sid: &'a str) -> TwilioResp<ListRecordingResp> {
        let url = format!("Recordings.json?CallSid={}", call_sid);
        self.execute(Method::GET, url, None).await
    }

    pub async fn created(&self, date_created: &'a str) -> TwilioResp<ListRecordingResp> {
        let url = format!("Recordings.json?DateCreated={}", date_created);
        self.execute(Method::GET, url, None).await
    }

    pub async fn range(&self, before: &'a str, after: &'a str) -> TwilioResp<ListRecordingResp> {
        let url = format!(
            "Recordings.json?DateCreatedBefore={}&DateCreatedAfter={}",
            before, after
        );
        self.execute(Method::GET, url, None).await
    }
}

#[derive(Deserialize, Debug)]
pub struct ListRecordingResp {
    pub recordings: Vec<RecordingResp>,
    pub end: usize,
    pub account_sid: String,
    pub start: usize,
    pub page: usize,
    pub page_size: usize,
}

#[derive(Deserialize, Debug)]
pub struct RecordingResp {
    pub account_sid: String,
    pub call_sid: String,
    pub channels: String,
    pub conference_sid: String,
    pub date_created: String,
    pub date_updated: String,
    pub end_time: String,
    pub start_time: String,
    pub price: String,
    pub price_unit: String,
    pub duration: String,
    pub encryption_details: EncryptionDetails,
    pub error_code: String,
    pub uri: String,
    pub status: RecordingStatus,
}

#[derive(Deserialize, Debug)]
pub struct EncryptionDetails {
    pub encryption_public_key_sid: String,
    pub encryption_cek: String,
    pub encryption_iv: String,
}

#[derive(Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum RecordingStatus {
    #[serde(rename = "in-progress")]
    inprogress,
    paused,
    stopped,
    processing,
    completed,
    failed,
}
