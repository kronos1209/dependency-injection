use actix_web::{get, http::StatusCode, web, HttpRequest, HttpResponse, Responder};

use crate::{application::startup::AppModule, infrastructure::adapter::http_user_context_adapter};

#[get("/")]
pub async fn get_product(req: HttpRequest,app_mudule: web::Data<AppModule>) -> impl Responder {
    let user_context = http_user_context_adapter::HttpUserContextAdapter::new(req.headers().clone());

    let Ok(product)= app_mudule.product_service().get_product("hogehoge".to_string(), &user_context).await else {
        return HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR).set_body(format!("not found product")).map_into_boxed_body()
    };

    HttpResponse::Ok().body(format!("{:?}",product))
}