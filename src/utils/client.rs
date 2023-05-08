use reqwest::{ Client, Error, Response };

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get(&self, url: &str) -> Result<Response, Error> {
        let response = self.client
            .get(url)
            .send()
            .await?;
        Ok(response)
    }

    pub async fn post(&self, url: &str, body: &str) -> Result<Response, Error> {
        let response = self.client
            .post(url)
            .body(body.to_string())
            .send()
            .await?;
        Ok(response)
    }
}