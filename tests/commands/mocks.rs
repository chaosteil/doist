use wiremock::{Mock, MockBuilder, ResponseTemplate, matchers};

use crate::setup::Tool;

pub async fn mock_tasks(tool: &Tool, times: u64) {
    mock_http(
        tool,
        "GET",
        "/rest/v2/tasks",
        200,
        super::fixtures::TASKS,
        times,
    )
    .await
}

pub async fn mock_tasks_all(tool: &Tool, times: u64) {
    mock_http_with_builder(
        tool,
        "GET",
        "/rest/v2/tasks",
        200,
        super::fixtures::TASKS,
        times,
        |mb| mb.and(matchers::query_param("filter", "all")),
    )
    .await
}

pub async fn mock_tasks_partial(tool: &Tool, times: u64) {
    mock_http(
        tool,
        "GET",
        "/rest/v2/tasks",
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
        "/rest/v2/labels",
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
        "/rest/v2/projects",
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
        "/rest/v2/sections",
        200,
        super::fixtures::SECTIONS,
        times,
    )
    .await
}

async fn mock_http_with_builder<F: Fn(MockBuilder) -> MockBuilder>(
    tool: &Tool,
    method: &str,
    path: &str,
    code: u16,
    body: &str,
    times: u64,
    matchers: F,
) {
    let mut mb = Mock::given(matchers::method(method)).and(matchers::path(path));
    mb = matchers(mb);
    mb.respond_with(ResponseTemplate::new(code).set_body_raw(body, "application/json"))
        .up_to_n_times(times)
        .mount(&tool.mock)
        .await
}

async fn mock_http(tool: &Tool, method: &str, path: &str, code: u16, body: &str, times: u64) {
    mock_http_with_builder(tool, method, path, code, body, times, |mb| mb).await
}
