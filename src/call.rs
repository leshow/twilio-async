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
    pub start_time: String,
    pub status: CallStatus,
    pub account_sid: String,
    pub caller_name: Option<String>,
    pub duration: String,
    pub price: String,
    pub price_unit: String,
    pub uri: String,
    pub url: String,
    pub date_created: String,
    pub end_time: String,
    pub phone_number_sid: String,
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
    pub fn url(self, url: &'a str) -> SendCall<'a> {
        SendCall {
            call: Call {
                url: Some(url),
                ..self.call
            },
            ..self
        }
    }
    pub fn sid(self, sid: &'a str) -> SendCall<'a> {
        SendCall {
            call: Call {
                sid: Some(sid),
                ..self.call
            },
            ..self
        }
    }
    pub fn callerid(self, callerid: &'a str) -> SendCall<'a> {
        SendCall {
            call: Call {
                callerid: Some(callerid),
                ..self.call
            },
            ..self
        }
    }
    pub fn machine_detection(self, machine_detection: bool) -> SendCall<'a> {
        SendCall {
            call: Call {
                machine_detection: Some(machine_detection),
                ..self.call
            },
            ..self
        }
    }
    pub fn record(self, record: bool) -> SendCall<'a> {
        SendCall {
            call: Call {
                record: Some(record),
                ..self.call
            },
            ..self
        }
    }
    pub fn send_digits(self, send_digits: &'a str) -> SendCall<'a> {
        SendCall {
            call: Call {
                send_digits: Some(send_digits),
                ..self.call
            },
            ..self
        }
    }
    pub fn status_callback(self, callback: &'a str) -> SendCall<'a> {
        SendCall {
            call: Call {
                status_callback: Some(callback),
                ..self.call
            },
            ..self
        }
    }
    pub fn callback_event(self, event: CallbackEvent) -> SendCall<'a> {
        SendCall {
            call: Call {
                callback_event: Some(event),
                ..self.call
            },
            ..self
        }
    }
    pub fn timeout(self, timeout: &'a str) -> SendCall<'a> {
        SendCall {
            call: Call {
                timeout: Some(timeout),
                ..self.call
            },
            ..self
        }
    }
}
