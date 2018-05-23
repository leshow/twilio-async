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
use {
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

pub struct Twilio {
    sid: String,
    token: String,
    auth: Authorization<Basic>,
    client: Rc<Client<HttpsConnector<HttpConnector>, hyper::Body>>,
    core: Rc<RefCell<Core>>,
}

impl Twilio {
    pub fn new<S>(sid: S, token: S) -> TwilioResult<Twilio>
    where
        S: Into<String>,
    {
        let core = Core::new()?;
        let handle = core.handle();
        let username = sid.into();
        let pwd = token.into();
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &handle)?)
            .build(&handle);

        Ok(Twilio {
            sid: username.clone(),
            token: pwd.clone(),
            auth: Authorization(Basic {
                username,
                password: Some(pwd),
            }),
            client: Rc::new(client),
            core: Rc::new(RefCell::new(core)),
        })
    }

    pub fn message<'a>(&'a self, msg: Msg<'a>) -> SendMsg<'a> {
        SendMsg { msg, client: &self }
    }

    // fn request<U, D>(
    //     &self,
    //     method: Method,
    //     url: U,
    //     t_type: RequestType,
    // ) -> Result<(hyper::Headers, hyper::StatusCode, Option<D>), TwilioErr>
    // where
    //     U: AsRef<str>,
    //     D: serde::de::DeserializeOwned,
    // {
    //     let mut core_ref = self.core.try_borrow_mut()?;
    //     let url = format!("{}/{}/{}.json", BASE, self.sid, url.as_ref()).parse::<hyper::Uri>()?;
    //     let content_type_header = header::ContentType::form_url_encoded();
    //     let mut request = Request::new(method, url);
    //     request.set_body(t_type.to_string());
    //     request.headers_mut().set(content_type_header);
    //     request.headers_mut().set(self.auth.clone());
    //     let fut_req = self.client.request(request).and_then(|res| {
    //         println!("Response: {}", res.status());
    //         println!("Headers: \n{}", res.headers());

    //         let header = res.headers().clone();
    //         let status = res.status();

    //         res.body()
    //             .fold(Vec::new(), |mut v, chunk| {
    //                 v.extend(&chunk[..]);
    //                 future::ok::<_, hyper::Error>(v)
    //             })
    //             .map(move |chunks| {
    //                 if chunks.is_empty() {
    //                     Ok((header, status, None))
    //                 } else {
    //                     Ok((header, status, Some(serde_json::from_slice(&chunks)?)))
    //                 }
    //             })
    //     });

    //     core_ref.run(fut_req)?
    // }
}

pub trait Execute {
    fn execute<U, D>(
        self,
        method: Method,
        url: U,
        t_type: String,
    ) -> Result<(hyper::Headers, hyper::StatusCode, Option<D>), TwilioErr>
    where
        U: AsRef<str>,
        D: serde::de::DeserializeOwned;
}

pub trait TwilioReq: Execute {
    fn send<D>(self) -> Result<(hyper::Headers, hyper::StatusCode, Option<D>), TwilioErr>
    where
        D: serde::de::DeserializeOwned;
}

pub fn encode_pairs<I, K, V>(pairs: I) -> Option<String>
where
    K: AsRef<str>,
    V: AsRef<str>,
    I: IntoIterator,
    I::Item: Borrow<(K, V)>,
{
    let mut partial = form_urlencoded::Serializer::new(String::new());
    for pair in pairs.into_iter() {
        let &(ref k, ref v) = pair.borrow();
        partial.append_pair(k.as_ref(), v.as_ref());
    }
    let encoded = partial.finish();
    Some(encoded)
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
