use second::application::app;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let app = app::App::from_env().await?;
    
    app.run().await?;

    Ok(())
}
