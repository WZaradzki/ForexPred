use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use repository::base::Repositories;
use scheduler::scheduler::start_scheduler;
use serde::Serialize;

// mod api;
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let repositories = Repositories::new();
    let app_data: web::Data<_> = web::Data::new(repositories);

    start_scheduler().await;

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            // .configure(api::api::config)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("127.0.0.1", 7894))?
    .run()
    .await
}
