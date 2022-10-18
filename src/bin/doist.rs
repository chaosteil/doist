use clap::Parser;
use color_eyre::Result;
use doist::Arguments;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Arguments::parse();
    args.exec().await
}
