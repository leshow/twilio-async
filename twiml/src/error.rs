use std::{error::Error, fmt, io, string};
use xml::writer;

// Errors
#[derive(Debug)]
pub enum TwimlErr {
    Io(io::Error),
    Utf8Err(string::FromUtf8Error),
    EmitterErr(writer::Error),
}

pub use super::TwimlErr::*;

pub type TwimlResult<T> = Result<T, TwimlErr>;

impl Error for TwimlErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            Io(ref e) => e.source(),
            Utf8Err(ref e) => e.source(),
            EmitterErr(ref e) => e.source(),
        }
    }
}

impl fmt::Display for TwimlErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Io(ref e) => write!(f, "IO Error: {}", e),
            Utf8Err(ref e) => write!(f, "Error converting to utf-8 string: {}", e),
            EmitterErr(ref e) => write!(f, "Error emitting xml: {}", e),
        }
    }
}

impl From<io::Error> for TwimlErr {
    fn from(e: io::Error) -> Self {
        Io(e)
    }
}

impl From<string::FromUtf8Error> for TwimlErr {
    fn from(e: string::FromUtf8Error) -> Self {
        Utf8Err(e)
    }
}

impl From<writer::Error> for TwimlErr {
    fn from(e: writer::Error) -> Self {
        EmitterErr(e)
    }
}
