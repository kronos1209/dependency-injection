use anyhow::Context;
use console_application::app::{config::AppConfig, sample::SampleApplication};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut console = SampleApplication::new(AppConfig::from_env().context("failed to read env")?);

    if let Err(e) = console.execute().await {
        println!("err: {}", e);
    };

    Ok(())
}
