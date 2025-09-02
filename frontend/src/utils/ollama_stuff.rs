use reqwest::Client;

#[derive(Clone)]
pub struct OllamaClient {
    url: String,
    client: Client
}

impl OllamaClient {
    pub fn new(client: Client, url: String) -> Self {
        Self {
            url,
            client
        }
    }
}