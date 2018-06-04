use hyper::{self, Method};
use serde;
use {encode_pairs, Execute, Twilio, TwilioErr, TwilioRequest, TwilioResp};

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

impl<'a> TwilioRequest for GetRecording<'a> {
    type Resp = RecordingResp;
    fn send(self) -> TwilioResp<Self::Resp> {
        let url = format!("Recordings/{}.json", self.recording.sid);
        self.execute(Method::Get, url, None)
    }
}

impl<'a> GetRecording<'a> {
    pub fn delete(self) -> TwilioResp<Option<bool>> {
        let url = format!("Recordings/{}.json", self.recording.sid);
        self.execute(Method::Delete, url, None)
    }
}

// GET ALL RECORDINGS
pub struct Recordings<'a> {
    pub client: &'a Twilio,
}

execute!(Recordings);

impl<'a> TwilioRequest for Recordings<'a> {
    type Resp = ListRecordingResp;
    fn send(self) -> TwilioResp<Self::Resp> {
        self.execute(Method::Get, "Recordings.json", None)
    }
}

impl<'a> Recordings<'a> {
    pub fn for_call(self, call_sid: &'a str) -> TwilioResp<ListRecordingResp> {
        let url = format!("Recordings.json?CallSid={}", call_sid);
        self.execute(Method::Get, url, None)
    }
    pub fn created(self, date_created: &'a str) -> TwilioResp<ListRecordingResp> {
        let url = format!("Recordings.json?DateCreated={}", date_created);
        self.execute(Method::Get, url, None)
    }
    pub fn range(self, before: &'a str, after: &'a str) -> TwilioResp<ListRecordingResp> {
        let url = format!(
            "Recordings.json?DateCreatedBefore={}&DateCreatedAfter={}",
            before, after
        );
        self.execute(Method::Get, url, None)
    }
}

#[derive(Deserialize, Debug)]
pub struct ListRecordingResp {
    recordings: Vec<RecordingResp>,
    end: usize,
    account_sid: String,
    start: usize,
    page: usize,
    page_size: usize,
}

#[derive(Deserialize, Debug)]
pub struct RecordingResp {
    account_sid: String,
    call_sid: String,
    channels: String,
    conference_sid: String,
    date_created: String,
    date_updated: String,
    end_time: String,
    start_time: String,
    price: String,
    price_unit: String,
    duration: String,
    encryption_details: EncryptionDetails,
    error_code: String,
    uri: String,
    status: RecordingStatus,
}

#[derive(Deserialize, Debug)]
pub struct EncryptionDetails {
    encryption_public_key_sid: String,
    encryption_cek: String,
    encryption_iv: String,
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
