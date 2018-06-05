#[derive(Debug, Serialize)]
pub struct Response<'a> {
    #[serde(rename = "Say")]
    pub say: Option<Say<'a>>,
    #[serde(rename = "Play")]
    pub play: Option<Play<'a>>,
}

#[derive(Debug, Serialize)]
pub struct Say<'a> {
    #[serde(rename = "$value")]
    pub body: &'a str,
}

#[derive(Debug, Serialize)]
pub struct Play<'a> {
    #[serde(rename = "$value")]
    pub body: &'a str,
}

impl<'a> Response<'a> {
    pub fn new() -> Self {
        Response {
            say: None,
            play: None,
        }
    }
    pub fn say(&'a mut self, body: &'a str) {
        self.say = Some(Say { body });
    }
    pub fn play(&'a mut self, body: &'a str) {
        self.play = Some(Play { body });
    }
}
