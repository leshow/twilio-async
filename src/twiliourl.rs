use std::borrow::Borrow;
use url;

pub struct Url {
    pub base: String,
    pub query: Option<String>,
}
impl Url {
    pub fn new<S: Into<String>>(base: S) -> Self {
        Url {
            base: base.into(),
            query: None,
        }
    }
    pub fn set_query_pairs<I, K, V>(&mut self, pairs: I)
    where
        K: AsRef<str>,
        V: AsRef<str>,
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
    {
        let mut partial = url::form_urlencoded::Serializer::new(String::new());
        for pair in pairs.into_iter() {
            let &(ref k, ref v) = pair.borrow();
            partial.append_pair(k.as_ref(), v.as_ref());
        }
        let encoded = partial.finish();
        // let joined: String = pairs
        //     .into_iter()
        //     .fold(Vec::new(), |mut acc, pair| {
        //         let (k, v) = pair.borrow();
        //         acc.push(format!("{}={}", k.as_ref(), v.as_ref()));
        //         acc
        //     })
        //     .join("&");
        self.query = Some(encoded);
    }
}

impl ToString for Url {
    fn to_string(&self) -> String {
        match self.query {
            Some(ref q) => format!("{}?{}", self.base, q),
            None => format!("{}", self.base),
        }
    }
}
