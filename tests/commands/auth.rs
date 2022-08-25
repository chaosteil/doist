use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use color_eyre::Result;
use doist::config::Config;
use std::process::Command;

#[test]
fn authentication() -> Result<()> {
    let tmp = assert_fs::TempDir::new()?;
    let mut cmd = Command::cargo_bin("doist")?;
    cmd.arg("auth")
        .arg("AUTH_KEY")
        .env("XDG_CONFIG_HOME", tmp.path())
        .assert();
    let cfg = tmp.child("doist/config.toml");
    cfg.assert(predicates::str::contains("AUTH_KEY"));
    let cfg: Config = toml::from_str(&std::fs::read_to_string(&cfg)?)?;
    assert_eq!(cfg.token.unwrap(), "AUTH_KEY");

    Ok(())
}
