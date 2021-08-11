use actix_web::{App, HttpServer};
use tera::{Tera};
use ookma_kyi::routes::home_routes::config_home_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new(
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/views/**/*")
        ).unwrap();
        App::new()
            .data(tera)
            .configure(config_home_routes)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}