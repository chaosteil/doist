use wiremock::{matchers, Mock, ResponseTemplate};

use crate::setup::Tool;

pub async fn mock_tasks(tool: &Tool, times: u64) {
    mock_http(
        tool,
        "GET",
        "/rest/v1/tasks",
        200,
        super::fixtures::TASKS,
        times,
    )
    .await
}

pub async fn mock_tasks_partial(tool: &Tool, times: u64) {
    mock_http(
        tool,
        "GET",
        "/rest/v1/tasks",
        200,
        super::fixtures::TASKS_PARTIAL,
        times,
    )
    .await
}

pub async fn mock_labels(tool: &Tool, times: u64) {
    mock_http(
        tool,
        "GET",
        "/rest/v1/labels",
        200,
        super::fixtures::LABELS,
        times,
    )
    .await
}

pub async fn mock_projects(tool: &Tool, times: u64) {
    mock_http(
        tool,
        "GET",
        "/rest/v1/projects",
        200,
        super::fixtures::PROJECTS,
        times,
    )
    .await
}

pub async fn mock_sections(tool: &Tool, times: u64) {
    mock_http(
        tool,
        "GET",
        "/rest/v1/sections",
        200,
        super::fixtures::SECTIONS,
        times,
    )
    .await
}

async fn mock_http(tool: &Tool, method: &str, path: &str, code: u16, body: &str, times: u64) {
    Mock::given(matchers::method(method))
        .and(matchers::path(path))
        .respond_with(ResponseTemplate::new(code).set_body_raw(body, "application/json"))
        .up_to_n_times(times)
        .mount(&tool.mock)
        .await
}
