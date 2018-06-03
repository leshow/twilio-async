use hyper::{self, Method};
use serde;
use {encode_pairs, Execute, Twilio, TwilioErr, TwilioRequest, TwilioResp};

#[derive(Debug, Default)]
pub struct Call<'a> {
    from: &'a str,
    to: &'a str,
    url: Option<&'a str>,
    sid: Option<&'a str>,
    callerid: Option<&'a str>,
    machine_detection: Option<bool>,
    record: Option<bool>,
    send_digits: Option<&'a str>,
    status_callback: Option<&'a str>,
    callback: Option<CallbackEvent>,
    timeout: Option<i32>,
}

#[derive(Debug)]
enum CallbackEvent {
    Initiated,
    Ringing,
    Answered,
    Completed,
}

use self::CallbackEvent::*;
impl ToString for CallbackEvent {
    fn to_string(&self) -> String {
        match *self {
            Initiated => "initiated".to_string(),
            Ringing => "ringing".to_string(),
            Answered => "answered".to_string(),
            Completed => "completed".to_string(),
        }
    }
}

impl<'a> Call<'a> {
    pub fn new(from: &'a str, to: &'a str) -> Call<'a> {
        Call {
            from,
            to,
            ..Call::default()
        }
    }
}

impl<'a> ToString for Call<'a> {
    fn to_string(&self) -> String {
        let mut pairs = vec![("To", self.to), ("From", self.from)];
        pair!(self, url, "Url", pairs);
        pair!(self, sid, "ApplicationSid", pairs);
        pair!(self, callerid, "CallerId", pairs);
        if let Some(detection) = self.machine_detection {
            if detection {
                pairs.push(("MachineDetection", "Enable"));
            }
        }
        if let Some(record) = self.record {
            if record {
                pairs.push(("Record", "true"));
            }
        }

        encode_pairs(pairs).unwrap()
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum CallStatus {
    queued,
    ringing,
    inprogress,
    canceled,
    completed,
    failed,
    busy,
    noanswer,
}
