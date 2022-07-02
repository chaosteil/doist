use color_eyre::eyre::bail;
use reqwest::StatusCode;

use super::{Task, TaskID};

pub struct Gateway {
    client: reqwest::Client,
    token: String,
    url: url::Url,
}

impl Gateway {
    pub fn new(token: &str, url: url::Url) -> Gateway {
        Gateway {
            client: reqwest::Client::new(),
            token: token.to_string(),
            url,
        }
    }

    pub async fn tasks(&self, filter: Option<&str>) -> color_eyre::Result<Vec<Task>> {
        let mut req = self.prepare_get("rest/v1/tasks")?;
        if let Some(filter) = filter {
            req = req.query(&[("filter", filter)]);
        }
        let data = req.send().await?.text().await?;
        Ok(serde_json::from_str(&data)?)
    }

    pub async fn close(&self, id: TaskID) -> color_eyre::Result<()> {
        let req = self.prepare_post(&format!("rest/v1/tasks/{}/close", id))?;
        let status = req.send().await?.status();
        if status != StatusCode::NO_CONTENT {
            bail!("Bad response from API: {}", status);
        }
        Ok(())
    }

    fn prepare_get(&self, path: &str) -> color_eyre::Result<reqwest::RequestBuilder> {
        Ok(self
            .client
            .get(self.url.join(path)?)
            .bearer_auth(&self.token))
    }

    fn prepare_post(&self, path: &str) -> color_eyre::Result<reqwest::RequestBuilder> {
        Ok(self
            .client
            .post(self.url.join(path)?)
            .bearer_auth(&self.token))
    }
}
