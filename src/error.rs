use hyper;
use hyper_tls;
use serde_json;
use serde_xml_rs;
use std::{cell, error::Error, fmt, io, string};

// Errors
#[derive(Debug)]
pub enum TwilioErr {
    Io(io::Error),
    TlsErr(hyper_tls::Error),
    UrlParse(hyper::error::UriError),
    NetworkErr(hyper::Error),
    SerdeErr(serde_json::Error),
    BorrowErr(cell::BorrowMutError),
    SerdeXml(serde_xml_rs::Error),
    Utf8Err(string::FromUtf8Error),
}

pub use TwilioErr::*;

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
            SerdeXml(ref e) => e.description(),
            Utf8Err(ref e) => e.description(),
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
            SerdeXml(ref e) => write!(f, "Error serializing XML: {}", e),
            Utf8Err(ref e) => write!(f, "Error converting to utf-8 string: {}", e),
        }
    }
}

from!(cell::BorrowMutError, BorrowErr);
from!(hyper::Error, NetworkErr);
from!(serde_json::Error, SerdeErr);
from!(hyper::error::UriError, UrlParse);
from!(hyper_tls::Error, TlsErr);
from!(io::Error, Io);
from!(serde_xml_rs::Error, SerdeXml);
from!(string::FromUtf8Error, Utf8Err);

#[derive(Deserialize, Serialize, Debug)]
pub struct TwilioErrorResp {
    pub code: i32,
    pub message: String,
}