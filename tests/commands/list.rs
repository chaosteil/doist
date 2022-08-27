use super::setup::Tool;
use assert_cmd::prelude::*;
use color_eyre::Result;
use wiremock::{matchers::*, Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn list() -> Result<()> {
    let mut cmd = Tool::init()?;

    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/rest/v1/tasks"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(super::fixtures::TASKS, "application/json"),
        )
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;

    // Connection failure
    cmd.cmd.arg("list").assert().failure();
    cmd.cmd.assert().failure();
    mock_server.verify().await;

    Ok(())
}
