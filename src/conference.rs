use hyper::{self, Method};
use serde;
use {encode_pairs, Execute, Twilio, TwilioErr, TwilioRequest, TwilioResp};

#[derive(Debug, Default)]
pub struct Conference<'a> {
    sid: &'a str,
    status: Option<&'a str>,
}

const COMPLETED: &str = "completed";

impl<'a> Conference<'a> {
    pub fn new(sid: &'a str) -> Conference<'a> {
        Conference { sid, status: None }
    }
}

// GET ONE CONFERENCE
pub struct GetConference<'a> {
    pub conference: Conference<'a>,
    pub client: &'a Twilio,
}

impl<'a> GetConference<'a> {
    pub fn end(mut self) -> GetConference<'a> {
        self.conference.status = Some(COMPLETED);
        self
    }
}

execute!(GetConference);

impl<'a> TwilioRequest for GetConference<'a> {
    type Resp = ConferenceResp;
    fn run(self) -> TwilioResp<Self::Resp> {
        let url = format!("Conferences/{}.json", self.conference.sid);
        match self.conference.status {
            Some(status) => self.execute(
                Method::Post,
                url,
                Some(encode_pairs(&[("Status", status)]).unwrap()),
            ),
            None => self.execute(Method::Get, url, None),
        }
    }
}
use futures::{future, Future, Stream};
impl<'a> GetConference<'a> {
    fn fut<U, D>(
        self,
        method: Method,
        url: U,
        body: Option<String>,
    ) -> Result<
        impl Future<Item = (hyper::Headers, hyper::StatusCode, Option<D>), Error = hyper::Error>,
        TwilioErr,
    >
    where
        U: AsRef<str>,
        D: for<'de> serde::Deserialize<'de>,
    {
        use {
            futures::{future, Future, Stream},
            hyper::{header, Request},
            serde_json,
        };
        const BASE: &str = "https://api.twilio.com/2010-04-01/Accounts";

        let mut core_ref = self.client.core.try_borrow_mut()?;
        let url = format!("{}/{}/{}", BASE, self.client.sid, url.as_ref()).parse::<hyper::Uri>()?;
        // println!("{:?}", url);
        let mut request = Request::new(method, url);

        if let Some(body) = body {
            // println!("{:?}", body);
            request.set_body(body);
            request
                .headers_mut()
                .set(header::ContentType::form_url_encoded());
        }
        // println!("{:?}", request);

        request.headers_mut().set(self.client.auth.clone());
        Ok(self.client.client.request(request).and_then(|res| {
            // println!("Response: {}", res.status());
            // println!("Headers: \n{}", res.headers());

            let header = res.headers().clone();
            let status = res.status();

            res.body()
                .fold(Vec::new(), |mut v, chunk| {
                    v.extend(&chunk[..]);
                    future::ok::<_, hyper::Error>(v)
                }).map(move |chunks| {
                    if chunks.is_empty() {
                        Ok((header, status, None))
                    } else {
                        // println!("{:?}", String::from_utf8(chunks.clone()));
                        Ok((header, status, Some(serde_json::from_slice(&chunks)?)))
                    }
                })
        }))
    }
}

// GET ALL CONFERENCES
pub struct Conferences<'a> {
    pub client: &'a Twilio,
}

execute!(Conferences);

impl<'a> TwilioRequest for Conferences<'a> {
    type Resp = ListConferencesResp;
    fn run(self) -> TwilioResp<Self::Resp> {
        self.execute(Method::Get, "Conferences.json", None)
    }
}

#[derive(Deserialize, Debug)]
pub struct ListConferencesResp {
    pub conferences: Vec<ConferenceResp>,
    pub end: usize,
    pub next_page_uri: Option<String>,
    pub previous_page_uri: Option<String>,
    pub uri: String,
    pub start: usize,
    pub page: usize,
    pub page_size: usize,
}

#[derive(Deserialize, Debug)]
pub struct ConferenceResp {
    pub account_sid: String,
    pub date_created: Option<String>,
    pub date_updated: String,
    pub friendly_name: String,
    pub region: String,
    pub sid: String,
    pub status: String,
    pub uri: String,
}
