use color_eyre::{
    eyre::{eyre, WrapErr},
    Result,
};
use reqwest::{Client, RequestBuilder, StatusCode};

use super::{CreateTask, Task, TaskID};

pub struct Gateway {
    client: Client,
    token: String,
    url: url::Url,
}

impl Gateway {
    pub fn new(token: &str, url: url::Url) -> Gateway {
        Gateway {
            client: Client::new(),
            token: token.to_string(),
            url,
        }
    }

    pub async fn tasks(&self, filter: Option<&str>) -> Result<Vec<Task>> {
        let mut req = self.prepare_get("rest/v1/tasks")?;
        if let Some(filter) = filter {
            req = req.query(&[("filter", filter)]);
        }
        let data = req
            .send()
            .await
            .wrap_err("Unable to send request to show task list")?
            .text()
            .await
            .wrap_err("Unable to read response")?;
        serde_json::from_str(&data).wrap_err("Unable to parse API response")
    }

    pub async fn close(&self, id: TaskID) -> Result<()> {
        let req = self.prepare_post(&format!("rest/v1/tasks/{}/close", id))?;
        let status = req
            .send()
            .await
            .wrap_err("Unable to send request to close task")?
            .status();
        if status != StatusCode::NO_CONTENT {
            return Err(eyre!("Bad response from API: {}", status));
        }
        Ok(())
    }

    /// Creates a task by calling the Todoist API.
    pub async fn create(&self, task: &CreateTask) -> Result<Task> {
        // TODO: implement retries/backoffs
        let req = self.prepare_post("rest/v1/tasks")?;
        let resp = req
            .body(serde_json::to_string(&task)?)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await
            .wrap_err("Unable to send request to create task")?;
        let status = resp.status();
        let text = resp.text().await.wrap_err("Unable to read response")?;
        if status != StatusCode::OK {
            return Err(eyre!("Bad response from API: {} - {}", status, text));
        }
        serde_json::from_str(&text).wrap_err("Unable to parse API response")
    }

    fn prepare_get(&self, path: &str) -> Result<RequestBuilder> {
        Ok(self
            .client
            .get(self.url.join(path)?)
            .bearer_auth(&self.token))
    }

    fn prepare_post(&self, path: &str) -> Result<RequestBuilder> {
        Ok(self
            .client
            .post(self.url.join(path)?)
            .bearer_auth(&self.token))
    }
}
