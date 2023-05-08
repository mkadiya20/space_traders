use crate::models::base::Base;
use crate::utils::client::HttpClient;

pub async fn get_base_data(client: &HttpClient) -> Result<Base, Box<dyn std::error::Error>> {
    let url: &str = "https://api.spacetraders.io/v2";
    let response = client.get(url).await?;
    let body: Base = serde_json::from_str(&response.text().await?)?;
    Ok(body)
}