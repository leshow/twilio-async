#[macro_use]
extern crate serde_derive;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;
extern crate url;

use self::TwilioErr::*;
use hyper::{
    client::HttpConnector, header::{Authorization, Basic}, Client, Method,
};
use hyper_tls::HttpsConnector;
use std::{borrow::Borrow, error::Error, fmt, io};
use tokio_core::reactor::Handle;
mod twiliourl;

pub struct Twilio {
    sid: String,
    token: String,
    auth: Authorization<Basic>,
    client: Client<HttpsConnector<HttpConnector>, hyper::Body>,
}

impl Twilio {
    pub fn new<S>(sid: S, token: S, handle: &Handle) -> TwilioResult<Twilio>
    where
        S: Into<String>,
    {
        let username = sid.into();
        let pwd = token.into();
        let client = Client::configure()
            .connector(HttpsConnector::new(1, &handle)?)
            .build(&handle);

        Ok(Twilio {
            sid: username.clone(),
            token: pwd.clone(),
            auth: Authorization(Basic {
                username,
                password: Some(pwd),
            }),
            client,
        })
    }

    pub fn authenticate<T>(&self) -> TwilioResult<()>
    where
        T: serde::ser::Serialize,
    {
        Ok(())
    }

    pub fn request<K, V, I>(
        &self,
        method: Method,
        url: url::Url,
        params: I,
    ) -> Result<(), TwilioErr>
    where
        K: AsRef<str>,
        V: AsRef<str>,
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
    {
        unimplemented!();
    }
}

// Errors
#[derive(Debug)]
pub enum TwilioErr {
    Io(io::Error),
    TlsErr(hyper_tls::Error),
}

pub type TwilioResult<T> = Result<T, TwilioErr>;

impl Error for TwilioErr {
    fn description(&self) -> &str {
        match *self {
            Io(ref e) => e.description(),
            TlsErr(ref e) => e.description(),
        }
    }
}

impl fmt::Display for TwilioErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Io(ref e) => write!(f, "IO Error: {}", e),
            TlsErr(ref e) => write!(f, "TLS Connection Error: {}", e),
        }
    }
}

impl From<hyper_tls::Error> for TwilioErr {
    fn from(e: hyper_tls::Error) -> Self {
        TlsErr(e)
    }
}
