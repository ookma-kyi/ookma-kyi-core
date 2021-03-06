use actix_web::{web};
use crate::controllers::account_controller::{login, signup};

pub fn config_account_routes(cfg: &mut web::ServiceConfig) {
    // domain includes: /account
    cfg.service(
        web::scope("/account")
            // path: account/login
            .service(
                web::resource("login")
                    .route(web::get().to(login))
    )
            .service(
                web::resource("signup")
                    .route(web::get().to(signup))
            )
    );
}