use reqwest::Client;
use secrecy::{ExposeSecret, Secret};

use crate::domains::Chain;

pub struct MoralisClient {
    http_client : Client,
    url: String,
    // chain:String,
    key: Secret<String>
}

impl MoralisClient {
    pub fn new(
        url:String,
        key:Secret<String>,
        // chain:String,
        timeout: std::time::Duration,
    )->Self {

        let http_client = Client::builder().timeout(timeout).build().unwrap();
        Self {
            http_client,
            url,
            // chain,
            key
        }
    }
    pub async fn get_request(&self, request: &str, chain:&Chain) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/{}", self.url, request);
        let response = self.http_client
        .get(format!("{}?chain={}",&url, chain.as_str()))
        .header(
            "X-API-Key",
            self.key.expose_secret(),
        )
        .send()
        .await?
        .error_for_status()?;
        Ok(response)
    }
}