
use actix_web::{web, HttpServer};
use anyhow::Context;

use crate::router;

use super::{config::AppConfig, startup::AppModule};

pub struct App {
    config: AppConfig
}

impl App {
    pub async fn from_env() -> anyhow::Result<Self> {
        let config = AppConfig::read_env().context("failed to build config")?;

        Ok(App {
            config: config
        }) 
    }
    pub async fn run(&self) -> anyhow::Result<()> {
        
        let app_module = AppModule::from_config(&self.config);
        let result = HttpServer::new(move|| {
            actix_web::App::new()
                .app_data(actix_web::web::Data::new(app_module.clone()))
                .service(
                    web::scope("/user")
                        .service(router::user::get_user))
                .service(
                    web::scope("/product")
                        .service(router::product::get_product)
                )
        }).bind(("0.0.0.0",8080))?
        .run()
        .await;

        result.context("internal server error")
    }
}