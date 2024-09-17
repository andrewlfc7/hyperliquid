use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::api::consts::BASE_URL;



pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: BASE_URL.to_string(),
        }
    }

    #[allow(dead_code)]
    pub async fn get<T>(&self, endpoint: &str) -> Result<T, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client
            .get(&url)
            .send()
            .await?;

        let result = response.json::<T>().await?;
        Ok(result)
    }

    #[allow(dead_code)]
    pub async fn post<T, B>(&self, endpoint: &str, body: B) -> Result<T, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let result = response.json::<T>().await?;
        Ok(result)
    }

    #[allow(dead_code)]
    pub async fn put<T, B>(&self, endpoint: &str, body: B) -> Result<T, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client
            .put(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let result = response.json::<T>().await?;
        Ok(result)
    }

    #[allow(dead_code)]
    pub async fn delete<T>(&self, endpoint: &str) -> Result<T, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client
            .delete(&url)
            .send()
            .await?;

        let result = response.json::<T>().await?;
        Ok(result)
    }
}
