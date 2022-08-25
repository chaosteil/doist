use assert_cmd::prelude::*;
use color_eyre::Result;
use std::process::Command;

pub struct Tool {
    pub cmd: Command,
    pub tmp: assert_fs::TempDir,
}

impl Tool {
    pub fn init() -> Result<Tool> {
        let tmp = assert_fs::TempDir::new()?;
        let mut cmd = Command::cargo_bin("doist")?;
        cmd.arg("auth")
            .arg("AUTH_KEY")
            .env("XDG_CONFIG_HOME", tmp.path())
            .assert();
        Ok(Tool { cmd, tmp })
    }
}
