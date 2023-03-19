use assert_cmd::prelude::*;
use color_eyre::Result;
use doist::config::Config;
use std::process::Command;
use wiremock::MockServer;

pub struct Tool {
    pub tmp: assert_fs::TempDir,
    pub cfg: Config,
    pub mock: MockServer,
}

impl Tool {
    pub async fn init() -> Result<Tool> {
        let tmp = assert_fs::TempDir::new()?;
        let mut cmd = Command::cargo_bin("doist")?;
        cmd.env("RUST_BACKTRACE", "1")
            .arg(format!("--config_prefix={}", tmp.path().display()))
            .arg("auth")
            .arg("AUTH_KEY")
            .assert()
            .success();

        let mock = MockServer::start().await;
        let mut cfg = Config::load_prefix(&tmp.path())?;
        cfg.url = Some(url::Url::parse(&mock.uri())?);
        cfg.override_time = Some(super::fixtures::FETCH_TIME.trim().parse()?);
        cfg.save()?;
        Ok(Tool { tmp, cfg, mock })
    }

    pub fn cmd(&self) -> Result<Command> {
        let mut cmd = Command::cargo_bin("doist")?;
        cmd.env("RUST_BACKTRACE", "1")
            .arg(format!("--config_prefix={}", self.tmp.path().display()));
        Ok(cmd)
    }
}
