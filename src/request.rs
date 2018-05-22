use message::Msg;
use RequestType::*;

use std::borrow::Borrow;
use url::form_urlencoded;

pub enum RequestType<'a> {
    Message(Msg<'a>),
}

impl<'a> ToString for RequestType<'a> {
    fn to_string(&self) -> String {
        match *self {
            Message(Msg { to, from, body }) => {
                encode_pairs([("To", to), ("From", from), ("Body", body)].iter()).unwrap()
            }
        }
    }
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
