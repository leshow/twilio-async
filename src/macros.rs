macro_rules! execute {
    ($x:ident) => {
        impl<'a> Execute for $x<'a> {
            fn request<U>(
                &self,
                method: Method,
                url: U,
                body: Option<String>,
            ) -> Result<hyper::Request<hyper::Body>, TwilioErr>
            where
                U: AsRef<str>,
            {
                use http::{header::HeaderValue, Request};
                use hyper::header::{HeaderMap, CONTENT_TYPE};
                use typed_headers::HeaderMapExt;
                const BASE: &str = "https://api.twilio.com/2010-04-01/Accounts";

                let url = format!("{}/{}/{}", BASE, self.client.sid, url.as_ref())
                    .parse::<hyper::Uri>()?;
                let mut request = Request::builder();
                request.method(method).uri(url);

                let mut hmap = HeaderMap::new();
                hmap.typed_insert(&self.client.auth);
                for (k, v) in hmap {
                    request.header(k.unwrap().as_str(), v);
                }
                Ok(match body {
                    Some(body) => {
                        request.header(
                            CONTENT_TYPE,
                            HeaderValue::from_static("application/x-www-form-urlencoded"),
                        );
                        request.body(hyper::Body::from(body))?
                    }
                    None => request.body(hyper::Body::empty())?,
                })
            }

            fn execute<U, D>(self, method: Method, url: U, body: Option<String>) -> TwilioResp<D>
            where
                U: AsRef<str>,
                D: for<'de> serde::Deserialize<'de>,
            {
                use futures::{future, Future, Stream};
                use serde_json;

                #[cfg(feature = "runtime")]
                let mut core_ref = self.client.core.try_borrow_mut()?;
                #[cfg(feature = "runtime")]
                let req = self.request(method, url, body)?;

                #[cfg(not(feature = "runtime"))]
                let req = self.request(method, url, body).unwrap();

                let fut_req = self
                    .client
                    .client
                    .request(req)
                    .and_then(move |res| {
                        // let header = res.headers().clone();
                        let status = res.status();

                        res.into_body()
                            .fold(Vec::new(), |mut v, chunk| {
                                v.extend(&chunk[..]);
                                future::ok::<_, hyper::Error>(v)
                            })
                            .map(move |chunks| {
                                if chunks.is_empty() {
                                    (status, None)
                                } else {
                                    let json_resp = serde_json::from_slice(&chunks).ok();
                                    (status, json_resp)
                                }
                            })
                    })
                    .map_err(TwilioErr::NetworkErr);

                #[cfg(not(feature = "runtime"))]
                return Box::new(fut_req);

                #[cfg(feature = "runtime")]
                return Ok(core_ref.run(fut_req)?);
            }
        }
    };
}

macro_rules! from {
    ($x:ty, $variant:ident) => {
        impl From<$x> for TwilioErr {
            fn from(e: $x) -> Self {
                $variant(e)
            }
        }
    };
}

macro_rules! pair {
    ($x:ident, $field:ident, $name:tt, $vec:ident) => {
        if let Some($field) = $x.$field {
            $vec.push(($name, $field));
        }
    };
}
