use tera::{Context};
use actix_web::{error, Error, HttpResponse, web};

pub async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    ctx.insert("title", "Home Page");
    let body = tmpl.render("index.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}