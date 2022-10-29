use super::mocks;
use super::setup::Tool;
use assert_cmd::prelude::*;
use color_eyre::Result;
use predicates::prelude::*;

#[tokio::test]
async fn list() -> Result<()> {
    for test in &[
        vec!["projects", "list"],
        vec!["p", "list"],
        vec!["projects"],
        vec!["p"],
    ] {
        let cmd = Tool::init().await?;

        mocks::mock_projects(&cmd, 1).await;

        let mut command = cmd.cmd()?;
        for arg in test {
            command.arg(arg);
        }
        command
            .env("RUST_BACKTRACE", "1")
            .assert()
            .success()
            .stdout(predicate::eq(super::fixtures::PROJECTS_OUTPUT));
        cmd.mock.verify().await;
    }

    Ok(())
}
