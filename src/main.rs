use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use repository::base::Repositories;
use reqwest;
use scheduler::scheduler::start_scheduler;
use serde::Serialize;

use crate::domains::data::config::Currencies;

// mod api;
mod domains;
mod models;
mod repository;
mod scheduler;

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
    let exchange_rate_api = domains::data::exchange_rate::ExchangeRateApi::new();
    let usd = Currencies::USD;

    let response = exchange_rate_api.get_latest_by_currency(usd.get_iso_4217()).await?;

    dbg!(response);

    Ok(())
}
