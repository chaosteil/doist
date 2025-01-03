use clap::Parser;
use color_eyre::Result;
use doist::Arguments;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::config::HookBuilder::new()
        .panic_section("consider reporting the bug at https://github.com/chaosteil/doist/issues")
        .display_env_section(false)
        .install()?;
    let args = Arguments::parse();
    args.exec().await
}
