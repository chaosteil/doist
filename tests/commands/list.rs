use super::mocks;
use super::setup::Tool;
use assert_cmd::prelude::*;
use color_eyre::Result;
use predicates::prelude::*;

#[tokio::test]
async fn list() -> Result<()> {
    for test in &[
        vec!["list", "--nointeractive"],
        vec!["l", "--nointeractive"],
        vec!["--nointeractive"],
    ] {
        let cmd = Tool::init().await?;

        mocks::mock_tasks(&cmd, 1).await;
        mocks::mock_labels(&cmd, 1).await;
        mocks::mock_projects(&cmd, 1).await;
        mocks::mock_sections(&cmd, 1).await;

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

    mocks::mock_tasks(&cmd, 1).await;
    mocks::mock_tasks_partial(&cmd, 1).await;
    mocks::mock_labels(&cmd, 1).await;
    mocks::mock_projects(&cmd, 1).await;
    mocks::mock_sections(&cmd, 1).await;

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
