use super::setup::Tool;
use assert_cmd::prelude::*;
use color_eyre::Result;
use wiremock::{matchers::*, Mock, ResponseTemplate};

#[tokio::test]
async fn list() -> Result<()> {
    struct Test {
        args: Vec<&'static str>,
        fetch_tasks: u64,
        fetch_labels: u64,
        fetch_projects: u64,
        fetch_sections: u64,
    }
    let tests = vec![
        Test {
            args: vec!["list", "--nointeractive"],
            fetch_tasks: 1,
            fetch_labels: 1,
            fetch_projects: 1,
            fetch_sections: 1,
        },
        Test {
            args: vec!["--nointeractive"],
            fetch_tasks: 1,
            fetch_labels: 1,
            fetch_projects: 1,
            fetch_sections: 1,
        },
        Test {
            args: vec!["l", "--nointeractive"],
            fetch_tasks: 1,
            fetch_labels: 1,
            fetch_projects: 1,
            fetch_sections: 1,
        },
    ];
    for test in tests {
        let cmd = Tool::init().await?;

        Mock::given(method("GET"))
            .and(path("/rest/v1/tasks"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(super::fixtures::TASKS, "application/json"),
            )
            .up_to_n_times(test.fetch_tasks)
            .mount(&cmd.mock)
            .await;
        Mock::given(method("GET"))
            .and(path("/rest/v1/labels"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_raw(super::fixtures::LABELS, "application/json"),
            )
            .up_to_n_times(test.fetch_labels)
            .mount(&cmd.mock)
            .await;
        Mock::given(method("GET"))
            .and(path("/rest/v1/projects"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_raw(super::fixtures::PROJECTS, "application/json"),
            )
            .up_to_n_times(test.fetch_projects)
            .mount(&cmd.mock)
            .await;
        Mock::given(method("GET"))
            .and(path("/rest/v1/sections"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_raw(super::fixtures::SECTIONS, "application/json"),
            )
            .up_to_n_times(test.fetch_sections)
            .mount(&cmd.mock)
            .await;

        let mut command = cmd.cmd()?;
        for arg in test.args {
            command.arg(arg);
        }
        command.env("RUST_BACKTRACE", "1").assert().success();
        cmd.mock.verify().await;
    }

    Ok(())
}
