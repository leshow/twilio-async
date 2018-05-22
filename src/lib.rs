#[macro_use]
extern crate serde_derive;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;
extern crate url;

mod message;
mod request;
mod twiliourl;

use self::TwilioErr::*;
use message::*;
use request::*;
use {
    futures::{future, Future, Stream},
    hyper::{
        client::HttpConnector, header::{Authorization, Basic, ContentType}, Client, Method, Request,
    },
    hyper_tls::HttpsConnector, std::{borrow::Borrow, error::Error, fmt, io},
    tokio_core::reactor::{Core, Handle}, url::Url,
};

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

    pub fn send_msg(&self, msg: Msg) -> TwilioResult<MsgResp> {
        unimplemented!()
    }

    pub fn request<U, K, D, V, I>(
        &self,
        mut core: Core,
        method: Method,
        url: U,
        t_type: RequestType,
    ) -> Result<D, TwilioErr>
    where
        U: AsRef<str>,
        K: AsRef<str>,
        V: AsRef<str>,
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        D: serde::de::DeserializeOwned,
    {
        let url = url.as_ref().parse::<hyper::Uri>()?;
        let body = t_type.to_string();
        let content_type_header = hyper::header::ContentType::form_url_encoded();
        let mut request = Request::new(method, url);
        request.set_body(body);
        request.headers_mut().set(content_type_header);
        let fut_req = self.client.request(request).and_then(|res| {
            println!("Response: {}", res.status());
            println!("Headers: \n{}", res.headers());

            res.body()
                .fold(Vec::new(), |mut v, chunk| {
                    v.extend(&chunk[..]);
                    future::ok::<_, hyper::Error>(v)
                })
                .and_then(|chunks| {
                    let s = String::from_utf8(chunks).unwrap();
                    future::ok::<_, hyper::Error>(s)
                })
        });
        match core.run(fut_req) {
            Ok(res) => {
                println!("{}", res);
                Ok(serde_json::from_str(&res)?)
            }
            Err(_) => Err(TwilioErr::RequestErr),
        }
    }
}

// Errors
#[derive(Debug)]
pub enum TwilioErr {
    Io(io::Error),
    TlsErr(hyper_tls::Error),
    UrlParse(hyper::error::UriError),
    RequestErr,
    SerdeErr(serde_json::Error),
}

pub type TwilioResult<T> = Result<T, TwilioErr>;

impl Error for TwilioErr {
    fn description(&self) -> &str {
        match *self {
            Io(ref e) => e.description(),
            TlsErr(ref e) => e.description(),
            UrlParse(ref e) => e.description(),
            SerdeErr(ref e) => e.description(),
            RequestErr => "Request Error",
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
            RequestErr => write!(f, "There was a network error"),
        }
    }
}

impl From<hyper_tls::Error> for TwilioErr {
    fn from(e: hyper_tls::Error) -> Self {
        TlsErr(e)
    }
}

impl From<hyper::error::UriError> for TwilioErr {
    fn from(e: hyper::error::UriError) -> Self {
        UrlParse(e)
    }
}
impl From<serde_json::Error> for TwilioErr {
    fn from(e: serde_json::Error) -> Self {
        SerdeErr(e)
    }
}
