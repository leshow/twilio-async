use message::Msg;
use RequestType::*;

pub enum RequestType<'a> {
    Message(Msg<'a>),
}

impl<'a> ToString for RequestType<'a> {
    fn to_string(&self) -> String {
        match *self {
            Message(Msg { to, from, body }) => {
                super::encode_pairs(&[("To", to), ("From", from), ("Body", body)]).unwrap()
            }
        }
    }
}
