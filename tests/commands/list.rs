use super::setup::Tool;
use assert_cmd::prelude::*;
use color_eyre::Result;
use predicates::prelude::*;
use wiremock::{matchers::*, Mock, ResponseTemplate};

#[tokio::test]
async fn list() -> Result<()> {
    for test in &[
        vec!["list", "--nointeractive"],
        vec!["l", "--nointeractive"],
        vec!["--nointeractive"],
    ] {
        let cmd = Tool::init().await?;

        Mock::given(method("GET"))
            .and(path("/rest/v1/tasks"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(super::fixtures::TASKS, "application/json"),
            )
            .up_to_n_times(1)
            .mount(&cmd.mock)
            .await;
        Mock::given(method("GET"))
            .and(path("/rest/v1/labels"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_raw(super::fixtures::LABELS, "application/json"),
            )
            .up_to_n_times(1)
            .mount(&cmd.mock)
            .await;
        Mock::given(method("GET"))
            .and(path("/rest/v1/projects"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_raw(super::fixtures::PROJECTS, "application/json"),
            )
            .up_to_n_times(1)
            .mount(&cmd.mock)
            .await;
        Mock::given(method("GET"))
            .and(path("/rest/v1/sections"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_raw(super::fixtures::SECTIONS, "application/json"),
            )
            .up_to_n_times(1)
            .mount(&cmd.mock)
            .await;

        let mut command = cmd.cmd()?;
        for arg in test {
            command.arg(arg);
        }
        command
            .env("RUST_BACKTRACE", "1")
            .assert()
            .success()
            .stdout(predicate::eq(super::fixtures::TASK_OUTPUT));
        cmd.mock.verify().await;
    }

    Ok(())
}

#[tokio::test]
async fn expand() -> Result<()> {
    let cmd = Tool::init().await?;

    Mock::given(method("GET"))
        .and(path("/rest/v1/tasks"))
        .and(query_param("filter", "all"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(super::fixtures::TASKS, "application/json"),
        )
        .up_to_n_times(1)
        .mount(&cmd.mock)
        .await;
    Mock::given(method("GET"))
        .and(path("/rest/v1/tasks"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_raw(super::fixtures::TASKS_PARTIAL, "application/json"),
        )
        .up_to_n_times(1)
        .mount(&cmd.mock)
        .await;
    Mock::given(method("GET"))
        .and(path("/rest/v1/labels"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(super::fixtures::LABELS, "application/json"),
        )
        .up_to_n_times(1)
        .mount(&cmd.mock)
        .await;
    Mock::given(method("GET"))
        .and(path("/rest/v1/projects"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(super::fixtures::PROJECTS, "application/json"),
        )
        .up_to_n_times(1)
        .mount(&cmd.mock)
        .await;
    Mock::given(method("GET"))
        .and(path("/rest/v1/sections"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(super::fixtures::SECTIONS, "application/json"),
        )
        .up_to_n_times(1)
        .mount(&cmd.mock)
        .await;

    let mut command = cmd.cmd()?;
    command
        .arg("-e")
        .arg("--nointeractive")
        .env("RUST_BACKTRACE", "1")
        .assert()
        .success()
        .stdout(predicate::eq(super::fixtures::TASK_EXPAND_OUTPUT));
    cmd.mock.verify().await;

    Ok(())
}
