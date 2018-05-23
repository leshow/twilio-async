pub struct Call<'a> {
    pub from: &'a str,
    pub to: &'a str,
    pub url: &'a str,
}

impl<'a> Call<'a> {
    pub fn new(from: &'a str, to: &'a str, url: &'a str) -> Call<'a> {
        Call { from, to, url }
    }
}

impl<'a> ToString for Call<'a> {
    fn to_string(&self) -> String {
        super::encode_pairs(&[("To", self.to), ("From", self.from), ("Url", self.url)]).unwrap()
    }
}
