use reqwest::Client;
use serde::de::DeserializeOwned;
use crate::api::client_struct::CandleRequest;

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn request<T>(&self, endpoint: &str, candle_request: &CandleRequest) -> Result<T, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(candle_request)
            .send()
            .await?;

        let result = response.json::<T>().await?;
        Ok(result)
    }
}
