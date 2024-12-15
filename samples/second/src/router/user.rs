use actix_web::{get, http::StatusCode, web, HttpResponse, Responder};

use crate::application::startup::AppModule;

#[get("/")]
pub async fn get_user(app_module: web::Data<AppModule>) -> impl Responder {
    let Ok(user) = app_module.user_service().get_user(String::from("hogehoge")).await else {
        return HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR).set_body(format!("not found user")).map_into_boxed_body()
    };

    HttpResponse::Ok().body(format!("user: {:?}",user))
}