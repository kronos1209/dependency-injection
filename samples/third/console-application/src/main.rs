use core::clone::Clone;

use anyhow::Context as _;
use console_application::app::{config::AppConfig, sample::SampleApplication};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let console = SampleApplication::new(AppConfig::from_env().context("failed to read env")?);

    if let Err(e) = console.execute().await {
        println!("err: {}", e);
    };
    Ok(())
}

#[derive(clap::Parser,Debug)]
struct MainCommand {
    #[command(subcommand)]
    sub: SubCommand
}

#[derive(clap::Subcommand,Debug)]
enum SubCommand {
    Sub1,
    Sub2,
    Sub3,
}