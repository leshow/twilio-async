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
    encryption_details: String,
    error_code: String,
    uri: String,
    status: RecordingStatus,
}

#[derive(Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum RecordingStatus {
    #[serde(rename = "in-progress")]
    in_progress,
    paused,
    stopped,
    processing,
    completed,
    failed,
}
