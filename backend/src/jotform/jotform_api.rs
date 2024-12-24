use super::raw_submission::RawSubmission;
use crate::models::Jotform;
use serde::Deserialize;

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

    pub async fn get_submissions(&self) -> Result<Vec<Jotform>, reqwest::Error> {
        let url = format!(
            "{}/form/{}/submissions?apiKey={}&limit=25&orderby=created_at",
            self.base_url, self.form_id, self.api_key
        );

        let response = self.client.get(&url).send().await?;
        let response_body = response.json::<JotFormApiResponse>().await?;
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
    pub response_code: u16,
    pub message: String,
    pub content: Vec<RawSubmission>,
}
