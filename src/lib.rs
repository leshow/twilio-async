#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate serde_derive;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;
extern crate url;

#[macro_use]
mod macros;
mod message;
mod request;
mod twiliourl;

use self::TwilioErr::*;
use message::*;
use request::*;
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

    pub fn send_msg<'a>(&'a self, from: &'a str, to: &'a str) -> SendMsg<'a> {
        SendMsg {
            msg: Msg::new(from, to),
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
}

pub trait Execute {
    fn execute<U, D>(
        self,
        method: Method,
        url: U,
        body: Option<String>,
    ) -> Result<(hyper::Headers, hyper::StatusCode, Option<D>), TwilioErr>
    where
        U: AsRef<str>,
        D: for<'de> serde::Deserialize<'de>;
}

pub trait TwilioRequest: Execute {
    type Resp: for<'de> serde::Deserialize<'de>;
    fn send(self) -> Result<(hyper::Headers, hyper::StatusCode, Option<Self::Resp>), TwilioErr>;
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

// Errors
#[derive(Debug)]
pub enum TwilioErr {
    Io(io::Error),
    TlsErr(hyper_tls::Error),
    UrlParse(hyper::error::UriError),
    NetworkErr(hyper::Error),
    SerdeErr(serde_json::Error),
    BorrowErr(cell::BorrowMutError),
}

pub type TwilioResult<T> = Result<T, TwilioErr>;

impl Error for TwilioErr {
    fn description(&self) -> &str {
        match *self {
            Io(ref e) => e.description(),
            TlsErr(ref e) => e.description(),
            UrlParse(ref e) => e.description(),
            SerdeErr(ref e) => e.description(),
            NetworkErr(ref e) => e.description(),
            BorrowErr(ref e) => e.description(),
        }
    }
}

impl fmt::Display for TwilioErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Io(ref e) => write!(f, "IO Error: {}", e),
            TlsErr(ref e) => write!(f, "TLS Connection Error: {}", e),
            UrlParse(ref e) => write!(f, "URL parse error: {}", e),
            SerdeErr(ref e) => write!(f, "Serde JSON Error: {}", e),
            NetworkErr(ref e) => write!(f, "There was a network error. {}", e),
            BorrowErr(ref e) => write!(f, "Error trying to get client reference. {}", e),
        }
    }
}

from!(cell::BorrowMutError, BorrowErr);
from!(hyper::Error, NetworkErr);
from!(serde_json::Error, SerdeErr);
from!(hyper::error::UriError, UrlParse);
from!(hyper_tls::Error, TlsErr);
from!(io::Error, Io);
