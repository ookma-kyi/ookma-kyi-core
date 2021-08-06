use actix_web::{App, HttpServer};
use ookma_kyi::routes::home_routes::config_home_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config_home_routes)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}