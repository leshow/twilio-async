macro_rules! execute {
    ($x:ident) => {
        impl<'a> Execute for $x<'a> {
            fn execute<U, D>(
                self,
                method: Method,
                url: U,
                body: Option<String>,
            ) -> Result<(hyper::Headers, hyper::StatusCode, Option<D>), TwilioErr>
            where
                U: AsRef<str>,
                D: for<'de> serde::Deserialize<'de>,
            {
                use {
                    futures::{future, Future, Stream}, hyper::{header, Request}, serde_json,
                };
                const BASE: &str = "https://api.twilio.com/2010-04-01/Accounts/";

                let mut core_ref = self.client.core.try_borrow_mut()?;
                let url =
                    format!("{}/{}/{}", BASE, self.client.sid, url.as_ref()).parse::<hyper::Uri>()?;
                let mut request = Request::new(method, url);

                if let Some(body) = body {
                    request.set_body(body);
                    request
                        .headers_mut()
                        .set(header::ContentType::form_url_encoded());
                }

                request.headers_mut().set(self.client.auth.clone());
                let fut_req = self.client.client.request(request).and_then(|res| {
                    println!("Response: {}", res.status());
                    println!("Headers: \n{}", res.headers());

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
