use super::setup::Tool;
use assert_cmd::prelude::*;
use color_eyre::Result;
use wiremock::{matchers::*, Mock, ResponseTemplate};

#[tokio::test]
async fn list() -> Result<()> {
    let mut cmd = Tool::init().await?;

    Mock::given(method("GET"))
        .and(path("/rest/v1/tasks"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(super::fixtures::TASKS, "application/json"),
        )
        .mount(&cmd.mock)
        .await;
    Mock::given(method("GET"))
        .and(path("/rest/v1/labels"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(super::fixtures::LABELS, "application/json"),
        )
        .mount(&cmd.mock)
        .await;
    Mock::given(method("GET"))
        .and(path("/rest/v1/projects"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(super::fixtures::PROJECTS, "application/json"),
        )
        .mount(&cmd.mock)
        .await;
    Mock::given(method("GET"))
        .and(path("/rest/v1/sections"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(super::fixtures::SECTIONS, "application/json"),
        )
        .mount(&cmd.mock)
        .await;

    cmd.cmd.arg("list").assert().success();
    cmd.mock.verify().await;

    Ok(())
}
