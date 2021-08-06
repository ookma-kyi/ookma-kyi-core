use actix_web::Responder;

pub async fn index() -> impl Responder {
    format!("Hello World!")
}