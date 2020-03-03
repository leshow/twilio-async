use super::{encode_pairs, Execute, Twilio, TwilioErr, TwilioRequest, TwilioResp};
use async_trait::async_trait;
use hyper::{self, Method};
use serde::Deserialize;

#[derive(Debug, Default)]
pub struct Call<'a> {
    from: &'a str,
    to: &'a str,
    url: &'a str,
    sid: Option<&'a str>,
    callerid: Option<&'a str>,
    machine_detection: Option<bool>,
    record: Option<bool>,
    send_digits: Option<&'a str>,
    status_callback: Option<&'a str>,
    callback_event: Option<CallbackEvent>,
    timeout: Option<&'a str>,
}

#[derive(Debug)]
pub enum CallbackEvent {
    Initiated,
    Ringing,
    Answered,
    Completed,
}

use self::CallbackEvent::*;

impl<'a> Call<'a> {
    pub fn new(from: &'a str, to: &'a str, url: &'a str) -> Call<'a> {
        Call {
            from,
            to,
            url,
            ..Call::default()
        }
    }
}

impl<'a> ToString for Call<'a> {
    fn to_string(&self) -> String {
        let mut pairs = vec![("To", self.to), ("From", self.from), ("Url", self.url)];
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
        if let Some(ref cb) = self.callback_event {
            let event = match *cb {
                Initiated => "initiated",
                Ringing => "ringing",
                Answered => "answered",
                Completed => "completed",
            };
            pairs.push(("StatusCallbackEvent", event));
        }
        pair!(self, timeout, "Timeout", pairs);
        pair!(self, send_digits, "SendDigits", pairs);
        pair!(self, status_callback, "StatusCallback", pairs);

        encode_pairs(pairs).unwrap()
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum CallStatus {
    queued,
    ringing,
    #[serde(rename = "in-progress")]
    inprogress,
    canceled,
    completed,
    failed,
    busy,
    #[serde(rename = "no-answer")]
    noanswer,
}

#[derive(Deserialize, Debug)]
pub struct CallResp {
    pub from: String,
    pub to: String,
    pub sid: String,
    pub start_time: Option<String>,
    pub status: CallStatus,
    pub account_sid: String,
    pub caller_name: Option<String>,
    pub duration: Option<i32>,
    pub price: Option<String>,
    pub price_unit: String,
    pub uri: String,
    pub date_created: Option<String>,
    pub end_time: Option<String>,
    pub phone_number_sid: String,
    pub direction: Direction,
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Direction {
    inbound,
    #[serde(rename = "outbound-api")]
    outbound_api,
    #[serde(rename = "outbound-dial")]
    outbound_dial,
    #[serde(rename = "trunking-terminating")]
    trunking_terminating,
    #[serde(rename = "trunking-originating")]
    trunking_originating,
}

pub struct SendCall<'a> {
    pub call: Call<'a>,
    pub client: &'a Twilio,
}

execute!(SendCall);

#[async_trait]
impl<'a> TwilioRequest for SendCall<'a> {
    type Resp = CallResp;

    async fn run(&self) -> TwilioResp<Self::Resp> {
        let call = self.call.to_string();
        self.execute(Method::POST, "Calls.json", Some(call)).await
    }
}

impl<'a> SendCall<'a> {
    pub fn sid(mut self, sid: &'a str) -> SendCall<'a> {
        self.call.sid = Some(sid);
        self
    }

    pub fn callerid(mut self, callerid: &'a str) -> SendCall<'a> {
        self.call.callerid = Some(callerid);
        self
    }

    pub fn machine_detection(mut self, machine_detection: bool) -> SendCall<'a> {
        self.call.machine_detection = Some(machine_detection);
        self
    }

    pub fn record(mut self, record: bool) -> SendCall<'a> {
        self.call.record = Some(record);
        self
    }

    pub fn send_digits(mut self, send_digits: &'a str) -> SendCall<'a> {
        self.call.send_digits = Some(send_digits);
        self
    }

    pub fn status_callback(mut self, callback: &'a str) -> SendCall<'a> {
        self.call.status_callback = Some(callback);
        self
    }

    pub fn callback_event(mut self, event: CallbackEvent) -> SendCall<'a> {
        self.call.callback_event = Some(event);
        self
    }

    pub fn timeout(mut self, timeout: &'a str) -> SendCall<'a> {
        self.call.timeout = Some(timeout);
        self
    }
}
