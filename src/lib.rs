extern crate hyper;
extern crate url;

use std::error::Error;
use std::fmt;
use std::io;

use self::TwilioErr::*;

pub struct Twilio {
    sid: String,
    token: String,
}

impl Twilio {
    pub fn new<S: Into<String>>(sid: S, token: S) -> Self {
        Twilio {
            sid: sid.into(),
            token: token.into(),
        }
    }
    pub fn authenticate() -> Result<(), TwilioErr> {
        unimplemented!();
    }
}

// Errors
#[derive(Debug)]
pub enum TwilioErr {
    Io(io::Error),
}

pub type TwilioResult<T> = Result<T, TwilioErr>;

impl Error for TwilioErr {
    fn description(&self) -> &str {
        match *self {
            Io(ref e) => e.description(),
        }
    }
    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for TwilioErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Io(ref e) => write!(f, "IO Error: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
