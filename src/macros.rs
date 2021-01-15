macro_rules! execute {
    ($ty:tt) => {
        #[async_trait]
        impl<'a> Execute for $ty<'a> {
            fn request<U>(
                &self,
                method: Method,
                url: U,
                body: Option<String>,
            ) -> Result<hyper::Request<hyper::Body>, TwilioErr>
            where
                U: AsRef<str>,
            {
                use hyper::{
                    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
                    Request,
                };
                use typed_headers::HeaderMapExt;
                const BASE: &str = "https://api.twilio.com/2010-04-01/Accounts";

                let url = format!("{}/{}/{}", BASE, self.client.sid, url.as_ref())
                    .parse::<hyper::Uri>()?;
                let mut request = Request::builder().method(method).uri(url);

                let mut hmap = HeaderMap::new();
                hmap.typed_insert(&self.client.auth);
                for (k, v) in hmap {
                    request = request.header(k.unwrap().as_str(), v);
                }
                Ok(match body {
                    Some(body) => request
                        .header(
                            CONTENT_TYPE,
                            HeaderValue::from_static("application/x-www-form-urlencoded"),
                        )
                        .body(hyper::Body::from(body))?,
                    None => request.body(hyper::Body::empty())?,
                })
            }

            async fn execute<U, D>(
                &self,
                method: Method,
                url: U,
                body: Option<String>,
            ) -> TwilioResp<crate::TwilioJson<D>>
            where
                U: AsRef<str> + Send,
                D: for<'de> serde::Deserialize<'de>,
            {
                use bytes::buf::Buf;
                use serde_json;

                let req = self.request(method, url, body)?;

                let res = self
                    .client
                    .client
                    .request(req)
                    .await
                    .map_err(TwilioErr::NetworkErr)?;

                let body = hyper::body::aggregate(res).await?;

                let json_resp = serde_json::from_reader(body.reader())?;
                Ok(json_resp)
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
