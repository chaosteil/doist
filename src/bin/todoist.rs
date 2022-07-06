use clap::Parser;
use color_eyre::Result;
use todoist::Args;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    args.exec().await
}
