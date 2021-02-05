use anyhow::Result;

// change format to json to get metadata
pub struct Joke {
    pub text: String,
}
pub struct YodaAPIClient {
    url: String,
    format: String,
}

impl YodaAPIClient {
    pub fn default(url: String, format: String) -> Self {
        Self { url, format }
    }

    pub fn make_request(&self) -> Result<Joke, anyhow::Error> {
        let url = format!("{}?text={}", self.url, self.format);
        println!("\n Url is {}", url);
        let joke = Joke {
            text: reqwest::blocking::get(&url)?.text()?,
        };
        Ok(joke)
    }
}
