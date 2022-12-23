use std::collections::HashMap;

use reqwest::Client;
use secrecy::{ExposeSecret, Secret};

pub struct MoralisClient {
    http_client : Client,
    url: String,
    chain:String,
    key: Secret<String>
}

impl MoralisClient {
    pub fn new(
        url:String,
        key:Secret<String>,
        chain:String,
        timeout: std::time::Duration,
    )->Self {

        let http_client = Client::builder().timeout(timeout).build().unwrap();
        Self {
            http_client,
            url,
            chain,
            key
        }
    }
    pub async fn get_request(&self, request: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}/{}", self.url, request);
        let mut params = HashMap::new();
        params.insert("chain", &self.chain);  
        let response = self.http_client
        .get(&url)
        .header(
            "X-API-Key",
            self.key.expose_secret(),
        )
        .send()
        .await?;
        let body = response.text().await?;
        Ok(body)
    }
}