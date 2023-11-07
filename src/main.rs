use std::sync::Arc;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use diesel::r2d2::event;
use reqwest;
// use scheduler::scheduler::start_scheduler;
use serde::Serialize;

use crate::currency::application::{
    service::currency_service::CurrencyService,
    validator::create_currency_validator::CurrencyCreateValidator,
};

mod currency;
mod rates;
mod shared;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> HttpResponse {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    HttpResponse::NotFound().json(response)
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     env_logger::init();

//     let repositories = Repositories::new();
//     let app_data: web::Data<_> = web::Data::new(repositories);

//     start_scheduler().await;

//     HttpServer::new(move || {
//         App::new()
//             .app_data(app_data.clone())
//             // .configure(api::api::config)
//             .service(healthcheck)
//             .default_service(web::route().to(not_found))
//             .wrap(actix_web::middleware::Logger::default())
//     })
//     .bind(("127.0.0.1", 7894))?
//     .run()
//     .await
// }

#[actix_web::main]
async fn main() -> Result<(), reqwest::Error> {
    let currency_service = CurrencyService::new();

    let currency_validator = CurrencyCreateValidator {
        name: "Rubl".to_string(),
        iso: "NDZ".to_string(),
    };
    let _ = currency_service.create(currency_validator).await;
    Ok(())
}
