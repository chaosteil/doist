use clap::Parser;
use todoist::Args;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    args.exec().await
}
