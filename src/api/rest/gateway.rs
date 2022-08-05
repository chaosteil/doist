use chrono::Utc;
use color_eyre::{
    eyre::{eyre, WrapErr},
    Result,
};
use lazy_static::lazy_static;
use reqwest::{Client, RequestBuilder, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

use super::{
    Comment, CreateComment, CreateTask, Label, LabelID, Project, ProjectID, Section, SectionID,
    Task, TaskDue, TaskID, UpdateTask,
};

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

    /// Returns the list of all comments attached to the given Project.
    pub async fn project_comments(&self, id: ProjectID) -> Result<Vec<Comment>> {
        self.get("rest/v1/comments", Some(&[("project_id", id)]))
            .await
            .wrap_err("unable to get comments")
    }

    /// Returns the list of all comments attached to the given Task.
    pub async fn task_comments(&self, id: TaskID) -> Result<Vec<Comment>> {
        self.get("rest/v1/comments", Some(&[("task_id", id)]))
            .await
            .wrap_err("unable to get comments")
    }

    /// Creates a comment by calling the API.
    pub async fn create_comment(&self, comment: &CreateComment) -> Result<Comment> {
        self.post("rest/v1/comments", comment)
            .await
            .wrap_err("unable to create comment")?
            .ok_or_else(|| eyre!("Unable to create comment"))
    }

    /// Returns details about a single project.
    ///
    /// * `id` - the ID as used by the Todoist API.
    pub async fn project(&self, id: ProjectID) -> Result<Project> {
        self.get::<(), _>(&format!("rest/v1/projects/{}", id), None)
            .await
            .wrap_err("unable to get project")
    }

    /// Returns details about a single section.
    ///
    /// * `id` - the ID as used by the Todoist API.
    pub async fn section(&self, id: SectionID) -> Result<Section> {
        self.get::<(), _>(&format!("rest/v1/sections/{}", id), None)
            .await
            .wrap_err("unable to get section")
    }

    /// Returns details about a single label.
    ///
    /// * `id` - the ID as used by the Todoist API.
    pub async fn label(&self, id: LabelID) -> Result<Label> {
        self.get::<(), _>(&format!("rest/v1/labels/{}", id), None)
            .await
            .wrap_err("unable to get label")
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

#[cfg(test)]
mod test {
    use wiremock::{
        matchers::{bearer_token, method, path},
        Mock, MockServer, ResponseTemplate,
    };

    use super::*;
    use crate::api::rest::{ProjectID, Task, TaskID};
    use color_eyre::Result;

    #[tokio::test]
    async fn has_authentication() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(bearer_token("hellothere"))
            .and(path("/rest/v1/tasks/123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(create_task(123, 456, "hello")))
            .mount(&mock_server)
            .await;
        let gw = gateway("hellothere", &mock_server);
        let task = gw.task(123).await;
        assert!(task.is_ok());
    }

    #[tokio::test]
    async fn task() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/rest/v1/tasks/123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(create_task(123, 456, "hello")))
            .mount(&mock_server)
            .await;
        let gw = gateway("", &mock_server);
        let task = gw.task(123).await.unwrap();
        mock_server.verify().await;
        assert_eq!(task.id, 123);
        assert!(gw.task(1234).await.is_err());
    }

    #[tokio::test]
    async fn tasks() -> Result<()> {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/rest/v1/tasks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&[
                create_task(123, 456, "hello there"),
                create_task(234, 567, "general kenobi"),
            ]))
            .mount(&mock_server)
            .await;
        let gw = gateway("", &mock_server);
        let tasks = gw.tasks(None).await.unwrap();
        mock_server.verify().await;
        assert_eq!(tasks.len(), 2);
        Ok(())
    }

    #[tokio::test]
    async fn close_task() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/rest/v1/tasks/123/close"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;
        let gw = gateway("", &mock_server);
        let closed = gw.close(123).await;
        assert!(closed.is_ok());
    }

    #[tokio::test]
    async fn complete_task() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/rest/v1/tasks/123"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;
        Mock::given(method("POST"))
            .and(path("/rest/v1/tasks/123/close"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;
        let gw = gateway("", &mock_server);
        let completed = gw.complete(123).await;
        mock_server.verify().await;
        assert!(completed.is_ok());
    }

    #[tokio::test]
    async fn update_task() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/rest/v1/tasks/123"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;
        let gw = gateway("", &mock_server);
        let completed = gw
            .update(
                123,
                &UpdateTask {
                    content: Some("hello".to_string()),
                    ..Default::default()
                },
            )
            .await;
        mock_server.verify().await;
        assert!(completed.is_ok());
    }

    #[tokio::test]
    async fn creates_task() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/rest/v1/tasks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(create_task(123, 456, "hello")))
            .mount(&mock_server)
            .await;
        let gw = gateway("", &mock_server);
        let task = gw
            .create(&CreateTask {
                content: "hello".to_string(),
                project_id: Some(456),
                ..Default::default()
            })
            .await
            .unwrap();
        mock_server.verify().await;
        assert_eq!(task.id, 123);
    }

    #[tokio::test]
    async fn lists_projects() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/rest/v1/projects"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(vec![Project::new(123, "one"), Project::new(456, "two")]),
            )
            .mount(&mock_server)
            .await;
        let gw = gateway("", &mock_server);
        let projects = gw.projects().await.unwrap();
        mock_server.verify().await;
        assert_eq!(projects.len(), 2);
    }

    #[tokio::test]
    async fn show_project() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/rest/v1/projects/123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(Project::new(123, "one")))
            .mount(&mock_server)
            .await;
        let gw = gateway("", &mock_server);
        let projects = gw.project(123).await.unwrap();
        mock_server.verify().await;
        assert_eq!(projects.id, 123);
        assert_eq!(projects.name, "one");
    }

    fn gateway(token: &str, ms: &MockServer) -> Gateway {
        Gateway::new(token, ms.uri().parse().unwrap())
    }

    fn create_task(id: TaskID, project_id: ProjectID, content: &str) -> Task {
        let mut task = crate::api::rest::Task::new(id, content);
        task.project_id = project_id;
        task
    }
}
