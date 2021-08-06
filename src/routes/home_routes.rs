use actix_web::{web};
use crate::controllers::home_controller::index;

pub fn config_home_routes(cfg: &mut web::ServiceConfig) {
    // domain includes: /
    cfg.service(
        web::scope("/")
            .service(
                web::resource("")
                    .route(web::get().to(index))
            ));
}