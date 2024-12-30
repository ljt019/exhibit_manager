use super::raw_submission::RawSubmission;
use crate::models::Jotform;
use log::info;
use rocket::async_trait;
use serde::Deserialize;

#[async_trait]
pub trait JotformApiTrait {
    async fn get_submissions(&self) -> Result<Vec<Jotform>, Box<dyn std::error::Error>>;
}

#[derive(Debug, Clone)]
pub struct JotformApi {
    api_key: String,
    form_id: String,
    base_url: String,
    client: reqwest::Client,
}

impl JotformApi {
    pub fn new(api_key: String, form_id: String, base_url: String) -> Self {
        let client = reqwest::Client::new();

        Self {
            api_key,
            form_id,
            base_url,
            client,
        }
    }
}

#[async_trait]
impl JotformApiTrait for JotformApi {
    async fn get_submissions(&self) -> Result<Vec<Jotform>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/form/{}/submissions?apiKey={}&limit=25",
            self.base_url, self.form_id, self.api_key
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| -> Box<dyn std::error::Error> { Box::new(e) })?;

        let response_body = response
            .json::<JotFormApiResponse>()
            .await
            .map_err(|e| -> Box<dyn std::error::Error> { Box::new(e) })?;

        let limit_left = response_body.limit_left;
        info!("JotForm API rate limit left: {}", limit_left);

        Ok(response_body
            .content
            .into_iter()
            .map(|raw| raw.to_jotform())
            .collect())
    }
}

#[derive(Debug, Deserialize)]
pub struct JotFormApiResponse {
    #[serde(rename = "responseCode")]
    #[allow(dead_code)]
    pub response_code: u16,
    #[allow(dead_code)]
    pub message: String,
    pub content: Vec<RawSubmission>,
    #[serde(rename = "limit-left")]
    pub limit_left: u32,
}
