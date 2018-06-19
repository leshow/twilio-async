use hyper;
use hyper_tls;
use serde_json;
use std::{cell, error::Error, fmt, io, string};
use xml::writer;

// Errors
#[derive(Debug)]
pub enum TwilioErr {
    Io(io::Error),
    TlsErr(hyper_tls::Error),
    UrlParse(hyper::error::UriError),
    NetworkErr(hyper::Error),
    SerdeErr(serde_json::Error),
    BorrowErr(cell::BorrowMutError),
    Utf8Err(string::FromUtf8Error),
    EmitterErr(writer::Error),
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
            Utf8Err(ref e) => e.description(),
            EmitterErr(ref e) => e.description(),
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
            Utf8Err(ref e) => write!(f, "Error converting to utf-8 string: {}", e),
            EmitterErr(ref e) => write!(f, "Error emitting xml: {}", e),
        }
    }
}

from!(cell::BorrowMutError, BorrowErr);
from!(hyper::Error, NetworkErr);
from!(serde_json::Error, SerdeErr);
from!(hyper::error::UriError, UrlParse);
from!(hyper_tls::Error, TlsErr);
from!(io::Error, Io);
from!(string::FromUtf8Error, Utf8Err);
from!(writer::Error, EmitterErr);
