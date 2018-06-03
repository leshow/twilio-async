use encode_pairs;

#[derive(Debug, Default)]
pub struct Call<'a> {
    pub from: &'a str,
    pub to: &'a str,
    pub url: Option<&'a str>,
    pub sid: Option<&'a str>,
}

impl<'a> Call<'a> {
    pub fn new(from: &'a str, to: &'a str) -> Call<'a> {
        Call {
            from,
            to,
            ..Call::default()
        }
    }
}

impl<'a> ToString for Call<'a> {
    fn to_string(&self) -> String {
        let mut pairs = vec![("To", self.to), ("From", self.from)];
        if let Some(url) = self.url {
            pairs.push(("Url", url));
        }
        if let Some(sid) = self.sid {
            pairs.push(("ApplicationSid", body));
        }
        encode_pairs(pairs).unwrap()
    }
}
