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
    inprogress,
    canceled,
    completed,
    failed,
    busy,
    noanswer,
}

#[derive(Deserialize, Debug)]
pub struct CallResp {
    from: String,
    to: String,
    sid: String,
    start_time: String,
    status: CallStatus,
    account_sid: String,
    caller_name: Option<String>,
    duration: String,
    price: String,
    price_unit: String,
    uri: String,
    url: String,
    date_created: String,
    end_time: String,
    phone_number_sid: String,
}

pub struct SendCall<'a> {
    pub call: Call<'a>,
    pub client: &'a Twilio,
}

execute!(SendCall);

impl<'a> TwilioRequest for SendCall<'a> {
    type Resp = CallResp;
    fn send(self) -> TwilioResp<Self::Resp> {
        let call = self.call.to_string();
        self.execute(Method::Post, "Calls.json", Some(call))
    }
}

impl<'a> SendCall<'a> {
    pub fn url(&mut self, url: &'a str) {
        self.call.url = Some(url);
    }
    pub fn sid(&mut self, sid: &'a str) {
        self.call.sid = Some(sid);
    }
    pub fn callerid(&mut self, callerid: &'a str) {
        self.call.callerid = Some(callerid);
    }
    pub fn machine_detection(&mut self, machine_detection: bool) {
        self.call.machine_detection = Some(machine_detection);
    }
    pub fn record(&mut self, record: bool) {
        self.call.record = Some(record);
    }
    pub fn send_digits(&mut self, send_digits: &'a str) {
        self.call.send_digits = Some(send_digits);
    }
    pub fn status_callback(&mut self, callback: &'a str) {
        self.call.status_callback = Some(callback);
    }
    pub fn callback_event(&mut self, event: CallbackEvent) {
        self.call.callback_event = Some(event);
    }
    pub fn timeout(&mut self, timeout: i32) {
        self.call.timeout = Some(timeout);
    }
}
