#![allow(dead_code)]
#[macro_use]
extern crate serde_derive;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate serde_json;
extern crate serde_xml_rs;
extern crate tokio_core;
extern crate url;

#[macro_use]
mod macros;
mod call;
mod conference;
pub mod error;
mod message;
mod recording;
pub mod twiml;

pub use call::*;
pub use conference::*;
pub use error::*;
pub use message::*;
pub use recording::*;

pub use {
    futures::{future, Future, Stream},
    hyper::{
        client::HttpConnector, header::{self, Authorization, Basic}, Client, Method, Request,
    },
    hyper_tls::HttpsConnector, std::{borrow::Borrow, error::Error, fmt, io},
    std::{
        cell::{self, RefCell}, rc::Rc,
    }, tokio_core::reactor::Core,
    url::form_urlencoded,
};

#[derive(Debug)]
pub struct Twilio {
    sid: String,
    auth: Authorization<Basic>,
    client: Rc<Client<HttpsConnector<HttpConnector>, hyper::Body>>,
    core: Rc<RefCell<Core>>,
}

pub type TwilioResp<T> = Result<(hyper::Headers, hyper::StatusCode, Option<T>), TwilioErr>;

impl Twilio {
    pub fn new<S>(sid: S, token: S) -> TwilioResult<Twilio>
    where
        S: Into<String>,
    {
        let core = Core::new()?;
        let handle = core.handle();
        let username = sid.into();
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &handle)?)
            .build(&handle);

        Ok(Twilio {
            sid: username.clone(),
            auth: Authorization(Basic {
                username,
                password: Some(token.into()),
            }),
            client: Rc::new(client),
            core: Rc::new(RefCell::new(core)),
        })
    }

    pub fn send_msg<'a>(&'a self, from: &'a str, to: &'a str, body: &'a str) -> SendMsg<'a> {
        SendMsg {
            msg: Msg::new(from, to, body),
            client: &self,
        }
    }

    pub fn msg<'a>(&'a self, message_sid: &'a str) -> GetMessage<'a> {
        GetMessage {
            message_sid,
            client: &self,
        }
    }

    pub fn msgs<'a>(&'a self) -> Messages<'a> {
        Messages { client: &self }
    }

    pub fn call<'a>(&'a self, from: &'a str, to: &'a str, url: &'a str) -> SendCall<'a> {
        SendCall {
            call: Call::new(from, to, url),
            client: &self,
        }
    }

    pub fn conference<'a>(&'a self, sid: &'a str) -> GetConference<'a> {
        GetConference {
            conference: Conference::new(sid),
            client: &self,
        }
    }

    pub fn conferences<'a>(&'a self) -> Conferences<'a> {
        Conferences { client: &self }
    }

    pub fn recording<'a>(&'a self, sid: &'a str) -> GetRecording<'a> {
        GetRecording {
            recording: Recording::new(sid),
            client: &self,
        }
    }

    pub fn recordings<'a>(&'a self) -> Recordings<'a> {
        Recordings { client: &self }
    }
}

pub trait Execute {
    fn execute<U, D>(self, method: Method, url: U, body: Option<String>) -> TwilioResp<D>
    where
        U: AsRef<str>,
        D: for<'de> serde::Deserialize<'de>;
}

pub trait TwilioRequest: Execute {
    type Resp: for<'de> serde::Deserialize<'de>;
    fn send(self) -> TwilioResp<Self::Resp>;
}

pub fn encode_pairs<I, K, V>(pairs: I) -> Option<String>
where
    K: AsRef<str>,
    V: AsRef<str>,
    I: IntoIterator,
    I::Item: Borrow<(K, V)>,
{
    let mut partial = form_urlencoded::Serializer::new(String::new());
    for pair in pairs {
        let &(ref k, ref v) = pair.borrow();
        partial.append_pair(k.as_ref(), v.as_ref());
    }
    let encoded = partial.finish();
    Some(encoded)
}

pub fn url_encode<I, K, V>(pairs: I) -> String
where
    K: AsRef<str>,
    V: AsRef<str>,
    I: IntoIterator,
    I::Item: Borrow<(K, V)>,
{
    pairs
        .into_iter()
        .map(|pair| {
            let &(ref k, ref v) = pair.borrow();
            format!("{}={}", k.as_ref(), v.as_ref())
        })
        .fold(String::new(), |mut acc, item| {
            acc.push_str(&item);
            acc.push_str("&");
            acc
        })
}
