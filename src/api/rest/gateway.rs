use chrono::Utc;
use color_eyre::{
    eyre::{eyre, WrapErr},
    Result,
};
use lazy_static::lazy_static;
use reqwest::{Client, RequestBuilder, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

use super::{CreateTask, Label, Project, ProjectID, Section, Task, TaskDue, TaskID, UpdateTask};

/// Makes network calls to the Todoist API and returns structs that can then be worked with.
pub struct Gateway {
    client: Client,
    token: String,
    url: url::Url,
}

lazy_static! {
    /// The default URL that specifies the endpont to use for the Todoist API.
    pub static ref TODOIST_API_URL: url::Url = {
        url::Url::parse("https://api.todoist.com/").unwrap()
    };
}

impl Gateway {
    /// Create a new [`Gateway`].
    ///
    /// * `token` - the API token used for network calls.
    /// * `url` - the base URL to call. See [`struct@TODOIST_API_URL`]
    pub fn new(token: &str, url: url::Url) -> Gateway {
        Gateway {
            client: Client::new(),
            token: token.to_string(),
            url,
        }
    }

    /// Retuns a [`Task`].
    ///
    /// * `id` - the ID as used by the Todoist API.
    pub async fn task(&self, id: TaskID) -> Result<Task> {
        self.get::<(), _>(&format!("rest/v1/tasks/{}", id), None)
            .await
            .wrap_err("unable to get task")
    }

    /// Returns a list of tasks as given by the API.
    ///
    /// * `filter` - a filter query as described in the [documentation](https://todoist.com/help/articles/205248842).
    pub async fn tasks(&self, filter: Option<&str>) -> Result<Vec<Task>> {
        self.get(
            "rest/v1/tasks",
            filter.map(|filter| vec![("filter", filter)]),
        )
        .await
        .wrap_err("unable to get tasks")
    }

    /// Closes a task.
    ///
    /// Equivalent to pushing the circle in the UI.
    pub async fn close(&self, id: TaskID) -> Result<()> {
        self.post_empty(
            &format!("rest/v1/tasks/{}/close", id),
            &serde_json::Map::new(),
        )
        .await
        .wrap_err("unable to close task")?;
        Ok(())
    }

    /// Complete will complete a task by first updating the due date to today, so if it's
    /// recurring, it will stop doing that.
    /// This is a bit hacky, but the REST API does not support completely closing tasks without
    /// deleting them.
    pub async fn complete(&self, id: TaskID) -> Result<()> {
        self.update(
            id,
            &UpdateTask {
                due: Some(TaskDue::DateTime(Utc::now())),
                ..Default::default()
            },
        )
        .await
        .wrap_err("unable to complete task")?;
        self.close(id).await.wrap_err("unable to complete task")?;
        Ok(())
    }

    /// Creates a task by calling the Todoist API.
    pub async fn create(&self, task: &CreateTask) -> Result<Task> {
        self.post("rest/v1/tasks", task)
            .await
            .wrap_err("unable to create task")?
            .ok_or_else(|| eyre!("Unable to create task"))
    }

    /// Updates a task with the data as specified in UpdateTask.
    pub async fn update(&self, id: TaskID, task: &UpdateTask) -> Result<()> {
        self.post_empty(&format!("rest/v1/tasks/{}", id), &task)
            .await
            .wrap_err("unable to update task")?;
        Ok(())
    }

    /// Returns the list of Projects.
    pub async fn projects(&self) -> Result<Vec<Project>> {
        self.get::<(), _>("rest/v1/projects", None)
            .await
            .wrap_err("unable to get projects")
    }

    /// Returns the list of all Sections.
    pub async fn sections(&self) -> Result<Vec<Section>> {
        self.get::<(), _>("rest/v1/sections", None)
            .await
            .wrap_err("unable to get sections")
    }

    /// Returns the list of all Labels.
    pub async fn labels(&self) -> Result<Vec<Label>> {
        self.get::<(), _>("rest/v1/labels", None)
            .await
            .wrap_err("unable to get labels")
    }

    /// Returns details about a single project.
    ///
    /// * `id` - the ID as used by the Todoist API.
    pub async fn _project(&self, id: ProjectID) -> Result<Project> {
        self.get::<(), _>(&format!("rest/v1/project/{}", id), None)
            .await
            .wrap_err("unable to get project")
    }

    /// Makes a GET request to the Todoist API with an optional query.
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

    /// Sends a POST request to the Todoist API with the given content.
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

    /// Same as [`Gateway::post`], but doesn't require content to be set for the POST request.
    async fn post_empty<T: Serialize>(&self, path: &str, content: &T) -> Result<()> {
        self.post::<_, String>(path, content).await?;
        Ok(())
    }
}

/// Does the actual call to the Todoist API and handles error handling.
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
