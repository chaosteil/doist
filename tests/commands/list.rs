use super::setup::Tool;
use assert_cmd::prelude::*;
use color_eyre::Result;

#[test]
fn list() -> Result<()> {
    let mut cmd = Tool::init()?;

    // Connection failure
    cmd.cmd.arg("list").assert().failure();
    cmd.cmd.assert().failure();

    Ok(())
}
