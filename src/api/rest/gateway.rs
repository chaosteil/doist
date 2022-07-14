use color_eyre::{
    eyre::{eyre, WrapErr},
    Result,
};
use reqwest::{Client, RequestBuilder, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

use super::{CreateTask, Project, ProjectID, Task, TaskID, UpdateTask};

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

    pub async fn task(&self, id: TaskID) -> Result<Task> {
        self.get::<(), _>(&format!("rest/v1/tasks/{}", id), None)
            .await
    }

    pub async fn tasks(&self, filter: Option<&str>) -> Result<Vec<Task>> {
        self.get(
            "rest/v1/tasks",
            filter.map(|filter| vec![("filter", filter)]),
        )
        .await
    }

    pub async fn close(&self, id: TaskID) -> Result<()> {
        self.post_empty(
            &format!("rest/v1/tasks/{}/close", id),
            &serde_json::Map::new(),
        )
        .await?;
        Ok(())
    }

    /// Creates a task by calling the Todoist API.
    pub async fn create(&self, task: &CreateTask) -> Result<Task> {
        self.post("rest/v1/tasks", task)
            .await?
            .ok_or_else(|| eyre!("Unable to create task"))
    }

    pub async fn update(&self, id: TaskID, task: &UpdateTask) -> Result<()> {
        self.post_empty(&format!("rest/v1/tasks/{}", id), &task)
            .await?;
        Ok(())
    }

    pub async fn projects(&self) -> Result<Vec<Project>> {
        self.get::<(), _>("rest/v1/projects", None).await
    }

    pub async fn project(&self, id: ProjectID) -> Result<Project> {
        self.get::<(), _>(&format!("rest/v1/project/{}", id), None)
            .await
    }

    async fn get<'a, T: 'a + Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        query: Option<T>,
    ) -> Result<R> {
        let req = self
            .client
            .get(self.url.join(path)?)
            .bearer_auth(&self.token);
        let req = if let Some(q) = query {
            req.query(&q)
        } else {
            req
        };
        handle_req(req)
            .await?
            .ok_or_else(|| eyre!("Invalid response from API"))
    }

    async fn post_empty<T: Serialize>(&self, path: &str, content: &T) -> Result<()> {
        self.post::<_, String>(path, content).await?;
        Ok(())
    }

    async fn post<T: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        content: &T,
    ) -> Result<Option<R>> {
        handle_req(
            self.client
                .post(self.url.join(path)?)
                .bearer_auth(&self.token)
                .body(serde_json::to_string(&content)?)
                .header(reqwest::header::CONTENT_TYPE, "application/json"),
        )
        .await
    }
}

async fn handle_req<R: DeserializeOwned>(req: RequestBuilder) -> Result<Option<R>> {
    // TODO: implement retries/backoffs
    let resp = req.send().await.wrap_err("Unable to send request")?;
    let status = resp.status();
    if status == StatusCode::NO_CONTENT {
        return Ok(None);
    }
    let text = resp.text().await.wrap_err("Unable to read response")?;
    if !status.is_success() {
        return Err(eyre!("Bad response from API: {} - {}", status, text));
    }
    let result = serde_json::from_str(&text).wrap_err("Unable to parse API response")?;
    Ok(Some(result))
}
