use std::env;
use actix_files as fs;
use actix_web::{App, HttpServer};
use tera::{Tera};
use ookma_kyi::routes::home_routes::config_home_routes;
use ookma_kyi::routes::utility_routes::config_utility_routes;
use ookma_kyi::routes::account_routes::config_account_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get the port number to listen on.
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");
    println!("Starting server on port {}", port);
    HttpServer::new(|| {
        let tera = Tera::new(
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/views/**/*")
        ).unwrap();
        App::new()
            .data(tera)
            // static files
            .service(fs::Files::new("/static", "src/public"))
            // routes
            .configure(config_account_routes)
            .configure(config_utility_routes)
            .configure(config_home_routes) // this must come last
    })
        .bind(("127.0.0.1", port))?
        .run()
        .await
}