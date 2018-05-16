extern crate hyper;
extern crate native_tls;
extern crate tokio_tls;
extern crate url;

use self::TwilioErr::*;
use hyper::{header::{Authorization, Basic},
            Method};
use std::{borrow::Borrow, error::Error, fmt, io};
mod twiliourl;

pub struct Twilio {
    sid: String,
    token: String,
    auth: Authorization<Basic>,
}

impl Twilio {
    pub fn new<S: Into<String>>(sid: S, token: S) -> Self {
        let username = sid.into();
        let pwd = token.into();
        Twilio {
            sid: username.clone(),
            token: pwd.clone(),
            auth: Authorization(Basic {
                username,
                password: Some(pwd),
            }),
        }
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
