use actix_web::{web};
use crate::controllers::utility_controller::robotstxt_file;

pub fn config_utility_routes(cfg: &mut web::ServiceConfig) {
    // path: /robots.txt
    cfg.service(
                web::resource("robots.txt")
                    .route(web::get().to(robotstxt_file))
            );
}