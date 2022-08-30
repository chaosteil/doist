use assert_cmd::prelude::*;
use color_eyre::Result;
use doist::config::Config;
use std::{env, process::Command};
use wiremock::MockServer;

pub struct Tool {
    pub cmd: Command,
    pub tmp: assert_fs::TempDir,
    pub cfg: Config,
    pub mock: MockServer,

    old_home: String,
}

impl Drop for Tool {
    fn drop(&mut self) {
        env::set_var("XDG_CONFIG_HOME", &self.old_home)
    }
}

impl Tool {
    pub async fn init() -> Result<Tool> {
        let tmp = assert_fs::TempDir::new()?;
        let mut cmd = Command::cargo_bin("doist")?;
        cmd.arg("auth")
            .arg("AUTH_KEY")
            .env("XDG_CONFIG_HOME", tmp.path())
            .assert()
            .success();
        let old_home = env::var("XDG_CONFIG_HOME").unwrap_or("".to_string());
        env::set_var("XDG_CONFIG_HOME", tmp.path());
        let mock = MockServer::start().await;
        let mut cfg = Config::load()?;
        cfg.url = url::Url::parse(&mock.uri())?;
        cfg.save()?;
        let cmd = Command::cargo_bin("doist")?;
        Ok(Tool {
            cmd,
            tmp,
            cfg,
            mock,
            old_home,
        })
    }
}
