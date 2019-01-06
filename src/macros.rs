macro_rules! execute {
    ($x:ident) => {
        impl<'a> Execute for $x<'a> {
            fn execute<U, D>(
                self,
                method: Method,
                url: U,
                body: Option<String>,
            ) -> Result<(http::HeaderMap, hyper::StatusCode, Option<D>), TwilioErr>
            where
                U: AsRef<str>,
                D: for<'de> serde::Deserialize<'de>,
            {
                use futures::{future, Future, Stream};
                // use http::HeaderMap;
                use hyper::{
                    header::{AUTHORIZATION, CONTENT_TYPE},
                    Request,
                };
                use hyperx::header::{Authorization, Headers};
                use serde_json;
                const BASE: &str = "https://api.twilio.com/2010-04-01/Accounts";

                let mut core_ref = self.client.core.try_borrow_mut()?;
                let url = format!("{}/{}/{}", BASE, self.client.sid, url.as_ref())
                    .parse::<hyper::Uri>()?;
                let mut request = Request::builder().method(method).uri(url);

                if let Some(body) = body {
                    // println!("{:?}", body);
                    request.body(body);
                    // let mut headers = HeaderMap::new();
                    // headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse()?);
                    request.header(CONTENT_TYPE, "application/x-www-form-urlencoded".parse()?);
                }
                let mut auth = Headers::new();
                auth.set(self.client.auth.clone());
                request.header(AUTHORIZATION, auth.get().unwrap()); // TODO try_trait

                let fut_req = self.client.client.request(request).and_then(|res| {
                    // println!("Response: {}", res.status());
                    // println!("Headers: \n{}", res.headers());

                    let header = res.headers().clone();
                    let status = res.status();

                    res.body()
                        .fold(Vec::new(), |mut v, chunk| {
                            v.extend(&chunk[..]);
                            future::ok::<_, hyper::Error>(v)
                        })
                        .map(move |chunks| {
                            if chunks.is_empty() {
                                Ok((header, status, None))
                            } else {
                                // println!("{:?}", String::from_utf8(chunks.clone()));
                                Ok((header, status, Some(serde_json::from_slice(&chunks)?)))
                            }
                        })
                });
                core_ref.run(fut_req)?
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
